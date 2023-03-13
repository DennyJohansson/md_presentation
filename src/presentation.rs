use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::md_parser;

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let slides = md_parser::create_slides("content/rust.md").unwrap();
    let path = &request_line.split(" ").collect::<Vec<&str>>()[1][1..];
    let current_slide = path.parse::<usize>().unwrap();
    let slide = md_parser::safe_get_current_slide(&slides, current_slide);

    let (status_line, content) = match md_parser::add_slide(&slide, &slides) {
        Ok(c) => ("HTTP/1.1 200 OK", c),
        Err(_) => ("HTTP/1.1 404 NOT FOUND", "<h1>404</h1>".to_string()),
    };

    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
