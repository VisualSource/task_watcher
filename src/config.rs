
use json::parse;
use std::fs::read_to_string;

#[derive(Debug)]
pub enum WatchType {
    JAVA,
    BEDROCK
}
impl WatchType {
    pub fn from_str(data: &str) -> Self {
        match data {
            "java" => Self::JAVA,
            "bedrock" => Self::BEDROCK,
            _ => Self::JAVA
        }
    }
}

#[derive(Debug)]
pub struct WatcherConfig {
    pub watcher_type: WatchType,
    pub ip: String,
    pub port: u16
}

#[derive(Debug)]
pub struct Config {
    pub debug: bool,
    pub watch_process: String,
    pub max_processes: usize,
    pub main_thread_sleep_sec: u64,
    pub watchers: Vec<WatcherConfig>
}

impl Config {
    pub fn load() -> Self {
        match read_to_string("./config.json") {
            Ok(value) => {
               match parse(&value) {
                   Ok(parsed) => {

                    let mut debug = false;
                    if parsed.has_key("debug") {
                        debug = parsed["debug"].as_bool().unwrap();
                    }
                    
                    let mut max_processes: usize = 2;
                    if parsed.has_key("max_processes") {
                        max_processes = parsed["max_processes"].as_usize().unwrap();
                    }

                    let mut main_thread_sleep_sec: u64 = 15;
                    if parsed.has_key("main_thread_sleep_sec") {
                        main_thread_sleep_sec = parsed["main_thread_sleep_sec"].as_u64().unwrap();
                    }

                    if !parsed.has_key("watch_process") {
                        panic!("Missing key 'watch_process' from config");
                    }

                    let mut watchers: Vec<WatcherConfig> = vec![];
                    if parsed.has_key("watchers") {
                        for watcher in parsed["watchers"].members() {
                            watchers.push(WatcherConfig{
                                watcher_type: WatchType::from_str(watcher["type"].as_str().unwrap()),
                                ip: watcher["ip"].as_str().unwrap().to_string(),
                                port: watcher["port"].as_u16().unwrap()
                            });
                        }
                    }

                    Config {
                        debug,
                        max_processes,
                        main_thread_sleep_sec,
                        watch_process: parsed["watch_process"].as_str().unwrap().to_string(),
                        watchers
                    }

                   }
                   Err(err) => {
                       panic!("{}",err.to_string());
                   }
               }
            }
            Err(err) => {
                panic!("{}",err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load() {
        println!("{:#?}",Config::load());
    }
}
