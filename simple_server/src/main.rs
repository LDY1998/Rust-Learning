use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use thread_pool::ThreadPool;

mod thread_pool;


fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let mut tp = thread_pool::ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        tp.execute(move || {
            handle_connection(stream);
        })
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1 \r\n";
    println!("buffer: {:?}", String::from_utf8_lossy(&buffer[..]));
    println!("start with get: {}", buffer.starts_with(get));
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!(
            "http/1.1 200 ok\r\ncontent-length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    } else {

        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}{}",
            status_line,
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

    }



}
