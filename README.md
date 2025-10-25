#  Visitor Counter

##  Use Case
The **Visitor Counter** program is a simple Solana application that counts how many times a user or visitor interacts with a website. 
Each visit triggers a transaction that increments a counter stored on-chain.  
It demonstrates a **basic Solana state management use case** updating and persisting a simple number on the blockchain.

**Example scenario:**
- A website can display the number of total visits powered by Solana transactions.
- Each time someone visits, the backend increments the on-chain counter.


##  Architecture Diagram


      ┌────────────────────────┐
      │          Website       │
      └───────────┬────────────┘
                  │
                  ▼
      ┌────────────────────────┐
      │  Anchor Client         │
      │ Calls increment() RPC  │
      └───────────┬────────────┘
                  │
                  ▼
      ┌────────────────────────┐
      │ Solana Program (Rust)  │
      │ - Stores visitor_count │
      │ - Increments counter   │
      └───────────┬────────────┘
                  │
                  ▼
      ┌────────────────────────┐
      │ On-chain Counter State │
      │ visitor_count: u64     │
      └────────────────────────┘

##  How to Run the Code

### Prerequisites
Make sure you have:
- Rust & Cargo installed  
- Solana CLI installed  
- Anchor CLI installed  


## Build the Program
```
anchor build
```

## Run the Tests
```
anchor test
```

