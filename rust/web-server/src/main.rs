use std::fs;
use web_server::ThreadPool;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream, Ipv4Addr };

struct Config {
    host: Ipv4Addr,
    port: u16,
}

impl Config {
    const fn new(host: Ipv4Addr, port: u16) -> Config {
        Config {host, port}
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let config = Config::new(Ipv4Addr::LOCALHOST, 8080);
    let listener = TcpListener::bind(config.host.to_string() + ":" + &config.port.to_string()).unwrap();
    println!("Start server on {}:{}", config.host, config.port);
    let thread_pool = ThreadPool::new(4);
    for stream in  listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}
