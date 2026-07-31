import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Legend } from 'recharts';

interface NetworkChartProps {
  data: Array<{ time: string; rx: number; tx: number }>;
  loading?: boolean;
}

function toMbps(bytes: number): number {
  return (bytes * 8) / (1024 * 1024);
}

export function NetworkChart({ data, loading }: NetworkChartProps) {
  if (loading) {
    return (
      <div className="flex h-full items-center justify-center text-slate-400 dark:text-slate-500 text-sm">
        Loading...
      </div>
    );
  }
  if (!data.length) {
    return (
      <div className="flex h-full items-center justify-center text-slate-400 dark:text-slate-500 text-sm">
        No network data available.
      </div>
    );
  }
  return (
    <ResponsiveContainer width="100%" height="100%">
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
        <XAxis dataKey="time" tick={{ fontSize: 12 }} stroke="#94a3b8" />
        <YAxis unit="Mbps" tick={{ fontSize: 12 }} stroke="#94a3b8" tickFormatter={(v: number) => toMbps(v).toFixed(1)} />
        <Tooltip formatter={(value: number) => [`${toMbps(value).toFixed(2)} Mbps`]} />
        <Legend />
        <Line type="monotone" dataKey="rx" stroke="#10b981" strokeWidth={2} dot={false} name="RX" />
        <Line type="monotone" dataKey="tx" stroke="#3b82f6" strokeWidth={2} dot={false} name="TX" />
      </LineChart>
    </ResponsiveContainer>
  );
}
