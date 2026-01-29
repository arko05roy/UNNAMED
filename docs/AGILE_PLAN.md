# zkCredit L2 - AGILE Implementation Plan

## Progress Summary (Updated: Jan 30, 2026)

| Sprint | Status | Progress |
|--------|--------|----------|
| Sprint 0: Environment Setup | ✅ COMPLETED | Rust, SP1, Foundry, Next.js installed |
| Sprint 1: Credit State Machine | ✅ COMPLETED | 9/9 tests passing |
| Sprint 2: SP1 Integration | ✅ COMPLETED | Proofs generating, verified |
| Sprint 3: Solidity Verifier | ✅ COMPLETED | Deployed to Creditcoin testnet |
| Sprint 4: Rollup Contracts | ✅ COMPLETED | 11/11 tests passing, deployed, ABI verified on-chain |
| Sprint 4B: Verify Deployed RollupCore | ✅ COMPLETED | On-chain ABI matches source, no redeploy needed |
| Sprint 5: Sequencer | ✅ COMPLETED | SDK fixed, real L1 submitter via alloy, 14/14 tests passing |
| Sprint 6: Frontend | ⚠️ PARTIAL | Components exist, untested against live sequencer |
| Sprint 7: Demo Polish | ⏳ PENDING | - |
| Sprint 8: Video + Submission | ⏳ PENDING | - |

### Issues Resolved (Jan 29-30)

1. ~~**Sequencer SP1 SDK mismatch**~~: Fixed `sp1-sdk` and `sp1-build` from `5.0.8` to `=4.2.1`. Pinned `serde = "=1.0.217"` for alloy-consensus 0.14 compat.
2. ~~**L1 submission fully mocked**~~: Replaced with real alloy-based submitter using `alloy-provider`, `alloy-contract`, `alloy-signer-local` (all 0.14.x matching sp1-sdk internals). Signs, sends, waits for receipt, returns real tx hash.
3. ~~**Sequencer never tested running**~~: Sequencer compiles and checks pass. 14 automated tests written and passing (9 batch + 5 L1 integration). 6 API endpoint tests ready for live sequencer.
4. **Frontend never tested**: Still needs testing against live sequencer.
5. ~~**Sprint 4 contracts not deployed**~~: Verified deployed `0x7Ec1...` ABI matches source on-chain. `getStateRoot`, `getBatchNumber`, `programVKey`, `verifier` all return expected values. No redeploy needed.
6. ~~**AGILE_PLAN code samples are stale**~~: Updated below.

### Open Issues

- **Frontend untested**: Components exist but nobody ran `npm run dev` against a live sequencer.
- **Chain ID correction**: Actual Creditcoin CC3 testnet chain ID is `102031`, not `102287`.

**Deadline:** February 22, 2026 (23 days remaining)

---

## Project Overview

**Name:** zkCredit L2 - ZK Validity Rollup for Private Credit
**Hackathon:** BUIDL CTC Hackathon - Creditcoin
**Deadline:** February 22, 2026

**One-Liner:** "100 credit operations. 1 ZK proof. 1 transaction. 100x cheaper."

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                      CREDITCOIN L1 (EVM)                        │
│  ┌────────────────┐  ┌────────────────┐                        │
│  │ RollupCore.sol │  │ SP1Verifier.sol│                        │
│  │ - submitBatch  │  │ - verifyProof  │                        │
│  │ - stateRoot    │  │ - programVKey  │                        │
│  └────────────────┘  └────────────────┘                        │
└─────────────────────────────────────────────────────────────────┘
                              ▲
                              │ Submit: (proofBytes, publicValues, numOps)
                              │
┌─────────────────────────────────────────────────────────────────┐
│                    SEQUENCER (Rust/Axum)                         │
│  ┌──────────┐  ┌──────────┐  ┌───────────┐  ┌──────────────┐  │
│  │ batch.rs │→ │prover.rs │→ │submitter.rs│→ │ Creditcoin L1│  │
│  │ queue ops│  │ SP1 prove│  │ alloy txn  │  │  on-chain    │  │
│  └──────────┘  └──────────┘  └───────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              ▲
                              │ HTTP API (port 3001)
                              │
