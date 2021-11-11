use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8000").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);
    let mut count = 0;

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (mut msg, other_addr) = result.unwrap();
                        let l = msg.len();
                        println!("{}, {:#?}", l, &msg);
                        let x = format!("{}{}{}", &msg[..l-1], count.to_string(), '\n');
                        println!("with count {:#?}", x);
                        writer.write_all(x.as_bytes()).await.unwrap();
                        count = count + 1;
                    }
                }
            }
        });
    }
}
