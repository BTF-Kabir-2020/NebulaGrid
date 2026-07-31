import { useCallback, useEffect, useState } from 'react';
import { Button } from '../components/common';
import { apiJson } from '../lib/auth';

interface AuditEvent {
  id: string;
  actor: string;
  action: string;
  resource: string;
  details: string;
  created_at: string;
}

interface Overview {
  total_nodes: number;
  online_nodes: number;
  total_containers: number;
  total_vms: number;
  active_alerts: number;
  avg_cpu_percent: number;
  avg_ram_percent: number;
}

interface HistoryPoint {
  collected_at: string;
  cpu_percent: number;
  ram_percent: number;
  disk_percent: number;
}

export function MonitoringPage() {
  const [overview, setOverview] = useState<Overview | null>(null);
  const [history, setHistory] = useState<HistoryPoint[]>([]);
  const [audit, setAudit] = useState<AuditEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [ov, nodes, events] = await Promise.all([
        apiJson<Overview>('/monitoring/overview'),
        apiJson<Array<{ id: string }>>('/nodes'),
        apiJson<AuditEvent[]>('/audit'),
      ]);
      setOverview(ov);
      setAudit(events);
      if (nodes[0]?.id) {
        const metrics = await apiJson<
          Array<{
            collected_at: string;
            cpu_percent: number;
            ram_percent: number;
            disk_percent: number;
          }>
        >(`/nodes/${nodes[0].id}/metrics`);
        setHistory(
          (Array.isArray(metrics) ? metrics : []).slice(-20).map((m) => ({
            collected_at: m.collected_at,
            cpu_percent: m.cpu_percent,
            ram_percent: m.ram_percent,
            disk_percent: m.disk_percent,
          })),
        );
      }
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load monitoring');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
    const t = setInterval(() => void load(), 15000);
    return () => clearInterval(t);
  }, [load]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Monitoring</h1>
          <p className="mt-1 text-sm text-slate-500">Cluster health, metrics history, and audit trail</p>
        </div>
        <Button variant="secondary" onClick={() => void load()}>
          Refresh
        </Button>
      </div>

      {loading && !overview && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {overview && (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          {[
            ['Online nodes', `${overview.online_nodes}/${overview.total_nodes}`],
            ['Containers', String(overview.total_containers)],
            ['VMs', String(overview.total_vms)],
            ['Active alerts', String(overview.active_alerts)],
            ['Avg CPU', `${overview.avg_cpu_percent.toFixed(1)}%`],
            ['Avg RAM', `${overview.avg_ram_percent.toFixed(1)}%`],
          ].map(([label, value]) => (
            <div
              key={label}
              className="rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900"
            >
              <p className="text-xs text-slate-500">{label}</p>
              <p className="mt-1 text-2xl font-semibold text-slate-900 dark:text-white">{value}</p>
            </div>
          ))}
        </div>
      )}

      <section className="space-y-3">
        <h2 className="text-lg font-semibold text-slate-900 dark:text-white">Metrics history</h2>
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Time</th>
                <th className="px-4 py-3 font-medium">CPU %</th>
                <th className="px-4 py-3 font-medium">RAM %</th>
                <th className="px-4 py-3 font-medium">Disk %</th>
              </tr>
            </thead>
            <tbody>
              {history.map((h, i) => (
                <tr key={`${h.collected_at}-${i}`} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">
                    {new Date(h.collected_at).toLocaleString()}
                  </td>
                  <td className="px-4 py-3">{h.cpu_percent.toFixed(1)}</td>
                  <td className="px-4 py-3">{h.ram_percent.toFixed(1)}</td>
                  <td className="px-4 py-3">{h.disk_percent.toFixed(1)}</td>
                </tr>
              ))}
              {history.length === 0 && (
                <tr>
                  <td colSpan={4} className="px-4 py-8 text-center text-slate-400">
                    No metrics samples yet — agent heartbeats will fill this
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </section>

      <section className="space-y-3">
        <h2 className="text-lg font-semibold text-slate-900 dark:text-white">Audit log</h2>
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Time</th>
                <th className="px-4 py-3 font-medium">Actor</th>
                <th className="px-4 py-3 font-medium">Action</th>
                <th className="px-4 py-3 font-medium">Resource</th>
                <th className="px-4 py-3 font-medium">Details</th>
              </tr>
            </thead>
            <tbody>
              {audit.map((e) => (
                <tr key={e.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 text-slate-500">{new Date(e.created_at).toLocaleString()}</td>
                  <td className="px-4 py-3">{e.actor}</td>
                  <td className="px-4 py-3 font-mono text-xs">{e.action}</td>
                  <td className="px-4 py-3 font-mono text-xs">{e.resource}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{e.details}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
