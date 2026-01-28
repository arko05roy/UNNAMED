'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { submitOperation, hexToAddress, type CreditOp } from '@/lib/api';

type OpType = 'RegisterLoan' | 'RecordRepayment' | 'UpdateCreditScore';

interface Props {
  onSubmit: () => void;
}

export function OperationForm({ onSubmit }: Props) {
  const [opType, setOpType] = useState<OpType>('RegisterLoan');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  // RegisterLoan fields
  const [borrower, setBorrower] = useState('0x1234567890abcdef1234567890abcdef12345678');
  const [amount, setAmount] = useState('1000');
  const [terms, setTerms] = useState('12');

  // RecordRepayment fields
  const [loanId, setLoanId] = useState('1');
  const [repayAmount, setRepayAmount] = useState('500');

  // UpdateCreditScore fields
  const [scoreAddress, setScoreAddress] = useState('0x1234567890abcdef1234567890abcdef12345678');
  const [newScore, setNewScore] = useState('750');

  const handleSubmit = async () => {
    setLoading(true);
    setMessage(null);

    let operation: CreditOp;

    switch (opType) {
      case 'RegisterLoan':
        operation = {
          RegisterLoan: {
            borrower: hexToAddress(borrower),
            amount: parseInt(amount),
            terms_months: parseInt(terms),
          },
        };
        break;
      case 'RecordRepayment':
        operation = {
          RecordRepayment: {
            loan_id: parseInt(loanId),
            amount: parseInt(repayAmount),
            timestamp: Math.floor(Date.now() / 1000),
          },
        };
        break;
      case 'UpdateCreditScore':
        operation = {
          UpdateCreditScore: {
            address: hexToAddress(scoreAddress),
            new_score: parseInt(newScore),
          },
        };
        break;
    }

    try {
      const res = await submitOperation(operation);
      setMessage(`Added to batch (position ${res.batch_position}/${res.batch_size})`);
      onSubmit();
    } catch (err) {
      setMessage(`Error: ${err instanceof Error ? err.message : 'Unknown error'}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-4">
      <div className="flex gap-2">
        {(['RegisterLoan', 'RecordRepayment', 'UpdateCreditScore'] as OpType[]).map((type) => (
          <Button
            key={type}
            variant={opType === type ? 'default' : 'outline'}
            size="sm"
            onClick={() => setOpType(type)}
          >
            {type === 'RegisterLoan' ? 'Loan' : type === 'RecordRepayment' ? 'Repay' : 'Score'}
          </Button>
        ))}
      </div>

      {opType === 'RegisterLoan' && (
        <div className="space-y-3">
          <div>
            <Label>Borrower Address</Label>
            <Input value={borrower} onChange={(e) => setBorrower(e.target.value)} className="font-mono text-xs" />
          </div>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <Label>Amount ($)</Label>
              <Input type="number" value={amount} onChange={(e) => setAmount(e.target.value)} />
            </div>
            <div>
              <Label>Terms (months)</Label>
              <Input type="number" value={terms} onChange={(e) => setTerms(e.target.value)} />
            </div>
          </div>
        </div>
      )}

      {opType === 'RecordRepayment' && (
        <div className="space-y-3">
          <div>
            <Label>Loan ID</Label>
            <Input type="number" value={loanId} onChange={(e) => setLoanId(e.target.value)} />
          </div>
          <div>
            <Label>Repayment Amount ($)</Label>
            <Input type="number" value={repayAmount} onChange={(e) => setRepayAmount(e.target.value)} />
          </div>
        </div>
      )}

      {opType === 'UpdateCreditScore' && (
        <div className="space-y-3">
          <div>
            <Label>Address</Label>
            <Input value={scoreAddress} onChange={(e) => setScoreAddress(e.target.value)} className="font-mono text-xs" />
          </div>
          <div>
            <Label>New Score (300-850)</Label>
            <Input type="number" value={newScore} onChange={(e) => setNewScore(e.target.value)} min="300" max="850" />
          </div>
        </div>
      )}

      <Button onClick={handleSubmit} disabled={loading} className="w-full">
        {loading ? 'Submitting...' : 'Submit Operation'}
      </Button>

      {message && (
        <p className={`text-sm ${message.startsWith('Error') ? 'text-red-500' : 'text-green-500'}`}>
          {message}
        </p>
      )}
    </div>
  );
}
