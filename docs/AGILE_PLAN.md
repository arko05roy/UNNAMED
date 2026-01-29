# zkCredit - Private Credit Scoring for DeFi Lending on Creditcoin

## STRATEGIC PIVOT (Jan 30, 2026)

**Old stack:** SP1 zkVM (Rust) → Sequencer (Rust/Axum) → Contracts → Frontend
**New stack:** Noir circuit → noir_js (browser proof gen) → Contracts → Frontend

**Why Noir over SP1:**
- Credit history never leaves the user's browser (strongest possible privacy story)
- No backend server needed (2 components instead of 3, nothing to crash during demo)
- Noir is purpose-built for "prove a property about private data" (SP1 is a general-purpose zkVM -- overkill)
- Simpler circuit, faster development, cleaner architecture
- Demo moment: judge generates proof IN THEIR BROWSER, data never transmitted

**Track:** DeFi (lending, credit, privacy)
**One-liner:** "Prove your creditworthiness without revealing your history. Zero-knowledge credit scoring, entirely in your browser."

**Why this wins on Creditcoin:**
- Creditcoin = "credit coin" → you're building private credit scoring on the credit chain
- Creditcoin's blog: "privacy for borrowers is essential"
- Tae Oh's mission: microloans for 1.4B unbanked, privacy-preserving
- DeFi track: lending + credit scoring = direct fit
- 30 participants: judges have time, technical depth is valued
- Demo Day Seoul (March 21): finalists fly out, needs to demo live
- ZK credit scoring is validated (zkMe, ETHGlobal winners)

---

## Progress Summary (Updated: Jan 30, 2026)

### Preserved from SP1 phase
| Sprint | Status | Notes |
|--------|--------|-------|
| Environment Setup | ✅ COMPLETED | Rust, Foundry, Next.js |
| Credit State Machine | ✅ COMPLETED | Scoring logic → port to Noir |
| SP1 Integration | ✅ COMPLETED | NOT USED in new arch (keep as depth proof) |
| Solidity Verifier (SP1) | ✅ COMPLETED | NOT USED (deploying Noir verifier instead) |
| RollupCore | ✅ COMPLETED | KEEP deployed, mention as bonus feature |
| Sequencer | ✅ COMPLETED | NOT USED in new arch (no backend needed) |

### New Noir-based sprints
| Sprint | Status | Progress |
|--------|--------|----------|
| N1: Noir Circuit | 🔄 TODO | Credit score proof circuit |
| N2: Noir Verifier Contract | 🔄 TODO | UltraHonk verifier + CreditVerifier + LendingPool |
| N3: Browser Proof Integration | 🔄 TODO | noir_js + barretenberg in Next.js |
| N4: Frontend (Lending UI) | 🔄 TODO | Borrower flow + privacy visualization |
| N5: Demo Polish | ⏳ PENDING | - |
| N6: Video + Submission | ⏳ PENDING | - |

**Deadline:** February 22, 2026 (23 days remaining)
**Demo Day:** March 21, 2026 (Seoul, if finalist)

---

## New Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     USER'S BROWSER (Next.js)                      │
│                                                                    │
│  ┌──────────────────────────────────────────────────────────────┐ │
│  │  PRIVATE ZONE (never leaves browser)                         │ │
│  │                                                               │ │
│  │  Credit History ──→ Noir Circuit ──→ ZK Proof                │ │
│  │  (5 loans,          (compute score,   (32 bytes,             │ │
│  │   3 repaid)          prove >= 650)     no history)            │ │
│  │                                                               │ │
│  │  noir_js + barretenberg.js (WASM)                            │ │
│  └──────────────────────────────────────────────────────────────┘ │
│                           │                                       │
│                           │ Only proof + public inputs go out     │
│                           ▼                                       │
└──────────────────────────────────────────────────────────────────┘
                            │
                            │ submitProof(proofBytes, publicInputs)
                            ▼
