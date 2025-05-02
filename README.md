# Financial Fraud Detection Network Analysis

## A. Project Overview

### Goal
My project analyzes a financial transaction networks to identify accounts likely involved in money laundering by detecting two key patterns:
1. **Collector accounts** - Accounts that accumulate/collect funds from multiple sources but rarely send money any money out. Additionally collector accounts recieve money from multiple sources unlike typical savings accounts which may exhibit the "rarely send money out pattern".
2. **Money mule accounts** - Accounts that quickly forward received funds to other destinations. These accounts recieve large amounts of money from tracked and untracked sources. They are typically ones that recieve large amounts of money from various sources and send them out in large amounts to various sources.

### Dataset
- **Source**: Synthetic Financial Dataset for Fraud Detection (modified version of Kaggle dataset)
- **Original File**: `Synthetic Financial Datasets For Fraud Detection.csv` (471MB)
- **Cleaned File**: `cleaned_fraud_dataset.csv` (128MB)
- **Structure**: CSV with 6 columns:
  - `step`: Time step (day) of the transaction
  - `type`: Type of transaction (TRANSFER or CASH_OUT)
  - `amount`: Amount of the transaction
  - `name_orig`: Origin account
  - `name_dest`: Destination account
  - `is_fraud`: Binary indicator (1 for fraudulent transaction, 0 for legitimate)
- **Dataset Statistics**:
  - 2.7+ million transactions after filtering
  - 3.2+ million unique accounts
  - 8,213 accounts flagged as fraudulent in original dataset
  - Average transaction value: $179,594.35
  - Median transaction value: $74,877.42
  - Approximately 94% of transactions have amounts below $500,000

## B. Data Processing

### Loading Data
- CSV data is loaded using the `csv` crate with manual parsing
- Each transaction record is parsed field-by-field from CSV records
- Custom error handling catches and reports parsing issues
- Transactions are filtered to include only TRANSFER and CASH_OUT types (removing PAYMENT, DEBIT, CASH_IN)
- **No external deserialization libraries**: The project uses manual CSV parsing instead of relying on serde for deserialization

### Data Transformation
- A custom directed graph data structure is built where:
  - Nodes = financial accounts
  - Edges = money transfers
  - Edge weights = transaction amounts
- All duplicate transactions between the same accounts are preserved as distinct edges
- Account metrics are calculated by analyzing the graph structure:
  - Incoming/outgoing transaction counts
  - Total incoming/outgoing transaction volumes
  - Retention rate (percent of received money kept in the account)
- **No external graph libraries**: The project implements its own graph data structure instead of using petgraph or other dependencies

## C. Code Structure

### Modules

#### `main.rs`
Purpose: Program entry point that orchestrates the data loading, analysis, and result presentation (for high-level logic).
Reason: Provides a clean interface for executing the full analysis pipeline drawing from the other 3 modules. I'm treating it like an orchestrator. 

#### `graph.rs`
Purpose: Implements a custom directed graph data structure and transaction graph representation.
Reason: Contains all graph-related operations and enables graph analysis without external dependencies.
#### `analysis.rs`
Purpose: Implements fraud detection algorithms to identify suspicious accounts.
Reason: Separates analysis logic from data structure implementation.

#### `utilities.rs`
Purpose: Provides helper functions for file I/O, timing, and error handling, and data loading operations.
Reason: Abstracts common utility functions for better code organization and reusability.

### Key Functions & Types

#### `DiGraph<N, W>` (in graph.rs)
Purpose: Custom directed graph implementation supporting nodes of type N and edge weights of type W.
Inputs/Outputs: Generic implementation that provides graph operations.

Core logic: The DiGraph<N, W> struct represents a directed graph where:
N: Node data type (e.g., account info, user name).
W: Edge weight type (e.g., transaction amount, time delay).

It uses:

HashMap<NodeIndex, N> for storing nodes.
HashMap<(NodeIndex, NodeIndex), W> for edges.

HashMap<NodeIndex, HashSet<NodeIndex>> for both outgoing and incoming edges, to enable efficient traversal.

#### `TransactionGraph` (in graph.rs)
Purpose: Specialized directed graph for financial transaction data.
Inputs/Outputs: Consumes Transaction objects and builds a graph representation.
Core logic: Maps accounts to nodes, creates weighted edges for money transfers, and maintains a transaction history.

#### `FraudAnalysis` (in analysis.rs)
Purpose: Analyzes transaction patterns to identify suspicious accounts.
Inputs/Outputs: Takes a graph structure, returns a hashmap of all account metrics.
Core logic: Uses rule-based filtering to find accounts matching collector and money mule patterns.

