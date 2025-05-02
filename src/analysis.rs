use crate::graph::{TransactionGraph, AccountMetrics};
use std::collections::HashMap;

pub struct FraudAnalysis {
    account_metrics: HashMap<String, AccountMetrics>,
}

impl FraudAnalysis {
    pub fn new(graph: &TransactionGraph) -> Self {
        let account_metrics = graph.calculate_account_metrics();
        
        FraudAnalysis {
            account_metrics,
        }
    }
    
    pub fn identify_collector_accounts(&self) -> Vec<(String, AccountMetrics)> {
        let mut collectors = Vec::new();
        
        for (account, metrics) in &self.account_metrics {
            if metrics.is_collector() {
                collectors.push((account.clone(), metrics.clone()));
            }
        }
        
        // Sort by incoming volume (descending)
        collectors.sort_by(|a, b| b.1.incoming_volume.partial_cmp(&a.1.incoming_volume).unwrap());
        collectors
    }
    
    pub fn identify_money_mule_accounts(&self) -> Vec<(String, AccountMetrics)> {
        let mut mules = Vec::new();
        
        for (account, metrics) in &self.account_metrics {
            if metrics.is_money_mule() {
                mules.push((account.clone(), metrics.clone()));
            }
        }
        
        // Sort by outgoing volume (descending)
        mules.sort_by(|a, b| b.1.outgoing_volume.partial_cmp(&a.1.outgoing_volume).unwrap());
        mules
    }
    
    pub fn print_collector_accounts(&self) {
        let collectors = self.identify_collector_accounts();
        
        println!("\n=== Total of {} accounts detected as fraudulent collector accounts ===", collectors.len());
        println!("{:<15} {:<12} {:<12} {:<15} {:<15} {:<10}", 
            "Account", "In Count", "Out Count", "In Volume", "Out Volume", "Retention");
            
        for (account, metrics) in &collectors {
            println!("{:<15} {:<12} {:<12} {:<15.2} {:<15.2} {:<10.2}", 
                account, 
                metrics.incoming_count, 
                metrics.outgoing_count,
                metrics.incoming_volume,
                metrics.outgoing_volume,
                metrics.retention_rate);
        }
    }
    
    pub fn print_money_mule_accounts(&self) {
        let mules = self.identify_money_mule_accounts();
        
        println!("\n=== Total of {} accounts detected as fraudulent money mule accounts ===", mules.len());
        println!("{:<15} {:<12} {:<12} {:<15} {:<15} {:<10}", 
            "Account", "In Count", "Out Count", "In Volume", "Out Volume", "Retention");
            
        for (account, metrics) in &mules {
            println!("{:<15} {:<12} {:<12} {:<15.2} {:<15.2} {:<10.2}", 
                account, 
                metrics.incoming_count, 
                metrics.outgoing_count,
                metrics.incoming_volume,
                metrics.outgoing_volume,
                metrics.retention_rate);
        }
    }
}
