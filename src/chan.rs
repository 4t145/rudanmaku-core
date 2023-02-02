use bilive_danmaku::{event::Event, Connector};
use futures_util::StreamExt;
use log::{error, warn};
use tokio::sync::broadcast;

use crate::{
    config::RoomConfigItem,
    consumer::{
        mongo::MongoConsumer,
        ws::{BincodeConvertor, JsonConvertor, WsConsumer},
        Bus, Consumer, postgres::PgConsumer,
    }
};
const MAX_RETRY_CNT: u64 = 10;
pub struct ConnectionGuard {
    pub guard_handle: tokio::task::JoinHandle<()>,
    reciever: broadcast::Receiver<Event>,
}

impl ConnectionGuard {
    pub async fn start(roomid: u64) -> Result<Self, String> {
        let connector = Connector::init(roomid)
            .await
            .map_err(|_| "初始化失败".to_string())?;
        let (broadcastor, reciever) = broadcast::channel::<Event>(128);
        let guard = async move {
            let mut retry_cnt = 0;
            loop {
                let connect_result = connector.connect().await;
                match connect_result {
                    Ok(mut stream) => {
                        retry_cnt = 0;
                        while let Some(result) = stream.next().await {
                            match result {
                                Ok(evt) => {
                                    broadcastor.send(evt).unwrap();
                                }
                                Err(e) => {
                                    use bilive_danmaku::connection::EventStreamError::*;
                                    match e {
                                        ConnectionClosed => error!("[{roomid}]<连接被关闭>"),
                                        WsError(e) => error!("[{roomid}]<连接错误>{e}"),
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        use tokio::time::*;
                        // 休息500ms
                        sleep(Duration::from_millis(500)).await;
                        let e = match e {
                            bilive_danmaku::ConnectError::HostListIsEmpty => "主机名单为空!",
                            bilive_danmaku::ConnectError::HandshakeError => "握手错误!",
                            bilive_danmaku::ConnectError::WsError(_) => "ws错误",
                        };
                        warn!("[{roomid}]<房间连接失败{retry_cnt}/{MAX_RETRY_CNT}>{e}");
                        retry_cnt += 1;
                        if retry_cnt > MAX_RETRY_CNT {
                            error!("[{roomid}]<房间连接失败>");
                            break;
                        }
                        // log error
                        continue;
                    }
                }
                // cooldown
            }
        };

        Ok(Self {
            guard_handle: tokio::spawn(guard),
            reciever,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.reciever.resubscribe()
    }

    pub fn broadcast_to<C: Consumer>(&self) -> C {
        C::launch(self.subscribe())
    }
}

pub struct RoomChannel {
    pub connection_guard: ConnectionGuard,
    pub bus: crate::consumer::Bus,
}

impl RoomChannel {
    pub async fn init(config: &RoomConfigItem) -> Result<Self, String> {
        match ConnectionGuard::start(config.roomid).await {
            Ok(connection_guard) => {
                let ws_json = config
                    .channel
                    .contains(&String::from("json"))
                    .then_some(connection_guard.broadcast_to::<WsConsumer<JsonConvertor>>());
                let ws_bincode = config
                    .channel
                    .contains(&String::from("bincode"))
                    .then_some(connection_guard.broadcast_to::<WsConsumer<BincodeConvertor>>());
                let db_mongo = config
                    .channel
                    .contains(&String::from("mongo"))
                    .then_some(connection_guard.broadcast_to::<MongoConsumer>());
                let db_pg = config
                    .channel
                    .contains(&String::from("pg"))
                    .then_some(connection_guard.broadcast_to::<PgConsumer>());
                return Ok(Self {
                    connection_guard,
                    bus: Bus {
                        ws_json,
                        ws_bincode,
                        db_mongo,
                        db_pg,
                    },
                });
            }
            Err(e) => return Err(e),
        }
    }
}
