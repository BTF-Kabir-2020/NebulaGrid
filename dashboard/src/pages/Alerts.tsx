import { useEffect, useState } from 'react';
import { useMetrics } from '../hooks/useMetrics';
import { Badge } from '../components/common/Badge';
import { API_URL } from '../lib/api-url';

const severityVariant: Record<string, 'success' | 'warning' | 'danger' | 'default'> = {
  info: 'success',
  warning: 'warning',
  critical: 'danger',
};

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token
    ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' }
    : { 'Content-Type': 'application/json' };
}

export function AlertsPage() {
  const { alerts, loading, error, fetchAlerts } = useMetrics();
  const [acking, setAcking] = useState<string | null>(null);

  useEffect(() => {
    fetchAlerts();
  }, [fetchAlerts]);

  async function acknowledge(id: string) {
    setAcking(id);
    try {
      const res = await fetch(`${API_URL}/monitoring/alerts/${id}/ack`, {
        method: 'POST',
        headers: authHeaders(),
      });
      if (!res.ok) throw new Error(`Ack failed (${res.status})`);
      await fetchAlerts();
    } finally {
      setAcking(null);
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Alerts</h1>
      </div>

      {loading && alerts.length === 0 && (
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <p className="text-slate-400">Loading alerts...</p>
        </div>
      )}

      {error && (
        <div className="rounded-xl border border-red-200 bg-red-50 p-6 dark:border-red-800 dark:bg-red-900/20">
          <p className="text-red-600 dark:text-red-400">Error: {error}</p>
        </div>
      )}

      {!loading && !error && alerts.length === 0 && (
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <p className="text-lg font-medium text-slate-400 dark:text-slate-500">No alerts</p>
          <p className="mt-1 text-sm text-slate-400">All systems operating normally.</p>
        </div>
      )}

      {alerts.length > 0 && (
        <div className="space-y-3">
          {alerts.map((alert) => (
            <div
              key={alert.id}
              className={`rounded-xl border bg-white p-5 dark:bg-slate-900 ${
                alert.acknowledged
                  ? 'border-slate-200 dark:border-slate-800'
                  : alert.severity === 'critical'
                    ? 'border-red-200 dark:border-red-800'
                    : 'border-amber-200 dark:border-amber-800'
              }`}
            >
              <div className="flex items-start justify-between gap-4">
                <div className="flex-1">
                  <div className="flex items-center gap-2">
                    <h3 className="font-semibold text-slate-900 dark:text-white">{alert.title}</h3>
                    <Badge variant={severityVariant[alert.severity] || 'default'}>{alert.severity}</Badge>
                    {alert.acknowledged && (
                      <span className="text-xs text-slate-400 dark:text-slate-500">(acknowledged)</span>
                    )}
                  </div>
                  <p className="mt-1 text-sm text-slate-600 dark:text-slate-400">{alert.message}</p>
                  <div className="mt-2 flex items-center gap-3 text-xs text-slate-400">
                    <span>Source: {alert.source}</span>
                    <span>{new Date(alert.created_at).toLocaleString()}</span>
                  </div>
                </div>
                {!alert.acknowledged && (
                  <button
                    type="button"
                    onClick={() => acknowledge(alert.id)}
                    disabled={acking === alert.id}
                    className="shrink-0 rounded-lg bg-slate-900 px-3 py-1.5 text-sm font-medium text-white hover:bg-slate-800 disabled:opacity-50 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-white"
                  >
                    {acking === alert.id ? 'Acking…' : 'Acknowledge'}
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
