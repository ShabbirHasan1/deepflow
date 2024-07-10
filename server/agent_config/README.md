# Resource {#resource}

## Limits {#resource.limits}

### CPU Limit (Cores) {#resource.limits.max_cpus}

**Tags**:

<mark></mark>
<mark>deprecated</mark>

**FQCN**:

`resource.limits.max_cpus`

Upgrade from version <= 6.5.9: `max_cpus`

**Default value**:
```yaml
resource:
  limits:
    max_cpus: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

### CPU Limit {#resource.limits.max_millicpus}

**Tags**:

`hot_update`

**FQCN**:

`resource.limits.max_millicpus`

Upgrade from version <= 6.5.9: `max_millicpus`

**Default value**:
```yaml
resource:
  limits:
    max_millicpus: 1000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Logical Milli Cores |
| Range | [1, 100000] |

**Description**:

deepflow-agent uses cgroups to limit CPU usage.
1 millicpu = 1 millicore = 0.001 core.

### Memory Limit {#resource.limits.max_memory}

**Tags**:

`hot_update`

**FQCN**:

`resource.limits.max_memory`

Upgrade from version <= 6.5.9: `max_memory`

**Default value**:
```yaml
resource:
  limits:
    max_memory: 768
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | MiB |
| Range | [128, 100000] |

**Description**:

deepflow-agent uses cgroups to limit memory usage.

### Remote Log Rate Limit {#resource.limits.log_threshold}

**Tags**:

`hot_update`

**FQCN**:

`resource.limits.log_threshold`

Upgrade from version <= 6.5.9: `log_threshold`

**Default value**:
```yaml
resource:
  limits:
    log_threshold: 300
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Lines/Hour |
| Range | [0, 10000] |

**Description**:

deepflow-agent will send logs to deepflow-server, 0 means no limit.

### Log File Size {#resource.limits.log_file_size}

**Tags**:

`hot_update`

**FQCN**:

`resource.limits.log_file_size`

Upgrade from version <= 6.5.9: `log_file_size`

**Default value**:
```yaml
resource:
  limits:
    log_file_size: 1000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | MiB |
| Range | [10, 10000] |

**Description**:

The maximum disk space allowed for deepflow-agent log files.

### Log Retention Time {#resource.limits.log_retention}

**Tags**:

`hot_update`

**FQCN**:

`resource.limits.log_retention`

Upgrade from version <= 6.5.9: `log_retention`

**Default value**:
```yaml
resource:
  limits:
    log_retention: 300d
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10d', '10000d'] |

**Description**:

The retention time for deepflow-agent log files.

## Alerts {#resource.alerts}

### Thread Limit {#resource.alerts.thread_threshold}

**Tags**:

`hot_update`

**FQCN**:

`resource.alerts.thread_threshold`

Upgrade from version <= 6.5.9: `thread_threshold`

**Default value**:
```yaml
resource:
  alerts:
    thread_threshold: 500
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 1000] |

**Description**:

Maximum number of threads that deepflow-agent is allowed to launch.

### Process Limit {#resource.alerts.process_threshold}

**Tags**:

`hot_update`

**FQCN**:

`resource.alerts.process_threshold`

Upgrade from version <= 6.5.9: `process_threshold`

**Default value**:
```yaml
resource:
  alerts:
    process_threshold: 10
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 100] |

**Description**:

Maximum number of processes that deepflow-agent is allowed to launch.

### Core File Checker {#resource.alerts.check_core_file_disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`resource.alerts.check_core_file_disabled`

Upgrade from version <= 6.5.9: `static_config.check-core-file-disabled`

**Default value**:
```yaml
resource:
  alerts:
    check_core_file_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When the host has an invalid NFS file system or a docker is running,
sometime program hang when checking the core file, so the core file
check provides a switch to prevent the process hang. Additional links:
- https://serverfault.com/questions/367438/ls-hangs-for-a-certain-directory
- https://unix.stackexchange.com/questions/495854/processes-hanging-when-trying-to-access-a-file

## Circuit Breakers {#resource.circuit_breakers}

### System Free Memory {#resource.circuit_breakers.sys_free_memory}

#### Threshold {#resource.circuit_breakers.sys_free_memory.sys_free_memory_limit}

**Tags**:

`hot_update`

**FQCN**:

`resource.circuit_breakers.sys_free_memory.sys_free_memory_limit`

Upgrade from version <= 6.5.9: `sys_free_memory_limit`

**Default value**:
```yaml
resource:
  circuit_breakers:
    sys_free_memory:
      sys_free_memory_limit: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | % |
| Range | [0, 100] |

**Description**:

The limit of the percentage of system free memory.
When the free percentage is lower than 90% of this value,
the agent will automatically restart.

### System Load {#resource.circuit_breakers.sys_load}

#### Threshold {#resource.circuit_breakers.sys_load.system_load_circuit_breaker_threshold}

**Tags**:

`hot_update`

**FQCN**:

`resource.circuit_breakers.sys_load.system_load_circuit_breaker_threshold`

Upgrade from version <= 6.5.9: `system_load_circuit_breaker_threshold`

**Default value**:
```yaml
resource:
  circuit_breakers:
    sys_load:
      system_load_circuit_breaker_threshold: 1.0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | float |
| Range | [0, 10] |

**Description**:

When the load of the Linux system divided by the number of
CPU cores exceeds this value, the agent automatically enters
the disabled state. It will automatically recover if it remains
below 90% of this value for a continuous 5 minutes. Setting it
to 0 disables this feature.

#### Recover {#resource.circuit_breakers.sys_load.system_load_circuit_breaker_recover}

**Tags**:

`hot_update`

**FQCN**:

`resource.circuit_breakers.sys_load.system_load_circuit_breaker_recover`

Upgrade from version <= 6.5.9: `system_load_circuit_breaker_recover`

**Default value**:
```yaml
resource:
  circuit_breakers:
    sys_load:
      system_load_circuit_breaker_recover: 0.9
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | float |
| Range | [0, 10] |

**Description**:

When the system load of the Linux system divided by the
number of CPU cores is continuously below this value for 5
minutes, the agent can recover from the circuit breaker
disabled state, and setting it to 0 means turning off the
circuit breaker feature.

#### Metric {#resource.circuit_breakers.sys_load.system_load_circuit_breaker_metric}

**Tags**:

`hot_update`

**FQCN**:

`resource.circuit_breakers.sys_load.system_load_circuit_breaker_metric`

Upgrade from version <= 6.5.9: `system_load_circuit_breaker_metric`

**Default value**:
```yaml
resource:
  circuit_breakers:
    sys_load:
      system_load_circuit_breaker_metric: load15
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| load1 | |
| load5 | |
| load15 | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The system load circuit breaker mechanism uses this metric,
and the agent will check this metric every 10 seconds by default.

### NPB (Network Packet Broker) {#resource.circuit_breakers.tx_bandwidth}

#### Threshold {#resource.circuit_breakers.tx_bandwidth.max_tx_bandwidth}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`resource.circuit_breakers.tx_bandwidth.max_tx_bandwidth`

Upgrade from version <= 6.5.9: `max_tx_bandwidth`

**Default value**:
```yaml
resource:
  circuit_breakers:
    tx_bandwidth:
      max_tx_bandwidth: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Mbps |
| Range | [0, 100000] |

**Description**:

When the outbound direction of the NPB interface
reaches or exceeds the threshold, the distribution will be
stopped, and then the distribution will be resumed if the
value is lower than (max_tx_bandwidth - max_npb_bps)*90%
within 5 consecutive monitoring intervals.

Attention: When configuring this value, it must be greater
than max_npb_bps. 0 means disable this feature.

#### Monitoring Interval {#resource.circuit_breakers.tx_bandwidth.bandwidth_probe_interval}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`resource.circuit_breakers.tx_bandwidth.bandwidth_probe_interval`

Upgrade from version <= 6.5.9: `bandwidth_probe_interval`

**Default value**:
```yaml
resource:
  circuit_breakers:
    tx_bandwidth:
      bandwidth_probe_interval: 10s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '60s'] |

**Description**:

Monitoring interval for outbound traffic rate of NPB interface.

## Tunning {#resource.tunning}

### CPU Affinity {#resource.tunning.cpu_affinity}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`resource.tunning.cpu_affinity`

Upgrade from version <= 6.5.9: `static_config.cpu-affinity`

**Default value**:
```yaml
resource:
  tunning:
    cpu_affinity: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 65536] |

**Description**:

CPU affinity is the tendency of a process to run on a given CPU for as long as possible
without being migrated to other processors. Example: `cpu-affinity: [1, 3, 5, 7, 9]`.

### Process Scheduling Priority {#resource.tunning.process_scheduling_priority}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`resource.tunning.process_scheduling_priority`

Upgrade from version <= 6.5.9: `static_config.process-scheduling-priority`

**Default value**:
```yaml
resource:
  tunning:
    process_scheduling_priority: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [-20, 19] |

**Description**:

The smaller the value of process scheduling priority, the higher the priority of the
process, and the larger the priority, the lower the priority.

### Memory Trim {#resource.tunning.memory_trim_disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`resource.tunning.memory_trim_disabled`

Upgrade from version <= 6.5.9: `static_config.memory-trim-disabled`

**Default value**:
```yaml
resource:
  tunning:
    memory_trim_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Using memory trimming can effectively reduce memory usage, but there may be
performance loss.

# Management {#management}

## NTP {#management.ntp}

### Synchronization Enabled {#management.ntp.ntp_enabled}

**Tags**:

`hot_update`

**FQCN**:

`management.ntp.ntp_enabled`

Upgrade from version <= 6.5.9: `ntp_enabled`

**Default value**:
```yaml
management:
  ntp:
    ntp_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to synchronize the clock to the deepflow-server, this behavior
will not change the time of the deepflow-agent running environment.

### Max Offset {#management.ntp.ntp_max_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.ntp.ntp_max_interval`

Upgrade from version <= 6.5.9: `static_config.ntp-max-interval`

**Default value**:
```yaml
management:
  ntp:
    ntp_max_interval: 300s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | [0, '365d'] |

**Description**:

When the timestamp fallback exceeds this value, the agent will restart.

### Min Offset {#management.ntp.ntp_min_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.ntp.ntp_min_interval`

Upgrade from version <= 6.5.9: `static_config.ntp-min-interval`

**Default value**:
```yaml
management:
  ntp:
    ntp_min_interval: 10s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | [0, '365d'] |

**Description**:

When the time difference exceeds this value, the timestamp will be corrected.

## Communication {#management.communication}

### Active Sync Interval {#management.communication.sync_interval}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.sync_interval`

Upgrade from version <= 6.5.9: `sync_interval`

**Default value**:
```yaml
management:
  communication:
    sync_interval: 60s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10s', '3600s'] |

**Description**:

The interval at which deepflow-agent actively requests configuration and
tag information from deepflow-server.

### Maximum Escape Time {#management.communication.max_escape_seconds}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.max_escape_seconds`

Upgrade from version <= 6.5.9: `max_escape_seconds`

**Default value**:
```yaml
management:
  communication:
    max_escape_seconds: 3600s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['600s', '30d'] |

**Description**:

The maximum time that the agent is allowed to work normally when it
cannot connect to the server. After the timeout, the agent automatically
enters the disabled state.

### Fixed Controller IP {#management.communication.proxy_controller_ip}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.proxy_controller_ip`

Upgrade from version <= 6.5.9: `proxy_controller_ip`

**Default value**:
```yaml
management:
  communication:
    proxy_controller_ip: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | ip |

**Description**:

When this value is set, deepflow-agent will use this IP to access the
control plane port of deepflow-server, which is usually used when
deepflow-server uses an external load balancer.

### Controller Port {#management.communication.proxy_controller_port}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.proxy_controller_port`

Upgrade from version <= 6.5.9: `proxy_controller_port`

**Default value**:
```yaml
management:
  communication:
    proxy_controller_port: 30035
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

The control plane port used by deepflow-agent to access deepflow-server.
The default port within the same K8s cluster is 20035, and the default port
of deepflow-agent outside the cluster is 30035.

### Fixed Ingester IP {#management.communication.analyzer_ip}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.analyzer_ip`

Upgrade from version <= 6.5.9: `analyzer_ip`

**Default value**:
```yaml
management:
  communication:
    analyzer_ip: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | ip |

**Description**:

When this value is set, deepflow-agent will use this IP to access the
data plane port of deepflow-server, which is usually used when
deepflow-server uses an external load balancer.

### Ingester Port {#management.communication.analyzer_port}

**Tags**:

`hot_update`

**FQCN**:

`management.communication.analyzer_port`

Upgrade from version <= 6.5.9: `analyzer_port`

**Default value**:
```yaml
management:
  communication:
    analyzer_port: 30033
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

The data plane port used by deepflow-agent to access deepflow-server.
The default port within the same K8s cluster is 20033, and the default port
of deepflow-agent outside the cluster is 30033.

### gRPC Socket Buffer Size {#management.communication.grpc_buffer_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.communication.grpc_buffer_size`

Upgrade from version <= 6.5.9: `static_config.grpc-buffer-size`

**Default value**:
```yaml
management:
  communication:
    grpc_buffer_size: 5
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | MiB |
| Range | [5, 1024] |

**Description**:

gRPC socket buffer size.

## Self Monitoring {#management.self_monitoring}

### Guard Interval {#management.self_monitoring.guard_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.self_monitoring.guard_interval`

Upgrade from version <= 6.5.9: `static_config.guard-interval`

**Default value**:
```yaml
management:
  self_monitoring:
    guard_interval: 10s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '3600s'] |

**Description**:

The agent will monitor:
1. System free memory;
2. Get the number of threads of the agent itself by reading the file information
   under the /proc directory;
3. Size and number of log files generated by the agent.
4. System load

### Log {#management.self_monitoring.log}

#### Remote Log Enabled {#management.self_monitoring.log.rsyslog_enabled}

**Tags**:

`hot_update`

**FQCN**:

`management.self_monitoring.log.rsyslog_enabled`

Upgrade from version <= 6.5.9: `rsyslog_enabled`

**Default value**:
```yaml
management:
  self_monitoring:
    log:
      rsyslog_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When enabled, deepflow-agent will send its own logs to deepflow-server.