┌──────────────────────────────────────────────────────────────────┐
│                    CREDITCOIN L1 (EVM)                             │
│                                                                    │
│  ┌──────────────────┐  ┌──────────────────┐                      │
│  │  LendingPool.sol │  │CreditVerifier.sol│                      │
│  │  - deposit()     │  │ - verifyProof()  │                      │
│  │  - borrow(proof) │  │ - UltraHonk      │                      │
│  │  - repay()       │  │   verification   │                      │
│  │  - getPoolStats()│  └──────────────────┘                      │
│  └──────────────────┘                                             │
│                                                                    │
│  ┌──────────────────┐  (BONUS: still deployed from SP1 phase)    │
│  │  RollupCore.sol  │  Batch credit history updates              │
│  │  SP1Verifier.sol │  Shows additional technical depth          │
│  └──────────────────┘                                             │
└──────────────────────────────────────────────────────────────────┘
```

**Key insight:** There is NO backend server. The browser does everything. Credit history enters the browser, proof exits the browser, history never touches a server or the chain.

---

## File Structure (New)

```
ctc/
├── circuits/                          # 🔄 NEW: Noir circuit
│   ├── Nargo.toml
│   └── src/
│       └── main.nr                    # Credit score proof circuit
├── contracts/                         # Partially reuse
│   ├── foundry.toml
│   ├── src/
│   │   ├── CreditVerifier.sol         # NEW: Noir UltraHonk verifier
│   │   ├── LendingPool.sol            # NEW: Deposit/borrow/repay
│   │   ├── RollupCore.sol             # EXISTING: keep deployed
│   │   └── SP1Verifier.sol            # EXISTING: keep deployed
│   └── test/
│       ├── LendingPool.t.sol          # NEW
│       └── RollupCore.t.sol           # EXISTING
├── frontend/                          # Redesign
│   ├── package.json                   # Add noir_js, barretenberg deps
│   └── src/
│       ├── app/
│       │   ├── layout.tsx
│       │   └── page.tsx               # Lending dashboard
│       ├── components/
│       │   ├── BorrowerFlow.tsx        # NEW: multi-step borrow flow
│       │   ├── LenderView.tsx          # NEW: deposit/withdraw
│       │   ├── PrivacyViz.tsx          # NEW: split-screen privacy
│       │   ├── ProofAnimation.tsx      # NEW: animated proof gen
│       │   ├── PoolStats.tsx           # NEW: lending pool stats
│       │   └── ui/ (shadcn - reuse)
│       └── lib/
│           ├── noir.ts                 # NEW: noir_js proof generation
│           ├── contracts.ts            # NEW: ethers/viem contract calls
│           └── utils.ts                # EXISTING
├── sp1-program/                       # KEEP: shows depth, not used in main flow
├── script/                            # KEEP: shows depth
├── sequencer/                         # KEEP: shows depth, not used in main flow
└── docs/
    └── AGILE_PLAN.md
```

---

## Sprint N1: Noir Circuit (Days 1-4)

### Goal
Noir circuit that computes credit score from loan history and proves score >= threshold.
All private inputs stay private. Only threshold + eligible + state commitment are public.

### Circuit Design

**Private inputs (witness -- never revealed):**
- `loans`: array of loan records (borrower, amount, terms, repaid_amount, status)
- `repayments`: array of repayment records
- `borrower_address`: which address we're scoring

**Public inputs (visible on-chain):**
- `threshold`: minimum required score (e.g., 650)
- `eligible`: bool (score >= threshold)
- `state_commitment`: hash of the credit history (proves which data was used without revealing it)

### `circuits/src/main.nr` (pseudocode)
```noir
// Credit score proof circuit
// Proves: compute_score(private_history) >= public_threshold
// Without revealing: the history, the score, or the borrower

struct Loan {
    amount: u64,
    repaid_amount: u64,
    is_repaid: bool,      // true if fully repaid
    terms_months: u32,
}

