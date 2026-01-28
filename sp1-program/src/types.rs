use serde::{Deserialize, Serialize};

pub type Address = [u8; 20];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Loan {
    pub id: u64,
    pub borrower: Address,
    pub amount: u64,
    pub terms_months: u32,
    pub repaid_amount: u64,
    pub status: LoanStatus,
    pub created_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repayment {
    pub loan_id: u64,
    pub amount: u64,
    pub timestamp: u64,
}
