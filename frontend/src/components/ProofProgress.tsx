'use client';

import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Progress } from '@/components/ui/progress';
import { forceBatch } from '@/lib/api';

interface Props {
  pendingOps: number;
  onComplete: (txHash: string, newRoot: string) => void;
}

export function ProofProgress({ pendingOps, onComplete }: Props) {
  const [generating, setGenerating] = useState(false);
  const [progress, setProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!generating) return;

    // Simulate progress animation
    const interval = setInterval(() => {
      setProgress((prev) => {
        if (prev >= 90) return 90; // Hold at 90% until real completion
        return prev + Math.random() * 15;
      });
    }, 500);

    return () => clearInterval(interval);
  }, [generating]);

  const handleForce = async () => {
    if (pendingOps === 0) {
      setError('No pending operations to prove');
      return;
    }

    setGenerating(true);
    setProgress(0);
    setError(null);

    try {
      const res = await forceBatch();
      if (res.success && res.tx_hash) {
        setProgress(100);
        onComplete(res.tx_hash, res.new_state_root || '');
      } else {
        setError(res.error || 'Proof generation failed');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setTimeout(() => {
        setGenerating(false);
        setProgress(0);
      }, 2000);
    }
  };

  return (
    <div className="space-y-4">
      {generating ? (
        <>
          <div className="text-sm font-medium">Generating ZK Proof...</div>
          <Progress value={progress} className="h-3" />
          <p className="text-xs text-muted-foreground">
            Proving {pendingOps} operations in SP1 zkVM
          </p>
        </>
      ) : (
        <>
          <p className="text-sm text-muted-foreground">
            {pendingOps > 0
              ? `${pendingOps} operation${pendingOps > 1 ? 's' : ''} ready to prove`
              : 'No pending operations'}
          </p>
          <Button
            onClick={handleForce}
            disabled={pendingOps === 0}
            className="w-full"
            variant="secondary"
          >
            Generate Proof & Submit to L1
          </Button>
        </>
      )}

      {error && <p className="text-sm text-red-500">{error}</p>}
    </div>
  );
}
