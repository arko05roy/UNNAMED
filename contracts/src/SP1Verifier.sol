// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// Import the SP1 Groth16 Verifier for v4.0.0-rc.3 (matches SP1 SDK 4.2.1)
// This verifier is used for on-chain verification of SP1 proofs
import {SP1Verifier as SP1VerifierImpl} from "@sp1-contracts/src/v4.0.0-rc.3/SP1VerifierGroth16.sol";

/// @title SP1Verifier
/// @notice Wrapper contract for the SP1 Groth16 verifier v4.0.0-rc.3
/// @dev This contract re-exports the SP1 verifier for deployment on Creditcoin
contract SP1Verifier is SP1VerifierImpl {
    /// @notice Returns the zkCredit program verification key
    /// @dev This key is generated when building the SP1 program
    /// @return The bytes32 verification key for the zkCredit program
    function ZKCREDIT_PROGRAM_VKEY() external pure returns (bytes32) {
        return 0x00d8368ebc6b3182ab36aa155e295897798a2b997db6c3bcb12a8387b571c476;
    }
}
