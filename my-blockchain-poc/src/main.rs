mod core;
mod contracts;
mod crypto;

use core::dag::{DagEngine, Transaction};
use contracts::TokenContract;
use crypto::aa_zk::{AccountAbstraction, ZkBridge};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing Next-Gen PoC Node...");

    // 1. Initialize Core DAG Engine
    let dag_engine = Arc::new(DagEngine::new());
    let engine_clone = dag_engine.clone();

    let _worker_handle = tokio::spawn(async move {
        engine_clone.run_pipeline().await;
    });

    // 2. Simulate Native AA and Social Recovery
    let mut account = AccountAbstraction::new(
        "alice.eth".to_string(),
        vec!["guardian1".to_string(), "guardian2".to_string()],
    );
    
    // ZK proof mock validation (threshold 2-of-2)
    let fake_zk_proof = vec![0xde, 0xad, 0xbe, 0xef];
    let recovered = account.verify_social_recovery(&fake_zk_proof, 2);
    println!("Social recovery status: {:?}", recovered);

    // 3. Execute Gas Sponsorship (Meta-Transaction)
    let tx = Transaction {
        id: [1; 32],
        parents: vec![],
        payload: b"transfer(bob, 100)".to_vec(),
    };
    let sponsor_sig = vec![0xaa; 64];
    let is_sponsored = account.verify_gas_sponsorship(&tx, &sponsor_sig);
    println!("Gas sponsorship valid: {}", is_sponsored);

    // 4. Trigger Wasm-flavor State Execution
    let mut contract = TokenContract::new("Alice".to_string());
    contract.mint("Alice".to_string(), 1000);
    contract.transfer("Alice".to_string(), "Bob".to_string(), 300);
    
    println!("Alice Balance: {}", contract.balance_of("Alice"));
    println!("Bob Balance: {}", contract.balance_of("Bob"));

    // 5. Run ZK Light Client State Transition Verification
    let l1_state_root = [0x55; 32];
    let bridge_proof = vec![0x11, 0x22, 0x33];
    let is_valid_cross_chain = ZkBridge::verify_l1_to_l2_state(l1_state_root, bridge_proof);
    println!("ZK Cross-Chain State verified: {}", is_valid_cross_chain);

    dag_engine.submit_transaction(tx).await;
    
    Ok(())
}
