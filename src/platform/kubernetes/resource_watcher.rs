use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::{Duration, SystemTime},
};

use enum_dispatch::enum_dispatch;
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::{
    api::{
        apps::v1::{DaemonSet, Deployment, ReplicaSet, StatefulSet},
        core::v1::{Namespace, Node, Pod, ReplicationController, Service},
        extensions, networking,
    },
    apimachinery::pkg::apis::meta::v1::ObjectMeta,
    Metadata,
};
use kube::{
    api::ListParams,
    runtime::{self, watcher::Event},
    Api, Client, Resource,
};
use log::{debug, info, warn};
use openshift_openapi::api::route::v1::Route;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use tokio::{
    runtime::Handle,
    task::JoinHandle,
    time::{self, Instant},
};

use crate::proto::trident::KubernetesApiInfo;

const LIST_INTERVAL: Duration = Duration::from_secs(600);
const REFRESH_INTERVAL: Duration = Duration::from_secs(3600);

#[enum_dispatch]
pub trait Watcher {
    fn start(&self) -> Option<JoinHandle<()>>;
    fn error(&self) -> Option<String>;
    fn entries(&self) -> Vec<String>;
    fn pb_entries(&self) -> Vec<KubernetesApiInfo>;
    fn version(&self) -> u64;
}

#[enum_dispatch(Watcher)]
#[derive(Clone)]
pub enum GenericResourceWatcher {
    Node(ResourceWatcher<Node>),
    Namespace(ResourceWatcher<Namespace>),
    Service(ResourceWatcher<Service>),
    Deployment(ResourceWatcher<Deployment>),
    Pod(ResourceWatcher<Pod>),
    StatefulSet(ResourceWatcher<StatefulSet>),
    DaemonSet(ResourceWatcher<DaemonSet>),
    ReplicationController(ResourceWatcher<ReplicationController>),
    ReplicaSet(ResourceWatcher<ReplicaSet>),
    V1Ingress(ResourceWatcher<networking::v1::Ingress>),
    V1beta1Ingress(ResourceWatcher<networking::v1beta1::Ingress>),
    ExtV1beta1Ingress(ResourceWatcher<extensions::v1beta1::Ingress>),
    Route(ResourceWatcher<Route>),
}

// 发生错误，需要重新构造实例
#[derive(Clone)]
pub struct ResourceWatcher<K> {
    api: Api<K>,
    entries: Arc<Mutex<HashMap<String, String>>>,
    err_msg: Arc<Mutex<Option<String>>>,
    kind: &'static str,
    running: Arc<Mutex<bool>>,
    version: Arc<AtomicU64>,
    runtime: Handle,
}

impl<K> Watcher for ResourceWatcher<K>
where
    K: Clone + Debug + Send + DeserializeOwned + Resource + Serialize + 'static,
    K: Metadata<Ty = ObjectMeta>,
{
    fn start(&self) -> Option<JoinHandle<()>> {
        let mut running_guard = self.running.lock().unwrap();
        if *running_guard {
            debug!("{} watcher has already running", self.kind);
            return None;
        }
        *running_guard = true;
        drop(running_guard);

        let entries = self.entries.clone();
        let version = self.version.clone();
        let kind = self.kind;
        let err_msg = self.err_msg.clone();

        let api = self.api.clone();

        let handle = self
            .runtime
            .spawn(Self::process(entries, version, api, kind, err_msg));

        info!("{} watcher started", self.kind);
        Some(handle)
    }

    fn version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }

    fn error(&self) -> Option<String> {
        self.err_msg.lock().unwrap().take()
    }

    fn pb_entries(&self) -> Vec<KubernetesApiInfo> {
        self.entries
            .lock()
            .unwrap()
            .values()
            .map(|entry| KubernetesApiInfo {
                r#type: Some(self.kind.to_string()),
                compressed_info: Some(Vec::from(entry.as_bytes())),
                info: None,
            })
            .collect::<Vec<_>>()
    }

    fn entries(&self) -> Vec<String> {
        self.entries
            .lock()
            .unwrap()
            .values()
            .map(Clone::clone)
            .collect::<Vec<_>>()
    }
}

