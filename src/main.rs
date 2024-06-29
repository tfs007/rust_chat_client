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
    print!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m\n");
    print!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m\n");
    print!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m\n");
    print!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m\n");
    print!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m\n");
    print!("\x1b[94mEnter '/instructions' to see instructions.\x1b[0m\n");

    // Read input from the user and send it to the server
    loop {
        // print!("Enter message (or '/quit' to exit): \n");
        // print!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m\n");
        // print!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m\n");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("/quit") {
            break;
        }
        if input.eq_ignore_ascii_case("/instructions") {
            print!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m\n");
            print!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m\n");
            print!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m\n");
            print!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m\n");
            print!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m\n");
            print!("\x1b[94mEnter '/instructions' to see instructions.\x1b[0m\n");
        }

        write.send(Message::Text(input.to_string())).await?;
    }

    println!("Disconnected");
    Ok(())
}