#### Log Level {#management.self_monitoring.log.log_level}

**Tags**:

`hot_update`

**FQCN**:

`management.self_monitoring.log.log_level`

Upgrade from version <= 6.5.9: `log_level`

**Default value**:
```yaml
management:
  self_monitoring:
    log:
      log_level: INFO
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| DEBUG | |
| INFO | |
| WARNING | |
| ERROR | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Log level of deepflow-agent.

### Profile {#management.self_monitoring.profile}

#### Profiler Enabled {#management.self_monitoring.profile.profiler}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.self_monitoring.profile.profiler`

Upgrade from version <= 6.5.9: `static_config.profiler`

**Default value**:
```yaml
management:
  self_monitoring:
    profile:
      profiler: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Only available for Trident (Golang version of Agent).

### Debug {#management.self_monitoring.debug}

#### Source Port for Debug {#management.self_monitoring.debug.debug_listen_port}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.self_monitoring.debug.debug_listen_port`

Upgrade from version <= 6.5.9: `static_config.debug-listen-port`

**Default value**:
```yaml
management:
  self_monitoring:
    debug:
      debug_listen_port: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 65535] |

**Description**:

Default value 0 means use a random client port number.
Only available for Trident (Golang version of Agent).

#### Statsd Counters for Sniffer {#management.self_monitoring.debug.enable_debug_stats}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.self_monitoring.debug.enable_debug_stats`

Upgrade from version <= 6.5.9: `static_config.enable-debug-stats`

**Default value**:
```yaml
management:
  self_monitoring:
    debug:
      enable_debug_stats: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Only available for Trident (Golang version of Agent).

## Standalone Mode {#management.standalone_mode}

### Data File Size {#management.standalone_mode.standalone_data_file_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.standalone_mode.standalone_data_file_size`

Upgrade from version <= 6.5.9: `static_config.standalone-data-file-size`

**Default value**:
```yaml
management:
  standalone_mode:
    standalone_data_file_size: 200
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | MiB |
| Range | [1, 1000000] |

**Description**:

When deepflow-agent runs in standalone mode, it will not be controlled by
deepflow-server, and the collected data will only be written to the local file.
Currently supported data types for writing are l4_flow_log and l7_flow_log. Each
type of data is written to a separate file. This configuration can be used to
specify the maximum size of the data file, and rotate when it exceeds this size.
A maximum of two files are kept for each type of data.

### Directory of Data File {#management.standalone_mode.standalone_data_file_dir}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.standalone_mode.standalone_data_file_dir`

Upgrade from version <= 6.5.9: `static_config.standalone-data-file-dir`

**Default value**:
```yaml
management:
  standalone_mode:
    standalone_data_file_dir: /var/log/deepflow_agent/
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Directory where data files are written to.

### Log File Path {#management.standalone_mode.log_file}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`management.standalone_mode.log_file`

Upgrade from version <= 6.5.9: `static_config.log-file`

**Default value**:
```yaml
management:
  standalone_mode:
    log_file: /var/log/deepflow_agent/deepflow_agent.log
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Note that this configuration is only used in standalone mode.

# Inputs {#inputs}

## cBPF {#inputs.cbpf}

### Common {#inputs.cbpf.common}

#### TAP (Traffic Access Point) Mode {#inputs.cbpf.common.tap_mode}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.common.tap_mode`

Upgrade from version <= 6.5.9: `tap_mode`

**Default value**:
```yaml
inputs:
  cbpf:
    common:
      tap_mode: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | Local |
| 1 | Virtual Mirror |
| 2 | Physical Mirror |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Mirror mode is used when deepflow-agent cannot directly capture the traffic from
the source. For example:
- in the K8s macvlan environment, capture the Pod traffic through the Node NIC
- in the Hyper-V environment, capture the VM traffic through the Hypervisor NIC
- in the ESXi environment, capture traffic through VDS/VSS local SPAN
- in the DPDK environment, capture traffic through DPDK ring buffer
Use Physical Mirror mode when deepflow-agent captures traffic through physical
switch mirroring.

<mark>`Physical Mirror` is only supported in the Enterprise Edition.</mark>

### AF_PACKET {#inputs.cbpf.af_packet}

#### NIC Name Regular Expression {#inputs.cbpf.af_packet.tap_interface_regex}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.af_packet.tap_interface_regex`

Upgrade from version <= 6.5.9: `tap_interface_regex`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      tap_interface_regex: ^(tap.*|cali.*|veth.*|eth.*|en[osipx].*|lxc.*|lo|[0-9a-f]+_h)$
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |
| Range | [0, 65535] |

**Description**:

Regular expression of NIC name for collecting traffic.

Explanation of the default configuration:
```
Localhost:     lo
Common NIC:    eth.*|en[osipx].*
QEMU VM NIC:   tap.*
Flannel:       veth.*
Calico:        cali.*
Cilium         lxc.*
Kube-OVN       [0-9a-f]+_h$
```

#### Bonding Sub-interfaces {#inputs.cbpf.af_packet.tap_interface_bond_groups}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.af_packet.tap_interface_bond_groups`

Upgrade from version <= 6.5.9: `static_config.tap-interface-bond-groups`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      tap_interface_bond_groups:
      - tap_interfaces: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Packets of interfaces in the same group can be aggregated together,
Only effective when tap_mode is 0.

#### Extra Network Namespace {#inputs.cbpf.af_packet.extra_netns_regex}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.af_packet.extra_netns_regex`

Upgrade from version <= 6.5.9: `extra_netns_regex`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      extra_netns_regex: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Traffic will be captured in regex matched namespaces besides the default
namespace. NICs captured in extra namespaces are also filtered with
`tap_interface_regex`.

Default value "" means no extra network namespace (default namespace only).

#### Extra Traffic Filter {#inputs.cbpf.af_packet.capture_bpf}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.af_packet.capture_bpf`

Upgrade from version <= 6.5.9: `capture_bpf`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      capture_bpf: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |
| Range | [0, 512] |

**Description**:

If not configured, all traffic will be collected. Please
refer to BPF syntax: https://biot.com/capstats/bpf.html

#### TAP Interfaces {#inputs.cbpf.af_packet.src_interfaces}

**Tags**:

<mark></mark>
<mark>deprecated</mark>

**FQCN**:

`inputs.cbpf.af_packet.src_interfaces`

Upgrade from version <= 6.5.9: `static_config.src-interfaces`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      src_interfaces: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

#### PCP of Mirror Traffic {#inputs.cbpf.af_packet.mirror_traffic_pcp}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.af_packet.mirror_traffic_pcp`

Upgrade from version <= 6.5.9: `static_config.mirror-traffic-pcp`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      mirror_traffic_pcp: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 8] |

**Description**:

Calculate TAP value from vlan tag only if vlan pcp matches this value.

#### BPF Filter Enabled {#inputs.cbpf.af_packet.bpf_disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.af_packet.bpf_disabled`

Upgrade from version <= 6.5.9: `static_config.bpf-disabled`

**Default value**:
```yaml
inputs:
  cbpf:
    af_packet:
      bpf_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

It is found that there may be bugs in BPF traffic filtering under some
versions of Linux Kernel. After this configuration is enabled, deepflow-agent
will not use the filtering capabilities of BPF, and will filter by itself after
capturing full traffic. Note that this may significantly increase the resource
overhead of deepflow-agent.

### Special Network {#inputs.cbpf.special_network}

#### DPDK {#inputs.cbpf.special_network.dpdk}

##### Enabled {#inputs.cbpf.special_network.dpdk.dpdk_enabled}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.dpdk.dpdk_enabled`

Upgrade from version <= 6.5.9: `static_config.dpdk-enabled`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      dpdk:
        dpdk_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

The DPDK RecvEngine is only started when this configuration item is turned on.
Note that you also need to set tap_mode to 1. Please refer to
https://dpdk-docs.readthedocs.io/en/latest/prog_guide/multi_proc_support.html

##### CPU Core List {#inputs.cbpf.special_network.dpdk.dpdk_core_list}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.dpdk.dpdk_core_list`

Upgrade from version <= 6.5.9: `static_config.dpdk_core_list`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      dpdk:
        dpdk_core_list: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Map lcore set to physical cpu set.
Format: `<lcores[@cpus]>[<,lcores[@cpus]>...]`

Examples:
- 1,2,3,4
- 1-4
- (1,2)(3-10)
- 1@3,2@4

lcores and cpus list are grouped by `(` and `)`. Within the group, `-` is used
for range separator, `,` is used for single number separator. `()` can be
omitted for single element group, `@` can be omitted if cpus and lcores have
the same value.

#### LibPCAP {#inputs.cbpf.special_network.libpcap}

##### Enabled {#inputs.cbpf.special_network.libpcap.libpcap_enabled}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.libpcap.libpcap_enabled`

Upgrade from version <= 6.5.9: `static_config.libpcap-enabled`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      libpcap:
        libpcap_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Supports running on Windows and Linux, Low performance when using multiple interfaces.
Default to true in Windows, false in Linux.

#### vHost User {#inputs.cbpf.special_network.vhost_user}

##### Enabled {#inputs.cbpf.special_network.vhost_user.vhost_socket_path}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.vhost_user.vhost_socket_path`

Upgrade from version <= 6.5.9: `static_config.vhost-socket-path`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      vhost_user:
        vhost_socket_path: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Supports running on Linux with mirror mode.

#### Switch {#inputs.cbpf.special_network.switch}

##### sFlow Server Ports {#inputs.cbpf.special_network.switch.sflow_ports}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.switch.sflow_ports`

Upgrade from version <= 6.5.9: `static_config.xflow-collector.sflow-ports`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      switch:
        sflow_ports: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

This feature is only supported by the Enterprise Edition of Trident.
In general, sFlow uses port 6343. Default value [] means that no sFlow
data will be collected.

##### NetFlow Server Ports {#inputs.cbpf.special_network.switch.netflow_ports}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.special_network.switch.netflow_ports`

Upgrade from version <= 6.5.9: `static_config.xflow-collector.netflow-ports`

**Default value**:
```yaml
inputs:
  cbpf:
    special_network:
      switch:
        netflow_ports: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

This feature is only supported by the Enterprise Edition of Trident.
Additionally, only NetFlow v5 is currently supported. In general, NetFlow
uses port 2055. Default value [] means that no NetFlow data will be collected.

### Tunning {#inputs.cbpf.tunning}

#### Local Dispatcher Count {#inputs.cbpf.tunning.local_dispatcher_count}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.tunning.local_dispatcher_count`

Upgrade from version <= 6.5.9: `static_config.local-dispatcher-count`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      local_dispatcher_count: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 64] |

**Description**:

The configuration takes effect when tap_mode is 0 and extra_netns_regex is null,
PACKET_FANOUT is to enable load balancing and parallel processing, which can improve
the performance and scalability of network applications. When the `local-dispatcher-count`
is greater than 1, multiple dispatcher threads will be launched, consuming more CPU and
memory. Increasing the `local-dispatcher-count` helps to reduce the operating system's
software interrupts on multi-core CPU servers.

#### Packet Fanout Mode {#inputs.cbpf.tunning.packet_fanout_mode}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.tunning.packet_fanout_mode`

Upgrade from version <= 6.5.9: `static_config.packet-fanout-mode`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      packet_fanout_mode: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | PACKET_FANOUT_HASH |
| 1 | PACKET_FANOUT_LB |
| 2 | PACKET_FANOUT_CPU |
| 3 | PACKET_FANOUT_ROLLOVER |
| 4 | PACKET_FANOUT_RND |
| 5 | PACKET_FANOUT_QM |
| 6 | PACKET_FANOUT_CBPF |
| 7 | PACKET_FANOUT_EBPF |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

The configuration is a parameter used with the PACKET_FANOUT feature in the Linux
kernel to specify the desired packet distribution algorithm. Refer to:
- https://github.com/torvalds/linux/blob/afcd48134c58d6af45fb3fdb648f1260b20f2326/include/uapi/linux/if_packet.h#L71
- https://www.stackpath.com/blog/bpf-hook-points-part-1/

#### Dispatcher Queues {#inputs.cbpf.tunning.dispatcher_queue}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.tunning.dispatcher_queue`

Upgrade from version <= 6.5.9: `static_config.dispatcher-queue`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      dispatcher_queue: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

The configuration takes effect when tap_mode is 0 or 2,
dispatcher-queue is always true when tap_mode is 2

#### AF_PACKET Blocks Config Enabled {#inputs.cbpf.tunning.afpacket_blocks_enabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.tunning.afpacket_blocks_enabled`

Upgrade from version <= 6.5.9: `static_config.afpacket-blocks-enabled`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      afpacket_blocks_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When tap_mode != 2, you need to explicitly turn on this switch to
configure 'afpacket-blocks'.

#### AF_PACKET Blocks {#inputs.cbpf.tunning.afpacket_blocks}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.tunning.afpacket_blocks`

Upgrade from version <= 6.5.9: `static_config.afpacket-blocks`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      afpacket_blocks: 128
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [8, 1000000] |

**Description**:

deepflow-agent will automatically calculate the number of blocks
used by AF_PACKET according to max_memory, which can also be specified
using this configuration item. The size of each block is fixed at 1MB.

#### Maximum Packet Capture Length {#inputs.cbpf.tunning.capture_packet_size}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.tunning.capture_packet_size`

Upgrade from version <= 6.5.9: `capture_packet_size`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      capture_packet_size: 65535
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | byte |
| Range | [128, 65535] |

**Description**:

DPDK environment does not support this configuration.

