use sha2::{Sha256, Digest};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use std::fs;

const DIFFICULTY: usize = 4; // Number of leading zeros for mining

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    sender: String,
    receiver: String,
    amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut nonce = 0;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        // Mining: find hash with DIFFICULTY leading zeros
        let mut hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash, nonce);
        while !hash.starts_with(&"0".repeat(DIFFICULTY)) {
            nonce += 1;
            hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash, nonce);
        }

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
            nonce,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(nonce.to_string());
        for tx in transactions {
            hasher.update(&tx.sender);
            hasher.update(&tx.receiver);
            hasher.update(tx.amount.to_string());
        }
        hasher.update(previous_hash);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain { blocks: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.blocks.last().unwrap();
        let new_index = previous_block.index + 1;
        let new_block = Block::new(new_index, transactions, previous_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn view_chain(&self) {
        println!("Blockchain:");
        println!("==========");
        for block in &self.blocks {
            println!("Block #{}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Nonce: {}", block.nonce);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
            if block.transactions.is_empty() {
                println!("Transactions: None");
            } else {
                println!("Transactions:");
                for tx in &block.transactions {
                    println!("  {} -> {} : {}", tx.sender, tx.receiver, tx.amount);
                }
            }
            println!("-------------------");
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            let recalculated_hash = Block::calculate_hash(
                current.index,
                current.timestamp,
                &current.transactions,
                &current.previous_hash,
                current.nonce,
            );

            if current.hash != recalculated_hash {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        fs::write(filename, json).expect("Unable to save blockchain");
    }

    pub fn load_from_file(filename: &str) -> Option<Self> {
        if let Ok(data) = fs::read_to_string(filename) {
            let bc: Blockchain = serde_json::from_str(&data).unwrap();
            Some(bc)
        } else {
            None
        }
    }
}

fn main() {
    let filename = "blockchain.json";
    let mut blockchain = Blockchain::load_from_file(filename).unwrap_or_else(Blockchain::new);

    println!("Mini Blockchain CLI with Mining & Transactions");
    println!("Commands:");
    println!("  add <sender> <receiver> <amount>  - Add a new transaction as a block");
    println!("  view                              - View the entire blockchain");
    println!("  validate                          - Check if blockchain is valid");
    println!("  exit                              - Exit the program");
    println!();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.as_slice() {
            ["add", sender, receiver, amount] => {
                if let Ok(amount) = amount.parse::<u32>() {
                    let tx = Transaction {
                        sender: sender.to_string(),
                        receiver: receiver.to_string(),
                        amount,
                    };
                    blockchain.add_block(vec![tx]);
                    println!("Block mined and added successfully!");
                    blockchain.save_to_file(filename);
                } else {
                    println!("Invalid amount");
                }
            }
            ["view"] => blockchain.view_chain(),
            ["validate"] => {
                println!("Blockchain valid? {}", blockchain.is_chain_valid());
            }
            ["exit"] => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command. Use 'add <sender> <receiver> <amount>', 'view', 'validate', or 'exit'");
            }
        }
        println!();
    }
}
