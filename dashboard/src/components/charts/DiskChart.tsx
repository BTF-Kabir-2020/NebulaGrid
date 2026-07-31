import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface DiskChartProps {
  data: Array<{ time: string; used: number; total: number }>;
  loading?: boolean;
}

function toGb(bytes: number): number {
  return bytes / (1024 * 1024 * 1024);
}

export function DiskChart({ data, loading }: DiskChartProps) {
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
        No disk data available.
      </div>
    );
  }
  return (
    <ResponsiveContainer width="100%" height="100%">
      <AreaChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
        <XAxis dataKey="time" tick={{ fontSize: 12 }} stroke="#94a3b8" />
        <YAxis unit="GB" tick={{ fontSize: 12 }} stroke="#94a3b8" tickFormatter={(v: number) => toGb(v).toFixed(1)} />
        <Tooltip formatter={(value: number) => [`${toGb(value).toFixed(2)} GB`]} />
        <Area type="monotone" dataKey="total" stroke="#94a3b8" fill="#94a3b8" fillOpacity={0.1} dot={false} />
        <Area type="monotone" dataKey="used" stroke="#3b82f6" fill="#3b82f6" fillOpacity={0.2} dot={false} />
      </AreaChart>
    </ResponsiveContainer>
  );
}
