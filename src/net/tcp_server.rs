use tokio::net::{TcpListener, TcpStream};
use mini_redis::{client, Connection, Frame};
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

#[tokio::main]
pub async fn tokio_test() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream)
{
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap(){
        info!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}