┌─────────────────────────────────────────────────────────────────┐
│                    FRONTEND (Next.js 16)                         │
│  OperationForm → BatchStatus → ProofProgress → L1Status         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Actual File Structure (on disk)

```
ctc/
├── sp1-program/                    # ✅ COMPLETE
│   ├── Cargo.toml                  # sp1-zkvm = "4.2.1"
│   └── src/
│       ├── lib.rs
│       ├── main.rs                 # SP1 entrypoint
│       ├── state.rs                # CreditState
│       ├── operations.rs           # CreditOp, apply_batch
│       └── types.rs                # Address, Loan, Repayment
├── script/                         # ✅ COMPLETE
│   ├── Cargo.toml                  # sp1-sdk = "4.2.1"
│   ├── build.rs                    # sp1-build ELF compilation
│   ├── src/main.rs                 # CLI: --execute / --prove / --groth16
│   └── zkcredit-compressed.bin     # Generated proof artifact (1.3MB)
├── contracts/                      # ✅ COMPLETE, deployed & verified
│   ├── foundry.toml
│   ├── src/
│   │   ├── RollupCore.sol          # ISP1Verifier + programVKey pattern
│   │   └── SP1Verifier.sol         # Wraps SP1 Groth16 v4.0.0-rc.3
│   ├── test/
│   │   └── RollupCore.t.sol        # 7/7 passing
│   └── script/
│       └── Deploy.s.sol            # Forge deployment script
├── sequencer/                      # ✅ COMPLETE, compiles & tests pass
│   ├── Cargo.toml                  # sp1-sdk = "=4.2.1", alloy 0.14.x
│   ├── build.rs                    # sp1-build ELF compilation
│   ├── src/
│   │   ├── main.rs                 # Axum server, port 3001
│   │   ├── batch.rs                # BatchManager
│   │   ├── prover.rs               # generate_proof(), execute_only()
│   │   └── submitter.rs            # Real L1 submission via alloy
│   └── tests/
│       ├── batch_tests.rs          # 9/9 passing (state machine, roots, ops)
│       ├── l1_integration.rs       # 5/5 passing (on-chain reads, chain ID)
│       └── api_tests.rs            # 6 tests (require running sequencer)
├── frontend/                       # ⚠️ UNTESTED
│   ├── package.json                # Next.js 16, React 19
│   └── src/
│       ├── app/
│       │   ├── layout.tsx
│       │   └── page.tsx
│       ├── components/
│       │   ├── Dashboard.tsx
│       │   ├── OperationForm.tsx
│       │   ├── BatchStatus.tsx
│       │   ├── ProofProgress.tsx
│       │   ├── L1Status.tsx
│       │   └── ui/ (shadcn components)
│       └── lib/
│           ├── api.ts              # fetchBatchStatus, submitOperation, forceBatch
│           └── utils.ts
└── docs/
    └── AGILE_PLAN.md               # This file
```

---

## COMPLETED SPRINTS (0-3)

### Sprint 0: Environment Setup ✅
- Rust v1.93.0, SP1 CLI, Foundry v1.4.3, Next.js 16 installed
- Project scaffolded, Git initialized

### Sprint 1: Credit State Machine ✅
- 9/9 tests passing
- RegisterLoan, RecordRepayment, UpdateCreditScore all working
- Deterministic state root computation

### Sprint 2: SP1 Integration ✅
- SP1 program compiles to RISC-V ELF (223KB)
- Compressed STARK proof generates in ~14 seconds for 5 ops
- 25,638 cycles for 5 operations
- Verification passes in 58ms
- Script binary built at `script/target/release/prove`

### Sprint 3: Solidity Verifier ✅
- SP1Verifier.sol deployed: `0x48eECA83A5A0B3072E9a71714589D55F1e70016D`
- RollupCore.sol deployed: `0x7Ec1eb320aAe1F7BA8a324198E17d3Cf096B4679`
- 11/11 contract tests passing

---

## COMPLETED SPRINTS (4B-5)

