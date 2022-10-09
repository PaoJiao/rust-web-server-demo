use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    fs,
    thread,
    time::Duration,
};

use web_server_demo::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899")
        .expect("bind err: ");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // println!("Connection established!")
        pool.execute(||{
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).expect("stream read err: ");
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
        
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")

    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).expect("read html err: ");
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).expect("stream write err: ");
    stream.flush().expect("stream flush err: ");
}
