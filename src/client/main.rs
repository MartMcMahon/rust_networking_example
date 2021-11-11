use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8000")
        .await
        .expect("could not open");
    let (mut reader, mut writer) = stream.split();

    loop {
        let mut input_buf = [0_u8; 10];
        let mut stdin = tokio::io::stdin();
        let n = stdin.read(&mut input_buf).await.unwrap();
        println!("read {:?}", input_buf);

        let res = writer.write(&input_buf[..n]).await;
        println!("written {:?}", res.unwrap());
        let mut line = [0_u8; 10];
        let len = reader.read(&mut line).await;
        println!("{:?} {:?}", line, len);
    }
}
