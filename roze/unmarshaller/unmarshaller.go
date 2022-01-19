package unmarshaller

import (
	"errors"
	"fmt"
	"strconv"
	"time"

	logging "github.com/op/go-logging"

	"gitlab.yunshan.net/yunshan/droplet-libs/app"
	"gitlab.yunshan.net/yunshan/droplet-libs/codec"
	"gitlab.yunshan.net/yunshan/droplet-libs/grpc"
	"gitlab.yunshan.net/yunshan/droplet-libs/queue"
	"gitlab.yunshan.net/yunshan/droplet-libs/receiver"
	"gitlab.yunshan.net/yunshan/droplet-libs/stats"
	"gitlab.yunshan.net/yunshan/droplet-libs/utils"
	"gitlab.yunshan.net/yunshan/droplet-libs/zerodoc"
	"gitlab.yunshan.net/yunshan/droplet-libs/zerodoc/pb"
	"gitlab.yunshan.net/yunshan/droplet/roze/dbwriter"
	"gitlab.yunshan.net/yunshan/droplet/roze/msg"
)

var log = logging.MustGetLogger("roze.unmarshaller")

const (
	QUEUE_BATCH_SIZE = 1024
	FLUSH_INTERVAL   = 5
	GET_MAX_SIZE     = 1024
	DOC_TIME_EXCEED  = 300
	HASH_SEED        = 17
)

type QueueCache struct {
	values []interface{}
}

type Counter struct {
	DocCount        int64 `statsd:"doc-count"`
	ErrDocCount     int64 `statsd:"err-doc-count"`
	AverageDelay    int64 `statsd:"average-delay"`
	MaxDelay        int64 `statsd:"max-delay"`
	MinDelay        int64 `statsd:"min-delay"`
	ExpiredDocCount int64 `statsd:"expired-doc-count"`
	FutureDocCount  int64 `statsd:"future-doc-count"`
	DropDocCount    int64 `statsd:"drop-doc-count"`

	FlowCount           int64 `statsd:"vtap-flow"`
	Flow1sCount         int64 `statsd:"vtap-flow-1s"`
	FlowPortCount       int64 `statsd:"vtap-flow-port"`
	FlowPort1sCount     int64 `statsd:"vtap-flow-port-1s"`
	FlowEdgeCount       int64 `statsd:"vtap-flow-edge"`
	FlowEdge1sCount     int64 `statsd:"vtap-flow-edge-1s"`
	FlowEdgePortCount   int64 `statsd:"vtap-flow-edge-port"`
	FlowEdgePort1sCount int64 `statsd:"vtap-flow-edge-port-1s"`
	AclCount            int64 `statsd:"vtap-acl"`
	OtherCount          int64 `statsd:"other-db-count"`
}

type Unmarshaller struct {
	index              int
	platformData       *grpc.PlatformInfoTable
	disableSecondWrite bool
	unmarshallQueue    queue.QueueReader
	dbwriter           *dbwriter.DbWriter
	queueBatchCache    QueueCache
	counter            *Counter
	dbCounter          [msg.MAX_INDEX]int64
	utils.Closable
}

func NewUnmarshaller(index int, platformData *grpc.PlatformInfoTable, disableSecondWrite bool, unmarshallQueue queue.QueueReader, dbwriter *dbwriter.DbWriter) *Unmarshaller {
	return &Unmarshaller{
		index:              index,
		platformData:       platformData,
		disableSecondWrite: disableSecondWrite,
		unmarshallQueue:    unmarshallQueue,
		counter:            &Counter{MaxDelay: -3600, MinDelay: 3600},
		dbwriter:           dbwriter,
	}
}

func max(a, b int64) int64 {
	if a > b {
		return a
	}
	return b
}

func min(a, b int64) int64 {
	if a < b {
		return a
	}
	return b
}

func (u *Unmarshaller) isGoodDocument(docTime int64) bool {
	delay := time.Now().Unix() - docTime
	u.counter.DocCount++
	u.counter.AverageDelay += delay
	u.counter.MaxDelay = max(u.counter.MaxDelay, delay)
	u.counter.MinDelay = min(u.counter.MinDelay, delay)
	if delay > DOC_TIME_EXCEED {
		u.counter.ExpiredDocCount++
		return false
	}
	if delay < -DOC_TIME_EXCEED {
		u.counter.FutureDocCount++
		return false
	}
	return true
}

