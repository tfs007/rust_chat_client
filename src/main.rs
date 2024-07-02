use std::io::{self, Write};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use url::Url;
use sha2::{Sha256, Digest};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
// use std::net::TcpStream;
use tokio::net::TcpStream;


async fn receive_server_message(read: &mut futures_util::stream::SplitStream<WebSocketStream<TcpStream>>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(message) = read.next().await {
        match message? {
            Message::Text(text) => Ok(text),
            _ => Err("Received non-text message".into()),
        }
    } else {
        Err("No message received".into())
    }
}

async fn handle_login(write: &mut futures_util::stream::SplitSink<WebSocketStream<TcpStream>, Message>, 
    read: &mut futures_util::stream::SplitStream<WebSocketStream<TcpStream>>) -> Result<(), Box<dyn std::error::Error>> {
// Send login command
    let login_command = "/login username password";
    write.send(Message::Text(login_command.to_string())).await?;

    // Receive the server's response
    match receive_server_message(read).await {
        Ok(response) => {
            // Store the response in a variable
            let login_result = response;

            println!("Login result: {}", login_result);

            if login_result == "ok" {
                println!("Login successful");
            } else {
                println!("Login failed");
            }
        },
        Err(e) => println!("Error receiving login result: {}", e),
    }

    Ok(())
}

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

// async fn handle_login(stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), Box<dyn std::error::Error>> {
//     let login_message = "/login username password".to_string(); // Replace with actual login details

//     // Send the login message
//     stream.send(Message::Text(login_message)).await?;

//     // Wait for the server's response
//     if let Some(message) = stream.next().await {
//         match message? {
//             Message::Text(text) => {
//                 match text.as_str() {
//                     "ok" => println!("Login successful"),
//                     "notok" => println!("Login failed"),
//                     _ => println!("Unexpected response: {}", text),
//                 }
//             },
//             _ => println!("Received non-text message"),
//         }
//     } else {
//         println!("No response received");
//     }

//     Ok(())
// }

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
    // tokio::spawn(async move {
    //     while let Some(message) = read.next().await {
    //         match message {
    //             Ok(msg) => println!("{}", msg),
    //             Err(e) => eprintln!("Error receiving message: {}", e),
    //         }
    //     }
    // });
    print_instructions();
    let mut username= "";
    let mut token= "";
    


    // Read input from the user and send it to the server
    loop {

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("/quit") {
            break;
        }

        if !(input.starts_with("/register") || input.starts_with("/login") || input.starts_with("/instructions")) {
            println!("\x1b[33mYou need to be logged in to execute this instruction.\x1b[0m");
            continue;
        }

        
        if input.starts_with("/register") || input.starts_with("/login") {
            
            let mut words: Vec<&str> = input.split_whitespace().collect();
            // println!("Words length: {}", words.len().to_string());
            if words.len() <= 2{
                println!("\x1b[33mPass username and password! Please see instructions below.\x1b[0m");
                print_instructions();
                // return;
            } else {
                let hash_pwd = hash_message(&words[2]);
                words[2] = &hash_pwd;
                let my_send_msg = words.join(" "); 
                if input.starts_with("/login") {
                    println!("LOGIN");
                    write.send(Message::Text(my_send_msg.to_string())).await?; 
                    // >>>
                    // Loop to read messages from the server
                while let Some(message) = read.next().await {
                    match message {
                        Ok(msg) => {
                            // Check if the message is a text message
                            if msg.is_text() {
                                // Get the text message
                                let text = msg.into_text().expect("Failed to get text");

                                // Print the received text message
                                println!("Received:--> {}", text);
                                break;
                            }
                        }
                        Err(e) => {
                            // Handle errors in receiving messages
                            eprintln!("Error receiving message: {}", e);
                        }
                    }
                }

                    // <<<

                } else {

                    write.send(Message::Text(my_send_msg.to_string())).await?;
                }            

            }
            
            



        } else {
            write.send(Message::Text(input.to_string())).await?;
        }
        if input.eq_ignore_ascii_case("/instructions") {
            print_instructions();
        }


    }

    println!("Disconnected");
    Ok(())
}