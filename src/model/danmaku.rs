use crate::schema::*;
use bilive_danmaku::{model as bilive_model};
use diesel::{prelude::*};
use chrono::*;

use super::ms_to_dt;

#[derive(Queryable, Insertable)]
pub struct Danmaku {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub flag: i32,
    pub message: String,
    pub is_emoticon: bool,
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
        let msg_json = serde_json::to_string(&message).unwrap_or_default();
        let (is_emoticon, essential_message) = match message {
            bilive_model::DanmakuMessage::Emoticon { emoticon:_, alt_message } => {
                (true, alt_message)
            },
            bilive_model::DanmakuMessage::Plain { message } => {
                (false, message)
            }
        };
        Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            user_name: user.uname,
            flag: flag as i32,
            message: msg_json,
            is_emoticon,
            essential_message,
            medal: fans_medal.map(|x|serde_json::to_string(&x).ok()).flatten(),
            timestamp: ms_to_dt(time),
        }
    }
}
