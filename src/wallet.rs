// src/wallet.rs
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    balance: HashMap<String, u64>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            balance: HashMap::new(),
        }
    }

    pub fn add_balance(&mut self, token: &str, amount: u64) {
        *self.balance.entry(token.to_string()).or_insert(0) += amount;
    }

    pub fn subtract_balance(&mut self, token: &str, amount: u64) -> Result<(), String> {
        let entry = self.balance.entry(token.to_string()).or_insert(0);
        if *entry < amount {
            return Err("Insufficient balance".into());
        }
        *entry -= amount;
        Ok(())
    }

    pub fn get_balance(&self, token: &str) -> u64 {
        *self.balance.get(token).unwrap_or(&0)
    }
}

pub type SharedWallet = Arc<Mutex<Wallet>>;
