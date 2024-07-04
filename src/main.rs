use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use sha2::{Sha256, Digest};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use tokio_rustls::TlsStream; 
use rustls::ClientConnection;


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
    println!("\x1b[94mEnter '/logout' to logout.\x1b[0m");
    println!("\x1b[94mEnter '/createroom <roomname>' to create a room.\x1b[0m");
    println!("\x1b[94mEnter '/room <roomname>' to enter a room.\x1b[0m");
    println!("\x1b[94mEnter '/leave' to leave the current room.\x1b[0m");
    println!("\x1b[94mEnter '/listrooms' to see a list of rooms.\x1b[0m");
    println!("\x1b[94mEnter '/dm <username>' to direct message to <username>.\x1b[0m");
    println!("\x1b[94mEnter '/history <username>' to see your DM history with <username>.\x1b[0m");
    println!("\x1b[94mEnter '/instructions' to see instructions.\x1b[0m");
    io::stdout().flush()?;  
    Ok(())
}

#[tokio::main]
// #![allow(warnings)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connect_addr = "ws://127.0.0.1:8080";
    let url = Url::parse(&connect_addr)?;

    println!("Connecting to {}", connect_addr);
    let (ws_stream, _) = connect_async(url).await?;
    println!("WebSocket handshake has been successfully completed");

    

    let local_addr = match ws_stream.get_ref() {
        MaybeTlsStream::Plain(tcp) => tcp.local_addr()?,
        MaybeTlsStream::Rustls(tls) => tls.get_ref().0.local_addr()?, // Access the TcpStream directly
        _ => return Err("Unexpected stream type".into()),
    };
    println!("Local socket address: {}", local_addr);

    let (mut write, mut read) = ws_stream.split();
    // Get the local address of the TcpStream
    // Get the local address of the TcpStream
    
    // let (tx, rx) = mpsc::channel::<Option<Vec<u8>>>(32);


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
    let mut username= String::from("default");
    let mut token= String::from("defaulttoken");
    let mut hash_pwd = String::new();



    // Read input from the user and send it to the server
    loop {
        
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();
        if !input.starts_with("/") {
            let new_input = format!("\x1b[32m{}\x1b[0m : {}", username, input.to_string());
            // write.send(Message::Text(input.to_string())).await?;
            write.send(Message::Text(new_input)).await?;

        }

        if input.starts_with("/register") || input.starts_with("/login") {
            let mut words: Vec<&str> = input.split_whitespace().collect();
            if words.len() <= 2{
                println!("\x1b[33mPass username and password! Please see instructions below.\x1b[0m");
                print_instructions();
                // return;
            } else {
                hash_pwd = hash_message(&words[2]);
                words[2] = &hash_pwd;
                // username= words[1].clone();
                // token=words[2].clone();
                // println!("Hash: {}", token);
                let my_send_msg = words.join(" ");
                // println!("My send msg: {}", my_send_msg);
                if input.starts_with("/login") {
                    // println!("LOGIN");
                    username = words[1].to_string().clone();
                    token = words[2].to_string().clone();
                    // println!("Local addr>> {}", local_addr);
                    let my_send_msg = format!("{} {}", my_send_msg, local_addr);
                    // println!("My send msg: {}", my_send_msg);

                    write.send(Message::Text(my_send_msg.to_string())).await?;
                } else {
                    write.send(Message::Text(my_send_msg.to_string())).await?;
                }
            }

        }

        if input.starts_with("/quit") {
            let new_input = format!("{} {} {} {}",input, username, token, local_addr); //NOTE, added local_addr
            write.send(Message::Text(new_input.to_string())).await?;
            break;
        }
        if input.eq_ignore_ascii_case("/instructions") {
            print_instructions();
        }

        if input.starts_with("/listrooms") || input.starts_with("/createroom") || 
        input.starts_with("/room") || input.starts_with("/leave") || input.starts_with("/listusers") || 
        input.starts_with("/logout") || input.starts_with("/dm") || input.starts_with("/history"){
            
            let new_input = format!("{} {} {} {}",input, username, token, local_addr); //NOTE, added local_addr
            write.send(Message::Text(new_input.to_string())).await?;
            
        }
        
           
        
    }

    println!("Disconnected");
    Ok(())
}