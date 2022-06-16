use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;

pub struct Request {
    pub method: String,
    pub url: String,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub body: Option<String>,
}

pub struct Response {
    pub status: i32,
    pub protocol: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parse_to_request(mut stream: &TcpStream) -> Request {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let content = from_utf8(&buffer).unwrap();

    let (head, body) = content.split_once("\r\n\r\n").unwrap();

    let lines = head.lines();

    let mut request = Request {
        method: "".to_string(),
        url: "".to_string(),
        protocol: "".to_string(),
        headers: HashMap::new(),
        query: HashMap::new(),
        body: None,
    };

    for (idx, line) in lines.enumerate() {
        if idx == 0 {
            let (method, rest) = line.split_once(' ').unwrap();
            let (url, protocol) = rest.split_once(' ').unwrap();
            request.method = method.to_string();
            request.url = url.to_string();
            request.protocol = protocol.to_string();
        } else {
            let (key, value) = line.split_once(": ").unwrap();
            request.headers.insert(key.to_string(), value.to_string());
        }
    }

    if body.len() > 0 {
        request.body = Some(body.to_string());
    }

    return request;
}

pub fn write_response(response: Response, mut stream: &TcpStream) {
    let mut response_text = format!("{} {} {}\r\n", response.protocol, response.status, "OK");

    for (key, value) in response.headers {
        response_text.push_str(format!("{}: {}\r\n", key, value).as_str());
    }

    response_text.push_str("\r\n");

    match response.body {
        Some(the_body) => response_text.push_str(the_body.as_str()),
        None => (),
    }

    print!("{}", response_text);

    stream.write(response_text.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn listen(ip: &str, port: u16, f: &dyn Fn(Request) -> Response) {
    let listener = TcpListener::bind((ip, port)).expect("Could not bind address");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let request = parse_to_request(&stream);
        let response = f(request);
        write_response(response, &stream);
    }
}