#### Traffic Capture Socket {#inputs.cbpf.tunning.capture_socket_type}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.tunning.capture_socket_type`

Upgrade from version <= 6.5.9: `capture_socket_type`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      capture_socket_type: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | Adaptive |
| 2 | AF_PACKET V2 |
| 3 | AF_PACKET V3 |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Traffic capture API in Linux environment.

#### Buffer Block Size for Raw Packet {#inputs.cbpf.tunning.analyzer_raw_packet_block_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.tunning.analyzer_raw_packet_block_size`

Upgrade from version <= 6.5.9: `static_config.analyzer-raw-packet-block-size`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      analyzer_raw_packet_block_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 16000000] |

**Description**:

Larger value will reduce memory allocation for raw packet, but will also
delay memory free.

#### Queue Size for Analyzer Mode {#inputs.cbpf.tunning.analyzer_queue_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.tunning.analyzer_queue_size`

Upgrade from version <= 6.5.9: `static_config.analyzer-queue-size`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      analyzer_queue_size: 131072
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues (only for tap_mode = 2):
- 0.1-bytes-to-parse
- 0.2-packet-to-flowgenerator
- 0.3-packet-to-pipeline

#### Packet Capture Rate Limit {#inputs.cbpf.tunning.max_collect_pps}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.tunning.max_collect_pps`

Upgrade from version <= 6.5.9: `max_collect_pps`

**Default value**:
```yaml
inputs:
  cbpf:
    tunning:
      max_collect_pps: 200
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Kpps |
| Range | [1, 1000000] |

**Description**:

Maximum packet rate allowed for collection.

### Preprocess {#inputs.cbpf.preprocess}

#### Decapsulation Tunnel Protocols {#inputs.cbpf.preprocess.decap_type}

**Tags**:

`hot_update`

**FQCN**:

`inputs.cbpf.preprocess.decap_type`

Upgrade from version <= 6.5.9: `decap_type`

**Default value**:
```yaml
inputs:
  cbpf:
    preprocess:
      decap_type:
      - 1
      - 2
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 1 | VXLAN |
| 2 | IPIP |
| 3 | GRE |
| 4 | Geneve |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Decapsulation tunnel protocols.

#### Trim Tunnel Header {#inputs.cbpf.preprocess.trim_tunnel_types}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.cbpf.preprocess.trim_tunnel_types`

Upgrade from version <= 6.5.9: `static_config.trim-tunnel-types`

**Default value**:
```yaml
inputs:
  cbpf:
    preprocess:
      trim_tunnel_types: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| ERSPAN | |
| VXLAN | |
| TEB | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Whether to remove the tunnel header in mirrored traffic.

#### Default Network Type for Mirror Traffic {#inputs.cbpf.preprocess.default_tap_type}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.preprocess.default_tap_type`

Upgrade from version <= 6.5.9: `static_config.default-tap-type`

**Default value**:
```yaml
inputs:
  cbpf:
    preprocess:
      default_tap_type: 3
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 3 | Cloud Network |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

deepflow-agent will mark the TAP (Traffic Access Point) location
according to the outer vlan tag in the mirrored traffic of the physical
switch. When the vlan tag has no corresponding TAP value, or the vlan
pcp does not match the 'mirror-traffic-pcp', it will assign the TAP value.
This configuration item. Default value 3 means Cloud Network.

#### Mirror Traffic Dedup {#inputs.cbpf.preprocess.analyzer_dedup_disabled}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.preprocess.analyzer_dedup_disabled`

Upgrade from version <= 6.5.9: `static_config.analyzer-dedup-disabled`

**Default value**:
```yaml
inputs:
  cbpf:
    preprocess:
      analyzer_dedup_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to enable mirror traffic deduplication when tap_mode = 2.

#### NFVGW Traffic {#inputs.cbpf.preprocess.cloud_gateway_traffic}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.cbpf.preprocess.cloud_gateway_traffic`

Upgrade from version <= 6.5.9: `static_config.cloud-gateway-traffic`

**Default value**:
```yaml
inputs:
  cbpf:
    preprocess:
      cloud_gateway_traffic: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether it is the mirrored traffic of NFVGW (cloud gateway).

## eBPF {#inputs.ebpf}

### Enabled {#inputs.ebpf.disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.disabled`

Upgrade from version <= 6.5.9: `static_config.ebpf.disabled`

**Default value**:
```yaml
inputs:
  ebpf:
    disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to enable eBPF features.

### Socket Event {#inputs.ebpf.socket_event}

#### Uprobe {#inputs.ebpf.socket_event.uprobe}

##### Regex for Process Name {#inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs}

The name of the process where each feature of ebpf uprobe takes effect,
which is configured using regular expressions

###### Golang-specific Symbol Table Parsing {#inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.golang_symbol}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.golang_symbol`

Upgrade from version <= 6.5.9: `static_config.ebpf.uprobe-process-name-regexs.golang-symbol`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      uprobe:
        uprobe_process_name_regexs:
          golang_symbol: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Process name to enable Golang-specific symbol table parsing.
Default: "", which means that this feature is not enabled for any process.

This feature acts on Golang processes that have trimmed the standard symbol
table. When this feature is enabled, for processes with Golang
version >= 1.13 and < 1.18, when the standard symbol table is missing, the
Golang-specific symbol table will be parsed to complete uprobe data collection.
Note that enabling this feature may cause the eBPF initialization process to
take ten minutes. The `golang-symbol` configuration item depends on the `golang`
configuration item, the `golang-symbol` is a subset of the `golang` configuration item.

Example:
- Ensure that the regular expression matching for the 'golang' configuration
  item is enabled, for example: `golang: .*`
- You've encountered the following warning log:
  ```
  [eBPF] WARNING: func resolve_bin_file() [user/go_tracer.c:558] Go process pid 1946
  [path: /proc/1946/root/usr/local/bin/kube-controller-manager] (version: go1.16). Not find any symbols!
  ```
  Suppose there is a Golang process with a process ID of '1946.'
- To initially confirm whether the executable file for this process has a symbol table:
  - Retrieve the executable file's path using the process ID:
    ```
    # ls -al /proc/1946/exe
    /proc/1946/exe -> /usr/local/bin/kube-controller-manager
    ```
  - Check if there is a symbol table:
    ```
    # nm /proc/1946/root/usr/local/bin/kube-controller-manager
    nm: /proc/1946/root/usr/local/bin/kube-controller-manager: no symbols
    ```
- If "no symbols" is encountered, it indicates the absence of a symbol table. In such a
  scenario, we need to configure the "golang-symbol" setting. To configure this:
  ```
  golang-symbol: ^(kube-controller-.*)$
  ```
  Explanation: Configure a regular expression for the trailing part 'kube-controller-manager'
  following the executable file path of the process. It can also be a regular expression for
  the process name (obtained through `/proc/<PID>/status` to extract the process `Name`).
- During the agent startup process, you will observe the following log information: (The entry
  address for the function `crypto/tls.(*Conn).Write` has already been resolved, i.e., entry:0x25fca0).
  ```
  [eBPF] INFO Uprobe [/proc/1946/root/usr/local/bin/kube-controller-manager] pid:1946 go1.16.0
  entry:0x25fca0 size:1952 symname:crypto/tls.(*Conn).Write probe_func:uprobe_go_tls_write_enter rets_count:0
  ```
  The logs indicate that the Golang program has been successfully hooked.

###### Golang Process Enable Uprobe {#inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.golang}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.golang`

Upgrade from version <= 6.5.9: `static_config.ebpf.uprobe-process-name-regexs.golang`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      uprobe:
        uprobe_process_name_regexs:
          golang: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The name of the Golang process that enables HTTP2/HTTPS protocol data collection
and auto-tracing. go auto-tracing also dependent go-tracing-timeout.
The default value is "", which means it is disabled for all Golang processes.

###### OpenSSL Process Enable Uprobe {#inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.openssl}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.uprobe.uprobe_process_name_regexs.openssl`

Upgrade from version <= 6.5.9: `static_config.ebpf.uprobe-process-name-regexs.openssl`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      uprobe:
        uprobe_process_name_regexs:
          openssl: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The name of the process that uses the openssl library to enable HTTPS protocol data collection.
Default value "" means that it is disabled for all processes that use the openssl library.
One can use the following method to determine whether an application process can use
`Uprobe hook openssl library` to access encrypted data:

Use the command `cat /proc/<PID>/maps | grep "libssl.so"` to check if it contains
information about openssl. If it does, it indicates that this process is using the
openssl library. After configuring the openssl options, deepflow-agent will retrieve process
information that matches the regular expression, hooking the corresponding encryption/decryption
interfaces of the openssl library.

In the logs, you will encounter a message similar to the following:
```
[eBPF] INFO openssl uprobe, pid:1005, path:/proc/1005/root/usr/lib64/libssl.so.1.0.2k
```

##### eBPF Golang Tracing Timeout {#inputs.ebpf.socket_event.uprobe.go_tracing_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.uprobe.go_tracing_timeout`

Upgrade from version <= 6.5.9: `static_config.ebpf.go-tracing-timeout`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      uprobe:
        go_tracing_timeout: 120s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | [0, '1d'] |

**Description**:

The expected maximum time interval between the server receiving the request and returning
the response, If the value is 0, this feature is disabled. Tracing only considers the
thread number.

#### Kprobe {#inputs.ebpf.socket_event.kprobe}

##### Kprobe Blacklist {#inputs.ebpf.socket_event.kprobe.kprobe_blacklist}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.kprobe.kprobe_blacklist`

Upgrade from version <= 6.5.9: `static_config.ebpf.kprobe-blacklist`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      kprobe:
        kprobe_blacklist:
          port_list: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

TCP&UDP Port Blacklist, Priority higher than kprobe-whitelist.

##### Kprobe Whitelist {#inputs.ebpf.socket_event.kprobe.kprobe_whitelist}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.socket_event.kprobe.kprobe_whitelist`

Upgrade from version <= 6.5.9: `static_config.ebpf.kprobe-whitelist`

**Default value**:
```yaml
inputs:
  ebpf:
    socket_event:
      kprobe:
        kprobe_whitelist:
          port_list: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

TCP&UDP Port Whitelist, Priority lower than kprobe-blacklist.

### IO Event {#inputs.ebpf.io_event}

#### Collect Mode {#inputs.ebpf.io_event.io_event_collect_mode}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.io_event.io_event_collect_mode`

Upgrade from version <= 6.5.9: `static_config.ebpf.io-event-collect-mode`

**Default value**:
```yaml
inputs:
  ebpf:
    io_event:
      io_event_collect_mode: 1
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | Disabled |
| 1 | Request Life Cycle |
| 2 | All |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Collection modes:
- 0: Indicates that no IO events are collected.
- 1: Indicates that only IO events within the request life cycle are collected.
- 2: Indicates that all IO events are collected.

#### Minimal Duration {#inputs.ebpf.io_event.io_event_minimal_duration}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.io_event.io_event_minimal_duration`

Upgrade from version <= 6.5.9: `static_config.ebpf.io-event-minimal-duration`

**Default value**:
```yaml
inputs:
  ebpf:
    io_event:
      io_event_minimal_duration: 1ms
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1ns', '1s'] |

**Description**:

Only collect IO events with delay exceeding this threshold.

### Profile {#inputs.ebpf.profile}

#### Java Symbol Update Defer {#inputs.ebpf.profile.java_symbol_file_refresh_defer_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.java_symbol_file_refresh_defer_interval`

Upgrade from version <= 6.5.9: `static_config.ebpf.java-symbol-file-refresh-defer-interval`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      java_symbol_file_refresh_defer_interval: 600s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['5s', '3600s'] |

**Description**:

When deepflow-agent finds that an unresolved function name appears in the function call
stack of a Java process, it will trigger the regeneration of the symbol file of the
process. Because Java utilizes the Just-In-Time (JIT) compilation mechanism, to obtain
more symbols for Java processes, the regeneration will be deferred for a period of time.

#### Maximum Java Symbol File Size {#inputs.ebpf.profile.java_symbol_file_max_space_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.java_symbol_file_max_space_limit`

Upgrade from version <= 6.5.9: `static_config.ebpf.java-symbol-file-max-space-limit`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      java_symbol_file_max_space_limit: 10
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | MiB |
| Range | [2, 100] |

**Description**:

All Java symbol files are stored in the '/tmp' directory mounted by the deepflow-agent.
To prevent excessive occupation of host node space due to large Java symbol files, a
maximum size limit is set for each generated Java symbol file.

#### eBPF On-CPU Profile {#inputs.ebpf.profile.on_cpu_profile}

##### Disabled {#inputs.ebpf.profile.on_cpu_profile.disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.on_cpu_profile.disabled`

Upgrade from version <= 6.5.9: `static_config.ebpf.on-cpu-profile.disabled`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      on_cpu_profile:
        disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

eBPF On-CPU profile switch.

##### Sampling Frequency {#inputs.ebpf.profile.on_cpu_profile.frequency}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.on_cpu_profile.frequency`

Upgrade from version <= 6.5.9: `static_config.ebpf.on-cpu-profile.frequency`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      on_cpu_profile:
        frequency: 99
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 1000] |

**Description**:

eBPF On-CPU profile sampling frequency.

##### Aggregate by CPU {#inputs.ebpf.profile.on_cpu_profile.cpu}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.on_cpu_profile.cpu`

Upgrade from version <= 6.5.9: `static_config.ebpf.on-cpu-profile.cpu`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      on_cpu_profile:
        cpu: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to obtain the value of CPUID and decide whether to participate in aggregation.
- Set to 1: Obtain the value of CPUID and will be included in the aggregation of stack
  trace data.
- Set to 0: It will not be included in the aggregation. Any other value is considered
  invalid, the CPU value for stack trace data reporting is a special value
  `CPU_INVALID: 0xfff` used to indicate that it is an invalid value.

