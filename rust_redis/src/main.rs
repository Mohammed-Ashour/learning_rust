mod resp;
mod memory;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::memory::MemoryMap;

use self::resp::{Req, ReqType, Command, Status};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let mut request: Vec<String> = Vec::new();
    let mut memory = MemoryMap::default();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 1024];
                stream.read(&mut buf).unwrap();
                println!("{}", String::from_utf8_lossy(&buf[..]));
                request = parse_request(String::from(String::from_utf8_lossy(&buf[..])));
                stream.write_all(b"+OK\r\n").unwrap();
                println!("{:?}", request);
                let mut req_obj = Req::new( String::from(String::from_utf8_lossy(&buf[..])));
                let resp = match req_obj.cmd{
                    Command::Get => memory.get(req_obj.get_group(), req_obj.get_name()),
                    Command::Set => memory.set(req_obj.get_group(), req_obj.get_name(), req_obj.get_value()),
                    Command::Ping => resp::Response { Status: Status::Success, Msg: String::from("Pong") },
                    Command::Unknown => resp::Response { Status: Status::Error, Msg: String::from("Command is not recognized!") }

                };
                
                
                println!("{:?}", req_obj);
                println!("{:?}", resp);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("Hello, world!");
}
fn parse_request(request_body: String) -> Vec<String> {
    let mut clean_tokens: Vec<String> = Vec::new();
    let tokens = request_body.split("\r\n");
    for token in tokens {
        if let Some(first_char) = token.chars().next() {
            if first_char != '\0' {
                clean_tokens.push(String::from(token))
            }
        }
    }
    return clean_tokens;
}
