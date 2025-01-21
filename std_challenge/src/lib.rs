use std::{io::{Read, Write}, net::TcpStream};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"response\":\"Hello world!\"}";
const ERR_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found";

pub fn handle_client(mut client: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = client.read(&mut buffer) {
        eprintln!("Could not process request : {}", e);
        return;
    }
    let request = String::from_utf8_lossy(&buffer[..]);

    if client.write_all(if request.contains("GET /api") { OK_RESPONSE.as_bytes() } else { ERR_RESPONSE.as_bytes() }).is_err() {
        if let Err(e) = client.write_all(OK_RESPONSE.as_bytes()) {
            eprintln!("Could not upload response : {}", e);
        }
    }
    if let Err(e) = client.flush() {
        eprintln!("Could not clear buffer : {}", e);
    }
}