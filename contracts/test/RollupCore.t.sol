// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console2} from "forge-std/Test.sol";
import {RollupCore} from "../src/RollupCore.sol";
import {SP1MockVerifier} from "@sp1-contracts/src/SP1MockVerifier.sol";

contract RollupCoreTest is Test {
    RollupCore public rollup;
    SP1MockVerifier public verifier;
    bytes32 public initialRoot = keccak256("initial");
    bytes32 public programVKey = keccak256("zkcredit-program-vkey");

    function setUp() public {
        verifier = new SP1MockVerifier();
        rollup = new RollupCore(address(verifier), programVKey, initialRoot);
    }

    function test_InitialState() public view {
        assertEq(rollup.stateRoot(), initialRoot);
        assertEq(rollup.batchNumber(), 0);
        assertEq(address(rollup.verifier()), address(verifier));
    }

    function test_SubmitBatch() public {
        bytes32 newRoot = keccak256("new-state");
        bytes memory publicValues = abi.encodePacked(initialRoot, newRoot);

        // Mock verifier accepts empty proof bytes
        rollup.submitBatch("", publicValues, 5);

        assertEq(rollup.stateRoot(), newRoot);
        assertEq(rollup.batchNumber(), 1);
    }

    function test_SubmitMultipleBatches() public {
        bytes32 root1 = keccak256("state-1");
        bytes32 root2 = keccak256("state-2");

        // Batch 1
        bytes memory pv1 = abi.encodePacked(initialRoot, root1);
        rollup.submitBatch("", pv1, 3);
        assertEq(rollup.batchNumber(), 1);

        // Batch 2
        bytes memory pv2 = abi.encodePacked(root1, root2);
        rollup.submitBatch("", pv2, 7);
        assertEq(rollup.batchNumber(), 2);
        assertEq(rollup.stateRoot(), root2);
    }

    function test_InvalidOldRootReverts() public {
        bytes32 wrongRoot = keccak256("wrong");
        bytes32 newRoot = keccak256("new");
        bytes memory publicValues = abi.encodePacked(wrongRoot, newRoot);

        vm.expectRevert(RollupCore.InvalidOldRoot.selector);
        rollup.submitBatch("", publicValues, 1);
    }

    function test_NoBatchDataReverts() public {
        bytes memory publicValues = abi.encodePacked(initialRoot, keccak256("new"));

        vm.expectRevert(RollupCore.NoBatchData.selector);
        rollup.submitBatch("", publicValues, 0);
    }

    function test_InvalidPublicValuesLength() public {
        bytes memory shortPv = abi.encodePacked(initialRoot);

        vm.expectRevert("Invalid public values length");
        rollup.submitBatch("", shortPv, 1);
    }

    event BatchSubmitted(
        uint256 indexed batchNumber,
        bytes32 oldRoot,
        bytes32 newRoot,
        uint256 numOperations
    );
    event StateRootUpdated(bytes32 newRoot);

    function test_EmitsEvents() public {
        bytes32 newRoot = keccak256("new-state");
        bytes memory publicValues = abi.encodePacked(initialRoot, newRoot);

        vm.expectEmit(true, false, false, true);
        emit BatchSubmitted(1, initialRoot, newRoot, 5);

        vm.expectEmit(false, false, false, true);
        emit StateRootUpdated(newRoot);

        rollup.submitBatch("", publicValues, 5);
    }
}
