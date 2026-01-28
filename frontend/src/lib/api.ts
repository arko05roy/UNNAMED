const API_BASE = process.env.NEXT_PUBLIC_SEQUENCER_URL || 'http://localhost:3001';

export interface BatchStatus {
  pending_ops: number;
  batch_capacity: number;
  last_batch_number: number;
  state_root: string;
}

export interface SubmitOpResponse {
  success: boolean;
  batch_position: number;
  batch_size: number;
}

export interface ForceResponse {
  success: boolean;
  tx_hash?: string;
  new_state_root?: string;
  error?: string;
}

export interface CreditOp {
  RegisterLoan?: {
    borrower: number[];
    amount: number;
    terms_months: number;
  };
  RecordRepayment?: {
    loan_id: number;
    amount: number;
    timestamp: number;
  };
  UpdateCreditScore?: {
    address: number[];
    new_score: number;
  };
}

export async function fetchBatchStatus(): Promise<BatchStatus> {
  const res = await fetch(`${API_BASE}/batch-status`);
  if (!res.ok) throw new Error('Failed to fetch batch status');
  return res.json();
}

export async function submitOperation(operation: CreditOp): Promise<SubmitOpResponse> {
  const res = await fetch(`${API_BASE}/submit-op`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ operation }),
  });
  if (!res.ok) throw new Error('Failed to submit operation');
  return res.json();
}

export async function forceBatch(): Promise<ForceResponse> {
  const res = await fetch(`${API_BASE}/force-batch`, {
    method: 'POST',
  });
  if (!res.ok) throw new Error('Failed to force batch');
  return res.json();
}

// Helper to create an address from a hex string
export function hexToAddress(hex: string): number[] {
  const clean = hex.replace('0x', '').padStart(40, '0');
  const bytes: number[] = [];
  for (let i = 0; i < 40; i += 2) {
    bytes.push(parseInt(clean.substring(i, i + 2), 16));
  }
  return bytes;
}
