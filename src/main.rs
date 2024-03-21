use std::fs;

use std::{
 io::{prelude::*, BufReader},
 net::{TcpListener, TcpStream},
};

fn main() {
 let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
 for stream in listener.incoming() { let stream = stream.unwrap();
 handle_connection(stream); }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let http_request_line = buf_reader
      .lines()
      .next()
      .unwrap()
      .unwrap();

  let request_path = http_request_line.split_whitespace().nth(1).unwrap();

  let (status_line, contents) = if request_path == "GET / HTTP/1.1" {
      let contents = fs::read_to_string("hello.html").unwrap();
      ("HTTP/1.1 200 OK", contents)
  } else {
      let contents =  fs::read_to_string("oops.html").unwrap();
      ("HTTP/1.1 404 NOT FOUND", contents.to_string())
  };

  let length = contents.len();

  let response = format!(
      "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
  );

  stream.write_all(response.as_bytes()).unwrap();
}