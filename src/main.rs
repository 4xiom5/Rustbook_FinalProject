use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn status_code_to_text(status_code: i32) -> Option<&'static str> {
    match status_code {
        200 => Some("OK"),
        404 => Some("Not found"),
        _ => None
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (html_file, status_code) = if request_line == "GET / HTTP/1.1" {
        ("hello.html", 200)
    } else {
        ("404.html", 404)
    };

    let status_text = status_code_to_text(status_code).unwrap();
    let status_line = format!("HTTP/1.1 {status_code} {status_text}");
    let contents = fs::read_to_string(html_file).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
