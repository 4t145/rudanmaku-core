use crate::schema::*;
use bilive_danmaku::{model as bilive_model};
use diesel::{prelude::*};
use chrono::*;

use super::ms_to_dt;

#[derive(Queryable, Insertable)]
pub struct Superchat {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub message: String,
    pub price: i32,
    pub medal: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Superchat {
    pub fn new(
        room_id: u64,
        user: bilive_model::User,
        fans_medal: Option<bilive_model::FansMedal>,
        price: u64,
        message: String,
        time: u64,
    ) -> Self {
        Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            price: price as i32,
            user_name: user.uname,
            message,
            medal: fans_medal.map(|x|serde_json::to_string(&x).ok()).flatten(),
            timestamp: ms_to_dt(time),
        }
    }
}
