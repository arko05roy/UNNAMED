use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use zkcredit_program::{CreditOp, CreditState};

const ELF: &[u8] = include_elf!("zkcredit-program");

/// Generate a ZK proof for a batch of credit operations.
/// Returns (proof_bytes, public_values) for on-chain submission.
pub fn generate_proof(
    state: &CreditState,
    ops: &[CreditOp],
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error + Send + Sync>> {
    let old_root = state.compute_root();

    let mut stdin = SP1Stdin::new();
    stdin.write(&old_root);
    stdin.write(&ops.to_vec());
    stdin.write(state);

    let client = ProverClient::from_env();
    let (pk, _vk) = client.setup(ELF);

    let proof = client
        .prove(&pk, &stdin)
        .run()
        .map_err(|e| format!("Proof generation failed: {}", e))?;

    let public_values = proof.public_values.to_vec();
    let proof_bytes = proof.bytes();

    Ok((proof_bytes, public_values))
}

/// Execute the program without generating a proof (for testing).
pub fn execute_only(
    state: &CreditState,
    ops: &[CreditOp],
) -> Result<([u8; 32], [u8; 32]), Box<dyn std::error::Error + Send + Sync>> {
    let old_root = state.compute_root();

    let mut stdin = SP1Stdin::new();
    stdin.write(&old_root);
    stdin.write(&ops.to_vec());
    stdin.write(state);

    let client = ProverClient::from_env();

    let (output, report) = client
        .execute(ELF, &stdin)
        .run()
        .map_err(|e| format!("Execution failed: {}", e))?;

    tracing::info!(
        "Program executed in {} cycles",
        report.total_instruction_count()
    );

    let pv = output.as_slice();
    if pv.len() != 64 {
        return Err(format!("Unexpected public values length: {}", pv.len()).into());
    }

    let mut old_root_out = [0u8; 32];
    let mut new_root_out = [0u8; 32];
    old_root_out.copy_from_slice(&pv[..32]);
    new_root_out.copy_from_slice(&pv[32..]);

    Ok((old_root_out, new_root_out))
}
