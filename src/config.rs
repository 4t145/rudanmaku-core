use serde::Deserialize;
use std::{path::Path};
use log::{error};
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
    pub mongo: Option<MongoConfig>,
    pub pg: Option<PgConfig>
}

#[derive(Deserialize)]
pub struct MongoConfig {
    pub db: String,
    pub host: String,
    pub port: u16
}

#[derive(Deserialize)]
pub struct PgConfig {
    pub url: String
}

pub fn get_config() -> Config {
    let mut args = std::env::args().skip(1);
    match args.next() {
        Some(first_arg) => {
            match first_arg.as_str() {
                "config" => {
                    if let Some(configfile) = args.next() {
                        use std::fs::*;
                        match read(Path::new(&configfile)) {
                            Ok(file) => {
                                toml::from_slice::<Config>(&file).unwrap()
                            },
                            Err(e) => {
                                error!("{}", e);
                                panic!("fail read config file")
                            },
                        }
                    } else {
                        panic!("missing arg: config file path")
                    }
                },
                maybe_number @_ => {
                    if let Ok(roomid) = u64::from_str_radix(maybe_number, 10) {
                        let port = if let Some(port) = args.next() {
                            if let Ok(port) = u16::from_str_radix(port.as_str(), 10) {
                                port
                            } else {
                                panic!("port should be a number of u64");
                            }
                        } else {
                            10200
                        };
                        let config = Config {
                            net: NetConfig {
                                ipv4: Some([127,0,0,1]),
                                ipv6: None,
                                port,
                            },
                            room: vec![RoomConfigItem {
                                roomid,
                                channel: vec![String::from("json"), String::from("bincode")]
                            }],
                            db: DbConfig { mongo: None, pg: None }
                        };
                        return config;
                    } else {
                        panic!("roomid should be a number of u64")
                    }
                }
            }
        },
        None => panic!("roomid should be a number of u64"),
    }
}