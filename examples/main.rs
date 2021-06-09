//! Example how to use TinyHTTP
//!
//! Greg Hairfield
//! CS410P Rust Programming
//! Spring 2021

use tiny_http;

fn main() {
    match tiny_http::tiny_http() {
        Ok(_) => (),
        Err(_) => panic!("An error occured in the server!"),
    }
}
