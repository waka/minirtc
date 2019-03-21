extern crate env_logger;
extern crate ws;
extern crate minirtc;

use std::env;
use std::thread;
use minirtc::signaling::{Server};

fn main() {
    env_logger::init();

    let port = match env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => 3000
        },
        Err(_) => {
            3000
        }
    };

    println!("WebSocket server is started for 127.0.0.1:{}", &port);

    let server = thread::spawn(move || {
        ws::listen(format!("127.0.0.1:{}",port), |out| Server { ws: out }).unwrap()
    });
    let _ = server.join();
}
