use tokio::sync::broadcast;
use bilive_danmaku::event::Event as BiliEvent;
use tokio_tungstenite::tungstenite::Message as WsMsg;

use crate::chan::ExtendedEvent;

#[derive(Clone)]
pub struct Outbound {
    pub ws: WsOutbound,
    pub db: DbOutbound
}

#[derive(Clone)]
pub struct WsOutbound {
    pub json: Option<broadcast::Sender<WsMsg>>,
    pub bincode: Option<broadcast::Sender<WsMsg>>,
}

#[derive(Clone)]
pub struct DbOutbound {
    pub mongo: Option<mongodb::Collection<ExtendedEvent>>
}

#[derive(Clone, Debug)]
pub enum PipeType {
    Json,
    Bincode,
}

pub async fn piping(mut inbound: broadcast::Receiver<BiliEvent>, outbound: Outbound) {
    while let Ok(evt) = inbound.recv().await {
        if let Some(bincode) = &outbound.ws.bincode {
            if let Ok(msg) = evt.to_bincode().map(|bincode|WsMsg::Binary(bincode)) {
                bincode.send(msg).unwrap_or_default();
            }
        }
        if let Some(json) = &outbound.ws.bincode {
            if let Ok(msg) = evt.to_json().map(|json|WsMsg::Text(json)) {
                json.send(msg).unwrap_or_default();
            }
        }
        if let Some(mongo) = &outbound.db.mongo {
            let ex_event = ExtendedEvent {
                event: evt,
                timestamp: chrono::Utc::now().timestamp_millis()
            };
            if let Err(e) = mongo.insert_one(ex_event, None).await {
                println!("db error {}", e);
            }
        }
    }
}