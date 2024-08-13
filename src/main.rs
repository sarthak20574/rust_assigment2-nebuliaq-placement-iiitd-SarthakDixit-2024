use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};
use std::sync::Arc;

// #[tokio::main]: This macro sets up the async runtime required by Tokio. It allows the main function to be async.
#[tokio::main]


async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // binds the server to the local address 127.0.0.1 on port 
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    //creates a channel with a capacity of 100 messages 
    let (tx, rx) = mpsc::channel(100);


    //a shared buffer for storing incoming messages
    // Arc allows multiple tasks to share ownership of the buffer, and the Mutex ensures that 
    //only one task can modify the buffer at a time.
    let message_buffer = Arc::new(Mutex::new(Vec::new()));

    // starts a task to handle buffering and sending messages
    let buffer = message_buffer.clone();
    tokio::spawn(async move {
        buffer_messages(rx, buffer).await;
    });

    // keep accepting new connections
    loop {

        //listener.accept().await?: waits for an incoming connection
        // returns a TcpStream representing the connection
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();

        tokio::spawn(async move {
            handle_connection(socket, tx).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, tx: mpsc::Sender<String>) {
    
    //Asynchronously reads data from the TCP stream into a buffer
    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break, // connection closed
            Ok(n) => {

                //convert the received bytes into a string
                let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                if tx.send(msg).await.is_err() {
                    eprintln!("Receiver dropped");
                }
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }
}

async fn buffer_messages(mut rx: mpsc::Receiver<String>, buffer: Arc<Mutex<Vec<String>>>) {
    
    //timer for 10 sec
    let mut interval = time::interval(Duration::from_secs(10));


    loop {

        //Waits for a new message from the channel or the timer to tick
        tokio::select! {

            //receives a message from the channel
            Some(message) = rx.recv() => {
                let mut buffer = buffer.lock().await;
                buffer.push(message);
                if buffer.len() >= 100 {
                    flush_buffer(&mut buffer).await;
                }
            }
            _ = interval.tick() => {//If the timer ticks and the buffer is not empty, the buffer is flushed.
                let mut buffer = buffer.lock().await;
                if !buffer.is_empty() {
                    flush_buffer(&mut buffer).await;
                }
            }
        }
    }
}

async fn flush_buffer(buffer: &mut Vec<String>) {
    if !buffer.is_empty() {
        //  send the buffer to the destination server
        println!("Sending buffer: {:?}", buffer);

        // Clear the buffer
        buffer.clear();
    }
}