fn compute_credit_score(loans: [Loan; MAX_LOANS], num_loans: u32) -> u32 {
    let mut score: u32 = 500;  // base score

    // Factor 1: Repayment ratio (35% weight, up to +175)
    let mut repaid_count: u32 = 0;
    for i in 0..MAX_LOANS {
        if i < num_loans {
            if loans[i].is_repaid {
                repaid_count += 1;
            }
        }
    }
    if num_loans > 0 {
        let ratio = (repaid_count * 100) / num_loans;
        score += (ratio * 175) / 100;
    }

    // Factor 2: Utilization (30% weight, up to +150)
    let mut total_borrowed: u64 = 0;
    let mut total_repaid: u64 = 0;
    for i in 0..MAX_LOANS {
        if i < num_loans {
            total_borrowed += loans[i].amount;
            total_repaid += loans[i].repaid_amount;
        }
    }
    if total_borrowed > 0 {
        let util = ((total_repaid * 100) / total_borrowed) as u32;
        let capped = if util > 100 { 100 } else { util };
        score += (capped * 150) / 100;
    }

    // Factor 3: History length (15% weight, up to +80)
    let capped_loans = if num_loans > 10 { 10 } else { num_loans };
    score += capped_loans * 8;

    // Clamp 300-850
    if score < 300 { score = 300; }
    if score > 850 { score = 850; }

    score
}

fn main(
    // Private inputs
    loans: [Loan; MAX_LOANS],
    num_loans: u32,

    // Public inputs
    threshold: pub u32,
    state_commitment: pub Field,   // poseidon hash of loan data
) -> pub bool {
    // 1. Verify state commitment matches the provided loans
    let computed_commitment = pedersen_hash(loans, num_loans);
    assert(computed_commitment == state_commitment);

    // 2. Compute credit score
    let score = compute_credit_score(loans, num_loans);

    // 3. Check eligibility
    let eligible = score >= threshold;

    // 4. Return eligibility (public output)
    eligible
}
```

### Key design decisions
- **MAX_LOANS = 10**: Fixed-size array for circuit (enough for demo, keeps proof fast)
- **State commitment**: Pedersen/Poseidon hash of loan data. Proves "this proof was computed from THIS specific history" without revealing the history. Prevents proof reuse with different data.
- **Score never revealed**: Only `eligible` (bool) is public. Lender knows "yes/no", never the actual score.

### Tasks
- [ ] Initialize Noir project: `nargo new circuits`
- [ ] Define `Loan` struct and `MAX_LOANS` constant
- [ ] Implement `compute_credit_score()` in Noir
- [ ] Implement `main()` with private/public input separation
- [ ] Implement state commitment (Pedersen hash of loans)
- [ ] Write Noir tests: `nargo test`
  - [ ] Empty history → score 500, threshold 650 → eligible = false
  - [ ] 5 loans, 3 repaid → score ~720, threshold 650 → eligible = true
  - [ ] 5 loans, 0 repaid → score ~540, threshold 650 → eligible = false
  - [ ] Wrong state commitment → assertion fails
- [ ] `nargo compile` succeeds
- [ ] `nargo prove` generates proof for test case
- [ ] `nargo verify` passes

### Checkpoint
- [ ] Circuit compiles
- [ ] All tests pass
- [ ] Proof generates for 5-loan history
- [ ] Proof generation time: ___ms (target: <10s for demo)
- [ ] Private inputs confirmed NOT in proof

---

## Sprint N2: Contracts (Days 5-8)

### Goal
Deploy Noir verifier + LendingPool on Creditcoin testnet

### Contracts

**1. Generate Noir verifier**
```bash
# Generate Solidity verifier from compiled circuit
nargo codegen-verifier
# or with bb (barretenberg):
bb write_vk -b ./target/circuits.json -o ./target/vk
bb contract -k ./target/vk -o ./contracts/src/NoirVerifier.sol
```

**2. `contracts/src/CreditVerifier.sol`**
- Wraps generated Noir verifier
- Parses public inputs: threshold, state_commitment, eligible
- Exposes `verifyCreditProof(bytes proof, bytes32[] publicInputs) → bool`

**3. `contracts/src/LendingPool.sol`**
- `deposit() payable` -- lender deposits CTC
- `withdraw(uint256)` -- lender withdraws
- `borrow(uint256 amount, bytes proof, bytes32[] publicInputs)` -- borrow with ZK proof
  - Calls CreditVerifier.verifyCreditProof()
  - If valid: transfer CTC to borrower, record loan
  - If invalid: revert
- `repay(uint256 loanId) payable` -- repay loan
- `getPoolStats() view` -- total deposited, borrowed, loan count
- `getLoan(uint256) view` -- individual loan details

**Keep simple:**
- No interest accrual (fixed 5% APR for display only)
- No liquidation engine
- No collateral management
- Just: deposit, prove, borrow, repay

### Tasks
- [ ] Generate Noir Solidity verifier (`nargo codegen-verifier` or `bb contract`)
- [ ] Write `CreditVerifier.sol` wrapping generated verifier
- [ ] Write `LendingPool.sol` with deposit/borrow/repay
- [ ] Write tests:
  - [ ] Deposit and withdraw
  - [ ] Borrow with valid proof succeeds
  - [ ] Borrow with invalid proof reverts
  - [ ] Repay updates loan status
  - [ ] Pool stats return correct values
  - [ ] Cannot borrow more than pool balance
- [ ] `forge test` passes all tests
- [ ] Deploy NoirVerifier to Creditcoin testnet
- [ ] Deploy CreditVerifier to Creditcoin testnet
- [ ] Deploy LendingPool to Creditcoin testnet
- [ ] Seed pool with test CTC (deposit from deployer)
- [ ] Verify `getPoolStats()` returns correct values

### Checkpoint
- [ ] All contract tests pass
- [ ] NoirVerifier deployed: `0x___`
- [ ] CreditVerifier deployed: `0x___`
- [ ] LendingPool deployed: `0x___`
- [ ] Pool seeded with test CTC
- [ ] View functions return correct data

---

## Sprint N3: Browser Proof Integration (Days 9-12)

### Goal
Generate Noir proofs in the browser using noir_js + barretenberg WASM

### Setup
```bash
cd frontend
npm install @noir-lang/noir_js @noir-lang/backend_barretenberg
```

### `frontend/src/lib/noir.ts`
```typescript
import { Noir } from '@noir-lang/noir_js';
import { BarretenbergBackend } from '@noir-lang/backend_barretenberg';
import circuit from '../../circuits/target/circuits.json';

