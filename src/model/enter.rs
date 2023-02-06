use crate::schema::*;
use bilive_danmaku::model as bilive_model;
use chrono::*;
use diesel::prelude::*;

use super::{medal_to_string, ms_to_dt};

#[derive(Queryable, Insertable)]
pub struct Enter {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub medal: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Enter {
    pub fn new(
        room_id: u64,
        user: bilive_model::User,
        medal: Option<bilive_model::FansMedal>,
        time: u64,
    ) -> Self {
        return Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            user_name: user.uname as String,
            medal: medal_to_string(medal),
            timestamp: ms_to_dt(time),
        };
    }
}
