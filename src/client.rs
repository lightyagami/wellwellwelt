use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use serde_json::json;

#[tokio::main]
async fn main() {
    #[cfg(unix)]
    {
        let mut stream = UnixStream::connect("/tmp/wallet.sock").await.unwrap();

        // Add balance
        let request = json!({
            "action": "add",
            "token": "token1",
            "amount": 50
        });
        let request_bytes = serde_json::to_vec(&request).unwrap();
        stream.write_all(&request_bytes).await.unwrap();

        // Read response
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        let response: serde_json::Value = serde_json::from_slice(&buf[..n]).unwrap();
        println!("Response: {}", response);

        // Get balance
        let request = json!({
            "action": "balance",
            "token": "token1"
        });
        let request_bytes = serde_json::to_vec(&request).unwrap();
        stream.write_all(&request_bytes).await.unwrap();

        // Read response
        let n = stream.read(&mut buf).await.unwrap();
        let response: serde_json::Value = serde_json::from_slice(&buf[..n]).unwrap();
        println!("Response: {}", response);
    }

    #[cfg(not(unix))]
    {
        println!("This client example only works on Unix-like systems.");
    }
}
