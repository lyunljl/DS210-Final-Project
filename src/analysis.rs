use crate::graph::{TransactionGraph, AccountMetrics};
use std::collections::HashMap;
// fraud detection and analysis for transaction networks.
// provides utilities to identify suspicious accounts based on transaction patterns.

pub struct FraudAnalysis {
    // analyzes transaction data to identify fraudulent account behavior.
    // uses network metrics to detect money mules and collector accounts.
    account_metrics: HashMap<String, AccountMetrics>,
}

impl FraudAnalysis {
    // creates a new fraud analysis from a transaction graph.
    // takes a reference to a transaction graph and calculates account metrics.
    // returns a new fraud analysis struct with calculated account metrics
    pub fn new(graph: &TransactionGraph) -> Self {
        let account_metrics = graph.calculate_account_metrics();
        FraudAnalysis {
            account_metrics,
        }
    }
    
    // identifies the accounts that collect large amounts of money with minimal outflows.
    // returns a vec of (account_id, metrics) pairs sorted by incoming volume (highest first)
    pub fn identify_collector_accounts(&self) -> Vec<(String, AccountMetrics)> {
        let mut collectors = Vec::new();
        // filter accounts based on collector criteria
        for (account, metrics) in &self.account_metrics {
            if metrics.is_collector() {
                collectors.push((account.clone(), metrics.clone()));
            }
        }
        // sort by incoming volume (descending) to prioritize largest volumed collectors
        collectors.sort_by(|a, b| b.1.incoming_volume.partial_cmp(&a.1.incoming_volume).unwrap());
        collectors
    }
    
    // identifies accounts that rapidly move money from many sources to other destinations.
    // returns a vec of (account_id, metrics) pairs sorted by outgoing volume (highest first)
    pub fn identify_money_mule_accounts(&self) -> Vec<(String, AccountMetrics)> {
        let mut mules = Vec::new();
        // filter accounts based on money mule criteria
        for (account, metrics) in &self.account_metrics {
            if metrics.is_money_mule() {
                mules.push((account.clone(), metrics.clone()));
            }
        }
        
        // sort by outgoing volume (descending) to prioritize most active mules
        mules.sort_by(|a, b| b.1.outgoing_volume.partial_cmp(&a.1.outgoing_volume).unwrap());
        mules
    }
    
    // prints a formatted table of collector accounts to the console.
    // limits output to first 500 accounts to prevent the weird terminal cutoffs
    #[cfg(not(test))]
    pub fn print_collector_accounts(&self) {
        let collectors = self.identify_collector_accounts();
        
        println!("\n=== Total of {} accounts detected as fraudulent collector accounts ===", collectors.len());
        println!("{:<15} {:<12} {:<12} {:<15} {:<15} {:<10}", 
            "Account", "In Count", "Out Count", "In Volume", "Out Volume", "Retention");
        
        let display_limit = 500; // edit this to change the number of accounts displayed
        let remaining = if collectors.len() > display_limit {
            collectors.len() - display_limit
        } else {
            0
        };
        
        // Print only up to the display limit to avoid weird terminal cutoffs
        for (i, (account, metrics)) in collectors.iter().enumerate() {
            if i >= display_limit { break; }
            
            println!("{:<15} {:<12} {:<12} {:<15.2} {:<15.2} {:<10.2}", 
                account, 
                metrics.incoming_count, 
                metrics.outgoing_count,
                metrics.incoming_volume,
                metrics.outgoing_volume,
                metrics.retention_rate);
        }
        
        // notify if more accounts were found but not displayed due to terminal cutoffs limit
        if remaining > 0 {
            println!("\n... and {} more accounts not shown", remaining);
        }
    }
    
    // prints a formatted table of money mule accounts to the console.
    // limits output to first 500 accounts to prevent terminal cut offs.
    #[cfg(not(test))]
    pub fn print_money_mule_accounts(&self) {
        let mules = self.identify_money_mule_accounts();
        
        println!("\n=== Total of {} accounts detected as fraudulent money mule accounts ===", mules.len());
        println!("{:<15} {:<12} {:<12} {:<15} {:<15} {:<10}", 
            "Account", "In Count", "Out Count", "In Volume", "Out Volume", "Retention");
        
        let display_limit = 500; // edit this to change the number of accounts displayed
        let remaining = if mules.len() > display_limit {
            mules.len() - display_limit
        } else {
            0
        };
        
        // Print only up to the display limit to avoid terminal cutoffs
        for (i, (account, metrics)) in mules.iter().enumerate() {
            if i >= display_limit { break; }
            
            println!("{:<15} {:<12} {:<12} {:<15.2} {:<15.2} {:<10.2}", 
                account, 
                metrics.incoming_count, 
                metrics.outgoing_count,
                metrics.incoming_volume,
                metrics.outgoing_volume,
                metrics.retention_rate);
        }
        
        // notify if more accounts were found but not displayed due to terminal cutoffs
        if remaining > 0 {
            println!("\n... and {} more accounts not shown", remaining);
        }
    }
    
    // tests to prevent warnings    
    #[cfg(test)]
    pub fn print_collector_accounts(&self) {
        let _ = self.identify_collector_accounts();
    }
    
    #[cfg(test)]
    pub fn print_money_mule_accounts(&self) {
        let _ = self.identify_money_mule_accounts();
    }
}
