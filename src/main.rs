use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get {
        key : String
    },
    Set {
        key: String,
        val: Bytes
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Arc<Mutex<HashMap<String, Vec<u8>>>>) {
    use mini_redis::Command::{self, Get, Set};
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}