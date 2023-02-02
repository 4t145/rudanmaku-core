use config::Config;
use consumer::ws::SerType;
use diesel::{PgConnection, r2d2};
use log::{error, info};
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
};
use tokio::sync::{oneshot, RwLock};
use tokio_tungstenite::tungstenite::{self as ws2, handshake::server::Callback};

mod chan;
mod config;
mod consumer;
mod netcontrol;
mod schema;
mod model;

use crate::{chan::RoomChannel, consumer::{Consumer, postgres::PgConsumer}};

#[tokio::main]
async fn main() {
    env_logger::init();
    // let rt = tokio::runtime::Builder::new_multi_thread().enable_all()
    // .build().unwrap();
    server(config::get_config()).await;
}

async fn batch_init(config: &Config) -> HashMap<u64, RoomChannel> {
    let mut map = HashMap::new();
    let total = config.room.len();
    let mut counter = 0;
    for room_config in &config.room {
        let roomid = room_config.roomid;
        info!("connecting room[{roomid}]");
        match RoomChannel::init(room_config).await {
            Ok(room_chan) => {
                map.insert(roomid, room_chan);
                counter += 1;
                info!("connected room[{roomid}]: {counter}/{total}");
            }
            Err(e) => {
                error!("fail to connect room[{roomid}] {e}");
            }
        }
        use tokio::time::*;
        // 休息500ms
        sleep(Duration::from_millis(500)).await;
    }
    map
}

pub struct Dbs {
    mongo: Option<mongodb::Database>,
    pg: Option<PgConnection>
}

async fn server(config: config::Config) {
    let port = config.net.port;
    let server_addr = if let Some(ipv4) = &config.net.ipv4 {
        IpAddr::V4(Ipv4Addr::from(ipv4.clone()))
    } else if let Some(ipv6) = &config.net.ipv6 {
        IpAddr::V6(Ipv6Addr::from(ipv6.clone()))
    } else {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    };
    let mongo = if let Some(mongo_config) = &config.db.mongo {
        use mongodb::options::*;
        info!("db connecting");
        let host = mongo_config.host.clone();
        let port = Some(mongo_config.port);
        let options = ClientOptions::builder()
            .hosts(vec![ServerAddress::Tcp { host, port }])
            .build();
        match mongodb::Client::with_options(options) {
            Ok(client) => {
                info!("db connected");
                Some(client.database(mongo_config.db.as_str()))
            }
            Err(e) => {
                error!("cannot connect to db: {e}");
                None
            }
        }
    } else {
        None
    };
    let pg = if let Some(pg_config) = &config.db.pg {
        use diesel::prelude::*;
        info!("init pg db connection pool");
        let manager = r2d2::ConnectionManager::<PgConnection>::new(&pg_config.url);
        let pool_result = r2d2::Pool::builder()
            .test_on_check_out(true)
            .build(manager);
        match pool_result {
            Ok(pg) => Some(pg),
            Err(e) => {
                error!("fail to build pg connection pool {}", e.to_string());
                None
            },
        }
    } else {
        None
    };
    
    let room_channel_list = batch_init(&config).await;
    for (room_id, chan) in &room_channel_list {
        if let (Some(c), Some(pg)) = (&chan.bus.db_pg, &pg) {
            c.accept((pg.clone(), *room_id))
        }
    }
    info!("init wss connection...");
    let socket_server = SocketAddr::new(server_addr, port);
    let tcp = tokio::net::TcpListener::bind(socket_server).await.unwrap();
    let room_channel_list_rwlock = Arc::new(RwLock::new(room_channel_list));
    while let Ok((stream, _peer_addr)) = tcp.accept().await {
        let (connect_param_tx, connect_param_rx) = oneshot::channel();
        let callback = ConnectCallback { connect_param_tx };
        match tokio_tungstenite::accept_hdr_async(stream, callback).await {
            Ok(ws_stream) => {
                // 等待连接回调读取参数
                if let Ok(param) = connect_param_rx.await {
                    // 有对应的频道
                    if let Some(room_channel) =
                        room_channel_list_rwlock.read().await.get(&param.roomid)
                    {
                        let bus = &room_channel.bus;
                        // 有对应的类型
                        match param.connect_type {
                            SerType::Json => {
                                if let Some(consumer) = &bus.ws_json {
                                    consumer.accept(ws_stream);
                                }
                            }
                            SerType::Bincode => {
                                if let Some(consumer) = &bus.ws_bincode {
                                    consumer.accept(ws_stream);
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // unimplemented!()
                // 需要补全
            }
        }
    }
}

struct ConnectCallback {
    connect_param_tx: oneshot::Sender<ConnectParam>,
}

#[derive(Debug, Clone)]
struct ConnectParam {
    roomid: u64,
    connect_type: SerType,
}

impl Callback for ConnectCallback {
    fn on_request(
        self,
        request: &ws2::handshake::server::Request,
        response: ws2::handshake::server::Response,
    ) -> Result<ws2::handshake::server::Response, ws2::handshake::server::ErrorResponse> {
        let mut path = request.uri().path().split('/').skip(1);
        if let Some(Ok(roomid)) = path.next().map(|s| u64::from_str_radix(s, 10)) {
            match path.next() {
                Some("json") => {
                    if self
                        .connect_param_tx
                        .send(ConnectParam {
                            roomid,
                            connect_type: SerType::Json,
                        })
                        .is_ok()
                    {
                        return Ok(response);
                    }
                }
                Some("bincode") => {
                    if self
                        .connect_param_tx
                        .send(ConnectParam {
                            roomid,
                            connect_type: SerType::Bincode,
                        })
                        .is_ok()
                    {
                        return Ok(response);
                    }
                }
                _ => {}
            }
        }
        return Err(ws2::handshake::server::ErrorResponse::new(Some(
            "bad args".into(),
        )));
    }
}
