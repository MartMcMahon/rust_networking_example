use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    let (mut reader, mut writer) = stream.split();

    loop {
        let mut input_buf = [0_u8; 10];
        let mut stdin = tokio::io::stdin();
        let n = stdin.read(&mut input_buf).await.unwrap();
        println!("read {:?}", input_buf);

        let len = writer.write(&input_buf[..n]).await;
        println!("written {:?}", len);
        let mut line = [0_u8]; //String::new();
        let len = reader.read(&mut line).await;
        println!("{:?} {:?}", line, len);
    }
}
