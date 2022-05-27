

use bilive_danmaku::{RoomService};
use tokio_tungstenite::tungstenite::Message as WsMsg;
use tokio::sync::broadcast;

use crate::pipe::PipeType;
pub struct Chan {
    pub json: bool,
    pub bincode: bool,
    pub roomid: u64
}



impl Chan {
    pub async fn start(self) -> Result<ChanOutbound, String> {
        use crate::pipe::*;
        let roomid = self.roomid;

        let mut service = match RoomService::new(roomid).init().await {
            Ok(service) => {
                service.connect().await.map_err(|_|"fail to connect")
            },
            Err(_) => {
                Err("fail to init")
            },
        }?;

        
        let mut chan_outbound = ChanOutbound {
            json_outbound:None,
            bincode_outbound:None,
        };

        let mut json_handle = if self.json {
            let config = PipeConfig{pipe_type: PipeType::Json};
            let inbound = service.subscribe();
            let (outbound, _) = tokio::sync::broadcast::channel(16);
            let json_handle = tokio::spawn(Pipe{inbound,outbound:outbound.clone()}.piping(config));
            chan_outbound.json_outbound = Some(outbound.clone());
            Some((json_handle, outbound))
        } else {
            None
        };

        let mut bincode_handle = if self.bincode {
            let config = PipeConfig{pipe_type: PipeType::Bincode};
            let inbound = service.subscribe();
            let (outbound, _) = tokio::sync::broadcast::channel(16);
            let bincode_handle = tokio::spawn(Pipe{inbound,outbound:outbound.clone()}.piping(config));
            chan_outbound.bincode_outbound = Some(outbound.clone());
            Some((bincode_handle, outbound))
        } else {
            None
        };

        let guard = async move {
            while let Some(_exception) = service.exception().await {
                let mut fallback = service.close();
                'retry: loop {
                    match fallback.connect().await {
                        Ok(new_service) => {
                            service = new_service;
                            if let Some((handle, outbound)) = json_handle {
                                handle.abort();
                                let config = PipeConfig{pipe_type: PipeType::Json};
                                let inbound = service.subscribe();
                                let handle = tokio::spawn(Pipe{inbound,outbound:outbound.clone()}.piping(config));
                                json_handle = Some((handle, outbound))
                            }
                            if let Some((handle, outbound)) = bincode_handle {
                                handle.abort();
                                let config = PipeConfig{pipe_type: PipeType::Bincode};
                                let inbound = service.subscribe();
                                let handle = tokio::spawn(Pipe{inbound,outbound:outbound.clone()}.piping(config));
                                bincode_handle = Some((handle, outbound))
                            }
                            break 'retry;
                        },
                        Err((new_fallback, _)) => {
                            fallback = new_fallback;
                            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                        },
                    }
                }
            }
        };

        tokio::spawn(guard);
        
        Ok(chan_outbound)
    }
}


pub struct ChanOutbound {
    json_outbound: Option<broadcast::Sender<WsMsg>>,
    bincode_outbound: Option<broadcast::Sender<WsMsg>>,
}

impl ChanOutbound {
    pub fn subscribe(&self, ptype: PipeType) -> Option<broadcast::Receiver<WsMsg>> {
        match ptype {
            PipeType::Json => {
                self.json_outbound.as_ref().map(|h|h.subscribe())
            },
            PipeType::Bincode => {
                self.bincode_outbound.as_ref().map(|h|h.subscribe())
            },
        }
    }
}