##### Process Regex {#inputs.ebpf.profile.on_cpu_profile.regex}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.profile.on_cpu_profile.regex`

Upgrade from version <= 6.5.9: `static_config.ebpf.on-cpu-profile.regex`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      on_cpu_profile:
        regex: ^deepflow-.*
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The process names which enable eBPF On-CPU profiling.

#### eBPF Off-CPU Profile {#inputs.ebpf.profile.off_cpu_profile}

##### Disabled {#inputs.ebpf.profile.off_cpu_profile.disabled}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.profile.off_cpu_profile.disabled`

Upgrade from version <= 6.5.9: `static_config.ebpf.off-cpu-profile.disabled`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      off_cpu_profile:
        disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

eBPF Off-CPU profile switch.

##### Process Regex {#inputs.ebpf.profile.off_cpu_profile.regex}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.profile.off_cpu_profile.regex`

Upgrade from version <= 6.5.9: `static_config.ebpf.off-cpu-profile.regex`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      off_cpu_profile:
        regex: ^deepflow-.*
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The process names which enable eBPF Off-CPU profiling.

##### Aggregate by CPU {#inputs.ebpf.profile.off_cpu_profile.cpu}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.profile.off_cpu_profile.cpu`

Upgrade from version <= 6.5.9: `static_config.ebpf.off-cpu-profile.cpu`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      off_cpu_profile:
        cpu: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to obtain the value of CPUID and decide whether to participate in aggregation.
- Set to 1: Obtain the value of CPUID and will be included in the aggregation of stack
  trace data.
- Set to 0: It will not be included in the aggregation. Any other value is considered
  invalid, the CPU value for stack trace data reporting is a special value
  `CPU_INVALID: 0xfff` used to indicate that it is an invalid value.

##### Minimum Blocking Event Time {#inputs.ebpf.profile.off_cpu_profile.minblock}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.profile.off_cpu_profile.minblock`

Upgrade from version <= 6.5.9: `static_config.ebpf.off-cpu-profile.minblock`

**Default value**:
```yaml
inputs:
  ebpf:
    profile:
      off_cpu_profile:
        minblock: 50us
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | [0, '1h'] |

**Description**:

If set to 0, there will be no minimum value limitation. Scheduler events are still
high-frequency events, as their rate may exceed 1 million events per second, so
caution should still be exercised.

If overhead remains an issue, you can configure the 'minblock' tunable parameter here.
If the off-CPU time is less than the value configured in this item, the data will be
discarded. If your goal is to trace longer blocking events, increasing this parameter
can filter out shorter blocking events, further reducing overhead. Additionally, we
will not collect events with a block time exceeding 1 hour.

### Tunning {#inputs.ebpf.tunning}

#### eBPF Packet Capture Rate Limit {#inputs.ebpf.tunning.global_ebpf_pps_threshold}

**Tags**:

`hot_update`

**FQCN**:

`inputs.ebpf.tunning.global_ebpf_pps_threshold`

Upgrade from version <= 6.5.9: `static_config.ebpf.global-ebpf-pps-threshold`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      global_ebpf_pps_threshold: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Per Second |
| Range | [0, 64000000] |

**Description**:

Default value 0 means no limitation.

#### Queue Size of eBPF Collector {#inputs.ebpf.tunning.ebpf_collector_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.ebpf_collector_queue_size`

Upgrade from version <= 6.5.9: `static_config.ebpf-collector-queue-size`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      ebpf_collector_queue_size: 65535
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [4096, 64000000] |

**Description**:

The length of the following queues:
- 0-ebpf-to-ebpf-collector
- 1-proc-event-to-sender
- 1-profile-to-sender

#### eBPF Worker Threads {#inputs.ebpf.tunning.thread_num}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.thread_num`

Upgrade from version <= 6.5.9: `static_config.ebpf.thread-num`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      thread_num: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 1024] |

**Description**:

The number of worker threads refers to how many threads participate
in data processing in user-space. The actual maximal value is the number
of CPU logical cores on the host.

#### eBPF Perf Pages Count {#inputs.ebpf.tunning.perf_pages_count}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.perf_pages_count`

Upgrade from version <= 6.5.9: `static_config.ebpf.perf-pages-count`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      perf_pages_count: 128
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [32, 8192] |

**Description**:

The number of page occupied by the shared memory of the kernel. The
value is `2^n (5 <= n <= 13)`. Used for perf data transfer. If the
value is between `2^n` and `2^(n+1)`, it will be automatically adjusted
by the ebpf configurator to the minimum value `2^n`.

#### eBPF Dispatcher Ring Size {#inputs.ebpf.tunning.ring_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.ring_size`

Upgrade from version <= 6.5.9: `static_config.ebpf.ring-size`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      ring_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [8192, 131072] |

**Description**:

The size of the ring cache queue, The value is `2^n (13 <= n <= 17)`.
If the value is between `2^n` and `2^(n+1)`, it will be automatically
adjusted by the ebpf configurator to the minimum value `2^n`.

#### eBPF Max Socket Entries {#inputs.ebpf.tunning.max_socket_entries}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.max_socket_entries`

Upgrade from version <= 6.5.9: `static_config.ebpf.max-socket-entries`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      max_socket_entries: 131072
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [100000, 2000000] |

**Description**:

Set the maximum value of hash table entries for socket tracking, depending
on the number of concurrent requests in the actual scenario

#### eBPF Socket Map Max Reclaim {#inputs.ebpf.tunning.socket_map_max_reclaim}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.socket_map_max_reclaim`

Upgrade from version <= 6.5.9: `static_config.ebpf.socket-map-max-reclaim`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      socket_map_max_reclaim: 120000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [100000, 2000000] |

**Description**:

The maximum threshold for cleaning socket map table entries.

#### eBPF Max Trace Entries {#inputs.ebpf.tunning.max_trace_entries}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.ebpf.tunning.max_trace_entries`

Upgrade from version <= 6.5.9: `static_config.ebpf.max-trace-entries`

**Default value**:
```yaml
inputs:
  ebpf:
    tunning:
      max_trace_entries: 131072
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [100000, 2000000] |

**Description**:

Set the maximum value of hash table entries for thread/coroutine tracking sessions.

### Preprocess {#inputs.ebpf.preprocess}

#### OOOR (Out-Of-Order-Reassembly) Cache Size {#inputs.ebpf.preprocess.syscall_out_of_order_cache_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.preprocess.syscall_out_of_order_cache_size`

Upgrade from version <= 6.5.9: `static_config.ebpf.syscall-out-of-order-cache-size`

**Default value**:
```yaml
inputs:
  ebpf:
    preprocess:
      syscall_out_of_order_cache_size: 16
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [8, 1024] |

**Description**:

When `syscall-out-of-order-reassembly` is enabled, up to `syscall-out-of-order-cache-size`
eBPF socket events (each event consuming up to `l7_log_packet_size` bytes) will be cached
in each TCP/UDP flow to prevent out-of-order events from impacting application protocol
parsing. Since eBPF socket events are sent to user space in batches, out-of-order scenarios
mainly occur when requests and responses within a single session are processed by different
CPUs, causing the response to reach user space before the request.

#### OOOR (Out-Of-Order-Reassembly) Enabled Protocols {#inputs.ebpf.preprocess.syscall_out_of_order_reassembly}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.preprocess.syscall_out_of_order_reassembly`

Upgrade from version <= 6.5.9: `static_config.ebpf.syscall-out-of-order-reassembly`

**Default value**:
```yaml
inputs:
  ebpf:
    preprocess:
      syscall_out_of_order_reassembly: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

When this capability is enabled for a specific application protocol, the agent will add
out-of-order-reassembly processing for it. Note that the agent will consume more memory
in this case, so please adjust the syscall-out-of-order-cache-size accordingly and monitor
the agent's memory usage.

Supported protocols: https://www.deepflow.io/docs/features/l7-protocols/overview/

Attention: use `HTTP2` for `gRPC` Protocol.

#### SR (Segmentation-Reassembly) Enabled Protocols {#inputs.ebpf.preprocess.syscall_segmentation_reassembly}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`inputs.ebpf.preprocess.syscall_segmentation_reassembly`

Upgrade from version <= 6.5.9: `static_config.ebpf.syscall-segmentation-reassembly`

**Default value**:
```yaml
inputs:
  ebpf:
    preprocess:
      syscall_segmentation_reassembly: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

When this capability is enabled for a specific application protocol, the agent will add
segmentation-reassembly processing to merge application protocol content spread across
multiple syscalls before parsing it. This enhances the success rate of application
protocol parsing. Note that `syscall-out-of-order-reassembly` must also be enabled for
this feature to be effective.

Supported protocols: https://www.deepflow.io/docs/features/l7-protocols/overview/

Attention: use `HTTP2` for `gRPC` Protocol.

## Procs {#inputs.procs}

### Enabled {#inputs.procs.os_proc_sync_enabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_sync_enabled`

Upgrade from version <= 6.5.9: `static_config.os-proc-sync-enabled`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_sync_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Only make sense when agent type is one of CHOST_VM, CHOST_BM, K8S_VM, K8S_BM.

### Sync Tagged Proc Only {#inputs.procs.os_proc_sync_tagged_only}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_sync_tagged_only`

Upgrade from version <= 6.5.9: `static_config.os-proc-sync-tagged-only`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_sync_tagged_only: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Only sync process with tag.

### /proc FS Mount Path {#inputs.procs.os_proc_root}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_root`

Upgrade from version <= 6.5.9: `static_config.os-proc-root`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_root: /proc
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The /proc fs mount path.

### Socket and Process Sync Interval {#inputs.procs.os_proc_socket_sync_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_socket_sync_interval`

Upgrade from version <= 6.5.9: `static_config.os-proc-socket-sync-interval`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_socket_sync_interval: 10s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1h'] |

**Description**:

The interval of socket info sync.

### Socket and Process Uptime Threshold {#inputs.procs.os_proc_socket_min_lifetime}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_socket_min_lifetime`

Upgrade from version <= 6.5.9: `static_config.os-proc-socket-min-lifetime`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_socket_min_lifetime: 3s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1h'] |

**Description**:

Socket and Process uptime threshold

### Process Tag Extraction Command {#inputs.procs.os_app_tag_exec}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_app_tag_exec`

Upgrade from version <= 6.5.9: `static_config.os-app-tag-exec`

**Default value**:
```yaml
inputs:
  procs:
    os_app_tag_exec: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Execute the command every time when scan the process, expect get the process tag
from stdout in yaml format, the example yaml format as follow:
```
- pid: 1
  tags:
  - key: xxx
    value: xxx
- pid: 2
  tags:
  - key: xxx
    value: xxx
```
Example configuration: `os_app_tag_exec: ["cat", "/tmp/tag.yaml"]`

### Username to Execute os_app_tag_exec {#inputs.procs.os_app_tag_exec_user}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_app_tag_exec_user`

Upgrade from version <= 6.5.9: `static_config.os-app-tag-exec-user`

**Default value**:
```yaml
inputs:
  procs:
    os_app_tag_exec_user: deepflow
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

The user who should execute the `os-app-tag-exec` command.

### Match Process and Rewrite Name {#inputs.procs.os_proc_regex}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.procs.os_proc_regex`

Upgrade from version <= 6.5.9: `static_config.os-proc-regex`

**Default value**:
```yaml
inputs:
  procs:
    os_proc_regex:
    - action: null
      match_regex: null
      match_type: null
      rewrite_name: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Will traverse over the entire array, so the previous ones will be matched first.
when match-type is parent_process_name, will recursive to match parent proc name, and rewrite-name field will ignore.
rewrite-name can replace by regexp capture group and windows style environment variable, for example:
`$1-py-script-%HOSTNAME%` will replace regexp capture group 1 and HOSTNAME env var.
if proc not match any regexp will be accepted (essentially will auto append '- match-regex: .*' at the end).

Configuration Item:
- match_regex: The regexp use for match the process, default value is `.*`
- match_type: regexp match field, default value is `process_name`, options are
  [process_name, cmdline, parent_process_name]
- action: Action when regex match, default value is `accept`, options are
  [accept, drop]
- rewrite_name: The name will replace the process name or cmd use regexp replace.
  Default value "" means no replacement.

Example:
```
os-proc-regex:
  - match-regex: python3 (.*)\.py
    match-type: cmdline
    action: accept
    rewrite-name: $1-py-script
  - match-regex: (?P<PROC_NAME>nginx)
    match-type: process_name
    rewrite-name: ${PROC_NAME}-%HOSTNAME%
  - match-regex: "nginx"
    match-type: parent_process_name
    action: accept
  - match-regex: .*sleep.*
    match-type: process_name
    action: drop
  - match-regex: .+ # match after concatenating a tag key and value pair using colon, i.e., an regex `app:.+` can match all processes has a `app` tag
    match-type: tag
    action: accept
```

## Tags {#inputs.tags}

### Platform Sync Interval {#inputs.tags.platform_sync_interval}

**Tags**:

`hot_update`

**FQCN**:

`inputs.tags.platform_sync_interval`

Upgrade from version <= 6.5.9: `platform_sync_interval`

**Default value**:
```yaml
inputs:
  tags:
    platform_sync_interval: 10s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10s', '3600s'] |

**Description**:

The interval at which deepflow-agent actively reports resource information
to deepflow-server.

### Cloud {#inputs.tags.cloud}

#### KVM/Host Metadata Collection {#inputs.tags.cloud.platform_enabled}

**Tags**:

`hot_update`

**FQCN**:

`inputs.tags.cloud.platform_enabled`

Upgrade from version <= 6.5.9: `platform_enabled`

**Default value**:
```yaml
inputs:
  tags:
    cloud:
      platform_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When enabled, deepflow-agent will automatically synchronize virtual
machine and network information on the KVM (or Host) to deepflow-server.

#### VM MAC Address Extraction {#inputs.tags.cloud.if_mac_source}

**Tags**:

`hot_update`

**FQCN**:

`inputs.tags.cloud.if_mac_source`

Upgrade from version <= 6.5.9: `if_mac_source`

**Default value**:
```yaml
inputs:
  tags:
    cloud:
      if_mac_source: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | Interface MAC Address |
