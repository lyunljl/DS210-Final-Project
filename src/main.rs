// financial transaction network analysis for fraud detection.
// this program analyzes financial transaction data to identify accounts 
// likely involved in money laundering and other fraudulent activities.
// it identifies two main types of suspicious accounts:
// 1. collector accounts - which accumulate money with minimal outflows
// 2. money mule accounts - which rapidly move money between accounts
mod graph;
mod analysis;
mod utilities;

use std::path::Path;
use analysis::FraudAnalysis;
use utilities::{Timer, handle_error, read_transaction_dataset};

// program entry point - loads transaction data, builds a graph representation, and performs fraud analysis to identify suspicious accounts.
fn main() {
    // path to the cleaned dataset
    let file_path = "data/cleaned_fraud_dataset.csv";
    
    // verify the data file exists before proceeding
    if !Path::new(file_path).exists() {
        handle_error(format!("File not found: {}", file_path));
        return;
    }
    
    println!("Money Laundering Detection Analysis");
    println!("===================================");
    
    // load data and build the transaction graph
    let load_timer = Timer::new("Data loading and graph construction");
    let graph = match read_transaction_dataset(file_path) {
        Ok(g) => g,
        Err(e) => {
            handle_error(format!("Failed to load data: {}", e));
            return;
        }
    };
    drop(load_timer);

    // output summary statistics about the loaded data
    println!("Loaded {} transactions, {} unique accounts", 
        graph.transactions.len(),
        graph.node_map.len());
    
    // create the fraud analysis module and run analysis
    let analysis_timer = Timer::new("Fraud analysis");
    let fraud_analysis = FraudAnalysis::new(&graph);
    
    // identify and print collector accounts (accounts that accumulate funds)
    fraud_analysis.print_collector_accounts();
    
    // identify and print money mule accounts (accounts that rapidly forward funds)
    fraud_analysis.print_money_mule_accounts();
    
    drop(analysis_timer);
    
    println!("\nAnalysis complete.");
}
