// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/src/ISP1Verifier.sol";

contract RollupCore {
    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;
    bytes32 public stateRoot;
    uint256 public batchNumber;

    event BatchSubmitted(
        uint256 indexed batchNumber,
        bytes32 oldRoot,
        bytes32 newRoot,
        uint256 numOperations
    );

    event StateRootUpdated(bytes32 newRoot);

    error InvalidOldRoot();
    error ProofVerificationFailed();
    error NoBatchData();

    constructor(address _verifier, bytes32 _programVKey, bytes32 _initialRoot) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
        stateRoot = _initialRoot;
    }

    /// @notice Submit a batch of credit operations with a ZK proof
    /// @param proofBytes The SP1 proof bytes
    /// @param publicValues ABI-encoded (oldRoot, newRoot) committed by the SP1 program
    /// @param numOperations Number of operations in this batch (for event logging)
    function submitBatch(
        bytes calldata proofBytes,
        bytes calldata publicValues,
        uint256 numOperations
    ) external {
        if (numOperations == 0) {
            revert NoBatchData();
        }

        // Decode public values: old_state_root (32 bytes) + new_state_root (32 bytes)
        require(publicValues.length == 64, "Invalid public values length");
        bytes32 oldRoot = bytes32(publicValues[0:32]);
        bytes32 newRoot = bytes32(publicValues[32:64]);

        // Verify old root matches current state
        if (oldRoot != stateRoot) {
            revert InvalidOldRoot();
        }

        // Verify SP1 proof on-chain
        verifier.verifyProof(programVKey, publicValues, proofBytes);

        // Update state
        stateRoot = newRoot;
        batchNumber++;

        emit BatchSubmitted(batchNumber, oldRoot, newRoot, numOperations);
        emit StateRootUpdated(newRoot);
    }

    function getStateRoot() external view returns (bytes32) {
        return stateRoot;
    }

    function getBatchNumber() external view returns (uint256) {
        return batchNumber;
    }
}
