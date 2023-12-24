use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut req_buffer = [0 as u8; 512];
    stream.read(&mut req_buffer).unwrap();
    if req_buffer.starts_with("GET / HTTP/1.1".as_bytes()) {
        let response_body = fs::read_to_string("demo.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type:text/html;charset=utf-8\r\n\r\n{}", response_body);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
