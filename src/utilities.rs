// utility functions and structs for the financial fraud detection system.
// provides timing, data loading, and error handling functionality.
use std::time::Instant;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use crate::graph::{Transaction, TransactionGraph};

// raii timer for measuring and reporting execution duration of code sections.
// automatically reports elapsed time when the timer goes out of scope.
pub struct Timer {
    start_time: Instant,
    name: String,
}

impl Timer {
    // creates a new timer with the given operation name.
    // prints a start message and begins timing.
    // takes in `name` as an argument- description of the operation being timed
    // returns a new timer instance
    pub fn new(name: &str) -> Self {
        println!("Starting {}", name);
        Timer {
            start_time: Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for Timer {
    // automatically called when the timer goes out of scope.
    // prints the elapsed time since the timer was created.
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        println!("{} completed in {:.2?}", self.name, duration);
    }
}

// creates a transaction from a csv record without using serde.
// takes in `record` as an argument- csv record containing transaction data
// returns a result containing either a transaction or an error message
fn transaction_from_record(record: &csv::StringRecord) -> Result<Transaction, Box<dyn Error>> {
    if record.len() < 6 {
        return Err(format!("Not enough fields in record: expected 6, got {}", record.len()).into());
    }
    
    let step = record[0].parse::<u32>()
        .map_err(|e| format!("Failed to parse step: {}", e))?;
        
    let r#type = record[1].to_string();
    
    let amount = record[2].parse::<f64>()
        .map_err(|e| format!("Failed to parse amount: {}", e))?;
        
    let name_orig = record[3].to_string();
    let name_dest = record[4].to_string();
    
    let is_fraud = record[5].parse::<u8>()
        .map_err(|e| format!("Failed to parse is_fraud: {}", e))?;
        
    Ok(Transaction {
        step,
        r#type,
        amount,
        name_orig,
        name_dest,
        is_fraud,
    })
}

// loads transaction data from a csv file and builds a transaction graph.
// uses manual parsing instead of serde deserialization.
// takes in `file_path` as an argument- path to the csv file containing transaction data
// returns a result containing either a populated transactiongraph or an error
// returns an error if the file cannot be opened or if csv parsing fails
pub fn read_transaction_dataset(file_path: &str) -> Result<TransactionGraph, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut graph = TransactionGraph::new();
    
    // parsing the csv file
    for result in reader.records() {
        match result {
            Ok(record) => {
                match transaction_from_record(&record) {
                    Ok(transaction) => graph.add_transaction(transaction),
                    Err(e) => eprintln!("Warning: failed to parse record: {}", e),
                }
            },
            Err(e) => eprintln!("Warning: error reading CSV record: {}", e),
        }
    }

    Ok(graph)
}

// prints an error message to stderr.
// takes in `err` as an argument- the error to display
pub fn handle_error<T: std::fmt::Display>(err: T) {
    eprintln!("Error: {}", err);
}