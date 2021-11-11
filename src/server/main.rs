use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || handle_client(stream));
    }
}

fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    loop {
        let mut buf = String::new();
        let n = stream.read_line(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        println!("recieved {:?}", &buf);
        stream.get_ref().write(buf.as_bytes()).unwrap();
    }
}
