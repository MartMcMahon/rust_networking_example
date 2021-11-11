use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8000").await.unwrap();
    println!("Connected to {}", &stream.peer_addr().unwrap());

    let (tx, rx) = broadcast::channel::<String>(10);
    let mut input_buf = [0_u8; 1024];
    let mut stdin = tokio::io::stdin();
    let mut network_buf = String::new();

    loop {
        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::select! {
            result = stdin
                .read(&mut input_buf) => {
                    if result.unwrap() == 0 {
                        break;
                    }

                    let x = std::array::IntoIter::new(input_buf).filter(|&x| x != 0 as u8);
                    writer.write_all(&input_buf).await.unwrap();
                println!("wrote {:#?} to network", x.clone());
                }
            result = reader.read_line(&mut network_buf) => {
                println!("read {:#?} on network", network_buf.clone());
                tx.send(network_buf.clone()).unwrap();
                network_buf.clear();
                println!("sent {:#?} on tx", network_buf.clone());
            }
            result = rx.recv() => {
                let msg = result.unwrap();
                println!("received {:#?} on rx", msg);
            }
        }
    }
}
