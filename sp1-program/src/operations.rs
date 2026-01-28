extern crate alloc;

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::state::CreditState;
use crate::types::{Address, Loan, LoanStatus, Repayment};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CreditOp {
    RegisterLoan {
        borrower: Address,
        amount: u64,
        terms_months: u32,
    },
    RecordRepayment {
        loan_id: u64,
        amount: u64,
        timestamp: u64,
    },
    UpdateCreditScore {
        address: Address,
        new_score: u32,
    },
}

#[derive(Debug)]
pub enum OpError {
    LoanNotFound,
    InvalidAmount,
    AlreadyRepaid,
}

impl core::fmt::Display for OpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OpError::LoanNotFound => write!(f, "Loan not found"),
            OpError::InvalidAmount => write!(f, "Invalid amount"),
            OpError::AlreadyRepaid => write!(f, "Loan already repaid"),
        }
    }
}

pub fn apply_op(state: &mut CreditState, op: CreditOp) -> Result<(), OpError> {
    match op {
        CreditOp::RegisterLoan {
            borrower,
            amount,
            terms_months,
        } => {
            if amount == 0 {
                return Err(OpError::InvalidAmount);
            }
            let loan = Loan {
                id: state.next_loan_id,
                borrower,
                amount,
                terms_months,
                repaid_amount: 0,
                status: LoanStatus::Active,
                created_at: state.nonce,
            };
            state.loans.push(loan);
            state.next_loan_id += 1;

            // Initialize credit score if not exists
            state.credit_scores.entry(borrower).or_insert(500);
            Ok(())
        }
        CreditOp::RecordRepayment {
            loan_id,
            amount,
            timestamp,
        } => {
            let loan = state
                .loans
                .iter_mut()
                .find(|l| l.id == loan_id)
                .ok_or(OpError::LoanNotFound)?;

            if loan.status != LoanStatus::Active {
                return Err(OpError::AlreadyRepaid);
            }

            if amount == 0 {
                return Err(OpError::InvalidAmount);
            }

            loan.repaid_amount += amount;

            // Check if fully repaid
            if loan.repaid_amount >= loan.amount {
                loan.status = LoanStatus::Repaid;
                // Boost credit score on full repayment
                if let Some(score) = state.credit_scores.get_mut(&loan.borrower) {
                    *score = (*score + 10).min(850);
                }
            }

            state.repayments.push(Repayment {
                loan_id,
                amount,
                timestamp,
            });
            Ok(())
        }
        CreditOp::UpdateCreditScore { address, new_score } => {
            state
                .credit_scores
                .insert(address, new_score.clamp(300, 850));
            Ok(())
        }
    }
}

pub fn apply_batch(state: &mut CreditState, ops: Vec<CreditOp>) -> Result<[u8; 32], OpError> {
    for op in ops {
        apply_op(state, op)?;
    }
    state.nonce += 1;
    Ok(state.compute_root())
}
