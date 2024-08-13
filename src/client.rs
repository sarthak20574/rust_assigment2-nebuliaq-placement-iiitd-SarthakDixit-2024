use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::{self, Duration};
use std::error::Error;


//macro sets up the Tokio runtime, allowing the main function to be asynchronous.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // sets up a time interval with a duration of 500 millisec
    let mut interval = time::interval(Duration::from_millis(500)); // send a message every 500ms

    loop {
        interval.tick().await;

        //If the connection is successful (TcpStream::connect returns Ok), the code inside this block is executed.
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(mut stream) => {
                let msg = "This is a log message";
                if let Err(e) = stream.write_all(msg.as_bytes()).await {
                    eprintln!("Failed to send message: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
            }
        }
    }
}
