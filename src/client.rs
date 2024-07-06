use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    if let Err(e) = run_client().await {
        eprintln!("Client error: {}", e);
    }
}

async fn run_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    
    // Add balance request
    let request = json!({
        "action": "add",
        "token": "token1",
        "amount": 50
    });
    let request_bytes = serde_json::to_vec(&request)?;
    stream.write_all(&request_bytes).await?;

    // Read add balance response
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Value = serde_json::from_slice(&buf[..n])?;
    println!("Add balance response: {}", response);

    // Get balance request
    let request = json!({
        "action": "balance",
        "token": "token1"
    });
    let request_bytes = serde_json::to_vec(&request)?;
    stream.write_all(&request_bytes).await?;

    // Read get balance response
    let n = stream.read(&mut buf).await?;
    let response: Value = serde_json::from_slice(&buf[..n])?;
    println!("Get balance response: {}", response);

    Ok(())
}
