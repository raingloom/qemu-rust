//! A library to interface with Qemu.  For more information about Qemu see
//! [qemu](http://www.qemu.org)
//!
extern crate bytes;
#[macro_use]
extern crate json;
#[macro_use]
extern crate log;
extern crate mio;
extern crate rustc_serialize;
extern crate simple_logger;
extern crate slab;

use mio::{EventLoop, EventSet, PollOpt, Token};
use mio::tcp::TcpStream;

pub mod commands;
pub mod qapi_connection;

pub mod enums;
pub mod structs;
pub mod events;

pub trait QemuCmd {
    // Return a json blob that we can send to the Qemu Server

    fn to_json(&self) -> String;
    // fn parse_qemu_response(&self, response: &String) -> rustc_serialize::json::DecodeResult<Self>
    //    where Self: std::marker::Sized;
}

fn connect_to_qemu() {
    const CLIENT: Token = Token(1);
    let addr = "127.0.0.1:4444".parse().unwrap();

    // Create an event loop
    let mut event_loop = EventLoop::new().unwrap();

    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    event_loop.register(&sock, CLIENT, EventSet::readable(), PollOpt::edge())
        .unwrap();

    // Start handling events
    event_loop.run(&mut qapi_connection::QApiConnection::new(sock)).unwrap();
}

fn main() {
    simple_logger::init_with_level(log::LogLevel::Warn).unwrap();

    // Lets have a chat with the server
    connect_to_qemu();
}