export interface LoanInput {
  amount: number;
  repaid_amount: number;
  is_repaid: boolean;
  terms_months: number;
}

export interface CreditProofResult {
  proof: Uint8Array;
  publicInputs: string[];
  eligible: boolean;
  proofTimeMs: number;
}

export async function generateCreditProof(
  loans: LoanInput[],
  threshold: number,
): Promise<CreditProofResult> {
  const backend = new BarretenbergBackend(circuit);
  const noir = new Noir(circuit, backend);

  // Pad loans to MAX_LOANS
  const paddedLoans = padLoans(loans, MAX_LOANS);
  const stateCommitment = computeCommitment(paddedLoans, loans.length);

  const input = {
    loans: paddedLoans,
    num_loans: loans.length,
    threshold: threshold,
    state_commitment: stateCommitment,
  };

  const start = performance.now();
  const proof = await noir.generateProof(input);
  const elapsed = performance.now() - start;

  return {
    proof: proof.proof,
    publicInputs: proof.publicInputs,
    eligible: proof.publicInputs[0] === '1', // or however noir encodes bool
    proofTimeMs: elapsed,
  };
}

export async function verifyProofLocally(
  proof: Uint8Array,
  publicInputs: string[],
): Promise<boolean> {
  const backend = new BarretenbergBackend(circuit);
  return backend.verifyProof({ proof, publicInputs });
}
```

### Key considerations
- Barretenberg WASM loads ~5-10MB, need loading indicator
- First proof is slower (WASM compilation), subsequent proofs faster
- Circuit JSON needs to be bundled with frontend (copy from `circuits/target/`)
- Test in Chrome, Firefox, Safari (barretenberg WASM compat)

### Tasks
- [ ] Install `@noir-lang/noir_js` and `@noir-lang/backend_barretenberg`
- [ ] Copy compiled circuit JSON to frontend
- [ ] Create `lib/noir.ts` with `generateCreditProof()` function
- [ ] Create `lib/contracts.ts` with ethers/viem contract interaction
  - [ ] `depositToPool(amount)`
  - [ ] `borrowWithProof(amount, proof, publicInputs)`
  - [ ] `repayLoan(loanId, amount)`
  - [ ] `getPoolStats()`
- [ ] Test proof generation in browser (console first)
- [ ] Measure proof generation time in browser
- [ ] Test submitting browser-generated proof to deployed contract
- [ ] Verify on-chain: proof accepted, loan created

### Checkpoint
- [ ] Proof generates in browser: ___ms
- [ ] Proof verifies on-chain on Creditcoin testnet
- [ ] Full flow: browser proof → contract borrow → loan created
- [ ] Works in Chrome and Firefox

---

## Sprint N4: Frontend (Lending UI) (Days 13-17)

### Goal
Interactive lending UI with privacy visualization. This is what wins.

### Pages & Components

**1. Landing Dashboard**
- Pool stats: Total deposited, total borrowed, APR, active loans
- Counter: "Loans issued: X | Data leaked: 0 bytes"
- Two CTAs: "I'm a Lender" / "I'm a Borrower"

**2. Borrower Flow (THE MAIN EVENT -- multi-step)**

Step 1: **Enter Credit History**
- Form with pre-filled sample: "Maria, Lagos, 5 microloans"
- Add/remove loans manually
- Shows computed score LOCALLY (never sent anywhere)
- Button: "Generate ZK Proof"

Step 2: **ZK Proof Generation** (the flashy moment)
- Split screen animation:
  - LEFT panel: "YOUR DATA (stays in browser)" -- shows credit history, fading/blurring
  - RIGHT panel: "ON-CHAIN (all that's sent)" -- shows proof bytes appearing, small
- Progress bar: "Generating zero-knowledge proof..."
- Timer counting up
- When done: "Proof generated in 8.3 seconds"

Step 3: **Proof Verified**
- Big checkmark: "Credit Score >= 650: VERIFIED"
- Below: "Score revealed: NEVER | History sent: NOWHERE | Data leaked: 0 bytes"
- Enter borrow amount field
- Button: "Borrow from Pool"

Step 4: **Loan Issued**
- Real Creditcoin tx hash
- Explorer link
- "You borrowed $1,000 without revealing your financial history."
- Confetti/celebration animation
- Dashboard stats update live

**3. Lender View**
- Deposit CTC: amount input + "Deposit" button
- Current deposits, pool share, APR
- Withdraw button
- "Your funds are protected by zero-knowledge proofs. Borrowers prove creditworthiness without revealing data."

**4. Privacy Comparison (visible on landing page)**
```
┌─────────────────────────┐  ┌──────────────────────────┐
│   TRADITIONAL LENDING   │  │      zkCredit            │
│                         │  │                          │
│  ✗ Name                 │  │  ✓ ZK Proof (32 bytes)   │
│  ✗ SSN / National ID    │  │  ✓ Score >= 650: YES     │
│  ✗ Income statements    │  │                          │
│  ✗ Bank statements      │  │  That's it.             │
│  ✗ Address              │  │  Nothing else.          │
│  ✗ Employment history   │  │                          │
│                         │  │  "Your history is yours. │
│  ALL PUBLIC ON-CHAIN    │  │   The proof is all       │
│                         │  │   they need."            │
└─────────────────────────┘  └──────────────────────────┘
```

### Tasks
- [ ] Redesign `page.tsx` as lending landing page with pool stats + privacy comparison
- [ ] Build `BorrowerFlow.tsx` (4-step flow with transitions)
- [ ] Build `LenderView.tsx` (deposit/withdraw)
- [ ] Build `PrivacyViz.tsx` (split-screen animation during proof gen)
- [ ] Build `ProofAnimation.tsx` (progress bar, timer, proof bytes reveal)
- [ ] Build `PoolStats.tsx` (live stats from contract)
- [ ] Connect wallet (MetaMask/injected provider for Creditcoin testnet)
- [ ] Wire up noir.ts proof generation to BorrowerFlow
- [ ] Wire up contracts.ts to LenderView and BorrowerFlow
- [ ] Test full flow 5x consecutively
- [ ] Dark mode

### Checkpoint
- [ ] Full borrower flow works: enter history → proof in browser → borrow on-chain
- [ ] Lender can deposit and see pool stats
- [ ] Privacy visualization renders correctly
- [ ] Real Creditcoin tx hash links to explorer
- [ ] 5 consecutive runs without crash

---

## Sprint N5: Demo Polish (Days 18-20)

### Tasks
- [ ] 10 consecutive runs without crash
- [ ] Pre-fill sample: "Maria, Nigerian entrepreneur, 5 microloans, 3 repaid on time"
- [ ] Handle WASM loading gracefully (first load is slower)
- [ ] Handle wallet not connected
- [ ] Handle Creditcoin testnet errors
- [ ] "Reset Demo" button
- [ ] Mobile responsive (Seoul demo day)
- [ ] Test with non-technical person
- [ ] Practice demo script 5x

### Demo Script
1. Open dashboard: "Pool: $10,000 available, 5% APR, 0 loans"
2. Show privacy comparison panel
3. Click "I'm a Borrower"
4. Load sample data: "Maria from Lagos, 5 microloans"
5. Click "Generate ZK Proof"
6. Watch split-screen: LEFT=data blurring, RIGHT=proof appearing
7. "Proof generated in 8.3 seconds. Score >= 650: VERIFIED."
8. "Notice: your credit history never left this browser."
9. Enter $1,000, click "Borrow"
10. Real Creditcoin tx, explorer link
11. Dashboard: "Loans: 1 | Data leaked: 0 bytes"

### Checkpoint
- [ ] 10 consecutive clean runs
- [ ] Demo script rehearsed 5x
- [ ] Non-technical person understood it

---

## Sprint N6: Video + Submission (Days 21-23)

### Day 21: README
- [ ] Write README.md:
  - "Prove your creditworthiness without revealing your history"
  - Architecture: no backend, browser proofs
  - Tech stack: Noir, barretenberg, Next.js, Foundry, Creditcoin
  - Contract addresses
  - How to run locally
  - Why it matters: 1.4B unbanked, privacy
  - Demo video link

### Day 22: Video
- [ ] Record 5+ takes
- [ ] Select best
- [ ] Add text overlays

### Video Structure (90 seconds)

| Time | Content |
|------|---------|
| 0:00-0:10 | "1.4 billion people can't get loans. When they can, their entire financial history becomes public." |
| 0:10-0:20 | "zkCredit: Zero-knowledge credit scoring on Creditcoin. Prove you're creditworthy. Reveal nothing." |
| 0:20-0:25 | Show lending pool dashboard |
| 0:25-0:35 | Enter credit history, click "Generate ZK Proof" |
| 0:35-0:45 | Split-screen: data stays private, only proof goes out |
| 0:45-0:55 | "Verified! Score >= 650." Borrow $1,000. Real Creditcoin tx. |
| 0:55-1:05 | "Your data never left your browser. Not to us. Not to anyone." |
| 1:05-1:15 | Privacy comparison panel + "Built on Creditcoin" |
| 1:15-1:30 | Architecture + tech stack + "The proof is all they need." |

### Day 23: Submit
- [ ] Full end-to-end test
- [ ] Upload video
- [ ] Submit
- [ ] Verify links

---

## Contract Addresses (Creditcoin CC3 Testnet)

| Contract | Testnet Address |
|----------|-----------------|
| NoirVerifier | TBD (deploy Sprint N2) |
| CreditVerifier | TBD (deploy Sprint N2) |
| LendingPool | TBD (deploy Sprint N2) |
| SP1Verifier (legacy) | `0x48eECA83A5A0B3072E9a71714589D55F1e70016D` |
| RollupCore (legacy) | `0x7Ec1eb320aAe1F7BA8a324198E17d3Cf096B4679` |

**Deployer:** `0xABaF59180e0209bdB8b3048bFbe64e855074C0c4`
**RPC:** `https://rpc.cc3-testnet.creditcoin.network`
**Chain ID:** `102031`

