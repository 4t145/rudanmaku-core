pub mod ws;
pub mod mongo;
pub mod postgres;

use bilive_danmaku::event::Event;
use tokio::sync::broadcast::Receiver;
use ws::*;
use mongo::*;

use self::postgres::PgConsumer;

pub trait Consumer: Sized {
    // fn transfer(evt: Event);
    type Out;
    fn launch(receiver: Receiver<Event>) -> Self where Self: Sized;
    fn accept(&self, out: Self::Out);
}

pub struct Bus {
    pub ws_json: Option<WsConsumer<JsonConvertor>>,
    pub ws_bincode: Option<WsConsumer<BincodeConvertor>>,
    pub db_mongo: Option<MongoConsumer>,
    pub db_pg: Option<PgConsumer>
}

