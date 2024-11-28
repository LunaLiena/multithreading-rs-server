use std::net::TcpListener;
use std::io::prelude::BufRead;
use std::{ fs, io::* };
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let contents = fs::read_to_string("./src/hello.html");

    let (status_line, response_body) = match contents {
        Ok(data) => {
            let length = data.len();
            println!("Data: {data}");
            println!("Len of file: {length}");
            ("HTTP/1.1 200 OK", data)
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            ("HTTP/1.1 404 NOT FOUND", "404 Not Found".to_string())
        }
    };

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        response_body.len(),
        response_body
    );
    stream.write_all(response.as_bytes()).unwrap()
}