impl<K> ResourceWatcher<K>
where
    K: Clone + Debug + Send + DeserializeOwned + Resource + Serialize + 'static,
    K: Metadata<Ty = ObjectMeta>,
{
    pub fn new(api: Api<K>, kind: &'static str, runtime: Handle) -> Self {
        Self {
            api,
            entries: Arc::new(Mutex::new(HashMap::new())),
            version: Arc::new(AtomicU64::new(0)),
            kind,
            err_msg: Arc::new(Mutex::new(None)),
            running: Arc::new(Mutex::new(false)),
            runtime,
        }
    }

    async fn process(
        entries: Arc<Mutex<HashMap<String, String>>>,
        version: Arc<AtomicU64>,
        api: Api<K>,
        kind: &'static str,
        err_msg: Arc<Mutex<Option<String>>>,
    ) {
        let ticker = time::sleep(LIST_INTERVAL);
        tokio::pin!(ticker);

        let mut stream = runtime::watcher(api.clone(), ListParams::default()).boxed();

        let mut last_update = SystemTime::now();
        let mut last_refresh = SystemTime::now();

        loop {
            // 当 `select!` 执行的时候， 多个通道有待处理的消息，只有一个通道有一个值弹出。所有其他通道保持不变，
            // 它们的消息保留在这些通道中，直到下一次循环迭代。没有消息丢失。
            // select 和 tokio::spawn 一起用的话，因为tokio runtime 调度 spawn 的task 可能与select 调度是
            // 同时运行在不同操作系统线程select 用于在单个task下多路复用 async futures
            tokio::select! {
                maybe_event = stream.try_next() => {
                    Self::resolve_event(
                        maybe_event,
                        &mut last_update,
                        &entries,
                        &version,
                        kind,
                        &err_msg,
                    );
                }
                _ = &mut ticker => {
                    if last_update.elapsed().unwrap() < LIST_INTERVAL
                        && last_refresh.elapsed().unwrap() < REFRESH_INTERVAL
                    {
                        continue;
                    }

                    last_update = SystemTime::now();
                    last_refresh = SystemTime::now();

                    Self::get_list_entry(&entries, &version, kind, &api, &err_msg).await;
                    ticker.as_mut().reset(Instant::now() + LIST_INTERVAL);
                }
            }
        }
    }

    async fn get_list_entry(
        entries: &Arc<Mutex<HashMap<String, String>>>,
        version: &Arc<AtomicU64>,
        kind: &str,
        api: &Api<K>,
        err_msg: &Arc<Mutex<Option<String>>>,
    ) {
        match api.list(&ListParams::default()).await {
            Ok(object_list) => {
                // 检查内存和List API查询结果是否一致
                let entries_lock = entries.lock().unwrap();
                if object_list.items.len() == entries_lock.len() {
                    let mut identical = true;
                    for object in object_list.items.iter() {
                        match object.meta().uid.as_ref() {
                            Some(uid) if entries_lock.contains_key(uid) => (),
                            _ => {
                                identical = false;
                                break;
                            }
                        }
                    }
                    if identical {
                        return;
                    }
                }
                drop(entries_lock);

                debug!("reload {} data", kind);

                let mut new_entries = HashMap::new();

                for mut object in object_list {
                    if object.meta().uid.as_ref().is_none() {
                        continue;
                    }
                    match serde_json::to_string(&object) {
                        Ok(serialized_object) => {
                            new_entries
                                .insert(object.meta_mut().uid.take().unwrap(), serialized_object);
                        }
                        Err(e) => warn!(
                            "failed serialized resource {} UID({}) to json Err: {}",
                            kind,
                            object.meta().uid.as_ref().unwrap(),
                            e
                        ),
                    }
                }

                if !new_entries.is_empty() {
                    *entries.lock().unwrap() = new_entries;
                    version.fetch_add(1, Ordering::SeqCst);
                }
            }
            Err(err) => {
                let msg = format!("{} watcher list failed: {}", kind, err);
                warn!("{}", msg);
                err_msg.lock().unwrap().replace(msg);
            }
        }
    }

    fn resolve_event(
        maybe_event: Result<Option<Event<K>>, runtime::watcher::Error>,
        last_update: &mut SystemTime,
        entries: &Arc<Mutex<HashMap<String, String>>>,
        version: &Arc<AtomicU64>,
        kind: &'static str,
        err_msg: &Arc<Mutex<Option<String>>>,
    ) {
        match maybe_event {
            Ok(Some(event)) => {
                match event {
                    Event::Applied(object) => {
                        Self::insert_object(object, entries, version, kind);
                    }
                    Event::Deleted(object) => {
                        if let Some(uid) = object.meta().uid.as_ref() {
                            // 只有删除时检查是否需要更新版本号，其余消息直接更新map内容
                            if entries.lock().unwrap().remove(uid).is_some() {
                                version.fetch_add(1, Ordering::SeqCst);
                            }
                        }
                    }
                    // 按照语义重启后应该拿改key对应最新的state，所以只取restart的最后一个
                    // restarted 存储的是某个key对应的object在重启过程中不同状态
                    Event::Restarted(mut objects) => {
                        if let Some(object) = objects.pop() {
                            Self::insert_object(object, entries, version, kind);
                        }
                    }
                }
                *last_update = SystemTime::now();
            }
            Ok(None) => (),
            Err(err) => {
                // 因为watcher 链接中断会有自动重连, 错误附在事件上,存储对应报错信息
                debug!("{} watcher retry watch", kind);
                match err {
                    runtime::watcher::Error::WatchStartFailed(_) => {
                        let msg = format!("{} watcher watch failed: {}", kind, err);
                        warn!("{}", msg);
                        err_msg.lock().unwrap().replace(msg);
                    }
                    // 正常的超时
                    runtime::watcher::Error::WatchError(err_res)
                        if err_res.message.contains("RST_STREAM") =>
                    {
                        debug!("{} watcher timeout retry watch", kind)
                    }
                    runtime::watcher::Error::TooManyObjects
                    | runtime::watcher::Error::WatchError(_)
                    | runtime::watcher::Error::InitialListFailed(_)
                    | runtime::watcher::Error::WatchFailed(_) => {
                        let msg = format!("{} watcher watch failed: {}", kind, err);
                        warn!("{}", msg);
                        err_msg.lock().unwrap().replace(msg);
                    }
                }
            }
        }
    }

    fn insert_object(
        mut object: K,
        entries: &Arc<Mutex<HashMap<String, String>>>,
        version: &Arc<AtomicU64>,
        kind: &str,
    ) {
        if let Some(uid) = object.meta_mut().uid.take() {
            match serde_json::to_string(&object) {
                Ok(serialized_object) => {
                    entries.lock().unwrap().insert(uid, serialized_object);
                    version.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => debug!(
                    "failed serialized resource {} UID({}) to json Err: {}",
                    kind, uid, e
                ),
            }
        }
    }
}

pub struct ResourceWatcherFactory {
    client: Client,
    runtime: Handle,
}

impl ResourceWatcherFactory {
    pub fn new(client: Client, runtime: Handle) -> Self {
        Self { client, runtime }
    }
    pub fn new_watcher(&self, resource: &'static str) -> Option<GenericResourceWatcher> {
        match resource {
            "nodes" => Some(GenericResourceWatcher::Node(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "namespaces" => Some(GenericResourceWatcher::Namespace(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "services" => Some(GenericResourceWatcher::Service(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "deployments" => Some(GenericResourceWatcher::Deployment(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "pods" => Some(GenericResourceWatcher::Pod(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "statefulsets" => Some(GenericResourceWatcher::StatefulSet(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "daemonsets" => Some(GenericResourceWatcher::DaemonSet(ResourceWatcher::new(
                Api::<DaemonSet>::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "replicationcontrollers" => Some(GenericResourceWatcher::ReplicationController(
                ResourceWatcher::new(
                    Api::all(self.client.clone()),
                    resource,
                    self.runtime.clone(),
                ),
            )),
            "replicasets" => Some(GenericResourceWatcher::ReplicaSet(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "v1ingresses" => Some(GenericResourceWatcher::V1Ingress(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            "v1beta1ingresses" => Some(GenericResourceWatcher::V1beta1Ingress(
                ResourceWatcher::new(
                    Api::all(self.client.clone()),
                    resource,
                    self.runtime.clone(),
                ),
            )),
            "extv1beta1ingresses" => Some(GenericResourceWatcher::ExtV1beta1Ingress(
                ResourceWatcher::new(
                    Api::all(self.client.clone()),
                    resource,
                    self.runtime.clone(),
                ),
            )),
            "routes" => Some(GenericResourceWatcher::Route(ResourceWatcher::new(
                Api::all(self.client.clone()),
                resource,
                self.runtime.clone(),
            ))),
            _ => None,
        }
    }
}
