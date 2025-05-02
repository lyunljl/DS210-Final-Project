// transactions as a graph representation and analysis for financial network data.
// implements a directed graph model for tracking money flows between accounts.
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// a unique identifier for a node in a graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

// an edge in a directed graph, with a weight
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Edge<W> {
    // source node index
    pub source: NodeIndex,
    // target node index
    pub target: NodeIndex,
    // edge weight
    pub weight: W,
}

// a reference to an edge
#[derive(Debug, Clone)]
pub struct EdgeRef<'a, N, W> {
    // source node index
    pub source_idx: NodeIndex,
    // target node index
    pub target_idx: NodeIndex,
    // edge weight reference
    pub weight: &'a W,
    // reference to the graph (to access nodes)
    #[allow(dead_code)]
    pub graph: &'a DiGraph<N, W>,
}

impl<'a, N, W> EdgeRef<'a, N, W> {
    // get the source node index
    pub fn source(&self) -> NodeIndex {
        self.source_idx
    }
    
    // get the target node index
    pub fn target(&self) -> NodeIndex {
        self.target_idx
    }
    
    // get the edge weight
    pub fn weight(&self) -> &W {
        self.weight
    }
}

// a directed graph with weighted edges
#[derive(Debug, Clone)]
pub struct DiGraph<N, W> {
    // the next available node index
    next_node_id: usize,
    // node storage - maps the indices to node data
    nodes: HashMap<NodeIndex, N>,
    // edge storage - maps the (source, target) to edge weight
    edges: HashMap<(NodeIndex, NodeIndex), W>,
    // outgoing edges for each node
    outgoing: HashMap<NodeIndex, HashSet<NodeIndex>>,
    // incoming edges for each node
    incoming: HashMap<NodeIndex, HashSet<NodeIndex>>,
}

impl<N, W> DiGraph<N, W> 
where
    N: Clone,
    W: Clone,
{
    // create a new empty directed graph
    pub fn new() -> Self {
        DiGraph {
            next_node_id: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
        }
    }
    
    // add a node to the graph
    pub fn add_node(&mut self, node: N) -> NodeIndex {
        let idx = NodeIndex(self.next_node_id);
        self.next_node_id += 1;
        
        self.nodes.insert(idx, node);
        self.outgoing.insert(idx, HashSet::new());
        self.incoming.insert(idx, HashSet::new());
        
        idx
    }
    
    // add an edge to the graph with the given weight
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, weight: W) {
        self.edges.insert((source, target), weight);
        
        if let Some(outgoing) = self.outgoing.get_mut(&source) {
            outgoing.insert(target);
        }
        
        if let Some(incoming) = self.incoming.get_mut(&target) {
            incoming.insert(source);
        }
    }
    
    // get a reference to a node by index
    #[allow(dead_code)]
    pub fn node_weight(&self, idx: NodeIndex) -> Option<&N> {
        self.nodes.get(&idx)
    }
    
    // get a reference to an edge's weight
    #[allow(dead_code)]
    pub fn edge_weight(&self, source: NodeIndex, target: NodeIndex) -> Option<&W> {
        self.edges.get(&(source, target))
    }
    
    // get all edges as references
    pub fn edge_references(&self) -> Vec<EdgeRef<N, W>> {
        let mut result = Vec::new();
        
        for ((source, target), weight) in &self.edges {
            result.push(EdgeRef {
                source_idx: *source,
                target_idx: *target,
                weight,
                graph: self,
            });
        }
        
        result
    }
    
    // get all outgoing neighbors of a node
    #[allow(dead_code)]
    pub fn neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.outgoing
            .get(&node)
            .into_iter()
            .flat_map(|neighbors| neighbors.iter().copied())
    }
    
    // get all incoming neighbors of a node
    #[allow(dead_code)]
    pub fn incoming_neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.incoming
            .get(&node)
            .into_iter()
            .flat_map(|neighbors| neighbors.iter().copied())
    }
    
    // check if a node exists in the graph
    #[allow(dead_code)]
    pub fn contains_node(&self, idx: NodeIndex) -> bool {
        self.nodes.contains_key(&idx)
    }
    
    // get the number of nodes in the graph
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    // get the number of edges in the graph
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

// enable indexing into a graph with a NodeIndex to get the node data
impl<N, W> std::ops::Index<NodeIndex> for DiGraph<N, W> {
    type Output = N;
    
    fn index(&self, idx: NodeIndex) -> &Self::Output {
        &self.nodes[&idx]
    }
}

// enable mutable indexing into a graph with a NodeIndex
impl<N, W> std::ops::IndexMut<NodeIndex> for DiGraph<N, W> {
    fn index_mut(&mut self, idx: NodeIndex) -> &mut Self::Output {
        self.nodes.get_mut(&idx).unwrap()
    }
}

// represents a single financial transaction between two accounts.
// contains all transaction details from the original dataset.
#[derive(Debug, Clone)]
    pub struct Transaction {
    // transaction step/time (sequential identifier)
    #[allow(dead_code)]
    pub step: u32,
    // transaction type (payment, transfer, etc.)
    #[allow(dead_code)]
    pub r#type: String,
    // monetary amount of the transaction
    pub amount: f64,
    // source account identifier
    pub name_orig: String,
    // destination account identifier
    pub name_dest: String,
    // fraud indicator (1 if fraudulent, 0 if legitimate)
    #[allow(dead_code)]
    pub is_fraud: u8,
}

