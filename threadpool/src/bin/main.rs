use std::net::{self, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::process;
use threadpool::ThreadPool;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(6).unwrap_or_else(|e| {
        println!("Error while creating ThreadPool: {}", e);
        process::exit(1);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET /genesis HTTP/1.1\r\n";
    let get_info = b"GET /info HTTP/1.1\r\n";

    let (response, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "genesis.html")
    } else if buffer.starts_with(get_info) {
        ("HTTP/1.1 200 OK", "info.html")
    }else {
        ("HTTP/1.1 404 Not Found", "not_found.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        response, 
        contents.len(), 
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}