use std::alloc::handle_alloc_error;
use std::thread;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "serve" => run_as_server(args),
            "connect" => run_as_client(args),
            _ => ()
        }
    }
}

fn run_as_client(args: Vec<String>) {

}

fn run_as_server(args: Vec<String>) {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

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