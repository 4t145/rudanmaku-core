use tokio::sync::broadcast;
use bilive_danmaku::event::Event as BiliEvent;
use tokio_tungstenite::tungstenite::Message as WsMsg;
pub struct Pipe {
    pub inbound: broadcast::Receiver<BiliEvent>,
    pub outbound: broadcast::Sender<WsMsg>
}
#[derive(Clone, Debug)]
pub enum PipeType {
    Json,
    Bincode
}
pub struct PipeConfig {
    pub pipe_type: PipeType
}

impl Pipe {
    pub async fn piping(mut self, config: PipeConfig) {
        while let Ok(evt) = self.inbound.recv().await {
            let msg =  match config.pipe_type {
                PipeType::Json => {
                    evt.to_json().map(|json|WsMsg::Text(json)).map_err(|_|"json encode error")
                },
                PipeType::Bincode => {
                    evt.to_bincode().map(|bincode|WsMsg::Binary(bincode)).map_err(|_|"bincode encode error")
                },
            };
            if let Ok(msg) = msg {
                self.outbound.send(msg).unwrap_or_default();
            }
        }
    }
}