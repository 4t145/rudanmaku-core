use bilive_danmaku::event::Event;
use tokio::sync::broadcast::Receiver;

use super::Consumer;

use crate::model;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
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
                    SuperChat { user, fans_medal, price, message, message_jpn: _ } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(superchats::table).values(model::superchat::Superchat::new(
                                room_id, user, fans_medal, price, message, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    Gift { user, fans_medal, blindbox, gift } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(gifts::table).values(model::gift::Gift::new(
                                room_id, user, fans_medal, blindbox, gift, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    GuardBuy { level, price, user } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(guard_buys::table).values(model::guard_buy::GuardBuy::new(
                                room_id, user, level, price, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    EnterRoom { user, fans_medal } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(enters::table).values(model::enter::Enter::new(
                                room_id, user, fans_medal, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    }
                    WatchedUpdate { num } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(watcheds::table).values(model::watched::Watched::new(
                                room_id, num, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    PopularityUpdate { popularity } => {
                        if let Ok(mut conn) = pool.get() {
                            diesel::insert_into(popularitys::table).values(model::popularity::Popularity::new(
                                room_id, popularity, time
                            )).execute(&mut conn).unwrap_or_default();
                        }
                    },
                    _ => {

                    }
                    //     BlindboxGift { user, fans_medal, blindbox_gift_type, gift } => todo!(),
                    //     GuardEnterRoom { user } => todo!(),
                    //     HotRankChanged { area, rank, description } => todo!(),
                    //     HotRankSettlement { uname, face, area, rank } => todo!(),
                }
            }
        });
    }
}