| 1 | Interface Name |
| 2 | Qemu XML File |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

How to extract the real MAC address of the virtual machine when the
agent runs on the KVM host.

Explanation of the options:
- 0: extracted from tap interface MAC address
- 1: extracted from tap interface name
- 2: extracted from the XML file of the virtual machine

#### VM XML File Directory {#inputs.tags.cloud.vm_xml_path}

**Tags**:

`hot_update`

**FQCN**:

`inputs.tags.cloud.vm_xml_path`

Upgrade from version <= 6.5.9: `vm_xml_path`

**Default value**:
```yaml
inputs:
  tags:
    cloud:
      vm_xml_path: /etc/libvirt/qemu/
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |
| Range | [0, 100] |

**Description**:

VM XML file directory.

#### TAP MAC Mapping Script {#inputs.tags.cloud.tap_mac_script}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.cloud.tap_mac_script`

Upgrade from version <= 6.5.9: `static_config.tap-mac-script`

**Default value**:
```yaml
inputs:
  tags:
    cloud:
      tap_mac_script: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |
| Range | [0, 100] |

**Description**:

The MAC address mapping relationship of TAP NIC in complex environment can be
constructed by writing a script. The following conditions must be met to use this
script:
1. if_mac_source = 2
2. tap_mode = 0
3. The name of the TAP NIC is the same as in the virtual machine XML file
4. The format of the script output is as follows:
   - tap2d283dfe,11:22:33:44:55:66
   - tap2d283223,aa:bb:cc:dd:ee:ff

### Kubernetes {#inputs.tags.kubernetes}

#### K8s Namespace {#inputs.tags.kubernetes.kubernetes_namespace}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.kubernetes.kubernetes_namespace`

Upgrade from version <= 6.5.9: `static_config.kubernetes-namespace`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      kubernetes_namespace: null
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Used when deepflow-agent has only one k8s namespace query permission.

#### K8s api resources {#inputs.tags.kubernetes.kubernetes_resources}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.kubernetes.kubernetes_resources`

Upgrade from version <= 6.5.9: `static_config.kubernetes-resources`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      kubernetes_resources:
      - name: namespaces
      - name: nodes
      - name: pods
      - name: replicationcontrollers
      - name: services
      - name: daemonsets
      - name: deployments
      - name: replicasets
      - name: statefulsets
      - name: ingresses
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Specify kubernetes resources to watch.

The schematics of entries in list is:
```
name: string
group: string
version: string
disabled: bool (default false)
field-selector: string
```

To disable a resource, add an entry to the list with `disabled: true`:
```
kubernetes-resources:
- name: services
  disabled: true
```

To enable a resource, add an entry of this resource to the list. Be advised that
this setting overrides the default of the same resource. For example, to enable
`statefulsets` in both group `apps` (the default) and `apps.kruise.io` will require
two entries:
```
kubernetes-resources:
- name: statefulsets
  group: apps
- name: statefulsets
  group: apps.kruise.io
  version: v1beta1
```

To watching `routes` in openshift you can use the following settings:
```
kubernetes-resources:
- name: ingresses
  disabled: true
- name: routes
```

#### K8s api list limit {#inputs.tags.kubernetes.kubernetes_api_list_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.kubernetes.kubernetes_api_list_limit`

Upgrade from version <= 6.5.9: `static_config.kubernetes-api-list-limit`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      kubernetes_api_list_limit: 1000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [10, 4294967295] |

**Description**:

Used when limit k8s api list entry size.

#### K8s api list interval {#inputs.tags.kubernetes.kubernetes_api_list_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.kubernetes.kubernetes_api_list_interval`

Upgrade from version <= 6.5.9: `static_config.kubernetes-api-list-interval`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      kubernetes_api_list_interval: 10m
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10m', '30d'] |

**Description**:

Interval of listing resource when watcher idles

#### Ingress Flavour {#inputs.tags.kubernetes.ingress_flavour}

**Tags**:

<mark></mark>
<mark>deprecated</mark>

**FQCN**:

`inputs.tags.kubernetes.ingress_flavour`

Upgrade from version <= 6.5.9: `static_config.ingress-flavour`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      ingress_flavour: kubernetes
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

#### Pod MAC/IP Address Query Method {#inputs.tags.kubernetes.kubernetes_poller_type}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.tags.kubernetes.kubernetes_poller_type`

Upgrade from version <= 6.5.9: `static_config.kubernetes-poller-type`

**Default value**:
```yaml
inputs:
  tags:
    kubernetes:
      kubernetes_poller_type: adaptive
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| adaptive | |
| active | |
| passive | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

In active mode, deepflow-agent enters the netns of other Pods through
setns syscall to query the MAC and IP addresses. In this mode, the setns
operation requires the SYS_ADMIN permission. In passive mode deepflow-agent
calculates the MAC and IP addresses used by Pods by capturing ARP/ND traffic.
When set to adaptive, active mode will be used first.

## Integration {#inputs.integration}

### Enabled {#inputs.integration.external_agent_http_proxy_enabled}

**Tags**:

`hot_update`

**FQCN**:

`inputs.integration.external_agent_http_proxy_enabled`

Upgrade from version <= 6.5.9: `external_agent_http_proxy_enabled`

**Default value**:
```yaml
inputs:
  integration:
    external_agent_http_proxy_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to enable receiving external data sources such as Prometheus,
Telegraf, OpenTelemetry, and SkyWalking.

### Listen Port {#inputs.integration.external_agent_http_proxy_port}

**Tags**:

`hot_update`

**FQCN**:

`inputs.integration.external_agent_http_proxy_port`

Upgrade from version <= 6.5.9: `external_agent_http_proxy_port`

**Default value**:
```yaml
inputs:
  integration:
    external_agent_http_proxy_port: 38086
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

Listen port of the data integration socket.

### Integration Data Compression {#inputs.integration.external_agent_http_proxy_compressed}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.integration.external_agent_http_proxy_compressed`

Upgrade from version <= 6.5.9: `static_config.external-agent-http-proxy-compressed`

**Default value**:
```yaml
inputs:
  integration:
    external_agent_http_proxy_compressed: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to compress the integrated data received by deepflow-agent. Currently,
only opentelemetry data is supported, and the compression ratio is about 5:1~10:1.
Turning on this feature will result in higher CPU consumption of deepflow-agent.

### Prometheus Extra Labels {#inputs.integration.prometheus_extra_config}

Support for getting extra labels from headers in http requests from RemoteWrite.

#### Enabled {#inputs.integration.prometheus_extra_config.enabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.integration.prometheus_extra_config.enabled`

Upgrade from version <= 6.5.9: `static_config.prometheus-extra-config.enabled`

**Default value**:
```yaml
inputs:
  integration:
    prometheus_extra_config:
      enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Prometheus extra labels switch.

#### Extra Labels {#inputs.integration.prometheus_extra_config.labels}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.integration.prometheus_extra_config.labels`

Upgrade from version <= 6.5.9: `static_config.prometheus-extra-config.labels`

**Default value**:
```yaml
inputs:
  integration:
    prometheus_extra_config:
      labels: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Labels list. Labels in this list are sent. Label is a string
matching the regular expression `[a-zA-Z_][a-zA-Z0-9_]*`

#### Key Size Limit {#inputs.integration.prometheus_extra_config.labels_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.integration.prometheus_extra_config.labels_limit`

Upgrade from version <= 6.5.9: `static_config.prometheus-extra-config.labels-limit`

**Default value**:
```yaml
inputs:
  integration:
    prometheus_extra_config:
      labels_limit: 1024
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | byte |
| Range | [1024, 1048576] |

**Description**:

The size limit of the parsed key.

#### Value Size Limit {#inputs.integration.prometheus_extra_config.values_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`inputs.integration.prometheus_extra_config.values_limit`

Upgrade from version <= 6.5.9: `static_config.prometheus-extra-config.values-limit`

**Default value**:
```yaml
inputs:
  integration:
    prometheus_extra_config:
      values_limit: 4096
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | byte |
| Range | [4096, 4194304] |

**Description**:

The size limit of the parsed value.

### Feature Control {#inputs.integration.feature_control}

#### Profile Integration {#inputs.integration.feature_control.external_profile_integration_disabled}

**Tags**:

<mark></mark>

**FQCN**:

`inputs.integration.feature_control.external_profile_integration_disabled`

Upgrade from version <= 6.5.9: `static_config.external-profile-integration-disabled`

**Default value**:
```yaml
inputs:
  integration:
    feature_control:
      external_profile_integration_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

#### Trace Integration {#inputs.integration.feature_control.external_trace_integration_disabled}

**Tags**:

<mark></mark>

**FQCN**:

`inputs.integration.feature_control.external_trace_integration_disabled`

Upgrade from version <= 6.5.9: `static_config.external-trace-integration-disabled`

**Default value**:
```yaml
inputs:
  integration:
    feature_control:
      external_trace_integration_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

#### Metric Integration {#inputs.integration.feature_control.external_metric_integration_disabled}

**Tags**:

<mark></mark>

**FQCN**:

`inputs.integration.feature_control.external_metric_integration_disabled`

Upgrade from version <= 6.5.9: `static_config.external-metric-integration-disabled`

**Default value**:
```yaml
inputs:
  integration:
    feature_control:
      external_metric_integration_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

#### Log Integration {#inputs.integration.feature_control.external_log_integration_disabled}

**Tags**:

<mark></mark>

**FQCN**:

`inputs.integration.feature_control.external_log_integration_disabled`

Upgrade from version <= 6.5.9: `static_config.external-log-integration-disabled`

**Default value**:
```yaml
inputs:
  integration:
    feature_control:
      external_log_integration_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

# Precessors {#processors}

## Packet {#processors.packet}

### Policy {#processors.packet.policy}

#### Fast Path Map Size {#processors.packet.policy.fast_path_map_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.policy.fast_path_map_size`

Upgrade from version <= 6.5.9: `static_config.fast-path-map-size`

**Default value**:
```yaml
processors:
  packet:
    policy:
      fast_path_map_size: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

When set to 0, deepflow-agent will automatically adjust the map size
according to max_memory.

#### Fast Path Disabled {#processors.packet.policy.fast_path_disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.policy.fast_path_disabled`

Upgrade from version <= 6.5.9: `static_config.fast-path-disabled`

**Default value**:
```yaml
processors:
  packet:
    policy:
      fast_path_disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When set to true, deepflow-agent will not use fast path.

#### Forward Table Capacity {#processors.packet.policy.forward_capacity}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.policy.forward_capacity`

Upgrade from version <= 6.5.9: `static_config.forward-capacity`

**Default value**:
```yaml
processors:
  packet:
    policy:
      forward_capacity: 16384
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [16384, 64000000] |

**Description**:

When this value is larger, the more memory usage may be.

#### Fast Path Level {#processors.packet.policy.first_path_level}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.policy.first_path_level`

Upgrade from version <= 6.5.9: `static_config.first-path-level`

**Default value**:
```yaml
processors:
  packet:
    policy:
      first_path_level: 8
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 16] |

**Description**:

When this value is larger, the memory overhead is smaller, but the
performance of policy matching is worse.

### TCP Header {#processors.packet.tcp_header}

#### Block Size {#processors.packet.tcp_header.packet_sequence_block_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.tcp_header.packet_sequence_block_size`

Upgrade from version <= 6.5.9: `static_config.packet-sequence-block-size`

**Default value**:
```yaml
processors:
  packet:
    tcp_header:
      packet_sequence_block_size: 256
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [16, 8192] |

**Description**:

When generating TCP header data, each flow uses one block to compress and
store multiple TCP headers, and the block size can be set here.

#### Output Queue Size {#processors.packet.tcp_header.packet_sequence_queue_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.tcp_header.packet_sequence_queue_size`

Upgrade from version <= 6.5.9: `static_config.packet-sequence-queue-size`

**Default value**:
```yaml
processors:
  packet:
    tcp_header:
      packet_sequence_queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues (to UniformCollectSender):
- 1-packet-sequence-block-to-uniform-collect-sender

#### Output Queue Count {#processors.packet.tcp_header.packet_sequence_queue_count}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.tcp_header.packet_sequence_queue_count`

Upgrade from version <= 6.5.9: `static_config.packet-sequence-queue-count`

**Default value**:
```yaml
processors:
  packet:
    tcp_header:
      packet_sequence_queue_count: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 64] |

**Description**:

The number of replicas for each output queue of the PacketSequence.

#### Header Fields {#processors.packet.tcp_header.packet_sequence_flag}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.tcp_header.packet_sequence_flag`

Upgrade from version <= 6.5.9: `static_config.packet-sequence-flag`

**Default value**:
```yaml
processors:
  packet:
    tcp_header:
      packet_sequence_flag: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 255] |

**Description**:

packet-sequence-flag determines which fields need to be reported, the default
value is 0, which means the feature is disabled, and 255, which means all fields
need to be reported all fields corresponding to each bit:
```
| FLAG | SEQ | ACK | PAYLOAD_SIZE | WINDOW_SIZE | OPT_MSS | OPT_WS | OPT_SACK |
8      7     6     5              4             3         2        1          0
```

### PCAP {#processors.packet.pcap}

#### Queue Size to PCAP Generator {#processors.packet.pcap.queue_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.pcap.queue_size`

Upgrade from version <= 6.5.9: `static_config.pcap.queue-size`

**Default value**:
```yaml
processors:
  packet:
    pcap:
      queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues:
- 1-mini-meta-packet-to-pcap

#### Buffer Size for Each Flow {#processors.packet.pcap.flow_buffer_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.pcap.flow_buffer_size`

Upgrade from version <= 6.5.9: `static_config.pcap.flow-buffer-size`

