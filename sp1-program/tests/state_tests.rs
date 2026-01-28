use zkcredit_program::{apply_batch, CreditOp, CreditState};

#[test]
fn test_register_loan() {
    let mut state = CreditState::new();
    let borrower = [1u8; 20];
    let ops = vec![CreditOp::RegisterLoan {
        borrower,
        amount: 1000,
        terms_months: 12,
    }];
    let result = apply_batch(&mut state, ops);
    assert!(result.is_ok());
    assert_eq!(state.loans.len(), 1);
    assert_eq!(state.loans[0].amount, 1000);
    assert_eq!(state.loans[0].id, 1);
    assert_eq!(*state.credit_scores.get(&borrower).unwrap(), 500);
}

#[test]
fn test_multiple_loans() {
    let mut state = CreditState::new();
    let borrower_a = [1u8; 20];
    let borrower_b = [2u8; 20];

    let ops = vec![
        CreditOp::RegisterLoan {
            borrower: borrower_a,
            amount: 1000,
            terms_months: 12,
        },
        CreditOp::RegisterLoan {
            borrower: borrower_b,
            amount: 2000,
            terms_months: 24,
        },
    ];
    let result = apply_batch(&mut state, ops);
    assert!(result.is_ok());
    assert_eq!(state.loans.len(), 2);
    assert_eq!(state.loans[0].id, 1);
    assert_eq!(state.loans[1].id, 2);
    assert_eq!(state.next_loan_id, 3);
}

#[test]
fn test_partial_repayment() {
    let mut state = CreditState::new();
    let borrower = [1u8; 20];

    // Register loan
    apply_batch(
        &mut state,
        vec![CreditOp::RegisterLoan {
            borrower,
            amount: 1000,
            terms_months: 12,
        }],
    )
    .unwrap();

    // Partial repayment
    apply_batch(
        &mut state,
        vec![CreditOp::RecordRepayment {
            loan_id: 1,
            amount: 500,
            timestamp: 100,
        }],
    )
    .unwrap();

    assert_eq!(state.loans[0].repaid_amount, 500);
    assert_eq!(
        state.loans[0].status,
        zkcredit_program::types::LoanStatus::Active
    );
    assert_eq!(state.repayments.len(), 1);
}

#[test]
fn test_full_repayment_boosts_score() {
    let mut state = CreditState::new();
    let borrower = [1u8; 20];

    // Register loan (initial score = 500)
    apply_batch(
        &mut state,
        vec![CreditOp::RegisterLoan {
            borrower,
            amount: 1000,
            terms_months: 12,
        }],
    )
    .unwrap();

    // Full repayment
    apply_batch(
        &mut state,
        vec![CreditOp::RecordRepayment {
            loan_id: 1,
            amount: 1000,
            timestamp: 200,
        }],
    )
    .unwrap();

    assert_eq!(
        state.loans[0].status,
        zkcredit_program::types::LoanStatus::Repaid
    );
    // Score should be boosted from 500 to 510
    assert_eq!(*state.credit_scores.get(&borrower).unwrap(), 510);
}

#[test]
fn test_credit_score_update() {
    let mut state = CreditState::new();
    let addr = [3u8; 20];

    apply_batch(
        &mut state,
        vec![CreditOp::UpdateCreditScore {
            address: addr,
            new_score: 750,
        }],
    )
    .unwrap();

    assert_eq!(*state.credit_scores.get(&addr).unwrap(), 750);
}

#[test]
fn test_credit_score_clamped() {
    let mut state = CreditState::new();
    let addr = [4u8; 20];

    // Score above max (850)
    apply_batch(
        &mut state,
        vec![CreditOp::UpdateCreditScore {
            address: addr,
            new_score: 999,
        }],
    )
    .unwrap();
    assert_eq!(*state.credit_scores.get(&addr).unwrap(), 850);

    // Score below min (300)
    apply_batch(
        &mut state,
        vec![CreditOp::UpdateCreditScore {
            address: addr,
            new_score: 100,
        }],
    )
    .unwrap();
    assert_eq!(*state.credit_scores.get(&addr).unwrap(), 300);
}

#[test]
fn test_repayment_nonexistent_loan() {
    let mut state = CreditState::new();

    let result = apply_batch(
        &mut state,
        vec![CreditOp::RecordRepayment {
            loan_id: 999,
            amount: 100,
            timestamp: 1,
        }],
    );

    assert!(result.is_err());
}

#[test]
fn test_state_root_deterministic() {
    let mut state1 = CreditState::new();
    let mut state2 = CreditState::new();

    let ops = vec![CreditOp::RegisterLoan {
        borrower: [1u8; 20],
        amount: 1000,
        terms_months: 12,
    }];

    let root1 = apply_batch(&mut state1, ops.clone()).unwrap();
    let root2 = apply_batch(&mut state2, ops).unwrap();

    assert_eq!(root1, root2);
}

#[test]
fn test_state_root_changes_with_ops() {
    let mut state = CreditState::new();
    let root_before = state.compute_root();

    apply_batch(
        &mut state,
        vec![CreditOp::RegisterLoan {
            borrower: [1u8; 20],
            amount: 1000,
            terms_months: 12,
        }],
    )
    .unwrap();

    let root_after = state.compute_root();
    assert_ne!(root_before, root_after);
}
