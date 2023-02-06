use crate::schema::*;
use bilive_danmaku::{model as bilive_model};
use diesel::{prelude::*};
use chrono::*;
use super::*;

#[derive(Queryable, Insertable)]
pub struct GuardBuy {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub guards_level: i16,
    pub price: i32,
    pub timestamp: DateTime<Utc>,
}

impl GuardBuy {
    pub fn new(room_id: u64, user: bilive_model::User, level: u64, price: u64, time: u64) -> Self {
        Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            user_name: user.uname,
            guards_level: level as i16,
            price: price as i32,
            timestamp: ms_to_dt(time),
        }
    }
}