**Default value**:
```yaml
processors:
  packet:
    pcap:
      flow_buffer_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [64, 64000000] |

**Description**:

Buffer flushes when one of the flows reach this limit.

#### Total Buffer Size {#processors.packet.pcap.buffer_size}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.pcap.buffer_size`

Upgrade from version <= 6.5.9: `static_config.pcap.buffer-size`

**Default value**:
```yaml
processors:
  packet:
    pcap:
      buffer_size: 88304
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

Buffer flushes when total data size reach this limit,
cannot exceed sender buffer size 128K.

#### Flow Flush Interval {#processors.packet.pcap.flush_interval}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.packet.pcap.flush_interval`

Upgrade from version <= 6.5.9: `static_config.pcap.flush-interval`

**Default value**:
```yaml
processors:
  packet:
    pcap:
      flush_interval: 1m
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '10m'] |

**Description**:

Flushes a flow if its first packet were older then this interval.

### TOA (TCP Option Address) {#processors.packet.toa}

#### Queue Size of TOA Sync {#processors.packet.toa.toa_sender_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.toa.toa_sender_queue_size`

Upgrade from version <= 6.5.9: `static_config.toa-sender-queue-size`

**Default value**:
```yaml
processors:
  packet:
    toa:
      toa_sender_queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

FIXME

#### TOA Cache Size {#processors.packet.toa.toa_lru_cache_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.packet.toa.toa_lru_cache_size`

Upgrade from version <= 6.5.9: `static_config.toa-lru-cache-size`

**Default value**:
```yaml
processors:
  packet:
    toa:
      toa_lru_cache_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 64000000] |

**Description**:

Size of tcp option address info cache size.

## Request Log {#processors.request_log}

### Application Protocol Inference {#processors.request_log.inference}

#### Maximum Fail Count {#processors.request_log.inference.l7_protocol_inference_max_fail_count}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.l7_protocol_inference_max_fail_count`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-inference-max-fail-count`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      l7_protocol_inference_max_fail_count: 5
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 10000] |

**Description**:

deepflow-agent will mark the long live stream and application protocol for each
<vpc, ip, protocol, port> tuple, when the traffic corresponding to a tuple fails
to be identified for many times (for multiple packets, Socket Data, Function Data),
the tuple will be marked as an unknown type to avoid deepflow-agent continuing to
try (incurring significant computational overhead) until the duration exceeds
l7-protocol-inference-ttl.

#### TTL of Inference Result {#processors.request_log.inference.l7_protocol_inference_ttl}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.l7_protocol_inference_ttl`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-inference-ttl`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      l7_protocol_inference_ttl: 60
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | [0, '1d'] |

**Description**:

deepflow-agent will mark the application protocol for each
<vpc, ip, protocol, port> tuple. In order to avoid misidentification caused by IP
changes, the validity period after successfully identifying the protocol will be
limited to this value.

#### Enabled Application Protocols {#processors.request_log.inference.l7_protocol_enabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.l7_protocol_enabled`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-enabled`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      l7_protocol_enabled:
      - HTTP
      - HTTP2
      - Dubbo
      - SofaRPC
      - FastCGI
      - bRPC
      - MySQL
      - PostgreSQL
      - Oracle
      - Redis
      - MongoDB
      - Kafka
      - MQTT
      - AMQP
      - OpenWire
      - NATS
      - Pulsar
      - ZMTP
      - DNS
      - TLS
      - Custom
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Turning off some protocol identification can reduce deepflow-agent resource consumption.

Supported protocols: https://www.deepflow.io/docs/features/l7-protocols/overview/

<mark>Oracle and TLS is only supported in the Enterprise Edition.</mark>

#### Allowed Port Numbers {#processors.request_log.inference.l7_protocol_ports}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.l7_protocol_ports`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-ports`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      l7_protocol_ports:
        AMQP: 1-65535
        Custom: 1-65535
        DNS: 53,5353
        Dubbo: 1-65535
        FastCGI: 1-65535
        HTTP: 1-65535
        HTTP2: 1-65535
        Kafka: 1-65535
        MQTT: 1-65535
        MongoDB: 1-65535
        MySQL: 1-65535
        NATS: 1-65535
        OpenWire: 1-65535
        Oracle: 1521
        PostgreSQL: 1-65535
        Pulsar: 1-65535
        Redis: 1-65535
        SofaRPC: 1-65535
        TLS: 443,6443
        ZMTP: 1-65535
        bRPC: 1-65535
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Port-list example: 80,1000-2000

HTTP2 and TLS are only used for kprobe, not applicable to uprobe.
All data obtained through uprobe is not subject to port restrictions.

Supported protocols: https://www.deepflow.io/docs/features/l7-protocols/overview/

<mark>Oracle and TLS is only supported in the Enterprise Edition.</mark>

Attention: use `HTTP2` for `gRPC` Protocol.

#### Oracle Wire Protocol {#processors.request_log.inference.oracle_protocol_configs}

##### Integer Byte Order {#processors.request_log.inference.oracle_protocol_configs.is_be}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.oracle_protocol_configs.is_be`

Upgrade from version <= 6.5.9: `static_config.oracle-parse-config.is-be`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      oracle_protocol_configs:
        is_be: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether the oracle integer encode is big endian.

##### Integer Compressed {#processors.request_log.inference.oracle_protocol_configs.int_compress}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.oracle_protocol_configs.int_compress`

Upgrade from version <= 6.5.9: `static_config.oracle-parse-config.int-compress`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      oracle_protocol_configs:
        int_compress: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether the oracle integer encode is compress.

##### Response with ID 0x04 {#processors.request_log.inference.oracle_protocol_configs.resp_0x04_extra_byte}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.inference.oracle_protocol_configs.resp_0x04_extra_byte`

Upgrade from version <= 6.5.9: `static_config.oracle-parse-config.resp-0x04-extra-byte`

**Default value**:
```yaml
processors:
  request_log:
    inference:
      oracle_protocol_configs:
        resp_0x04_extra_byte: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Due to the response with data id 0x04 has different struct in
different version, it may has one byte before row affect.

### Filters {#processors.request_log.filters}

#### Blacklist {#processors.request_log.filters.l7_log_blacklist}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.filters.l7_log_blacklist`

Upgrade from version <= 6.5.9: `static_config.l7-log-blacklist`

**Default value**:
```yaml
processors:
  request_log:
    filters:
      l7_log_blacklist:
        AMQP: []
        DNS: []
        Dubbo: []
        FastCGI: []
        HTTP: []
        HTTP2: []
        Kafka: []
        MQTT: []
        MongoDB: []
        MySQL: []
        NATS: []
        OpenWire: []
        Oracle: []
        PostgreSQL: []
        Pulsar: []
        Redis: []
        SOFARPC: []
        TLS: []
        ZMTP: []
        bRPC: []
        gRPC: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Blacklist example:
```
HTTP:
- field-name: request_resource  # endpoint, request_type, request_domain, request_resource
  operator: equal               # equal, prefix
  value: somevalue
```
A l7_flow_log blacklist can be configured for each protocol, preventing request logs matching
the blacklist from being collected by the agent or included in application performance metrics.
It's recommended to only place non-business request logs like heartbeats or health checks in this
blacklist. Including business request logs might lead to breaks in the distributed tracing tree.

Supported protocols: https://www.deepflow.io/docs/features/l7-protocols/overview/

<mark>Oracle and TLS is only supported in the Enterprise Edition.</mark>

#### Unconcerned DNS NXDOMAIN Responses {#processors.request_log.filters.unconcerned_dns_nxdomain_response_suffixes}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.filters.unconcerned_dns_nxdomain_response_suffixes`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-advanced-features.unconcerned-dns-nxdomain-response-suffixes`

**Default value**:
```yaml
processors:
  request_log:
    filters:
      unconcerned_dns_nxdomain_response_suffixes: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

You might not be concerned about certain DNS NXDOMAIN errors and may wish to ignore
them. For example, when a K8s Pod tries to resolve an external domain name, it first
concatenates it with the internal domain suffix of the cluster and attempts to resolve
it. All these attempts will receive an NXDOMAIN reply before it finally requests the
original domain name directly, and these errors may not be of concern to you. In such
cases, you can configure their `response_result` suffix here, so that the corresponding
`response_status` in the l7_flow_log is forcibly set to `Success`.

### Timeouts {#processors.request_log.timeouts}

#### Maximal Duration - TCP {#processors.request_log.timeouts.rrt_tcp_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.timeouts.rrt_tcp_timeout`

Upgrade from version <= 6.5.9: `static_config.rrt-tcp-timeout`

**Default value**:
```yaml
processors:
  request_log:
    timeouts:
      rrt_tcp_timeout: 1800s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10s', '3600s'] |

**Description**:

The timeout of l7 log info rrt calculate, when rrt exceed the value will act as timeout and will not
calculate the sum and average and will not merge the request and response in session aggregate. the value
must greater than session aggregate SLOT_TIME (const 10s) and less than 3600 on tcp.

#### Maximal Duration - UDP {#processors.request_log.timeouts.rrt_udp_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.timeouts.rrt_udp_timeout`

Upgrade from version <= 6.5.9: `static_config.rrt-udp-timeout`

**Default value**:
```yaml
processors:
  request_log:
    timeouts:
      rrt_udp_timeout: 150s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['10s', '300s'] |

**Description**:

The timeout of l7 log info rrt calculate, when rrt exceed the value will act as timeout and will not
calculate the sum and average and will not merge the request and response in session aggregate. the value
must greater than session aggregate SLOT_TIME (const 10s) and less than 300 on udp.

#### Aggregate Window {#processors.request_log.timeouts.l7_log_session_aggr_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.timeouts.l7_log_session_aggr_timeout`

Upgrade from version <= 6.5.9: `static_config.l7-log-session-aggr-timeout`

**Default value**:
```yaml
processors:
  request_log:
    timeouts:
      l7_log_session_aggr_timeout: 120s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['20s', '300s'] |

**Description**:

l7_flow_log aggregate window.

### Tags {#processors.request_log.tags}

#### Tracing {#processors.request_log.tags.tracing}

##### HTTP Real Client {#processors.request_log.tags.tracing.http_log_proxy_client}

**Tags**:

`hot_update`

**FQCN**:

`processors.request_log.tags.tracing.http_log_proxy_client`

Upgrade from version <= 6.5.9: `http_log_proxy_client`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      tracing:
        http_log_proxy_client: X_Forwarded_For
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

It is used to extract the real client IP field in the HTTP header,
such as X-Forwarded-For, etc. Leave it empty to disable this feature.

##### HTTP X-Request-ID {#processors.request_log.tags.tracing.http_log_x_request_id}

**Tags**:

`hot_update`

**FQCN**:

`processors.request_log.tags.tracing.http_log_x_request_id`

Upgrade from version <= 6.5.9: `http_log_x_request_id`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      tracing:
        http_log_x_request_id: X_Request_ID
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

It is used to extract the fields in the HTTP header that are used
to uniquely identify the same request before and after the gateway,
such as X-Request-ID, etc. This feature can be turned off by setting
it to empty.

##### APM TraceID {#processors.request_log.tags.tracing.http_log_trace_id}

**Tags**:

`hot_update`

**FQCN**:

`processors.request_log.tags.tracing.http_log_trace_id`

Upgrade from version <= 6.5.9: `http_log_trace_id`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      tracing:
        http_log_trace_id:
        - traceparent
        - sw8
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Used to extract the TraceID field in HTTP and RPC headers, supports filling
in multiple values separated by commas. This feature can be turned off by
setting it to empty.

##### APM SpanID {#processors.request_log.tags.tracing.http_log_span_id}

**Tags**:

`hot_update`

**FQCN**:

`processors.request_log.tags.tracing.http_log_span_id`

Upgrade from version <= 6.5.9: `http_log_span_id`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      tracing:
        http_log_span_id:
        - traceparent
        - sw8
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Used to extract the SpanID field in HTTP and RPC headers, supports filling
in multiple values separated by commas. This feature can be turned off by
setting it to empty.

#### HTTP Endpoint Extraction {#processors.request_log.tags.http_endpoint_extraction}

##### Disabled {#processors.request_log.tags.http_endpoint_extraction.disabled}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.tags.http_endpoint_extraction.disabled`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-advanced-features.http-endpoint-extraction.disabled`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      http_endpoint_extraction:
        disabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

HTTP endpoint extration is enabled by default.

##### Match Rules {#processors.request_log.tags.http_endpoint_extraction.match_rules}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.tags.http_endpoint_extraction.match_rules`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-advanced-features.http-endpoint-extraction.match-rules`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      http_endpoint_extraction:
        match_rules:
        - keep_segments: 2
          prefix: ''
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Extract endpoint according to the following rules:
- Find a longest prefix that can match according to the principle of
  "longest prefix matching"
- Intercept the first few paragraphs in URL (the content between two
  / is regarded as one paragraph) as endpoint

By default, two segments are extracted from the URL. For example, the
URL is /a/b/c?query=xxx", whose segment is 3, extracts "/a/b" as the
endpoint.

#### Extra Header Field Extraction {#processors.request_log.tags.extra_log_fields}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.tags.extra_log_fields`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-advanced-features.extra-log-fields`

**Default value**:
```yaml
processors:
  request_log:
    tags:
      extra_log_fields:
        HTTP: []
        HTTP2: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | dict |

**Description**:

Configuration to extract the customized header fields of HTTP, HTTP2, gRPC protocol etc.

Attention: use `HTTP2` for `gRPC` Protocol.

### Transform {#processors.request_log.transform}

#### Obfuscation {#processors.request_log.transform.obfuscate_enabled_protocols}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.transform.obfuscate_enabled_protocols`

Upgrade from version <= 6.5.9: `static_config.l7-protocol-advanced-features.obfuscate-enabled-protocols`

