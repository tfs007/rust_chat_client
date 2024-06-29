use reqwest;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:8080";

    loop {
        println!("\nChoose an action:");
        println!("1. Get index");
        println!("2. Send echo");
        println!("3. Quit");
        print!("Enter your choice (1-3): ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                let response = client.get(&format!("{}/", base_url)).send().await?;
                println!("Response: {}\n", response.text().await?);
            }
            "2" => {
                print!("Enter message to echo: ");
                io::stdout().flush()?;
                let mut message = String::new();
                io::stdin().read_line(&mut message)?;

                let response = client
                    .post(&format!("{}/echo", base_url))
                    .body(message.trim().to_string())
                    .send()
                    .await?;
                println!("Response: {}", response.text().await?);
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}