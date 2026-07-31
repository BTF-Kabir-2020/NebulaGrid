import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface RamChartProps {
  data: Array<{ time: string; value: number }>;
  total?: number;
  loading?: boolean;
}

function toGb(bytes: number): number {
  return bytes / (1024 * 1024 * 1024);
}

export function RamChart({ data, total: _total, loading }: RamChartProps) {
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
        No RAM data available.
      </div>
    );
  }
  return (
    <ResponsiveContainer width="100%" height="100%">
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
        <XAxis dataKey="time" tick={{ fontSize: 12 }} stroke="#94a3b8" />
        <YAxis unit="GB" tick={{ fontSize: 12 }} stroke="#94a3b8" tickFormatter={(v: number) => toGb(v).toFixed(1)} />
        <Tooltip formatter={(value: number) => [`${toGb(value).toFixed(2)} GB`, 'RAM']} />
        <Line type="monotone" dataKey="value" stroke="#10b981" strokeWidth={2} dot={false} />
      </LineChart>
    </ResponsiveContainer>
  );
}