**Default value**:
```yaml
processors:
  request_log:
    transform:
      obfuscate_enabled_protocols:
      - Redis
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| MySQL | |
| PostgreSQL | |
| HTTP | |
| HTTP2 | |
| Redis | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

For the sake of data security, the data of the protocol that needs
to be desensitized is configured here and is not processed by default.

### Tunning {#processors.request_log.tunning}

#### Payload Truncation {#processors.request_log.tunning.l7_log_packet_size}

**Tags**:

`hot_update`

**FQCN**:

`processors.request_log.tunning.l7_log_packet_size`

Upgrade from version <= 6.5.9: `l7_log_packet_size`

**Default value**:
```yaml
processors:
  request_log:
    tunning:
      l7_log_packet_size: 1024
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | byte |
| Range | [256, 65535] |

**Description**:

The maximum data length used for application protocol identification,
note that the effective value is less than or equal to the value of
capture_packet_size.

NOTE: For eBPF data, the largest valid value is 16384.

#### Capacity of Aggregation Time Slot {#processors.request_log.tunning.l7_log_session_slot_capacity}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.request_log.tunning.l7_log_session_slot_capacity`

Upgrade from version <= 6.5.9: `static_config.l7-log-session-slot-capacity`

**Default value**:
```yaml
processors:
  request_log:
    tunning:
      l7_log_session_slot_capacity: 1024
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1024, 1000000] |

**Description**:

By default, unidirectional l7_flow_log is aggregated into bidirectional
request_log (session) with a caching time window of 2 minutes. During this
period, every 5 seconds is considered as a time slot (i.e., a LRU). This
configuration is used to specify the maximum number of unidirectional l7_flow_log
entries that can be cached in each time slot.

If the number of l7_flow_log entries cached in a time slot exceeds this
configuration, 10% of the data in that time slot will be evicted based on the
LRU strategy to reduce memory consumption. Note that the evicted data will not be
discarded; instead, they will be sent to the deepflow-server as unidirectional
request_log.

The following metrics can be used as reference data for adjusting this configuration:
- Metric `deepflow_system.deepflow_agent_l7_session_aggr.cached-request-resource`
  Used to record the total memory occupied by the request_resource field of the
  unidirectional l7_flow_log cached in all time slots at the current moment, in bytes.
- Metric `deepflow_system.deepflow_agent_l7_session_aggr.over-limit`
  Used to record the number of times eviction is triggered due to reaching the
  LRU capacity limit.

## Flow Metrics {#processors.flow_metrics}

### Time Window {#processors.flow_metrics.time_window}

#### Extra Tolerance for Raw FlowLog {#processors.flow_metrics.time_window.second_flow_extra_delay_second}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.time_window.second_flow_extra_delay_second`

Upgrade from version <= 6.5.9: `static_config.second-flow-extra-delay-second`

**Default value**:
```yaml
processors:
  flow_metrics:
    time_window:
      second_flow_extra_delay_second: 0s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '10s'] |

**Description**:

Extra tolerance for QuadrupleGenerator receiving 1s-FlowLog.

#### Maximum Tolerable Packet Delay {#processors.flow_metrics.time_window.packet_delay}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.time_window.packet_delay`

Upgrade from version <= 6.5.9: `static_config.packet-delay`

**Default value**:
```yaml
processors:
  flow_metrics:
    time_window:
      packet_delay: 1s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '10s'] |

**Description**:

Extra tolerance for QuadrupleGenerator receiving 1s-FlowLog.

### Connection Tracking (a.k.a. Flow Map) {#processors.flow_metrics.conntrack}

#### Flush Interval of FlowMap Output Queue {#processors.flow_metrics.conntrack.flush_interval}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.flush_interval`

Upgrade from version <= 6.5.9: `static_config.flow.flush-interval`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      flush_interval: 1s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1m'] |

**Description**:

Flush interval of the queue connected to the collector.

#### Flow Key {#processors.flow_metrics.conntrack.flow_key}

##### Ignore MAC when Generate Flow {#processors.flow_metrics.conntrack.flow_key.ignore_tor_mac}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.flow_key.ignore_tor_mac`

Upgrade from version <= 6.5.9: `static_config.flow.ignore-tor-mac`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      flow_key:
        ignore_tor_mac: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When the MAC addresses of the two-way traffic collected at the same
location are asymmetrical, the traffic cannot be aggregated into a Flow.
You can set this value at this time. Only valid for Cloud (not IDC) traffic.

##### Ignore L2End when Generate Flow {#processors.flow_metrics.conntrack.flow_key.ignore_l2_end}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.flow_key.ignore_l2_end`

Upgrade from version <= 6.5.9: `static_config.flow.ignore-l2-end`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      flow_key:
        ignore_l2_end: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

For Cloud traffic, only the MAC address corresponding to the side with
L2End = true is matched when generating the flow. Set this value to true to
force a double-sided MAC address match and only aggregate traffic with
exactly equal MAC addresses.

##### Ignore VLAN when Generate Flow {#processors.flow_metrics.conntrack.flow_key.ignore_idc_vlan}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`processors.flow_metrics.conntrack.flow_key.ignore_idc_vlan`

Upgrade from version <= 6.5.9: `static_config.flow.ignore-idc-vlan`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      flow_key:
        ignore_idc_vlan: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When the VLAN of the two-way traffic collected at the same location
are asymmetrical, the traffic cannot be aggregated into a Flow. You can
set this value at this time. Only valid for IDC (not Cloud) traffic.

#### Timeouts {#processors.flow_metrics.conntrack.timeouts}

##### Established {#processors.flow_metrics.conntrack.timeouts.established_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.timeouts.established_timeout`

Upgrade from version <= 6.5.9: `static_config.flow.established-timeout`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      timeouts:
        established_timeout: 300s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1d'] |

**Description**:

Timeouts for TCP State Machine - Established.

##### Closing Reset {#processors.flow_metrics.conntrack.timeouts.closing_rst_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.timeouts.closing_rst_timeout`

Upgrade from version <= 6.5.9: `static_config.flow.closing-rst-timeout`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      timeouts:
        closing_rst_timeout: 35s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1d'] |

**Description**:

Timeouts for TCP State Machine - Closing Reset.

##### Others {#processors.flow_metrics.conntrack.timeouts.others_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.timeouts.others_timeout`

Upgrade from version <= 6.5.9: `static_config.flow.others-timeout`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      timeouts:
        others_timeout: 5s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1d'] |

**Description**:

Timeouts for TCP State Machine - Others.

##### Opening Reset {#processors.flow_metrics.conntrack.timeouts.opening_rst_timeout}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.conntrack.timeouts.opening_rst_timeout`

Upgrade from version <= 6.5.9: `static_config.flow.opening-rst-timeout`

**Default value**:
```yaml
processors:
  flow_metrics:
    conntrack:
      timeouts:
        opening_rst_timeout: 1s
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | duration |
| Range | ['1s', '1d'] |

**Description**:

Timeouts for TCP State Machine - Opening Reset.

### Tunning {#processors.flow_metrics.tunning}

#### HashSlot Size of FlowMap {#processors.flow_metrics.tunning.flow_slots_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.flow_slots_size`

Upgrade from version <= 6.5.9: `static_config.flow.flow-slots-size`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      flow_slots_size: 131072
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1024, 64000000] |

**Description**:

Since FlowAggregator is the first step in all processing, this value
is also widely used in other hash tables such as QuadrupleGenerator,
Collector, etc.

#### Maximum Flow {#processors.flow_metrics.tunning.flow_count_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.flow_count_limit`

Upgrade from version <= 6.5.9: `static_config.flow.flow-count-limit`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      flow_count_limit: 65535
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1024, 64000000] |

**Description**:

Maximum number of flows that can be stored in FlowMap, It will also affect the capacity of
the RRT cache, Example: rrt-cache-capacity = flow-count-limit. When rrt-cache-capacity is
not enough, it will be unable to calculate the rrt of l7.

#### Size of Memory Pool {#processors.flow_metrics.tunning.memory_pool_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.memory_pool_size`

Upgrade from version <= 6.5.9: `static_config.flow.memory-pool-size`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      memory_pool_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1024, 64000000] |

**Description**:

This value is used to set max length of memory pool in FlowMap
Memory pools are used for frequently create and destroy objects like
FlowNode, FlowLog, etc.

#### Max Size of Batched Buffer {#processors.flow_metrics.tunning.batched_buffer_size_limit}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.batched_buffer_size_limit`

Upgrade from version <= 6.5.9: `static_config.batched-buffer-size-limit`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      batched_buffer_size_limit: 131072
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1024, 64000000] |

**Description**:

Only TaggedFlow allocation is affected at the moment.
Structs will be allocated in batch to minimalize malloc calls.
Total memory size of a batch will not exceed this limit.
A number larger than 128K is not recommended because the default
MMAP_THRESHOLD is 128K, allocating chunks larger than 128K will
result in calling mmap and more page faults.

#### Queue Size of FlowAggregator {#processors.flow_metrics.tunning.flow_aggr_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.flow_aggr_queue_size`

Upgrade from version <= 6.5.9: `static_config.flow.flow-aggr-queue-size`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      flow_aggr_queue_size: 65535
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues:
- 2-second-flow-to-minute-aggrer

#### Queue Size of FlowGenerator Output {#processors.flow_metrics.tunning.flow_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.flow_queue_size`

Upgrade from version <= 6.5.9: `static_config.flow-queue-size`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      flow_queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues:
- 1-tagged-flow-to-quadruple-generator
- 1-tagged-flow-to-app-protocol-logs
- 0-{flow_type}-{port}-packet-to-tagged-flow (flow_type: sflow, netflow)

#### Queue Size of QuadrupleGenerator Output {#processors.flow_metrics.tunning.quadruple_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`processors.flow_metrics.tunning.quadruple_queue_size`

Upgrade from version <= 6.5.9: `static_config.quadruple-queue-size`

**Default value**:
```yaml
processors:
  flow_metrics:
    tunning:
      quadruple_queue_size: 262144
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [262144, 64000000] |

**Description**:

The length of the following queues:
- 2-flow-with-meter-to-second-collector
- 2-flow-with-meter-to-minute-collector

# Outputs {#outputs}

## Socket {#outputs.socket}

### Data Socket Type {#outputs.socket.collector_socket_type}

**Tags**:

`hot_update`

**FQCN**:

`outputs.socket.collector_socket_type`

Upgrade from version <= 6.5.9: `collector_socket_type`

**Default value**:
```yaml
outputs:
  socket:
    collector_socket_type: TCP
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| TCP | |
| UDP | |
| FILE | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

It can only be set to FILE in standalone mode, in which case
l4_flow_log and l7_flow_log will be written to local files.

### PCAP Socket Type {#outputs.socket.compressor_socket_type}

**Tags**:

`hot_update`

**FQCN**:

`outputs.socket.compressor_socket_type`

Upgrade from version <= 6.5.9: `compressor_socket_type`

**Default value**:
```yaml
outputs:
  socket:
    compressor_socket_type: TCP
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| TCP | |
| UDP | |
| RAW_UDP | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

RAW_UDP uses RawSocket to send UDP packets, which has the highest
performance, but there may be compatibility issues in some environments.

### RAW_UDP Sender Performance Optimization {#outputs.socket.enable_qos_bypass}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`outputs.socket.enable_qos_bypass`

Upgrade from version <= 6.5.9: `static_config.enable-qos-bypass`

**Default value**:
```yaml
outputs:
  socket:
    enable_qos_bypass: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When sender uses RAW_UDP to send data, this feature can be enabled to
improve performance. Linux Kernel >= 3.14 is required. Note that the data
sent when this feature is enabled cannot be captured by tcpdump.

## Flow Log {#outputs.flow_log}

### Filters {#outputs.flow_log.filters}

#### Capture Network Types for l4_flow_log {#outputs.flow_log.filters.l4_log_tap_types}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.filters.l4_log_tap_types`

Upgrade from version <= 6.5.9: `l4_log_tap_types`

**Default value**:
```yaml
outputs:
  flow_log:
    filters:
      l4_log_tap_types:
      - 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| -1 | Disabled |
| 0 | All TAPs |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

The list of TAPs to collect l4_flow_log, you can also set a list of TAPs to
be collected.

#### Capture Network Types for l7_flow_log {#outputs.flow_log.filters.l7_log_store_tap_types}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.filters.l7_log_store_tap_types`

Upgrade from version <= 6.5.9: `l7_log_store_tap_types`

**Default value**:
```yaml
outputs:
  flow_log:
    filters:
      l7_log_store_tap_types: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| -1 | Disabled |
| 0 | All TAPs |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

The list of TAPs to collect l7_flow_log, you can also set a list of TAPs to
be collected.

#### Ignored Observation Point for l4_flow_log {#outputs.flow_log.filters.l4_log_ignore_tap_sides}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.filters.l4_log_ignore_tap_sides`

Upgrade from version <= 6.5.9: `l4_log_ignore_tap_sides`

**Default value**:
```yaml
outputs:
  flow_log:
    filters:
      l4_log_ignore_tap_sides: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | rest, Other NIC |
| 1 | c, Client NIC |
| 2 | s, Server NIC |
| 4 | local, Local NIC |
| 9 | c-nd, Client K8s Node |
| 10 | s-nd, Server K8s Node |
| 17 | c-hv, Client VM Hypervisor |
| 18 | s-hv, Server VM Hypervisor |
| 25 | c-gw-hv, Client-side Gateway Hypervisor |
| 26 | s-gw-hv, Server-side Gateway Hypervisor |
| 33 | c-gw, Client-side Gateway |
| 34 | s-gw, Server-side Gateway |
| 41 | c-p, Client Process |
| 42 | s-p, Server Process |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Use the value of tap_side to control which l4_flow_log should be ignored for
collection. This configuration also applies to tcp_sequence and pcap data in
the Enterprise Edition. Default value [] means store everything.

#### Ignored Observation Point for l7_flow_log {#outputs.flow_log.filters.l7_log_ignore_tap_sides}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.filters.l7_log_ignore_tap_sides`

