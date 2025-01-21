use std_challenge::*;


fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:8000")
        .expect("Could not start server");
    println!("Listening at 127.0.0.1:8000 ...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
                ()
            },
            Err(_) => eprintln!("Could not process request :/"),
        };
    }
}