func (u *Unmarshaller) GetCounter() interface{} {
	var counter *Counter
	counter, u.counter = u.counter, &Counter{MaxDelay: -3600, MinDelay: 3600}

	if counter.DocCount != 0 {
		counter.AverageDelay /= counter.DocCount
	} else {
		counter.MaxDelay = 0
		counter.MinDelay = 0
	}

	counter.FlowCount, u.dbCounter[msg.VTAP_FLOW] = u.dbCounter[msg.VTAP_FLOW], 0
	counter.Flow1sCount, u.dbCounter[msg.VTAP_FLOW_1S] = u.dbCounter[msg.VTAP_FLOW_1S], 0
	counter.FlowPortCount, u.dbCounter[msg.VTAP_FLOW_PORT] = u.dbCounter[msg.VTAP_FLOW_PORT], 0
	counter.FlowPort1sCount, u.dbCounter[msg.VTAP_FLOW_PORT_1S] = u.dbCounter[msg.VTAP_FLOW_PORT_1S], 0
	counter.FlowEdgeCount, u.dbCounter[msg.VTAP_FLOW_EDGE] = u.dbCounter[msg.VTAP_FLOW_EDGE], 0
	counter.FlowEdge1sCount, u.dbCounter[msg.VTAP_FLOW_EDGE_1S] = u.dbCounter[msg.VTAP_FLOW_EDGE_1S], 0
	counter.FlowEdgePortCount, u.dbCounter[msg.VTAP_FLOW_EDGE_PORT] = u.dbCounter[msg.VTAP_FLOW_EDGE_PORT], 0
	counter.FlowEdgePort1sCount, u.dbCounter[msg.VTAP_FLOW_EDGE_PORT_1S] = u.dbCounter[msg.VTAP_FLOW_EDGE_PORT_1S], 0
	counter.AclCount, u.dbCounter[msg.VTAP_ACL] = u.dbCounter[msg.VTAP_ACL], 0
	counter.OtherCount, u.dbCounter[msg.INVALID_INDEX] = u.dbCounter[msg.INVALID_INDEX], 0

	return counter
}

func (u *Unmarshaller) putStoreQueue(si *msg.RozeDocument) {
	queueCache := &u.queueBatchCache
	queueCache.values = append(queueCache.values, si)

	if len(queueCache.values) >= QUEUE_BATCH_SIZE {
		u.dbwriter.Put(queueCache.values...)
		queueCache.values = queueCache.values[:0]
	}
}

func (u *Unmarshaller) flushStoreQueue() {
	queueCache := &u.queueBatchCache
	if len(queueCache.values) > 0 {
		u.dbwriter.Put(queueCache.values...)
		queueCache.values = queueCache.values[:0]
	}
}

func DecodeForQueueMonitor(item interface{}) (interface{}, error) {
	var ret interface{}
	bytes, ok := item.(*receiver.RecvBuffer)
	if !ok {
		return nil, errors.New("only support data(type: RecvBuffer) to unmarshall")
	}

	ret, err := decodeForDebug(bytes.Buffer[bytes.Begin:bytes.End])
	return ret, err
}

type BatchDocument []*app.Document

func (bd BatchDocument) String() string {
	docs := []*app.Document(bd)
	str := fmt.Sprintf("batch msg num=%d\n", len(docs))
	for i, doc := range docs {
		str += fmt.Sprintf("%d%s", i, doc.String())
	}
	return str
}

func decodeForDebug(b []byte) (BatchDocument, error) {
	if b == nil {
		return nil, errors.New("No input byte")
	}

	decoder := &codec.SimpleDecoder{}
	decoder.Init(b)
	docs := make([]*app.Document, 0)

	for !decoder.IsEnd() {
		doc, err := app.DecodeForQueueMonitor(decoder)
		if err != nil {
			return nil, err
		}
		docs = append(docs, doc)
	}
	return BatchDocument(docs), nil
}

func GetDocHashValue(doc *app.Document, encoder *codec.SimpleEncoder) uint64 {
	encoder.Reset()
	tag := doc.Tagger.(*zerodoc.Tag)
	// 分组时tid不同，code相同的doc需要分在一组
	tag.EncodeByCodeTID(tag.Code, 0, encoder)
	return utils.DJBHash(HASH_SEED, encoder.String())
}

func (u *Unmarshaller) QueueProcess() {
	stats.RegisterCountable("unmarshaller", u, stats.OptionStatTags{"thread": strconv.Itoa(u.index)})
	rawDocs := make([]interface{}, GET_MAX_SIZE)
	decoder := &codec.SimpleDecoder{}
	pbDoc := &pb.Document{}
	for {
		n := u.unmarshallQueue.Gets(rawDocs)
		for i := 0; i < n; i++ {
			value := rawDocs[i]
			if recvBytes, ok := value.(*receiver.RecvBuffer); ok {
				bytes := recvBytes.Buffer[recvBytes.Begin:recvBytes.End]
				decoder.Init(bytes)
				for !decoder.Failed() && !decoder.IsEnd() {
					pbDoc.Reset()
					doc, err := app.DecodePB(decoder, pbDoc)
					if err != nil {
						u.counter.ErrDocCount++
						log.Warningf("Decode failed, bytes len=%d err=%s", len([]byte(bytes)), err)
						break
					}
					u.isGoodDocument(int64(doc.Timestamp))

					// 秒级数据是否写入
					if u.disableSecondWrite &&
						doc.Flags&app.FLAG_PER_SECOND_METRICS != 0 {
						app.ReleaseDocument(doc)
						continue
					}

					rd := DocToRozeDocuments(doc, u.platformData)
					if rd == nil {
						u.counter.DropDocCount++
						app.ReleaseDocument(doc)
						continue
					}
					DBIndex := rd.DatabaseIndex()
					u.dbCounter[DBIndex]++

					u.putStoreQueue(rd)
				}
				receiver.ReleaseRecvBuffer(recvBytes)

			} else if value == nil { // flush ticker
				u.flushStoreQueue()
			} else {
				log.Warning("get unmarshall queue data type wrong")
			}
		}
	}
}