#### `calculate_account_metrics()` (in graph.rs)
Purpose: Computes statistical metrics for all accounts based on their transaction behavior.
Inputs/Outputs: Takes a graph, returns account metrics for all nodes.
Core logic: Traverses the graph to calculate incoming/outgoing transaction counts, volumes, and retention rates.

#### `read_transaction_dataset()` (in utilities.rs)
Purpose: Loads and parses transaction data from CSV files.
Inputs/Outputs: Takes a file path, returns a vector of Transaction objects.
Core logic: Manual CSV parsing with type conversion and error handling.

### Main Workflow

1. Load transaction data from CSV file
2. Build a directed graph from transactions
   - Each account becomes a node
   - Each transaction becomes a weighted edge
3. Calculate account metrics across the network
4. Apply rule-based filtering to detect:
   - Collector accounts (high retention rate)
   - Money mule accounts (high throughput rate)
5. Output results to console with formatted tables

## D. Tests

### Test Output
```
running 3 tests
test test_basic_graph_construction ... ok
test test_collector_detection ... ok
test test_money_mule_detection ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Test Coverage

#### `test_basic_graph_construction`
Checks that the transaction graph correctly creates and stores nodes and edges.
Importance: Validates core graph data structure functionality.

#### `test_collector_detection`
Verifies that collector accounts (with high incoming/low outgoing transactions) are correctly identified.
Importance: Ensures the primary fraud detection algorithm works for collector patterns.

#### `test_money_mule_detection`
Confirms that money mule accounts (with balanced high-volume throughput) are correctly identified.
Importance: Validates the secondary fraud detection algorithm works for mule patterns.

## E. Results
The program successfully identifies two types of suspicious accounts using rule-based filtering.
- Note: You can change the number of acounts you want displayed by changing the values in analysis.rs

### Collector Accounts
- 168,170 accounts identified as potential collector accounts
- These accounts receive funds from multiple sources but rarely send money out
- Average retention rate of 99.99% (virtually all money received is kept)
- Top collectors accumulate billions in funds
- Detection uses rule-based filtering: high incoming volume with low outgoing volume and high retention rate

### Money Mule Accounts
- 196 accounts identified as potential money mules
- These accounts show rapid forwarding behavior
- Closely balanced input/output ratio with high transaction velocity
- Often serve as intermediaries in transaction chains
- Detection uses rule-based filtering: matching incoming/outgoing volumes and significant transaction volume

### Program Output Sample
```
=== Total of 168170 accounts detected as fraudulent collector accounts ===
Account         In Count     Out Count    In Volume       Out Volume      Retention
C439737079      13           0            356810055.18    0.00            1.00
C707403537      15           0            299153669.70    0.00            1.00
C167875008      21           0            273927022.99    0.00            1.00
...

=== Total of 196 accounts detected as fraudulent money mule accounts ===
Account         In Count     Out Count    In Volume       Out Volume      Retention
C1625226992     7            1            2944129.66      7144323.89      -1.43
C658346861      1            1            15345.96        4366622.92      -283.55
C1797311698     6            1            892365.02       3438912.34      -2.85
...
```

### Interpretation
The results show a large number of potential collector accounts (likely destinations of laundered money) and a smaller number of money mules (the accounts that move money through the system). This matches expected patterns in money laundering operations, where funds are typically funneled through a few mule accounts to many collector accounts.

## F. Usage Instructions

### Building and Running
```
# Clone the repository
git clone https://github.com/lyunljl/DS210-Final-Project.git

# Download the Data
Synthetic Financial Datasets For Fraud Detection from Kaggle: https://www.kaggle.com/datasets/ealaxi/paysim1/data 

# Confirm Project Structure
move the downloaded datset to the data folder

# Run automated data cleaning
run the cleaning-data.ipynb script to prepare dataset for analysis

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Runtime Expectations
- Program takes approximately 7 seconds to analyze the cleaned dataset (3.83s for loading, 3.83s for analysis)
- Memory usage peaks at around 800MB during graph construction and analysis

## G. AI-Assistance 
- ChatGPT was used for:
  - Suggesting the overall project structure
  - Various debugging/syntax errors help
  - Assistance with documentation formatting
  - Other substanstive AI-support: https://chatgpt.com/share/681545c2-0618-8010-a330-d99dcaff50bf 
  - Used AI to initially help me brainstorm: https://chatgpt.com/share/6815466c-1768-8010-bd14-c07fcb2f85ec 