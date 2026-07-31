import { useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Server, ArrowLeft } from 'lucide-react';
import { useNodes } from '../hooks/useNodes';
import { Badge } from '../components/common/Badge';

const statusVariant: Record<string, 'success' | 'warning' | 'danger' | 'default'> = {
  online: 'success',
  warning: 'warning',
  error: 'danger',
  offline: 'default',
};

export function ServerDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { node, nodeMetrics, loading, error, fetchNode, fetchNodeMetrics } = useNodes();

  useEffect(() => {
    if (id) {
      fetchNode(id);
      fetchNodeMetrics(id);
    }
  }, [id, fetchNode, fetchNodeMetrics]);

  if (loading && !node) {
    return (
      <div className="space-y-6">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Server Details</h1>
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <Server className="mx-auto mb-4 h-12 w-12 animate-pulse text-slate-400" />
          <p className="text-slate-400">Loading server...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="space-y-6">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Server Details</h1>
        <div className="rounded-xl border border-red-200 bg-red-50 p-12 text-center dark:border-red-800 dark:bg-red-900/20">
          <p className="text-red-500">Error: {error}</p>
        </div>
      </div>
    );
  }

  if (!node) {
    return (
      <div className="space-y-6">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Server Details</h1>
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <Server className="mx-auto mb-4 h-12 w-12 text-slate-400" />
          <p className="text-slate-400">Server not found</p>
        </div>
      </div>
    );
  }

  const latest = nodeMetrics && nodeMetrics.length > 0 ? nodeMetrics[nodeMetrics.length - 1] : null;

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link
          to="/servers"
          className="flex items-center gap-2 rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-50 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-800"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Servers
        </Link>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
        <div className="border-b border-slate-200 px-6 py-4 dark:border-slate-800">
          <div className="flex items-center gap-3">
            <Server className="h-6 w-6 text-indigo-500" />
            <h1 className="text-xl font-bold text-slate-900 dark:text-white">{node.hostname}</h1>
            <Badge variant={statusVariant[node.status] || 'default'}>{node.status}</Badge>
          </div>
        </div>

        <div className="p-6">
          <dl className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">IP Address</dt>
              <dd className="mt-1 font-mono text-sm text-slate-900 dark:text-white">{node.ip_address}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">OS</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{node.os_name}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Status</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{node.status}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">CPU Cores</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{node.cpu_cores}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">RAM</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">
                {node.ram_total_bytes ? `${(node.ram_total_bytes / 1073741824).toFixed(1)} GB` : '—'}
              </dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Disk</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">
                {node.disk_total_bytes ? `${(node.disk_total_bytes / 1073741824).toFixed(0)} GB` : '—'}
              </dd>
            </div>
          </dl>
        </div>
      </div>

      {latest && (
        <div className="grid gap-6 sm:grid-cols-3">
          <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
            <p className="text-sm text-slate-500 dark:text-slate-400">CPU</p>
            <p className="mt-1 text-3xl font-bold text-blue-500">{latest.cpu_percent.toFixed(1)}%</p>
            <div className="mt-2 h-2 rounded-full bg-slate-200 dark:bg-slate-700">
              <div className="h-2 rounded-full bg-blue-500" style={{ width: `${latest.cpu_percent}%` }} />
            </div>
          </div>
          <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
            <p className="text-sm text-slate-500 dark:text-slate-400">RAM</p>
            <p className="mt-1 text-3xl font-bold text-emerald-500">{latest.ram_percent.toFixed(1)}%</p>
            <div className="mt-2 h-2 rounded-full bg-slate-200 dark:bg-slate-700">
              <div className="h-2 rounded-full bg-emerald-500" style={{ width: `${latest.ram_percent}%` }} />
            </div>
          </div>
          <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
            <p className="text-sm text-slate-500 dark:text-slate-400">Disk</p>
            <p className="mt-1 text-3xl font-bold text-violet-500">{latest.disk_percent.toFixed(1)}%</p>
            <div className="mt-2 h-2 rounded-full bg-slate-200 dark:bg-slate-700">
              <div className="h-2 rounded-full bg-violet-500" style={{ width: `${latest.disk_percent}%` }} />
            </div>
          </div>
        </div>
      )}

      {nodeMetrics && nodeMetrics.length > 1 && (
        <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
          <h2 className="mb-4 text-lg font-semibold text-slate-900 dark:text-white">Recent Metrics</h2>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="px-3 py-2 text-left text-slate-500 dark:text-slate-400">Time</th>
                  <th className="px-3 py-2 text-left text-slate-500 dark:text-slate-400">CPU</th>
                  <th className="px-3 py-2 text-left text-slate-500 dark:text-slate-400">RAM</th>
                  <th className="px-3 py-2 text-left text-slate-500 dark:text-slate-400">Disk</th>
                  <th className="px-3 py-2 text-left text-slate-500 dark:text-slate-400">Network</th>
                </tr>
              </thead>
              <tbody>
                {[...nodeMetrics].reverse().map((m, i) => (
                  <tr key={i} className="border-b border-slate-100 dark:border-slate-800">
                    <td className="px-3 py-2 text-slate-600 dark:text-slate-400">{new Date(m.collected_at).toLocaleTimeString()}</td>
                    <td className="px-3 py-2 text-slate-900 dark:text-white">{m.cpu_percent.toFixed(1)}%</td>
                    <td className="px-3 py-2 text-slate-900 dark:text-white">{m.ram_percent.toFixed(1)}%</td>
                    <td className="px-3 py-2 text-slate-900 dark:text-white">{m.disk_percent.toFixed(1)}%</td>
                    <td className="px-3 py-2 text-slate-900 dark:text-white">
                      RX: {(m.net_rx_bytes / 1048576).toFixed(1)}MB / TX: {(m.net_tx_bytes / 1048576).toFixed(1)}MB
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
}
