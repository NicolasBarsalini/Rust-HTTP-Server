use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;


const IP: &str = "0.0.0.0"; // == localhost connection
const PORT: &str = "8080";

fn read_static_file(file_path: &str) -> Option<String> {
    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Failure to open file");
            Some(contents)
        }
        Err(_) => None,
    }
}

fn response_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failure to read stream!");

    println!("Recepted: {}", String::from_utf8_lossy(&buffer));

    let response = if buffer.starts_with(b"GET /style.css") {
        match read_static_file("response_files/style.css") {
            Some(contents) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/css\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            ),
            None => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
        }
    } else {
        match read_static_file("response_files/index.html") {
            Some(contents) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            ),
            None => "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string(),
        }
    };

    stream.write(response.as_bytes()).expect("Failure to send a response!");
    stream.flush().expect("Error!");
}


fn main() {
    let listener = match TcpListener::bind(format!("{}:{}", IP, PORT)) {
        Ok(value) => value,
        Err(e) => {
            println!("Error: {}", e.to_string());
            panic!("{}", e.to_string())
        }
    };

    println!("Server connected at http://{}:{}", IP, PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    response_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error to accept the connection: {}", e);
            }
        }
    }
}
