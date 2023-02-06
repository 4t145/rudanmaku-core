pub mod danmaku;
pub mod superchat;
pub mod gift;
pub mod guard_buy;
pub mod enter;
pub mod watched;
pub mod popularity;

use chrono::*;
#[inline]
fn ms_to_dt(time: u64) -> DateTime<Utc> {
    DateTime::from_local(NaiveDateTime::from_timestamp_opt((time/1000) as i64, 1_000_000*(time%1000) as u32).unwrap(), Utc)
}

#[inline]
fn medal_to_string(fans_medal: Option<bilive_danmaku::model::FansMedal>) -> Option<String> {
    fans_medal.map(|x|serde_json::to_string(&x).ok()).flatten()
}