---

## Tech Stack

| Component | Technology |
|-----------|-----------|
| ZK Circuit | Noir |
| Browser Proofs | noir_js + barretenberg (WASM) |
| Smart Contracts | Solidity 0.8.20, Foundry |
| Frontend | Next.js 16, React 19, Tailwind, shadcn/ui |
| Chain | Creditcoin CC3 Testnet (EVM) |
| Wallet | MetaMask (injected provider) |

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Noir verifier doesn't deploy on Creditcoin EVM | Low | High | Test deployment Day 5, fall back to mock verifier |
| Browser proof gen too slow (>30s) | Medium | Medium | Reduce MAX_LOANS to 5, optimize circuit |
| barretenberg WASM fails in some browsers | Low | Medium | Test Chrome + Firefox early, Safari backup |
| LendingPool contract bugs | Medium | Medium | Keep contract dead simple, no interest math |
| Frontend redesign takes too long | Medium | Medium | Focus on borrower flow only, skip lender UI polish |

---

## What To Say When Judges Ask

**"Why Noir instead of SP1/Risc Zero/other zkVM?"**
"SP1 and Risc Zero are general-purpose zkVMs -- great for complex computation but require a backend server to generate proofs. For credit scoring, the borrower's data should never leave their device. Noir lets us generate proofs entirely in the browser. The credit history enters the browser and the proof exits the browser. Nothing else."