// models a network of financial transactions as a directed graph.
// nodes represent accounts and edges represent money transfers.
pub struct TransactionGraph {
    // directed graph with accounts as nodes and money transfers as weighted edges
    pub graph: DiGraph<String, f64>,
    // maps account ids to their corresponding node indices in the graph
    pub node_map: HashMap<String, NodeIndex>,
    // original transaction records
    pub transactions: Vec<Transaction>,
}

impl TransactionGraph {
    // creates a new empty transaction graph.
    // returns a TransactionGraph with no nodes or edges
    pub fn new() -> Self {
        TransactionGraph {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
            transactions: Vec::new(),
        }
    }

    // adds a transaction to the graph, creating nodes if needed.
    // takes in  `transaction` - The transaction to add
    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Add nodes if they don't exist
        let orig_idx = *self.node_map.entry(transaction.name_orig.clone()).or_insert_with(|| {
            self.graph.add_node(transaction.name_orig.clone())
        });

        let dest_idx = *self.node_map.entry(transaction.name_dest.clone()).or_insert_with(|| {
            self.graph.add_node(transaction.name_dest.clone())
        });

        // Add edge with weight as transaction amount
        self.graph.add_edge(orig_idx, dest_idx, transaction.amount);
        
        // Store the transaction
        self.transactions.push(transaction);
    }

    // Analyzes the transaction graph to calculate metrics for each account.
    // Computes incoming/outgoing counts, volumes, and retention rates.
    // returns HashMap mapping account IDs to their calculated metrics
    pub fn calculate_account_metrics(&self) -> HashMap<String, AccountMetrics> {
        let mut metrics = HashMap::new();
        
        // initialize metrics for all nodes
        for (account, _) in self.node_map.iter() {
            metrics.insert(account.clone(), AccountMetrics::new());
        }
        
        // process all transactions to compute metrics
        for edge in self.graph.edge_references() {
            let source = self.graph[edge.source()].clone();
            let target = self.graph[edge.target()].clone();
            let amount = *edge.weight();
            
            // update outgoing metrics for source
            if let Some(source_metrics) = metrics.get_mut(&source) {
                source_metrics.outgoing_count += 1;
                source_metrics.outgoing_volume += amount;
            }
            
            // update incoming metrics for target
            if let Some(target_metrics) = metrics.get_mut(&target) {
                target_metrics.incoming_count += 1;
                target_metrics.incoming_volume += amount;
            }
        }
        
        // calculate retention rates
        for (_, metrics) in metrics.iter_mut() {
            metrics.calculate_retention_rate();
        }
        
        metrics
    }
}

// holds statistical metrics for an account's transaction behavior.
// used to identify suspicious activity patterns.
#[derive(Debug, Clone)]
pub struct AccountMetrics {
    // number of incoming transactions
    pub incoming_count: u32,
    // number of outgoing transactions
    pub outgoing_count: u32,
    // total monetary volume received
    pub incoming_volume: f64,
    // total monetary volume sent
    pub outgoing_volume: f64, 
    // fraction of incoming funds retained (not forwarded)
    pub retention_rate: f64,
}

impl AccountMetrics {
    // creates a new accountmetrics with zero values. 
    // returns empty accountmetrics instance
    pub fn new() -> Self {
        AccountMetrics {
            incoming_count: 0,
            outgoing_count: 0,
            incoming_volume: 0.0,
            outgoing_volume: 0.0,
            retention_rate: 0.0,
        }
    }
    
    // calculates what fraction of incoming funds are retained by the account.
    // a negative retention rate indicates the account sent more than it received.
    pub fn calculate_retention_rate(&mut self) {
        if self.incoming_volume > 0.0 {
            let retained = self.incoming_volume - self.outgoing_volume;
            // Calculate retention rate - can be negative if outgoing > incoming
            self.retention_rate = retained / self.incoming_volume;
        }
    }
    
    // determines if an account exhibits collector behavior.
    // collectors receive money from many sources but rarely send it out.
    // returns true if the account matches collector patterns
    pub fn is_collector(&self) -> bool {
        // a collector has high incoming volume, significantly more incoming than outgoing 
        // transactions, and high retention rate
        self.incoming_count > 5 && 
        self.incoming_count > self.outgoing_count * 3 && 
        self.retention_rate > 0.7
    }
    
    // determines if an account exhibits money mule behavior.
    // money mules receive and quickly forward large amounts of money. 
    // returns true if the account matches money mule patterns
    pub fn is_money_mule(&self) -> bool {
        // a money mule primarily forwards most incoming funds
        self.incoming_count >= 1 &&
        self.outgoing_count >= 1 &&
        // high volume throughput (most money comes in and goes right back out)
        self.outgoing_volume > 0.5 * self.incoming_volume &&
        // low retention rate (they don't keep much of the money)
        self.retention_rate < 0.4 &&
        // significant transaction volume
        self.incoming_volume > 10000.0
    }
}
