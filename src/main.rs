mod store;
use std::sync::{Arc, Mutex};
use tokio::{io::AsyncWriteExt, net::TcpListener};

use crate::store::Store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = Arc::new(Mutex::new(Store::new()));

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        let _store = Arc::clone(&store); // shadowed clone
        tokio::spawn(async move {
            socket
                .write(handler().as_bytes())
                .await
                .expect("expected to write to socket");
            // store is usable here
        });
    }
}

fn handler() -> String {
    "+PONG\r\n".to_string()
}
