use sha2::{Sha256, Digest};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, data: &str, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[derive(Debug)]
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
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_index = previous_block.index + 1;
        let new_block = Block::new(new_index, data, previous_block.hash.clone());
        self.blocks.push(new_block);
    }

    pub fn view_chain(&self) {
        println!("Blockchain:");
        println!("==========");
        for block in &self.blocks {
            println!("Block #{}", block.index);
            println!("Timestamp: {}", block.timestamp);
            println!("Data: {}", block.data);
            println!("Previous Hash: {}", block.previous_hash);
            println!("Hash: {}", block.hash);
            println!("-------------------");
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    
    println!("Mini Blockchain CLI");
    println!("Commands:");
    println!("  add <data>  - Add a new block with data");
    println!("  view        - View the entire blockchain");
    println!("  exit        - Exit the program");
    println!();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        match parts.as_slice() {
            ["add", data] => {
                blockchain.add_block(data.to_string());
                println!("Block added successfully!");
            }
            ["view"] => {
                blockchain.view_chain();
            }
            ["exit"] => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid command. Use 'add <data>', 'view', or 'exit'");
            }
        }
        println!();
    }
}
