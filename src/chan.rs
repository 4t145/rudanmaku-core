use bilive_danmaku::{RoomService, event::Event};
use mongodb::bson::doc;
use tokio_tungstenite::tungstenite::Message as WsMsg;
use tokio::sync::broadcast;
use log::{info, warn, error};

use crate::pipe::{PipeType, Outbound};
const MAX_RETRY_CNT:u64 = 10;
const MAX_UNSEND_HB:u8 = 3;
#[derive(serde::Serialize)]
pub struct ExtendedEvent {
    #[serde(flatten)]
    pub event: Event,
    // #[serde(with = "u32_as_timestamp")]
    pub timestamp: i64
}

pub struct Chan {
    pub json: bool,
    pub bincode: bool,
    pub mongo: Option<mongodb::Collection<ExtendedEvent>>,
    pub roomid: u64,
    pub cooldown: crate::netcontrol::Cooldown
}


impl Chan {
    pub async fn start(self) -> Result<ChanHandle, String> {
        use crate::pipe::*;
        let roomid = self.roomid;

        let fallback = RoomService::new(roomid).init().await.map_err(|_|"fail to init")?;
        let mut service = fallback.connect().await.map_err(|_|"fail to connect")?;

        let json = self.json.then_some(broadcast::channel(16).0);
        let bincode = self.bincode.then_some(broadcast::channel(16).0);

        let inbound = service.subscribe();
        let outbound = Outbound {
            ws: WsOutbound { json, bincode },
            db: DbOutbound { mongo: self.mongo }
        };
        let ret = Ok(ChanHandle{outbound: outbound.clone()});
        let mut handle = tokio::spawn(piping(inbound, outbound.clone()));
        let cooldown = self.cooldown.clone();
        let guard = async move {
            let mut send_error_cnt = 0;
            while let Some(exception) = service.exception().await {
                use bilive_danmaku::Exception::*;
                warn!("room[{roomid}]: exception {:?}", exception);
                match exception {
                    UnexpectedMessage(_) => {
                        continue;
                    }
                    WsSendError(_) => {
                        send_error_cnt += 1;
                        if send_error_cnt < MAX_UNSEND_HB {
                            continue;
                        } 
                    },
                    _ => {
                        // should restart!
                    }
                }
                service.close();
                handle.abort();
                warn!("room[{roomid}]: service closed, reconnecting");
                'retry: loop {
                    let mut retry_cnt = 0;
                    match fallback.connect().await {
                        Ok(new_service) => {
                            info!("room[{roomid}]: reconnected");
                            service = new_service;
                            let inbound = service.subscribe();
                            handle = tokio::spawn(piping(inbound, outbound.clone()));
                            break 'retry;
                        },
                        Err(e) => {
                            retry_cnt += 1;
                            error!("room[{roomid}]: reconnect failed [{retry_cnt}], error: {e:?}");
                            if retry_cnt >= MAX_RETRY_CNT {
                                error!("room[{roomid}]: quit gurad");
                            }
                            cooldown.cooldown().await;
                            // tokio::time::sleep(tokio::time::Duration::from_secs(5*MAX_RETRY_CNT*MAX_RETRY_CNT)).await;
                        },
                    }
                }
            }
        };

        tokio::spawn(guard);
        self.cooldown.cooldown().await;
        ret
    }
}


pub struct ChanHandle {
    outbound: Outbound
}

impl ChanHandle {
    pub fn ws_subscribe(&self, ptype: PipeType) -> Option<broadcast::Receiver<WsMsg>> {
        match ptype {
            PipeType::Json => {
                self.outbound.ws.json.as_ref().map(|h|h.subscribe())
            },
            PipeType::Bincode => {
                self.outbound.ws.bincode.as_ref().map(|h|h.subscribe())
            }
        }
    }
}