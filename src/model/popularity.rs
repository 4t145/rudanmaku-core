use crate::schema::*;
use diesel::{prelude::*};
use chrono::*;
use super::*;

#[derive(Queryable, Insertable)]
pub struct Popularity {
    pub id: Option<i32>,
    pub room_id: i64,
    pub count: i32,
    pub timestamp: DateTime<Utc>,
}

impl Popularity {
    pub fn new(room_id: u64, count: u32, time: u64) -> Self {
        Self {
            id: None,
            room_id: room_id as i64,
            count: count as i32,
            timestamp: ms_to_dt(time),
        }
    }
}