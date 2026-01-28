'use client';

import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import type { BatchStatus as BatchStatusType } from '@/lib/api';

interface Props {
  status: BatchStatusType | null;
}

export function BatchStatus({ status }: Props) {
  if (!status) {
    return <p className="text-muted-foreground">Loading...</p>;
  }

  const fillPercent = (status.pending_ops / status.batch_capacity) * 100;

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <span className="text-sm font-medium">Pending Operations</span>
        <Badge variant="secondary">
          {status.pending_ops} / {status.batch_capacity}
        </Badge>
      </div>
      <Progress value={fillPercent} className="h-3" />
      <div className="grid grid-cols-2 gap-4 text-sm">
        <div>
          <p className="text-muted-foreground">Last Batch</p>
          <p className="font-mono font-bold">#{status.last_batch_number}</p>
        </div>
        <div>
          <p className="text-muted-foreground">State Root</p>
          <p className="font-mono text-xs truncate" title={status.state_root}>
            0x{status.state_root.slice(0, 16)}...
          </p>
        </div>
      </div>
    </div>
  );
}