**"Did you actually build the SP1 version too?"**
"Yes -- we built a full ZK rollup with SP1 first (show RollupCore on explorer). But when we realized the privacy story is stronger with client-side proofs, we pivoted to Noir. The SP1 rollup is still deployed on Creditcoin testnet if you want to see it. We chose the architecture that gives borrowers the most privacy."

**"How is this different from zkMe or other ZK credit projects?"**
"zkMe bridges FICO scores on-chain -- it requires you to already have a credit score from a traditional bureau. zkCredit works for the 1.4 billion people who DON'T have traditional credit scores. We compute the score from on-chain loan history and prove it in zero knowledge. No bureau needed. No intermediary needed."

**"Is this production-ready?"**
"This is a working prototype on Creditcoin testnet. For production: add more scoring factors, integrate with Creditcoin's existing credit transaction records, add circuit auditing, and deploy on mainnet. The ZK primitives are production-grade (Noir/barretenberg), the lending pool would need formal verification."

---

## Success Criteria

- [ ] **DeFi track fit**: Lending protocol with ZK credit scoring
- [ ] **Privacy story**: "Data never leaves your browser" (strongest possible claim)
- [ ] **No backend**: Browser → Chain (judges can verify no server involved)
- [ ] **Interactive demo**: Judge enters credit history, sees proof generate in their browser
- [ ] **Real L1 transaction**: Borrow tx on Creditcoin explorer
- [ ] **Compelling video**: 90s showing full flow with privacy split-screen
- [ ] **Narrative match**: "Private credit scoring on the credit chain"
- [ ] **Technical depth**: Noir circuit + deployed contracts + browser proofs + legacy SP1 rollup
- [ ] **Judge reaction**: "They built ZK credit scoring that runs in the browser on Creditcoin"
