use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connect_addr = "ws://127.0.0.1:8080";
    let url = Url::parse(&connect_addr)?;

    println!("Connecting to {}", connect_addr);
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to read messages from the server
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => println!("Received: {}", msg),
                Err(e) => eprintln!("Error receiving message: {}", e),
            }
        }
    });

    // Read input from the user and send it to the server
    loop {
        print!("Enter message (or 'quit' to exit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        write.send(Message::Text(input.to_string())).await?;
    }

    println!("Disconnected");
    Ok(())
}