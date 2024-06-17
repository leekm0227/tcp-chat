use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").await.unwrap();
    let (tx, _) = broadcast::channel::<Vec<u8>>(100);

    println!("run server...");

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("connect: {addr}");
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut read_buff = [0; 1024];

            loop {
                tokio::select! {
                    Ok(msg) = rx.recv() => {
                        writer.write_all(&msg).await.unwrap();
                    }
                    Ok(len) = reader.read(&mut read_buff) =>{
                        if len > 0 {
                            tx.send(read_buff.to_vec()).ok();
                        }
                    }
                }
            }
        });
    }
}
