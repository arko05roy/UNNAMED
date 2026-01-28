#![no_main]
sp1_zkvm::entrypoint!(main);

use zkcredit_program::{apply_batch, CreditOp, CreditState};

pub fn main() {
    // Read inputs from host
    let old_state_root: [u8; 32] = sp1_zkvm::io::read();
    let ops: Vec<CreditOp> = sp1_zkvm::io::read();
    let mut state: CreditState = sp1_zkvm::io::read();

    // Verify old state matches expected root
    let computed_old_root = state.compute_root();
    assert_eq!(
        computed_old_root, old_state_root,
        "State root mismatch: provided root does not match computed root"
    );

    // Apply batch of operations
    let new_state_root = apply_batch(&mut state, ops).expect("Failed to apply batch");

    // Commit public outputs (old root + new root)
    sp1_zkvm::io::commit(&old_state_root);
    sp1_zkvm::io::commit(&new_state_root);
}
