use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    collections::HashMap, sync::Arc,
};

use web_server_demo::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8899").expect("bind err: ");
    let pool = ThreadPool::new(6);

    let index_html = fs::read_to_string("index.html").expect("read index.html err: ");
    let not_found_html = fs::read_to_string("404.html").expect("read index.html err: ");
    let mut file_cache_map = HashMap::new();
    file_cache_map.insert("index.html", index_html);
    file_cache_map.insert("404.html", not_found_html);
    let shared_data = Arc::new(file_cache_map);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // println!("Connection established!")
        let shared_data = shared_data.clone();
        pool.execute(move || {
            handle_connection(stream, shared_data);
        });
    }
}

fn handle_connection(mut stream: TcpStream, shared_data: Arc<HashMap<&str, String>>) {
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

    // let contents = fs::read_to_string(filename).expect("read html err: ");
    let default_contents = &"html file not exists!".to_string();
    let contents = shared_data.get(filename).unwrap_or(default_contents);
    // let contents = shared_data.get(filename).map_or("", |v| v);
    // let contents = match contents {
    //     Some(contents) => contents,
    //     _ => "",
    // };
    let response = format!("{}{}", status_line, contents);
    stream
        .write(response.as_bytes())
        .expect("stream write err: ");
    stream.flush().expect("stream flush err: ");
}
