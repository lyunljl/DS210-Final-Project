use std::time::Instant;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::graph::{Transaction, TransactionGraph};

// Timing utility for performance measurement
pub struct Timer {
    start_time: Instant,
    name: String,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        println!("Starting {}", name);
        Timer {
            start_time: Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        println!("{} completed in {:.2?}", self.name, duration);
    }
}

// Function to read dataset from CSV file
pub fn read_transaction_dataset(file_path: &str) -> Result<TransactionGraph, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut graph = TransactionGraph::new();
    
    for result in reader.deserialize() {
        let transaction: Transaction = result?;
        graph.add_transaction(transaction);
    }

    Ok(graph)
}

// Error handling utility
pub fn handle_error<T: std::fmt::Display>(err: T) {
    eprintln!("Error: {}", err);
}
