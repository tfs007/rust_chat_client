use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use sha2::{Sha256, Digest};

fn hash_message(message: &str) -> String {
    // Create a Sha256 hasher instance
    let mut hasher = Sha256::new();

    // Write input message
    hasher.update(message);

    // Read hash digest and consume hasher
    let result = hasher.finalize();

    // Convert the hash result to a hex string
    hex::encode(result)
}

fn print_instructions() -> io::Result<()> {
    println!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m");
    println!("\x1b[94mEnter '/register <username> <password>' to register.\x1b[0m");
    println!("\x1b[94mEnter '/login <username> <password>' to login.\x1b[0m");
    println!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m");
    println!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m");
    println!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m");
    println!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m");
    println!("\x1b[94mEnter '/instructions' to see instructions.\x1b[0m");
    io::stdout().flush()?;  
    Ok(())
}

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
                Ok(msg) => println!("{}", msg),
                Err(e) => eprintln!("Error receiving message: {}", e),
            }
        }
    });
    print_instructions();
    // println!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m");
    // println!("\x1b[94mEnter '/register <username> <password>' to register.\x1b[0m");
    // println!("\x1b[94mEnter '/login <username> <password>' to login.\x1b[0m");
    // println!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m");
    // println!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m");
    // println!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m");
    // println!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m");
    // println!("\x1b[94mEnter '/instructions' to see instructions.\x1b[0m");
    // io::stdout().flush()?;


    // Read input from the user and send it to the server
    loop {
        // print!("Enter message (or '/quit' to exit): \n");
        // print!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m\n");
        // print!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m\n");
        // print!("\x1b[94mEnter message (or '/quit' to exit): \x1b[0m\n");
        // io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("/quit") {
            break;
        }
        // let mut send_msg = input.clone();
        if input.starts_with("/register") {
            
            let mut words: Vec<&str> = input.split_whitespace().collect();
            
            let hash_pwd = hash_message(&words[2]);
            words[2] = &hash_pwd;
            let my_send_msg = words.join(" ");
            println!("Modified pwd: {}", my_send_msg);


        }
        if input.eq_ignore_ascii_case("/instructions") {
            print_instructions();
        }


        write.send(Message::Text(input.to_string())).await?;
        // write.send(Message::Text(send_msg.to_string())).await?;
    }

    println!("Disconnected");
    Ok(())
}