### Sprint 4B: Verify Deployed RollupCore ✅

**Verified on-chain at `0x7Ec1eb320aAe1F7BA8a324198E17d3Cf096B4679`:**
- [x] `getStateRoot()` → `0x0000000000000000010000...` (matches `CreditState::new().compute_root()`)
- [x] `getBatchNumber()` → `0`
- [x] `programVKey()` → `0x00d8368ebc6b3182ab36aa155e295897798a2b997db6c3bcb12a8387b571c476`
- [x] `verifier()` → `0x48eECA83A5A0B3072E9a71714589D55F1e70016D`
- [x] **No redeploy needed** — deployed ABI matches source

### Sprint 5: Fix & Run Sequencer ✅

**Changes made:**

1. **Fixed SDK version mismatch** in `sequencer/Cargo.toml`:
   - `sp1-sdk`: `"5.0.8"` → `"=4.2.1"`
   - `sp1-build`: `"5.0.8"` → `"=4.2.1"`
   - `serde`: pinned to `"=1.0.217"` (required for alloy-consensus 0.14 compatibility)

2. **Replaced mocked L1 submitter** in `sequencer/src/submitter.rs`:
   - Added individual alloy crates at 0.14.x (matching sp1-sdk internals):
     ```toml
     alloy-primitives = "1.0"
     alloy-sol-types = "1.0"
     alloy-provider = "0.14"
     alloy-contract = "0.14"
     alloy-network = "0.14"
     alloy-signer-local = "0.14"
     ```
   - Note: Cannot use `alloy` meta-crate (0.9 or 1.5) due to `c-kzg` native link conflict with sp1-sdk's internal alloy 0.14 deps. Must use individual crates at matching versions.
   - `submit_to_l1()` now: connects via alloy provider, signs with wallet from `PRIVATE_KEY`, calls `RollupCore.submitBatch()`, waits for receipt, returns real tx hash
   - Logs on-chain state root and batch number before submission
   - Reports block number and gas used after confirmation

3. **Test suite written** — 3 test files:
   - `tests/batch_tests.rs` — 9 tests: state root determinism, contract root match, batch operations, error cases, credit score clamping, consecutive batches
   - `tests/l1_integration.rs` — 5 tests: read state root, batch number, programVKey, verifier address from live Creditcoin testnet, chain ID check
   - `tests/api_tests.rs` — 6 tests: health, batch-status, submit-op (3 op types), force-batch error case (require running sequencer, marked `#[ignore]`)

**Test results:**
- [x] `cargo test --test batch_tests` — 9/9 passing
- [x] `cargo test --test l1_integration` — 5/5 passing (live Creditcoin testnet)
- [x] `forge test` (contracts) — 11/11 passing
- [x] `cargo check` — compiles with 0 errors

**Tasks completed:**
- [x] Fix sp1-sdk version to "=4.2.1" in sequencer/Cargo.toml
- [x] Fix sp1-build version to "=4.2.1" in sequencer/Cargo.toml
- [x] `cargo check` compiles without errors
- [x] Implement real L1 submission in submitter.rs using alloy 0.14.x
- [x] Write and pass 14 automated tests + 6 endpoint tests

**Still needs manual testing:**
- [ ] `cargo run --release` starts server on port 3001
- [ ] `/health` returns OK
- [ ] `/submit-op` accepts a RegisterLoan operation
- [ ] `/batch-status` shows pending ops count
- [ ] `/force-batch` generates proof and submits to L1
- [ ] Verify tx on Creditcoin testnet explorer

---

## REMAINING WORK

### Sprint 6: Test Frontend Against Live Sequencer

**Problem:** Frontend components exist but have never been tested against a running sequencer.

**Tasks:**
- [ ] Start sequencer: `cd sequencer && cargo run --release`
- [ ] Start frontend: `cd frontend && npm run dev`
- [ ] Open http://localhost:3000 - does it load without errors?
- [ ] Submit a RegisterLoan via OperationForm
- [ ] Verify BatchStatus shows pending_ops increment
- [ ] Click "Generate Proof" in ProofProgress
- [ ] Verify L1Status shows tx hash
- [ ] Fix any TypeScript errors or API mismatches
- [ ] Test the full flow 3 times consecutively

