use std::collections::HashMap;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
#[allow(dead_code)]
// Fields preserved for CSV deserialization and tests
pub struct Transaction {
    pub step: u32,
    pub r#type: String,
    pub amount: f64,
    pub nameOrig: String,
    pub nameDest: String,
    pub isFraud: u8,
}

pub struct TransactionGraph {
    pub graph: DiGraph<String, f64>,
    pub node_map: HashMap<String, NodeIndex>,
    pub transactions: Vec<Transaction>,
}

impl TransactionGraph {
    pub fn new() -> Self {
        TransactionGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Add nodes if they don't exist
        let orig_idx = *self.node_map.entry(transaction.nameOrig.clone()).or_insert_with(|| {
            self.graph.add_node(transaction.nameOrig.clone())
        });

        let dest_idx = *self.node_map.entry(transaction.nameDest.clone()).or_insert_with(|| {
            self.graph.add_node(transaction.nameDest.clone())
        });

        // Add edge with weight as transaction amount
        self.graph.add_edge(orig_idx, dest_idx, transaction.amount);
        
        // Store the transaction
        self.transactions.push(transaction);
    }

    // Calculate account metrics for money mule and collector detection
    pub fn calculate_account_metrics(&self) -> HashMap<String, AccountMetrics> {
        let mut metrics = HashMap::new();
        
        // Initialize metrics for all nodes
        for (account, _) in self.node_map.iter() {
            metrics.insert(account.clone(), AccountMetrics::new());
        }
        // Process all transactions to compute metrics
        for edge in self.graph.edge_references() {
            let source = self.graph[edge.source()].clone();
            let target = self.graph[edge.target()].clone();
            let amount = *edge.weight();
            
            // Update outgoing metrics for source
            if let Some(source_metrics) = metrics.get_mut(&source) {
                source_metrics.outgoing_count += 1;
                source_metrics.outgoing_volume += amount;
            }
            
            // Update incoming metrics for target
            if let Some(target_metrics) = metrics.get_mut(&target) {
                target_metrics.incoming_count += 1;
                target_metrics.incoming_volume += amount;
            }
        }
        
        // Calculate retention rates
        for (_, metrics) in metrics.iter_mut() {
            metrics.calculate_retention_rate();
        }
        
        metrics
    }
}

#[derive(Debug, Clone)]
pub struct AccountMetrics {
    pub incoming_count: u32,
    pub outgoing_count: u32,
    pub incoming_volume: f64,
    pub outgoing_volume: f64, 
    pub retention_rate: f64,
}

impl AccountMetrics {
    pub fn new() -> Self {
        AccountMetrics {
            incoming_count: 0,
            outgoing_count: 0,
            incoming_volume: 0.0,
            outgoing_volume: 0.0,
            retention_rate: 0.0,
        }
    }
    
    pub fn calculate_retention_rate(&mut self) {
        if self.incoming_volume > 0.0 {
            let retained = self.incoming_volume - self.outgoing_volume;
            // Calculate retention rate - can be negative if outgoing > incoming
            self.retention_rate = retained / self.incoming_volume;
        }
    }
    
    pub fn is_collector(&self) -> bool {
        // A collector has high incoming volume, significantly more incoming than outgoing 
        // transactions, and high retention rate
        self.incoming_count > 5 && 
        self.incoming_count > self.outgoing_count * 3 && 
        self.retention_rate > 0.7
    }
    
    pub fn is_money_mule(&self) -> bool {
        // A money mule primarily forwards most incoming funds
        self.incoming_count >= 1 &&
        self.outgoing_count >= 1 &&
        // High volume throughput (most money comes in and goes right back out)
        self.outgoing_volume > 0.5 * self.incoming_volume &&
        // Low retention rate (they don't keep much of the money)
        self.retention_rate < 0.4 &&
        // Significant transaction volume
        self.incoming_volume > 10000.0
    }
}
