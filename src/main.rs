mod graph;
mod analysis;
mod utilities;

use std::path::Path;
use analysis::FraudAnalysis;
use utilities::{Timer, handle_error, read_transaction_dataset};

fn main() {
    // Path to the cleaned dataset
    let file_path = "data/cleaned_fraud_dataset.csv";
    
    if !Path::new(file_path).exists() {
        handle_error(format!("File not found: {}", file_path));
        return;
    }
    
    println!("Money Laundering Detection Analysis");
    println!("===================================");
    
    // Load data and build the transaction graph
    let load_timer = Timer::new("Data loading and graph construction");
    let graph = match read_transaction_dataset(file_path) {
        Ok(g) => g,
        Err(e) => {
            handle_error(format!("Failed to load data: {}", e));
            return;
        }
    };
    drop(load_timer);
    
    println!("Loaded {} transactions, {} unique accounts", 
        graph.transactions.len(),
        graph.node_map.len());
    
    // Create the fraud analysis module
    let analysis_timer = Timer::new("Fraud analysis");
    let fraud_analysis = FraudAnalysis::new(&graph);
    
    // Identify and print collector accounts
    fraud_analysis.print_collector_accounts();
    
    // Identify and print money mule accounts
    fraud_analysis.print_money_mule_accounts();
    
    drop(analysis_timer);
    
    println!("\nAnalysis complete.");
}
