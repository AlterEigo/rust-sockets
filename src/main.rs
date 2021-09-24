use std::alloc::handle_alloc_error;
use std::fmt::format;
use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Not enough arguments!");
    }

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "serve" => run_as_server(args.as_slice()[2..].to_vec()),
            "connect" => run_as_client(args.as_slice()[2..].to_vec()),
            _ => panic!("You have to run in either client or server mode.")
        }
    }
}

fn run_as_client(args: Vec<String>) {
    let (addr, port) = (
        args.get(0).unwrap(),
        args.get(1).unwrap()
    );
    let addr = format!("{}:{}", addr, port);
    let mut stream = TcpStream::connect(addr).unwrap();

    let greeting = b"client echo";
    stream.write(greeting).unwrap();
    loop {}
}

fn run_as_server(args: Vec<String>) {
    let (addr, port) = (
        args.get(0).unwrap(),
        args.get(1).unwrap()
    );
    let addr = format!("{}:{}", addr, port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("[Server] : New connection established.");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}