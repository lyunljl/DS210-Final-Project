// integration tests for the transaction fraud detection system.
// tests graph construction, account metric calculations, and fraud detection algorithms.

// allow dead code in test context
#![allow(dead_code)]

// tests for fraud detection core components
mod graph {
    include!("../src/graph.rs");
}
mod analysis {
    include!("../src/analysis.rs");
}

use graph::{TransactionGraph, Transaction};
use analysis::FraudAnalysis;

// creates a standardized transaction for testing purposes. 
// arguments
// * `amount` - transaction monetary amount
// * `from` - source account id
// * `to` - destination account id
// returns
// a transaction with default values for non-critical fields
fn create_transaction(amount: f64, from: &str, to: &str) -> Transaction {
    Transaction {
        step: 1,
        r#type: "TRANSFER".to_string(),
        amount,
        name_orig: from.to_string(),
        name_dest: to.to_string(),
        is_fraud: 0,
    }
}

// verifies that the transaction graph correctly stores nodes and edges.
// tests basic graph construction and properties.
#[test]
fn test_basic_graph_construction() {
    let mut graph = TransactionGraph::new();
    
    // add 3 simple transactions
    graph.add_transaction(create_transaction(100.0, "A", "B"));
    graph.add_transaction(create_transaction(200.0, "B", "C"));
    graph.add_transaction(create_transaction(300.0, "A", "C"));
    
    // verify basic graph properties
    assert_eq!(graph.transactions.len(), 3);
    assert_eq!(graph.node_map.len(), 3);
}

// tests that the system correctly identifies collector accounts.
// creates a pattern of transactions with a clear collector pattern and verifies detection.
#[test]
fn test_collector_detection() {
    let mut graph = TransactionGraph::new();
    
    // create a collector account (many incoming transactions, few outgoing)
    graph.add_transaction(create_transaction(1000.0, "User1", "Collector"));
    graph.add_transaction(create_transaction(2000.0, "User2", "Collector"));
    graph.add_transaction(create_transaction(3000.0, "User3", "Collector"));
    graph.add_transaction(create_transaction(4000.0, "User4", "Collector"));
    graph.add_transaction(create_transaction(5000.0, "User5", "Collector"));
    graph.add_transaction(create_transaction(6000.0, "User6", "Collector"));
    // small outgoing transaction
    graph.add_transaction(create_transaction(500.0, "Collector", "User7"));
    
    // create a normal account for comparison
    graph.add_transaction(create_transaction(1000.0, "Normal", "User8"));
    graph.add_transaction(create_transaction(800.0, "Normal", "User9"));
    
    // run analysis
    let fraud_analysis = FraudAnalysis::new(&graph);
    let collectors = fraud_analysis.identify_collector_accounts();
    
    // check detection
    let collector_names: Vec<&String> = collectors.iter().map(|(account, _)| account).collect();
    assert!(collector_names.contains(&&"Collector".to_string()), "Failed to detect collector account");
    assert!(!collector_names.contains(&&"Normal".to_string()), "Incorrectly flagged normal account as collector");
}

// tests that the system correctly identifies money mule accounts.
// creates transaction patterns for both a mule account and a normal high-volume account.
#[test]
fn test_money_mule_detection() {
    let mut graph = TransactionGraph::new();
    
    // create a money mule pattern (receives and quickly sends out most funds)
    graph.add_transaction(create_transaction(20000.0, "Source", "Mule"));
    graph.add_transaction(create_transaction(9000.0, "Mule", "Dest1"));
    graph.add_transaction(create_transaction(10000.0, "Mule", "Dest2"));
    
    // create a normal high-volume account that retains most funds
    graph.add_transaction(create_transaction(20000.0, "Investor", "Normal"));
    graph.add_transaction(create_transaction(5000.0, "Normal", "Expense1"));
    
    // run analysis
    let fraud_analysis = FraudAnalysis::new(&graph);
    let mules = fraud_analysis.identify_money_mule_accounts();
    
    // check detection
    let mule_names: Vec<&String> = mules.iter().map(|(account, _)| account).collect();
    assert!(mule_names.contains(&&"Mule".to_string()), "Failed to detect money mule account");
    assert!(!mule_names.contains(&&"Normal".to_string()), "Incorrectly flagged normal account as money mule");
} 