use crate::schema::*;
use bilive_danmaku::{event as bilive_event, model as bilive_model};
use diesel::{prelude::*};
use chrono::*;
#[inline]
fn ms_to_dt(time: u64) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp_opt((time/1000) as i64, 1_000_000*(time%1000) as u32).unwrap(), Utc)
}
#[derive(Queryable, Insertable)]
pub struct Danmaku {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub flag: i32,
    pub message: String,
    pub essential_message: String,
    pub medal: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Danmaku {
    pub fn new(
        room_id: u64,
        flag: u64,
        message: bilive_model::DanmakuMessage,
        user: bilive_model::User,
        fans_medal: Option<bilive_model::FansMedal>,
        time: u64,
    ) -> Self {
        Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            user_name: user.uname,
            flag: flag as i32,
            message: serde_json::to_string(&message).unwrap_or_default(),
            essential_message: message.to_string(),
            medal: fans_medal.map(|x|serde_json::to_string(&x).ok()).flatten(),
            timestamp: ms_to_dt(time),
        }
    }
}
