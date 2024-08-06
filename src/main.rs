use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multithread_web_server::ThreadPool;

fn main() {
    let host = "127.0.0.1";
    let port = "3000";
    let address = format!("{}:{}", host, port);
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let index = "index.html";
    let not_found = "404.html";

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", &index),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", &index)
        }
        _ => ("HTTP/1.1 404 NOT FOUND", &not_found),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
