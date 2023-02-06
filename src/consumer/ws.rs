use std::{marker::PhantomData};

use bilive_danmaku::event::Event;
use log::error;
use tokio::{
    sync::broadcast::{channel, Receiver},
    task::JoinHandle, net::TcpStream,
};
use tokio_tungstenite::{tungstenite::{Message}, WebSocketStream};
use futures_util::SinkExt;
use super::Consumer;

#[derive(Clone, Copy, Debug)]
pub enum SerType {
    Json,
    Bincode,
}

pub trait Convertor {
    fn convert(evt: Event) -> Message;
}

pub struct WsConsumer<C: Convertor> {
    _handle: JoinHandle<()>,
    receiver: Receiver<Message>,
    _convertor: PhantomData<C>,
}

impl<C: Convertor> WsConsumer<C> {
    pub fn subscribe(&self) -> Receiver<Message> {
        return self.receiver.resubscribe();
    }
}

impl<C: Convertor> Consumer for WsConsumer<C> {
    type Out = WebSocketStream<TcpStream>;

    fn launch(mut receiver: Receiver<Event>) -> Self {
        let (message_sender, message_receiver) = channel(128);
        let handle = tokio::spawn(async move {
            while let Ok(evt) = receiver.recv().await {
                let message = C::convert(evt);
                message_sender.send(message).unwrap_or_default();
            }
        });
        Self {
            _handle: handle,
            receiver: message_receiver,
            _convertor: PhantomData,
        }
    }

    fn accept(&self, mut ws_stream: Self::Out) {
        let mut rx = self.subscribe();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = ws_stream.send(msg).await {
                    error!("<ws发送错误>{e}")
                }
            }
            ws_stream.close(None).await.unwrap();
        });
    }

}
// impl super::Outbound<'_> for WsOutbound {
//     fn run(&self) {
//         self.ws_stream.chain(other)
//     }
// }
pub struct JsonConvertor;

impl Convertor for JsonConvertor {
    fn convert(evt: Event) -> Message {
        let json = evt.to_json().unwrap();
        // dbg!(&json);
        Message::Text(json)
    }
}

pub struct BincodeConvertor;

impl Convertor for BincodeConvertor {
    fn convert(evt: Event) -> Message {
        // dbg!(&evt);
        let bincode = evt.to_bincode();
        Message::Binary(bincode.unwrap_or_default())
    }
}


