use alloy_network::EthereumWallet;
use alloy_primitives::{Address, Bytes, U256};
use alloy_provider::ProviderBuilder;
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::sol;

sol! {
    #[sol(rpc)]
    interface IRollupCore {
        function submitBatch(bytes calldata proofBytes, bytes calldata publicValues, uint256 numOperations) external;
        function getStateRoot() external view returns (bytes32);
        function getBatchNumber() external view returns (uint256);
    }
}

/// Submit a batch proof to the RollupCore contract on Creditcoin L1.
///
/// Reads CREDITCOIN_RPC, PRIVATE_KEY, and ROLLUP_ADDRESS from environment.
/// Sends a real transaction — no mocks.
pub async fn submit_to_l1(
    proof_bytes: &[u8],
    public_values: &[u8],
    _old_root: [u8; 32],
    _new_root: [u8; 32],
    num_ops: usize,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let rpc_url = std::env::var("CREDITCOIN_RPC")
        .map_err(|_| "CREDITCOIN_RPC env var not set")?;
    let private_key = std::env::var("PRIVATE_KEY")
        .map_err(|_| "PRIVATE_KEY env var not set")?;
    let rollup_address = std::env::var("ROLLUP_ADDRESS")
        .map_err(|_| "ROLLUP_ADDRESS env var not set")?;

    let rollup_addr: Address = rollup_address.parse()
        .map_err(|e| format!("Invalid ROLLUP_ADDRESS: {}", e))?;

    let signer: PrivateKeySigner = private_key.parse()
        .map_err(|e| format!("Invalid PRIVATE_KEY: {}", e))?;

    let wallet = EthereumWallet::from(signer);

    let rpc_url_parsed: url::Url = rpc_url.parse()
        .map_err(|e| format!("Invalid CREDITCOIN_RPC URL: {}", e))?;

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_http(rpc_url_parsed);

    let contract = IRollupCore::new(rollup_addr, &provider);

    tracing::info!(
        "Submitting batch to L1: {} ops, proof={} bytes, publicValues={} bytes",
        num_ops,
        proof_bytes.len(),
        public_values.len()
    );

    // Query current on-chain state for logging
    match contract.getStateRoot().call().await {
        Ok(root_ret) => {
            tracing::info!("On-chain state root: 0x{}", hex::encode(root_ret.0.as_slice()));
        }
        Err(e) => {
            tracing::warn!("Could not read on-chain state root: {}", e);
        }
    }
    match contract.getBatchNumber().call().await {
        Ok(batch_ret) => {
            tracing::info!("On-chain batch number: {}", batch_ret);
        }
        Err(e) => {
            tracing::warn!("Could not read on-chain batch number: {}", e);
        }
    }

    // Send the real transaction
    let pending_tx = contract
        .submitBatch(
            Bytes::copy_from_slice(proof_bytes),
            Bytes::copy_from_slice(public_values),
            U256::from(num_ops),
        )
        .send()
        .await
        .map_err(|e| format!("Failed to send submitBatch tx: {}", e))?;

    let tx_hash = format!("0x{}", hex::encode(pending_tx.tx_hash().as_slice()));
    tracing::info!("Transaction sent: {}", tx_hash);

    // Wait for confirmation
    let receipt = pending_tx
        .get_receipt()
        .await
        .map_err(|e| format!("Failed to get tx receipt: {}", e))?;

    tracing::info!(
        "Transaction confirmed in block {}. Gas used: {}",
        receipt.block_number.unwrap_or(0),
        receipt.gas_used
    );

    if !receipt.status() {
        return Err(format!("Transaction reverted: {}", tx_hash).into());
    }

    Ok(tx_hash)
}
