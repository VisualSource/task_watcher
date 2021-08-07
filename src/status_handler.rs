extern crate craftping;

use json::parse;
use craftping::sync::ping;
use std::process::Command;

pub fn get_status(host: &str, port: u16) -> (bool,usize) {
    match ping(host,port) {
        Ok(value) => {
            (true,value.online_players)
        }
        Err(_) => {
            (false, 0)
        }
    }
    
}
pub fn get_bedrock_status(hostname: &str, port: u16) -> (bool, usize) {
    match Command::new("powershell").arg(format!("bedrockping {} {}",hostname,port).to_string()).output() {
        Ok(value) => {
            match String::from_utf8(value.stdout){
                Ok(text) => {
                    match parse(text.as_str()){
                        Ok(res) => {
                            (true,res["onlinePlayers"].as_usize().unwrap())
                        }
                        Err(parse_err) => {
                            eprintln!("{}",parse_err);
                            (false,0)
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{}",error);
                    (false,0)
                }
            }
        }
        Err(err) => {
            eprintln!("{}",err);
            (false,0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_buffers(){
       
    }
    #[test]
    fn test_bedrock(){
        let status = get_bedrock_status("play.cosmicpe.me", 19132);
    
        println!("Server is {:#?}", status);
    }

    #[test]
    fn test_java(){
        let status = get_status("play.applecraftmc.org",25565);
    
        println!("Server is {} with {} players", status.0,status.1);
        
    }
}