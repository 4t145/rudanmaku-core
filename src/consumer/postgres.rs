use bilive_danmaku::event::Event;
use tokio::sync::broadcast::Receiver;

use super::Consumer;

use crate::model;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
type PgPool = Pool<ConnectionManager<PgConnection>>;
pub struct PgConsumer {
    receiver: Receiver<Event>,
}

impl Consumer for PgConsumer {
    type Out = (PgPool, u64);
    fn launch(receiver: Receiver<Event>) -> Self {
        return Self { receiver };
    }
    fn accept(&self, out: Self::Out) {
        let (pool, room_id) = out;
        use bilive_danmaku::event::EventData::*;
        let mut receiver = self.receiver.resubscribe();
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                use crate::schema::*;
                let time = event.timestamp;
                match event.data {
                    Danmaku { flag, message, user, fans_medal } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(danmakus::table).values(model::danmaku::Danmaku::new(
                                room_id, flag, message, user, fans_medal, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    _ => {

                    }
                    //     EnterRoom { user, fans_medal } => todo!(),
                    //     BlindboxGift { user, fans_medal, blindbox_gift_type, gift } => todo!(),
                    //     Gift { user, fans_medal, blindbox, gift } => todo!(),
                    //     GuardBuy { level, price, user } => todo!(),
                    //     SuperChat { user, fans_medal, price, message, message_jpn } => todo!(),
                    //     WatchedUpdate { num } => todo!(),
                    //     PopularityUpdate { popularity } => todo!(),
                    //     GuardEnterRoom { user } => todo!(),
                    //     HotRankChanged { area, rank, description } => todo!(),
                    //     HotRankSettlement { uname, face, area, rank } => todo!(),
                }
            }
        });
    }
}
