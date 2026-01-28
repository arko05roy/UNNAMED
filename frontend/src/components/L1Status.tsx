'use client';

import { Badge } from '@/components/ui/badge';

interface Props {
  txHash: string | null;
  newStateRoot: string | null;
}

export function L1Status({ txHash, newStateRoot }: Props) {
  if (!txHash) {
    return (
      <p className="text-sm text-muted-foreground">
        No L1 settlement yet. Submit operations and generate a proof.
      </p>
    );
  }

  const explorerUrl = `https://explorer.testnet.creditcoin.org/tx/${txHash}`;

  return (
    <div className="space-y-3">
      <div className="flex items-center gap-2">
        <Badge variant="default" className="bg-green-600">Settled</Badge>
        <span className="text-sm">Transaction confirmed on Creditcoin</span>
      </div>
      <div>
        <p className="text-xs text-muted-foreground">Transaction Hash</p>
        <a
          href={explorerUrl}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs font-mono text-blue-500 hover:underline break-all"
        >
          {txHash}
        </a>
      </div>
      {newStateRoot && (
        <div>
          <p className="text-xs text-muted-foreground">New State Root</p>
          <p className="text-xs font-mono break-all">0x{newStateRoot}</p>
        </div>
      )}
    </div>
  );
}
