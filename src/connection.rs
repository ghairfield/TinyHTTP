use std::net::{ TcpListener, TcpStream, Shutdown };
use std::io::{ Read, Write };
use std::thread;

use crate::request::Header;

pub fn listen() -> () {
    let listen = TcpListener::bind("127.0.0.1:8080").unwrap();
    // TODO
    //  listen.set_ttl(X)
    //  listen.set_nonblocking(true).expect("Cannot set non-blocking")

    for stream in listen.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    new_connection(stream);
                });
            }
            Err(e) => {
                // TODO log error
                println!("Error connecting to client! {}", e);             
            }
        }
    }
}

fn new_connection(mut conn: TcpStream) -> () {
    println!("New connection from {}", conn.peer_addr().unwrap());

    let mut buf = [0 as u8; 1024];

    while match conn.read(&mut buf) {
        Ok(size) => {
            let header = Header::new(&buf).unwrap();
            header.print();
            true
        }
        Err(e) => {
            println!("An error occured while reading the stream! ip: {}, err: {}",
                conn.peer_addr().unwrap(), e);
            conn.shutdown(Shutdown::Both).unwrap();
            false
        }
    }{}
}