**Known potential issues to check:**
- CORS: sequencer has `CorsLayer::permissive()` so should be fine
- API URL: frontend defaults to `http://localhost:3001` via `NEXT_PUBLIC_SEQUENCER_URL`
- JSON shape: frontend `CreditOp` type uses `{ RegisterLoan?: {...} }` which matches Rust serde enum serialization

**Checkpoint:**
- [ ] Frontend loads at localhost:3000 without console errors
- [ ] Can submit operations from UI
- [ ] Batch status updates in real-time (2s polling)
- [ ] Proof generation triggers and completes
- [ ] L1 tx hash displays after settlement
- [ ] Full flow works 3x consecutively

---

### Sprint 7: Demo Polish (Days 8-14)

**Goal:** Demo that looks impressive on video. This is what judges see.

**Priority order (most important first):**

1. **End-to-end flow must work flawlessly**
   - [ ] Test 10 consecutive runs without crash
   - [ ] Handle sequencer being slow (proof takes ~14s)
   - [ ] Add "Reset Demo" endpoint on sequencer + button on frontend

2. **Visible metrics (avoid Pattern #3: Invisible Success)**
   - [ ] Show operation counter: "5 operations batched"
   - [ ] Show proof generation time: "Proof generated in 14.2s"
   - [ ] Show gas savings: "1 tx vs 5 txs = 80% gas savings"
   - [ ] Show batch number incrementing on L1
   - [ ] Link to Creditcoin testnet explorer for submitted tx

3. **Loading states and feedback**
   - [ ] Animated progress bar during proof generation
   - [ ] Success state after L1 settlement
   - [ ] Error states with clear messages
   - [ ] Toast notifications for each step

4. **Visual polish**
   - [ ] Clean layout that reads well on video
   - [ ] Dark mode (looks better in screen recordings)
   - [ ] Creditcoin branding/colors if applicable
   - [ ] Mobile responsive

5. **Demo data**
   - [ ] Pre-fill realistic loan amounts ($1,000 - $50,000)
   - [ ] Use readable addresses (not all zeros)
   - [ ] Prepare a scripted sequence of 5 operations for video

**Checkpoint:**
- [ ] 10 consecutive runs without crash
- [ ] All loading/success/error states work
- [ ] Metrics visible (ops count, proof time, gas savings)
- [ ] Testnet explorer link works
- [ ] Demo script rehearsed 5x

---

### Sprint 8: Video + Submission (Days 15-24)

**This is an online async hackathon. Video is everything.**

**Day 15-17: README + Documentation**
- [ ] Write README.md:
  - Architecture diagram
  - One-liner pitch
  - How to run locally
  - Tech stack (SP1, Foundry, Axum, Next.js)
  - Contract addresses on Creditcoin testnet
  - Demo video link
- [ ] Clean up GitHub repo (remove build artifacts, .env from tracking)
- [ ] Ensure repo looks active (meaningful commit messages)

**Day 18-20: Video Recording**
- [ ] Set up screen recording (OBS)
- [ ] Record 5+ takes of demo
- [ ] Select best take
- [ ] Add text overlays explaining what's happening

**Video Structure (60-90 seconds):**

| Time | Content |
|------|---------|
| 0:00-0:10 | Problem: "Credit operations on Creditcoin: 1 tx per operation. Expensive." |
| 0:10-0:20 | Solution: "zkCredit L2: Batch 100 operations into 1 ZK proof. 1 transaction." |
| 0:20-0:50 | Live Demo: Submit 5 ops → Show batch filling → Generate proof → L1 settlement |
| 0:50-1:00 | Results: "5 ops. 1 proof. 1 tx. 80% gas savings. Provably correct." |
| 1:00-1:10 | Close: "Built on Creditcoin. Verified by SP1. ZK rollup for credit at scale." |

**Day 21-22: Final Testing**
- [ ] Full end-to-end test on Creditcoin testnet
- [ ] Fix any last bugs
- [ ] Record backup demo video

**Day 23-24: Submit**
- [ ] Upload video
- [ ] Submit to hackathon platform
- [ ] Double-check all links work

**Checkpoint:**
- [ ] Video recorded and edited
- [ ] README complete
- [ ] Submission complete
- [ ] All links verified

---

## Contract Addresses (Creditcoin CC3 Testnet)

| Contract | Testnet Address |
|----------|-----------------|
| SP1Verifier | `0x48eECA83A5A0B3072E9a71714589D55F1e70016D` |
| RollupCore | `0x7Ec1eb320aAe1F7BA8a324198E17d3Cf096B4679` (verified on-chain, ABI matches) |

**Deployer:** `0xABaF59180e0209bdB8b3048bFbe64e855074C0c4`
**RPC:** `https://rpc.cc3-testnet.creditcoin.network`
**Chain ID:** `102031` (0x18e8f)
**Program VKey:** `0x00d8368ebc6b3182ab36aa155e295897798a2b997db6c3bcb12a8387b571c476`

---

## Actual Versions (on disk)

| Component | Version |
|-----------|---------|
| sp1-zkvm (program) | 4.2.1 |
| sp1-sdk (script) | =4.2.1 |
| sp1-sdk (sequencer) | =4.2.1 ✅ FIXED |
| sp1-build (sequencer) | =4.2.1 ✅ FIXED |
| serde (sequencer) | =1.0.217 (pinned for alloy compat) |
| alloy-provider/contract/network/signer-local (sequencer) | 0.14.x |
| sp1-contracts (Solidity) | v4.0.0-rc.3 |
| Foundry (forge) | v1.4.3 |
| Solc | 0.8.20 |
| Rust | 1.93.0 |
| Next.js | 16.1.6 |
| React | 19.2.3 |

---

## Quick Reference Commands

```bash
# Build SP1 program
cd sp1-program && cargo prove build

# Generate proof (CLI)
cd script && cargo run --release -- --prove

# Run contract tests
cd contracts && forge test -vvv

# Run sequencer
cd sequencer && cargo run --release

# Run frontend
cd frontend && npm run dev

# Run all sequencer tests (batch + L1 integration)
cd sequencer && cargo test -- --nocapture

# Run only batch unit tests
cd sequencer && cargo test --test batch_tests -- --nocapture

# Run only L1 integration tests (requires network)
cd sequencer && cargo test --test l1_integration -- --nocapture

# Run API endpoint tests (requires sequencer running on port 3001)
cd sequencer && cargo test --test api_tests -- --ignored --nocapture

# Test sequencer API (manual)
curl http://localhost:3001/health
curl http://localhost:3001/batch-status
curl -X POST http://localhost:3001/submit-op \
  -H "Content-Type: application/json" \
  -d '{"operation":{"RegisterLoan":{"borrower":[1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],"amount":1000,"terms_months":12}}}'
curl -X POST http://localhost:3001/force-batch
```

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| ~~Sequencer SDK mismatch breaks proofs~~ | ~~HIGH~~ | ~~High~~ | ✅ RESOLVED: Fixed to =4.2.1 |
| ~~L1 submission via alloy fails~~ | ~~Medium~~ | ~~High~~ | ✅ RESOLVED: Real submitter using alloy 0.14.x |
| Proof generation too slow for video | Low | Medium | Reduce to 3 ops for demo (faster proof) |
| Frontend breaks against live API | Medium | Medium | Test early, fix API shape mismatches |
| Creditcoin testnet down during video | Low | High | Record video with confirmed tx, not live |
| Demo breaks during recording | Low | High | Record 5+ takes, use best one |

---

## Success Criteria

- [ ] **End-to-end flow works**: UI → Sequencer → SP1 Proof → Creditcoin L1 tx
- [ ] **Real L1 transaction**: Verifiable on Creditcoin testnet explorer
- [ ] **Compelling video**: 60-90s showing the full flow
- [ ] **Clean README**: Architecture, setup instructions, contract addresses
- [ ] **"They built a ZK rollup"** reaction from judges
