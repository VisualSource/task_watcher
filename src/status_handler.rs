extern crate craftping;

use craftping::sync::ping;

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

pub fn get_bedrock_status(_hostname: &str, _port: u16) -> (bool, usize) {
    (false,0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_bedrock(){
        let status = get_bedrock_status("play.cosmicpe.me", 19132);
    
        println!("Server is {}", status.0);
    }

    #[test]
    fn test_java(){
        let status = get_status("play.applecraftmc.org",25565);
    
        println!("Server is {} with {} players", status.0,status.1);
        
    }
}