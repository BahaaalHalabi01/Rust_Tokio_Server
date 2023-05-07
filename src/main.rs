use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);
    loop {
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let (mut socket, addr) = listener.accept().await.unwrap();

        //this is similar to creating an async function
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            //uncomment the below to get an error
            //regarding the reader and writer being present in the same place
            //because of how split works
            // tokio::spawn(async move {
            //     reader.read_line(&mut line).await.unwrap();
            // });
            // tx.send(("".to_string(), addr)).unwrap();

            loop {
                //the below enables the server to not block/await the user read line but select
                //from the task that gets recieved first/incase the message gets sent do not wait
                //for read_line from the user to recieve it
                tokio::select! {
                    //select is very useful when you have things that need
                    //to operate on a shared state
                    //and you have a finite number of things
                    //
                    //the below reader and writer have to exist together
                    //as they are a reference to the underlying tcp stream
                    //and can't be split
                    //
                    //both of the below futures(async functions)
                    //can be ran at the same time
                    result = reader.read_line(&mut line)=>{
                        //inside here, only one will be running at the same time
                        if result.unwrap() == 0{
                            break;
                        }
                    tx.send((line.clone(),addr)).unwrap();
                    line.clear();
                    }
                    result = rx.recv()=>{
                        let (msg,other_addr) = result.unwrap();
                        if addr != other_addr {

                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                };
            }
        });
    }
}
