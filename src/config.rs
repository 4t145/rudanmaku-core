use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub net: NetConfig,
    pub room: Vec<RoomConfigItem>,
    pub db: DbConfig
}


#[derive(Deserialize)]
pub struct RoomConfigItem {
    pub roomid: u64,
    pub channel: Vec<String>
}

#[derive(Deserialize)]

pub struct NetConfig {
    pub ipv4: Option<[u8;4]>,
    pub ipv6: Option<[u16;8]>,
    pub port: u16,
}


#[derive(Deserialize)]
pub struct DbConfig {
    pub mongo: Option<MongoConfig>
}

#[derive(Deserialize)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16
}