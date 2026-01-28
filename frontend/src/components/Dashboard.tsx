'use client';

import { useState, useEffect, useCallback } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { BatchStatus } from './BatchStatus';
import { OperationForm } from './OperationForm';
import { ProofProgress } from './ProofProgress';
import { L1Status } from './L1Status';
import { fetchBatchStatus, type BatchStatus as BatchStatusType } from '@/lib/api';

export function Dashboard() {
  const [batchStatus, setBatchStatus] = useState<BatchStatusType | null>(null);
  const [lastTxHash, setLastTxHash] = useState<string | null>(null);
  const [lastStateRoot, setLastStateRoot] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    try {
      const data = await fetchBatchStatus();
      setBatchStatus(data);
    } catch {
      // Sequencer not running - show offline state
      setBatchStatus(null);
    }
  }, []);

  useEffect(() => {
    refresh();
    const interval = setInterval(refresh, 2000);
    return () => clearInterval(interval);
  }, [refresh]);

  const handleProofComplete = (txHash: string, newRoot: string) => {
    setLastTxHash(txHash);
    setLastStateRoot(newRoot);
    refresh();
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
      <Card>
        <CardHeader>
          <CardTitle>Submit Operation</CardTitle>
        </CardHeader>
        <CardContent>
          <OperationForm onSubmit={refresh} />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Batch Status</CardTitle>
        </CardHeader>
        <CardContent>
          <BatchStatus status={batchStatus} />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Proof Generation</CardTitle>
        </CardHeader>
        <CardContent>
          <ProofProgress
            pendingOps={batchStatus?.pending_ops ?? 0}
            onComplete={handleProofComplete}
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>L1 Settlement</CardTitle>
        </CardHeader>
        <CardContent>
          <L1Status txHash={lastTxHash} newStateRoot={lastStateRoot} />
        </CardContent>
      </Card>
    </div>
  );
}
