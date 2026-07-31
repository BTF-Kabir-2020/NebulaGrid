import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface CpuChartProps {
  data: Array<{ time: string; value: number }>;
  loading?: boolean;
}

export function CpuChart({ data, loading }: CpuChartProps) {
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
        No CPU data available.
      </div>
    );
  }
  return (
    <ResponsiveContainer width="100%" height="100%">
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
        <XAxis dataKey="time" tick={{ fontSize: 12 }} stroke="#94a3b8" />
        <YAxis domain={[0, 100]} unit="%" tick={{ fontSize: 12 }} stroke="#94a3b8" />
        <Tooltip />
        <Line type="monotone" dataKey="value" stroke="#3b82f6" strokeWidth={2} dot={false} />
      </LineChart>
    </ResponsiveContainer>
  );
}
