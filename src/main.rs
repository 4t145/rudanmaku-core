use std::{
    net::{Ipv4Addr, Ipv6Addr, IpAddr, SocketAddr}, 
    time::Duration,
    sync::Arc,
    collections::HashMap,
};
use tokio_tungstenite::{tungstenite::{self as ws2, handshake::server::Callback}, WebSocketStream};
use futures_util::{SinkExt, StreamExt, stream::SplitSink, stream::SplitStream};
use tokio::{sync::{broadcast, oneshot, RwLock}, net::TcpStream};
use log::{info, warn, error};

mod pipe;
mod chan;
mod config;
mod netcontrol;

use pipe::PipeType;

#[tokio::main]
async fn main() {
    env_logger::init();
    // let rt = tokio::runtime::Builder::new_multi_thread().enable_all()
    // .build().unwrap();
    server(config::get_config()).await;
}

async fn batch_init(configs: Vec<config::RoomConfigItem>, dbs: Dbs, cooldown: Duration) -> Arc<RwLock<HashMap<u64, chan::ChanHandle>>> {
    let mut map = HashMap::new();
    let total = configs.len();
    let mut counter = 0;
    let cooldown = netcontrol::Cooldown::new(cooldown);
    for room_config in configs {
        let roomid = room_config.roomid;
        info!("connecting room[{roomid}]");

        let collection = dbs.mongo.as_ref().map(|db|{
            db.collection::<chan::ExtendedEvent>(roomid.to_string().as_str())
        });
        
        let chan = chan::Chan {
            json: room_config.channel.contains(&"json".to_owned()),
            bincode: room_config.channel.contains(&"bincode".to_owned()),
            mongo: collection,
            roomid,
            cooldown:cooldown.clone(),
        };
        
        match chan.start().await {
            Ok(handle) => {
                map.insert(roomid, handle);
                counter += 1;
                info!("connected room[{roomid}]: {counter}/{total}");
            },
            Err(e) => {
                error!("fail to connect room[{roomid}] {e}");
            }
        }
        // tokio::time::sleep(cooldown).await;
    }
    Arc::new(RwLock::new(map))
}

async fn bridge(mut inbound: broadcast::Receiver<ws2::Message>, mut outbound: SplitSink<WebSocketStream<TcpStream>, ws2::Message>) {
    loop {
        match inbound.recv().await {
            Ok(msg) => {
                outbound.send(msg).await.unwrap_or_default();
            },
            Err(e) => {
                warn!("bridge encounter error: {e}");
                match e {
                    broadcast::error::RecvError::Closed => {
                        warn!("{e}");
                        break;
                    },
                    broadcast::error::RecvError::Lagged(n) => {
                        warn!("skipped {n}");
                    },
                }
            }
        }
    }
}

async fn wait_close(mut rx:SplitStream<WebSocketStream<TcpStream>>, handle: tokio::task::JoinHandle<()>) {
    while let Some(Ok(recv)) = rx.next().await {
        if recv.is_close() {
            handle.abort();
            return;
        }
    } 
}

pub struct Dbs {
    mongo: Option<mongodb::Database>
}

async fn server(config: config::Config) {
    let port = config.net.port;
    let server_addr = if let Some(ipv4) = config.net.ipv4 {
        IpAddr::V4(Ipv4Addr::from(ipv4))
    } else if let Some(ipv6) = config.net.ipv6 {
        IpAddr::V6(Ipv6Addr::from(ipv6))
    } else {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    };
    let mongo = 
    if let Some(mongo_config) = config.db.mongo {
        use  mongodb::options::*;
        info!("db connecting");
        let host = mongo_config.host;
        let port = Some(mongo_config.port);
        let options = ClientOptions::builder().hosts(vec![ServerAddress::Tcp{host, port}]).build();
        match mongodb::Client::with_options(options) {
            Ok(client) => {
                info!("db connected");
                Some(client.database(mongo_config.db.as_str()))
            },
            Err(e) => {
                error!("cannot connect to db: {e}");
                None
            }
        }
    } else {
        None
    };
    let dbs = Dbs {
        mongo,
    };
    info!("init wss connection...");

    let service_list = batch_init(config.room, dbs, Duration::from_micros(500)).await;
    let socket_server = SocketAddr::new(server_addr, port);
    let tcp = tokio::net::TcpListener::bind(socket_server).await.unwrap();
    while let Ok((stream, _peer_addr)) = tcp.accept().await {
        let (connect_param_tx,connect_param_rx) = oneshot::channel();
        let callback = ConnectCallback {
            connect_param_tx,
        };
        match tokio_tungstenite::accept_hdr_async(stream, callback).await {
            Ok(ws_stream) => {
                if let Ok(param) = connect_param_rx.await {
                    if let Some(chan) = service_list.read().await.get(&param.roomid) {
                        if let Some(inbound) = chan.ws_subscribe(param.connect_type) {
                            let (outbound, rx) = ws_stream.split();
                            tokio::spawn(
                                wait_close(rx, 
                                    tokio::spawn(bridge(inbound, outbound))
                                )
                            );
                        }
                    }
                }
            },
            Err(_) => {

            }
        }
    }
}

struct ConnectCallback  {
    connect_param_tx: oneshot::Sender<ConnectParam>
}

#[derive(Debug, Clone)]
struct ConnectParam {
    roomid: u64,
    connect_type: PipeType
}

impl Callback for ConnectCallback {
    fn on_request(
        self,
        request: &ws2::handshake::server::Request,
        response: ws2::handshake::server::Response,
    ) -> Result<ws2::handshake::server::Response, ws2::handshake::server::ErrorResponse> {
        let mut path = request.uri().path().split('/').skip(1);
        if let Some(Ok(roomid)) = path.next().map(|s|{u64::from_str_radix(s, 10)}) {
            match path.next() {
                Some("json") => {
                    if self.connect_param_tx.send(ConnectParam {
                        roomid,
                        connect_type: PipeType::Json,
                    }).is_ok() {
                        return Ok(response);
                    }
                },
                Some("bincode") => {
                    if self.connect_param_tx.send(ConnectParam {
                        roomid,
                        connect_type: PipeType::Bincode,
                    }).is_ok() {
                        return Ok(response);
                    }
                },
                _ => {
                }
            }
        } 
        return Err(ws2::handshake::server::ErrorResponse::new(Some("bad args".into())));
    }
}