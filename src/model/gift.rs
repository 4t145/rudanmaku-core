use crate::schema::*;
use bilive_danmaku::model as bilive_model;
use chrono::*;
use diesel::prelude::*;

use super::*;

#[derive(Queryable, Insertable)]
pub struct Gift {
    pub id: Option<i32>,
    pub room_id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub gift_id: i32,
    pub gift_name: String,
    pub gift_price: i32,
    pub gift_count: i32,
    pub gift_paid: bool,
    pub medal: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl Gift {
    pub fn new(
        room_id: u64,
        user: bilive_model::User,
        fans_medal: Option<bilive_model::FansMedal>,
        _blindbox: Option<bilive_model::GiftType>,
        gift: bilive_model::Gift,
        time: u64
    ) -> Self {
        let gift_paid = match gift.coin_type {
            bilive_model::CoinType::Silver => false,
            bilive_model::CoinType::Gold => true,
        };
        return Self {
            id: None,
            room_id: room_id as i64,
            user_id: user.uid as i64,
            user_name: user.uname,
            gift_id: gift.gift_id as i32,
            gift_name: gift.gift_name,
            gift_price: gift.price as i32,
            gift_count: gift.coin_count as i32,
            gift_paid,
            medal: medal_to_string(fans_medal),
            timestamp: ms_to_dt(time),
        }
    }
}
