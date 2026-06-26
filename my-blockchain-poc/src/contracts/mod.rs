use std::collections::HashMap;

// Mock context mirroring low-level Wasm host functions (Env bindings)
pub struct Context {
    pub caller: String,
    pub storage_nonce: u64,
}

pub struct TokenContract {
    // Emulating high-level Storage macro expansion to key-value maps
    state_balances: HashMap<String, u64>,
    ctx: Context,
}

impl TokenContract {
    pub fn new(deployer: String) -> Self {
        Self {
            state_balances: HashMap::new(),
            ctx: Context { caller: deployer, storage_nonce: 0 },
        }
    }

    pub fn mint(&mut self, recipient: String, amount: u64) {
        let bal = self.state_balances.entry(recipient).or_insert(0);
        *bal = bal.checked_add(amount).expect("Storage overflow protection");
        self.ctx.storage_nonce += 1;
    }

    pub fn transfer(&mut self, sender: String, recipient: String, amount: u64) {
        let sender_bal = self.state_balances.get(&sender).copied().unwrap_or(0);
        if sender_bal >= amount {
            self.state_balances.insert(sender, sender_bal - amount);
            let rec_bal = self.state_balances.get(&recipient).copied().unwrap_or(0);
            self.state_balances.insert(recipient, rec_bal + amount);
        }
        self.ctx.storage_nonce += 1;
    }

    pub fn balance_of(&self, account: &str) -> u64 {
        self.state_balances.get(account).copied().unwrap_or(0)
    }
}
