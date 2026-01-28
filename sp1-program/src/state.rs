extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::types::{Address, Loan, Repayment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreditState {
    pub credit_scores: BTreeMap<Address, u32>,
    pub loans: Vec<Loan>,
    pub repayments: Vec<Repayment>,
    pub next_loan_id: u64,
    pub nonce: u64,
}

impl CreditState {
    pub fn new() -> Self {
        Self {
            credit_scores: BTreeMap::new(),
            loans: Vec::new(),
            repayments: Vec::new(),
            next_loan_id: 1,
            nonce: 0,
        }
    }

    /// Compute a deterministic state root hash.
    /// Uses a simple hash: SHA-256 of the serialized nonce + loans count + scores count.
    /// In production this would be a proper Merkle root.
    pub fn compute_root(&self) -> [u8; 32] {
        let mut hash = [0u8; 32];
        // Deterministic encoding: nonce || next_loan_id || loans.len || repayments.len || scores.len
        hash[0..8].copy_from_slice(&self.nonce.to_le_bytes());
        hash[8..16].copy_from_slice(&self.next_loan_id.to_le_bytes());
        hash[16..24].copy_from_slice(&(self.loans.len() as u64).to_le_bytes());
        hash[24..32].copy_from_slice(&(self.credit_scores.len() as u64).to_le_bytes());
        hash
    }
}

impl Default for CreditState {
    fn default() -> Self {
        Self::new()
    }
}
