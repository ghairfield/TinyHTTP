//! Todo Move this to lib.rs
//!
//! Greg Hairfield
//! CS410P Rust Programming
//! Spring 2021

mod configuration;
mod protocol;
mod request;
mod response;

use crate::configuration::CONFIG;
use crate::request::Header;
use crate::response::Response;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

type Result<T> = std::result::Result<T, TinyHttpError>;

pub struct TinyHttpError;

pub fn tiny_http() -> Result<()> {
    //println!("Config is: {:?}", *CONFIG);

    match listen() {
        Ok(_) => Ok(()),
        Err(_) => Err(TinyHttpError),
    }
}

fn listen() -> Result<()> {
    let listen = TcpListener::bind(format!("{}:{}", CONFIG.host, CONFIG.port)).unwrap();

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
    Ok(())
}

fn new_connection(mut conn: TcpStream) {
    println!("New connection from {}", conn.peer_addr().unwrap());

    let mut buf = [0_u8; 2048];
    let result = conn.read(&mut buf);

    match result {
        Ok(_size) => {
            // TODO Who knows if we are going to need size yet??
            let header = Header::new(&buf);
            let mut res = Response::new(&header);
            //header.print();
            let r = res.respond();
            conn.write_all(&r).unwrap();
            conn.flush().unwrap();
            println!("----Responded----");
        }
        Err(e) => {
            println!(
                "An error occured while reading the stream! ip: {}, err: {}",
                conn.peer_addr().unwrap(),
                e
            );
            conn.shutdown(Shutdown::Both).unwrap();
        }
    };
}
