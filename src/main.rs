mod wallet;
use wallet::{Wallet, SharedWallet};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::{Arc, Mutex};
use serde_json::json;

#[tokio::main]
async fn main() {
    let wallet: SharedWallet = Arc::new(Mutex::new(Wallet::new()));

    // Example usage
    let wallet_clone: Arc<Mutex<Wallet>> = Arc::clone(&wallet);
    tokio::spawn(async move {
        let mut wallet = wallet_clone.lock().unwrap();
        wallet.add_balance("token1", 100);
        println!("Initial balance: {}", wallet.get_balance("token1"));
    });

    #[cfg(unix)]
    {
        use tokio::net::UnixListener;
        let listener = UnixListener::bind("/tmp/wallet.sock").unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let wallet = Arc::clone(&wallet);
            tokio::spawn(async move {
                handle_client(wallet, socket).await;
            });
        }
    }


    // if no one got me i know cfg(unix) always got me MY GOAT

    #[cfg(not(unix))]
    {
        println!("This only works on Unix-like systems.");
    }
}

#[cfg(unix)]
async fn handle_client(wallet: SharedWallet, mut socket: tokio::net::UnixStream) {
    let mut buf = vec![0; 1024];
    let n = socket.read(&mut buf).await.unwrap();
    let request: serde_json::Value = serde_json::from_slice(&buf[..n]).unwrap();

    let response = match request["action"].as_str() {
        Some("add") => {
            let amount = request["amount"].as_u64().unwrap();
            let token = request["token"].as_str().unwrap();
            let mut wallet = wallet.lock().unwrap();
            wallet.add_balance(token, amount);
            json!({"status": "success"})
        }
        Some("subtract") => {
            let amount = request["amount"].as_u64().unwrap();
            let token = request["token"].as_str().unwrap();
            let mut wallet = wallet.lock().unwrap();
            match wallet.subtract_balance(token, amount) {
                Ok(_) => json!({"status": "success"}),
                Err(e) => json!({"status": "error", "message": e}),
            }
        }
        Some("balance") => {
            let token = request["token"].as_str().unwrap();
            let wallet = wallet.lock().unwrap();
            let balance = wallet.get_balance(token);
            json!({"status": "success", "balance": balance})
        }
        _ => json!({"status": "error", "message": "invalid action"}),
    };

    let response_bytes = serde_json::to_vec(&response).unwrap();
    socket.write_all(&response_bytes).await.unwrap();
}
