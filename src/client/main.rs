// use std::io::BufRead;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, Stdin};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8000")
        .await
        .expect("Could not connect.");
    loop {
        let mut input = Vec::new();
        // let mut buffer = String::new();
        tokio::io::stdin()
            .read_exact(&mut input)
            .await
            .expect("Failed to read from stdin");
        stream
            .write(&input)
            .await
            .expect("Failed to write to server");

        // let mut reader = BufReader::new(stream);

        // reader
        //     .read_line(&mut buffer)
        //     .await
        //     .expect("Could not read into buffer");
        // print!(
        //     "{}",
        //     std::str::from_utf8(&buffer).expect("Could not write buffer as string")
        // );
    }
}
