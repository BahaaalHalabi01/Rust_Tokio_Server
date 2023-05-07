use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    let mut buffer = [0u8; 1024];

   let bytes_read =  socket.read(&mut buffer).await.unwrap();

   socket.write_all(&buffer[..bytes_read]).await.unwrap();
}
