extern crate winapi;
extern crate json;

mod icon_handler;
mod icon_process;
mod config;
mod status_handler;

use std::thread;
use std::time;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use std::process::Command;
use std::str;

use config::{Config, WatchType};
use icon_process::TrayIconProcess;

use status_handler::{get_status,get_bedrock_status};

//START "TaskWatcher" ./task_watcher.exe -WindowStyle Hidden -RedirectStandardOutput "./service.log"
fn get_current_tasks(task: &String) -> Result<Vec<String>,String>{
    match Command::new("tasklist").output() {
        Ok(result) => {
            let mut tasks: Vec<String> = vec![];
            let output = String::from_utf8_lossy(&result.stdout);
            for process in output.split('\n') {
                let string_pro = String::from(process);
                if string_pro.contains(task) {
                  let contents: Vec<&str> = string_pro.split_whitespace().collect();
                  tasks.push(contents[1].to_string());
                }
            }  
            Ok(tasks)          
        }
        Err(err) => {
            eprintln!("{}",err);
            Err(err.to_string())
        }
    }
}

pub fn main() {
    let config = Config::load();
  
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let icon_offline = include_bytes!("icon_offline.ico");
    let icon_online = include_bytes!("icon_online.ico");
    println!("Watching proccess {} with a max of {}", config.watch_process,config.max_processes);

    let mut handles: Vec<icon_process::TrayIconProcess> = vec![];
    let mut watchers = 0;

    while running.load(Ordering::SeqCst) {

        match get_current_tasks(&config.watch_process) {
            Ok(value) => {
                if (value.len() > watchers) && (watchers < config.max_processes) {
                    println!("Added {} watchers", value.len() - watchers);
                    for i in 0..(value.len() - watchers) {
                        if watchers < config.max_processes {
                            let hid = 1000 + watchers as u32;
                            handles.push(TrayIconProcess::create(icon_offline,hid,value[i].clone()).unwrap());
                            watchers += 1;
                        }
                    }
                }else if watchers > value.len() {
                    println!("Remove {} watchers", watchers - value.len());
                    for _ in value.len()..watchers {
                        handles.pop().unwrap().kill();
                        watchers -= 1;
                    }
                }
            }
            Err(err) => {
                eprintln!("{}",err);
            }
        }

        if !(watchers < config.watchers.len()) {
            for i in 0..config.watchers.len() {
                match config.watchers[i].watcher_type {
                    WatchType::BEDROCK => {
                        let status = get_bedrock_status(config.watchers[i].ip.as_str(), config.watchers[i].port);
                        if status.0 != handles[i].status {
                            if status.0 {
                                handles[i].set_tooltip(format!("Bedrock server | Players  {}",status.1).to_string());
                                handles[i].set_icon(icon_online);
                            }else {
                                handles[i].set_tooltip("Bedrock server | Offline".to_string());
                                handles[i].set_icon(icon_offline);
                            }
                        }
                    }
                    WatchType::JAVA => {
                        let status = get_status(config.watchers[i].ip.as_str(), config.watchers[i].port);
                        if status.0 != handles[i].status {
                            if status.0 {
                                handles[i].set_tooltip(format!("Java Server | Players {}",status.1).to_string());
                                handles[i].set_icon(icon_online);
                            }else {
                                handles[i].set_tooltip("Java Server | Offline".to_string());
                                handles[i].set_icon(icon_offline);
                            }
                        }
                    }
                }
            }
        }
        

        thread::sleep(time::Duration::from_secs(config.main_thread_sleep_sec));
    }
    println!("Kill Main Task Process");
    
    for mut handle in handles {
        handle.kill();
    }
}
#[cfg(test)]
mod tests { 
    use super::*;
    #[test]
    fn test_load_config(){
       
    }
    #[test]
    fn test_task_checker(){
        match get_current_tasks(&"powershell.exe".to_string()) {
            Ok(value) => {
                println!("{:#?}",value);
            }
            Err(err) => {
                eprintln!("{}",err);
            }
        }
    }
}