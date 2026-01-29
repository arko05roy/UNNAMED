use zkcredit_program::{apply_batch, CreditOp, CreditState};

use crate::prover::generate_proof;
use crate::submitter::submit_to_l1;
use crate::BatchStatusResponse;

pub struct BatchManager {
    pub(crate) pending_ops: Vec<CreditOp>,
    pub(crate) current_state: CreditState,
    pub(crate) batch_capacity: usize,
    pub(crate) last_batch_number: u64,
}

impl BatchManager {
    pub fn new() -> Self {
        Self {
            pending_ops: Vec::new(),
            current_state: CreditState::new(),
            batch_capacity: 100,
            last_batch_number: 0,
        }
    }

    pub fn add_operation(&mut self, op: CreditOp) -> (usize, usize) {
        self.pending_ops.push(op);
        let position = self.pending_ops.len();
        (position, self.batch_capacity)
    }

    pub fn get_status(&self) -> BatchStatusResponse {
        BatchStatusResponse {
            pending_ops: self.pending_ops.len(),
            batch_capacity: self.batch_capacity,
            last_batch_number: self.last_batch_number,
            state_root: hex::encode(self.current_state.compute_root()),
        }
    }

    pub async fn force_prove_and_submit(
        &mut self,
    ) -> Result<(String, [u8; 32]), Box<dyn std::error::Error + Send + Sync>> {
        if self.pending_ops.is_empty() {
            return Err("No pending operations".into());
        }

        let ops = std::mem::take(&mut self.pending_ops);
        let num_ops = ops.len();
        let old_root = self.current_state.compute_root();

        tracing::info!("Processing batch of {} operations", num_ops);

        // Generate proof
        tracing::info!("Generating ZK proof...");
        let (proof_bytes, public_values) = generate_proof(&self.current_state, &ops)?;

        // Apply operations to local state
        let new_root = apply_batch(&mut self.current_state, ops)
            .map_err(|e| format!("Failed to apply batch: {:?}", e))?;

        tracing::info!(
            "Batch applied. New state root: 0x{}",
            hex::encode(new_root)
        );

        // Submit to L1
        tracing::info!("Submitting to L1...");
        let tx_hash =
            submit_to_l1(&proof_bytes, &public_values, old_root, new_root, num_ops).await?;

        self.last_batch_number += 1;
        tracing::info!(
            "Batch #{} submitted. TX: {}",
            self.last_batch_number,
            tx_hash
        );

        Ok((tx_hash, new_root))
    }
}
