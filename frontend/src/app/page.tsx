import { Dashboard } from '@/components/Dashboard';

export default function Home() {
  return (
    <main className="container mx-auto py-8 px-4">
      <div className="mb-8">
        <h1 className="text-4xl font-bold tracking-tight">zkCredit L2</h1>
        <p className="text-lg text-muted-foreground mt-2">
          100 credit operations. 1 ZK proof. 1 transaction. 100x cheaper.
        </p>
        <p className="text-sm text-muted-foreground mt-1">
          ZK Validity Rollup for Private Credit on Creditcoin
        </p>
      </div>
      <Dashboard />
    </main>
  );
}
