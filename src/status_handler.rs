extern crate craftping;
extern crate chrono;
extern crate rand;
extern crate hex;
extern crate byteorder;

use rand::Rng;
use craftping::sync::ping;
use std::net::UdpSocket;
use std::time::{Duration,SystemTime, UNIX_EPOCH};
use byteorder::{WriteBytesExt, BigEndian};
//use std::thread;
use hex::FromHex;


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

/// @author Jviguy 
/// @see 
/// https://github.com/Jviguy/rsquery/blob/master/src/lib.rs
pub fn get_bedrock_status(hostname: &str, port: u16) -> (bool, usize) {
    let socket = match UdpSocket::bind(("0.0.0.0",8000)) {
        Ok(socket) => socket,
        Err(_err) => {
            eprintln!("{}","failed to bind host socket");
            return (false,0);
        }
    };
    socket.set_read_timeout(Some(Duration::new(5,0))).expect("Failed to set read timeout");
    //https://github.com/AnvilMC/anvil_bedrock/blob/main/raknet/src/protocol/unconnected_ping.rs
    let mut random = rand::thread_rng();
    let offline_msg_data = Vec::from_hex("00ffff00fefefefefdfdfdfd12345678").expect("Failed to read binary string!");
    //Initalize Buf with 0x01 being the ID_UNCONNECTED_PING
    let mut buf: Vec<u8> = vec![0x01];
    //Write the current time stamp
    buf.write_i64::<BigEndian>(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64).expect("Failed to write time");
    //Hex literal for Offline Message Data ID
    buf.extend(&offline_msg_data);
    //Write a random client id
    buf.write_u64::<BigEndian>(random.gen::<u64>()).expect("Failed to write uuid");
   //Send query to remote socket
  
    match socket.send_to(&buf,(hostname,port)) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}",err);
            return (false, 0);
        }
    }
    
    let mut responce = (false, 0);

    let mut buf = [0u8; u16::MAX as usize];

    match socket.recv_from(&mut buf) {
        Ok((len,_)) => {
            let data: Vec<String> = String::from_utf8_lossy(&buf[offline_msg_data.len()+19..=len]).split(';').map(String::from).collect();
            responce.0 = true;
            responce.1 = data[4].parse::<usize>().unwrap();
        }
        Err(err) => {
            eprintln!("Socket Recv Error {}",err);
            eprintln!("Possible invailed host or port");
        }
    }
    responce
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_buffers(){
    }
    #[test]
    fn test_bedrock(){
        let status = get_bedrock_status("192.168.1.5", 19132);
    
        println!("Server is {:#?}", status);
    }

    #[test]
    fn test_java(){
        let status = get_status("play.applecraftmc.org",25565);
    
        println!("Server is {} with {} players", status.0,status.1);
        
    }
}