Upgrade from version <= 6.5.9: `l7_log_ignore_tap_sides`

**Default value**:
```yaml
outputs:
  flow_log:
    filters:
      l7_log_ignore_tap_sides: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | rest, Other NIC |
| 1 | c, Client NIC |
| 2 | s, Server NIC |
| 4 | local, Local NIC |
| 9 | c-nd, Client K8s Node |
| 10 | s-nd, Server K8s Node |
| 17 | c-hv, Client VM Hypervisor |
| 18 | s-hv, Server VM Hypervisor |
| 25 | c-gw-hv, Client-side Gateway Hypervisor |
| 26 | s-gw-hv, Server-side Gateway Hypervisor |
| 33 | c-gw, Client-side Gateway |
| 34 | s-gw, Server-side Gateway |
| 41 | c-p, Client Process |
| 42 | s-p, Server Process |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Use the value of tap_side to control which l7_flow_log should be ignored for
collection.

### Throttle {#outputs.flow_log.throttle}

#### Maximum Sending Rate for l4_flow_log {#outputs.flow_log.throttle.l4_log_collect_nps_threshold}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.throttle.l4_log_collect_nps_threshold`

Upgrade from version <= 6.5.9: `l4_log_collect_nps_threshold`

**Default value**:
```yaml
outputs:
  flow_log:
    throttle:
      l4_log_collect_nps_threshold: 10000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Per Second |
| Range | [100, 1000000] |

**Description**:

The maximum number of rows of l4_flow_log sent per second, when the actual
number of rows exceeds this value, sampling is triggered.

#### Maximum Sending Rate for l7_flow_log {#outputs.flow_log.throttle.l7_log_collect_nps_threshold}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_log.throttle.l7_log_collect_nps_threshold`

Upgrade from version <= 6.5.9: `l7_log_collect_nps_threshold`

**Default value**:
```yaml
outputs:
  flow_log:
    throttle:
      l7_log_collect_nps_threshold: 10000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Per Second |
| Range | [100, 1000000] |

**Description**:

The maximum number of rows of l7_flow_log sent per second, when the actual
number of rows exceeds this value, sampling is triggered.

### Tunning {#outputs.flow_log.tunning}

#### Queue Size of FlowAggregator/SessionAggregator Output {#outputs.flow_log.tunning.flow_sender_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`outputs.flow_log.tunning.flow_sender_queue_size`

Upgrade from version <= 6.5.9: `static_config.flow-sender-queue-size`

**Default value**:
```yaml
outputs:
  flow_log:
    tunning:
      flow_sender_queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues:
- 3-flow-to-collector-sender
- 3-protolog-to-collector-sender

#### Queue Count of FlowAggregator/SessionAggregator Output {#outputs.flow_log.tunning.flow_sender_queue_count}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`outputs.flow_log.tunning.flow_sender_queue_count`

Upgrade from version <= 6.5.9: `static_config.flow-sender-queue-count`

**Default value**:
```yaml
outputs:
  flow_log:
    tunning:
      flow_sender_queue_count: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 64] |

**Description**:

The number of replicas for each output queue of the
FlowAggregator/SessionAggregator.

## Flow Metrics {#outputs.flow_metrics}

### Filters {#outputs.flow_metrics.filters}

#### AutoMetrics & AutoLogging {#outputs.flow_metrics.filters.collector_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.collector_enabled`

Upgrade from version <= 6.5.9: `collector_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      collector_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When disabled, deepflow-agent will not send metrics and logging data
collected using eBPF and cBPF.

#### Detailed Metrics for Inactive Port {#outputs.flow_metrics.filters.inactive_server_port_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.inactive_server_port_enabled`

Upgrade from version <= 6.5.9: `inactive_server_port_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      inactive_server_port_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When closed, deepflow-agent will not generate detailed metrics for each
inactive port (ports that only receive data, not send data), and the data of
all inactive ports will be aggregated into the metrics with a tag
'server_port = 0'.

#### Detailed Metrics for Inactive IP Address {#outputs.flow_metrics.filters.inactive_ip_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.inactive_ip_enabled`

Upgrade from version <= 6.5.9: `inactive_ip_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      inactive_ip_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When closed, deepflow-agent will not generate detailed metrics for each
inactive IP address (IP addresses that only receive data, not send data), and
the data of all inactive IP addresses will be aggregated into the metrics with
a tag 'ip = 0'.

#### NPM Metrics {#outputs.flow_metrics.filters.l4_performance_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.l4_performance_enabled`

Upgrade from version <= 6.5.9: `l4_performance_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      l4_performance_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When closed, deepflow-agent only collects some basic throughput metrics.

#### APM Metrics {#outputs.flow_metrics.filters.l7_metrics_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.l7_metrics_enabled`

Upgrade from version <= 6.5.9: `l7_metrics_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      l7_metrics_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

When closed, deepflow-agent will not collect RED (request/error/delay) metrics.

#### 1s Metrics {#outputs.flow_metrics.filters.vtap_flow_1s_enabled}

**Tags**:

`hot_update`

**FQCN**:

`outputs.flow_metrics.filters.vtap_flow_1s_enabled`

Upgrade from version <= 6.5.9: `vtap_flow_1s_enabled`

**Default value**:
```yaml
outputs:
  flow_metrics:
    filters:
      vtap_flow_1s_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Second granularity metrics.

### Tunning {#outputs.flow_metrics.tunning}

#### Queue Size of Collector Output {#outputs.flow_metrics.tunning.collector_sender_queue_size}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`outputs.flow_metrics.tunning.collector_sender_queue_size`

Upgrade from version <= 6.5.9: `static_config.collector-sender-queue-size`

**Default value**:
```yaml
outputs:
  flow_metrics:
    tunning:
      collector_sender_queue_size: 65536
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [65536, 64000000] |

**Description**:

The length of the following queues:
- 2-doc-to-collector-sender

#### Queue Count of Collector Output {#outputs.flow_metrics.tunning.collector_sender_queue_count}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`outputs.flow_metrics.tunning.collector_sender_queue_count`

Upgrade from version <= 6.5.9: `static_config.collector-sender-queue-count`

**Default value**:
```yaml
outputs:
  flow_metrics:
    tunning:
      collector_sender_queue_count: 1
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 64] |

**Description**:

The number of replicas for each output queue of the collector.

## NPB (Network Packet Broker) {#outputs.npb}

### UDP maximum MTU {#outputs.npb.mtu}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.mtu`

Upgrade from version <= 6.5.9: `mtu`

**Default value**:
```yaml
outputs:
  npb:
    mtu: 1500
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | byte |
| Range | [500, 10000] |

**Description**:

Maximum MTU allowed when using UDP to transfer data.

Attention: Public cloud service providers may modify the content of the
tail of the UDP packet whose packet length is close to 1500 bytes. When
using UDP transmission, it is recommended to set a slightly smaller value.

### Raw UDP VLAN Tag {#outputs.npb.output_vlan}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.output_vlan`

Upgrade from version <= 6.5.9: `output_vlan`

**Default value**:
```yaml
outputs:
  npb:
    output_vlan: 0
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 4095] |

**Description**:

When using Raw Socket to transmit UDP data, this value can be used to
set the VLAN tag. Default value 0 means no VLAN tag.

### NPB Socket Type {#outputs.npb.npb_socket_type}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.npb_socket_type`

Upgrade from version <= 6.5.9: `npb_socket_type`

**Default value**:
```yaml
outputs:
  npb:
    npb_socket_type: RAW_UDP
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| UDP | |
| RAW_UDP | |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

RAW_UDP uses RawSocket to send UDP packets, which has the highest
performance, but there may be compatibility issues in some environments.

### Inner Additional Header {#outputs.npb.npb_vlan_mode}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.npb_vlan_mode`

Upgrade from version <= 6.5.9: `npb_vlan_mode`

**Default value**:
```yaml
outputs:
  npb:
    npb_vlan_mode: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | None |
| 1 | 802.1Q |
| 2 | QinQ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Whether to add an extra 802.1Q header to NPB traffic, when this value is
set, deepflow-agent will insert a VLAN Tag into the NPB traffic header, and
the value is the lower 12 bits of TunnelID in the VXLAN header.

### Global Deduplication {#outputs.npb.npb_dedup_enabled}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.npb_dedup_enabled`

Upgrade from version <= 6.5.9: `npb_dedup_enabled`

**Default value**:
```yaml
outputs:
  npb:
    npb_dedup_enabled: true
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Whether to enable global (distributed) traffic deduplication for the
NPB feature.

### Server Port for NPB {#outputs.npb.npb_port}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.npb_port`

Upgrade from version <= 6.5.9: `static_config.npb-port`

**Default value**:
```yaml
outputs:
  npb:
    npb_port: 4789
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [1, 65535] |

**Description**:

Server port for NPB.

### Reserve Flags for VXLAN {#outputs.npb.vxlan_flags}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.vxlan_flags`

Upgrade from version <= 6.5.9: `static_config.vxlan-flags`

**Default value**:
```yaml
outputs:
  npb:
    vxlan_flags: 255
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Range | [0, 255] |

**Description**:

NPB uses the first byte of the VXLAN Flag to identify the sending traffic to
prevent the traffic sent by NPB from being collected by deepflow-agent. To ensure
that the VNI bit is set, the value configured here will be used after |= 0x8.

### Ignoring VLAN Header in Overlay {#outputs.npb.ignore_overlay_vlan}

**Tags**:

<mark>agent_restart</mark>
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.ignore_overlay_vlan`

Upgrade from version <= 6.5.9: `static_config.ignore-overlay-vlan`

**Default value**:
```yaml
outputs:
  npb:
    ignore_overlay_vlan: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

This configuration only ignores the VLAN header in the captured original message
and does not affect the configuration item: npb_vlan_mode

### Tx Traffic Limit {#outputs.npb.max_npb_bps}

**Tags**:

`hot_update`
<mark>ee_feature</mark>

**FQCN**:

`outputs.npb.max_npb_bps`

Upgrade from version <= 6.5.9: `max_npb_bps`

**Default value**:
```yaml
outputs:
  npb:
    max_npb_bps: 1000
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |
| Unit | Mbps |
| Range | [1, 100000] |

**Description**:

Maximum traffic rate allowed for npb sender.

# Plugins {#plugins}

## Wasm Plugins {#plugins.wasm_plugins}

**Tags**:

`hot_update`

**FQCN**:

`plugins.wasm_plugins`

Upgrade from version <= 6.5.9: `wasm_plugins`

**Default value**:
```yaml
plugins:
  wasm_plugins: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Wasm plugin need to load in agent

## .so Plugins {#plugins.so_plugins}

**Tags**:

`hot_update`

**FQCN**:

`plugins.so_plugins`

Upgrade from version <= 6.5.9: `so_plugins`

**Default value**:
```yaml
plugins:
  so_plugins: []
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

so plugin need to load in agent. so plugin use dlopen flag RTLD_LOCAL
and RTLD_LAZY to open the so file, it mean that the so must solve the
link problem by itself

# Server Side {#server_side}

## Request NAT IP {#server_side.nat_ip_enabled}

**Tags**:

`hot_update`

**FQCN**:

`server_side.nat_ip_enabled`

Upgrade from version <= 6.5.9: `nat_ip_enabled`

**Default value**:
```yaml
server_side:
  nat_ip_enabled: false
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | bool |

**Description**:

Used when deepflow-agent uses an external IP address to access
deepflow-server. For example, when deepflow-server is behind a NAT gateway,
or the host where deepflow-server is located has multiple node IP addresses
and different deepflow-agents need to access different node IPs, you can
set an additional NAT IP for each deepflow-server address, and modify this
value to true.

## Resource MAC/IP Address Delivery {#server_side.domains}

**Tags**:

`hot_update`

**FQCN**:

`server_side.domains`

Upgrade from version <= 6.5.9: `domains`

**Default value**:
```yaml
server_side:
  domains:
  - 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| _DYNAMIC_OPTIONS_ | _DYNAMIC_OPTIONS_ |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

Default value 0 means all domains, or can be set to a list of lcuuid of a
series of domains, you can get lcuuid through 'deepflow-ctl domain list'.

Note: The list of MAC and IP addresses is used by deepflow-agent to inject tags
into data. This configuration can reduce the number and frequency of MAC and
IP addresses delivered by deepflow-server to deepflow-agent. When there is no
cross-domain service request, deepflow-server can be configured to only deliver
the information in the local domain to deepflow-agent.

## Pod MAC/IP Address Delivery {#server_side.pod_cluster_internal_ip}

**Tags**:

`hot_update`

**FQCN**:

`server_side.pod_cluster_internal_ip`

Upgrade from version <= 6.5.9: `pod_cluster_internal_ip`

**Default value**:
```yaml
server_side:
  pod_cluster_internal_ip: 0
```

**Enum options**:
| Key  | Name                         |
| ---- | ---------------------------- |
| 0 | All K8s Cluster |
| 1 | Local K8s Cluster |

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | int |

**Description**:

The list of MAC and IP addresses is used by deepflow-agent to inject tags
into data. This configuration can reduce the number and frequency of MAC and IP
addresses delivered by deepflow-server to deepflow-agent. When the Pod IP is not
used for direct communication between the K8s cluster and the outside world,
deepflow-server can be configured to only deliver the information in the local
K8s cluster to deepflow-agent.

# Dev {#dev}

## Feature Flags {#dev.feature_flags}

**Tags**:

<mark>agent_restart</mark>

**FQCN**:

`dev.feature_flags`

Upgrade from version <= 6.5.9: `static_config.feature-flags`

**Default value**:
```yaml
dev:
  feature_flags: []
```

**Schema**:
| Key  | Value                        |
| ---- | ---------------------------- |
| Type | string |

**Description**:

Unreleased deepflow-agent features can be turned on by setting this switch.

