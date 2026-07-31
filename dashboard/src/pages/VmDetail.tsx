import { useEffect, useState, useCallback } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Monitor, ArrowLeft, Play, Square, RotateCcw, CheckCircle, XCircle, Loader2, Camera, HardDrive } from 'lucide-react';
import { useVms } from '../hooks/useVms';
import { apiJson } from '../lib/auth';

interface Snapshot {
  name: string;
  created_at: string;
  size_bytes: number;
}

function formatBytes(n: number): string {
  return `${(n / 1024 / 1024 / 1024).toFixed(2)} GB`;
}

export function VmDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { vm, loading, error, fetchVm } = useVms();
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [actionMsg, setActionMsg] = useState<{ type: 'success' | 'error'; text: string } | null>(null);
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);

  const loadSnapshots = useCallback(async () => {
    if (!id) return;
    try {
      setSnapshots(await apiJson<Snapshot[]>(`/vms/${id}/snapshots`));
    } catch {
      setSnapshots([]);
    }
  }, [id]);

  useEffect(() => {
    if (id) {
      fetchVm(id);
      void loadSnapshots();
    }
  }, [id, fetchVm, loadSnapshots]);

  const doAction = useCallback(
    async (action: string) => {
      if (!id) return;
      setActionLoading(action);
      setActionMsg(null);
      try {
        const data = await apiJson<{ message?: string; name?: string }>(`/vms/${id}/${action}`, {
          method: 'POST',
        });
        setActionMsg({
          type: 'success',
          text: data.message ?? (data.name ? `Snapshot ${data.name} created` : `VM ${action} successful`),
        });
        fetchVm(id);
        if (action === 'snapshot') await loadSnapshots();
      } catch (e) {
        setActionMsg({ type: 'error', text: e instanceof Error ? e.message : 'Action failed' });
      } finally {
        setActionLoading(null);
      }
    },
    [id, fetchVm, loadSnapshots],
  );

  const statusColors: Record<string, string> = {
    running: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400',
    stopped: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
    paused: 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400',
  };

  if (loading && !vm) {
    return (
      <div className="space-y-6">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Virtual Machine Details</h1>
        <p className="text-slate-400">Loading virtual machine...</p>
      </div>
    );
  }

  if ((error && !vm) || !vm) {
    return (
      <div className="space-y-6">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Virtual Machine Details</h1>
        <p className="text-red-500">{error || 'Virtual machine not found'}</p>
      </div>
    );
  }

  const isRunning = vm.status === 'running';
  const statusColor =
    statusColors[vm.status.toLowerCase()] ?? 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-400';

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link
          to="/vms"
          className="flex items-center gap-2 rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-50 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-800"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to VMs
        </Link>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
        <div className="border-b border-slate-200 px-6 py-4 dark:border-slate-800">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="flex items-center gap-3">
              <Monitor className="h-6 w-6 text-indigo-500" />
              <h1 className="text-xl font-bold text-slate-900 dark:text-white">{vm.name}</h1>
              <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${statusColor}`}>
                {vm.status}
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-2">
              {!isRunning && (
                <button
                  onClick={() => doAction('start')}
                  disabled={actionLoading !== null}
                  className="flex items-center gap-1.5 rounded-lg bg-emerald-500 px-3 py-1.5 text-xs font-medium text-white hover:bg-emerald-600 disabled:opacity-50"
                >
                  {actionLoading === 'start' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Play className="h-3.5 w-3.5" />}
                  Start
                </button>
              )}
              {isRunning && (
                <button
                  onClick={() => doAction('stop')}
                  disabled={actionLoading !== null}
                  className="flex items-center gap-1.5 rounded-lg bg-red-500 px-3 py-1.5 text-xs font-medium text-white hover:bg-red-600 disabled:opacity-50"
                >
                  {actionLoading === 'stop' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Square className="h-3.5 w-3.5" />}
                  Stop
                </button>
              )}
              <button
                onClick={() => doAction('restart')}
                disabled={actionLoading !== null}
                className="flex items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 disabled:opacity-50"
              >
                {actionLoading === 'restart' ? (
                  <Loader2 className="h-3.5 w-3.5 animate-spin" />
                ) : (
                  <RotateCcw className="h-3.5 w-3.5" />
                )}
                Restart
              </button>
              <button
                onClick={() => doAction('snapshot')}
                disabled={actionLoading !== null}
                className="flex items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 disabled:opacity-50"
              >
                {actionLoading === 'snapshot' ? (
                  <Loader2 className="h-3.5 w-3.5 animate-spin" />
                ) : (
                  <Camera className="h-3.5 w-3.5" />
                )}
                Snapshot
              </button>
              <button
                onClick={() => doAction('backup')}
                disabled={actionLoading !== null}
                className="flex items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 disabled:opacity-50"
              >
                {actionLoading === 'backup' ? (
                  <Loader2 className="h-3.5 w-3.5 animate-spin" />
                ) : (
                  <HardDrive className="h-3.5 w-3.5" />
                )}
                Backup
              </button>
            </div>
          </div>
        </div>

        {actionMsg && (
          <div
            className={`mx-6 mt-4 flex items-center gap-2 rounded-lg p-3 text-sm ${
              actionMsg.type === 'success'
                ? 'bg-emerald-50 text-emerald-700 dark:bg-emerald-900/20 dark:text-emerald-400'
                : 'bg-red-50 text-red-700 dark:bg-red-900/20 dark:text-red-400'
            }`}
          >
            {actionMsg.type === 'success' ? <CheckCircle className="h-4 w-4" /> : <XCircle className="h-4 w-4" />}
            {actionMsg.text}
          </div>
        )}

        <div className="p-6">
          <dl className="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <div>
              <dt className="text-sm font-medium text-slate-500">OS</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{vm.os_type}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500">CPU Cores</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{vm.cpu_cores}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500">RAM</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{(vm.ram_mb / 1024).toFixed(1)} GB</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500">Disk</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{vm.disk_gb} GB</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500">IP Address</dt>
              <dd className="mt-1 font-mono text-sm text-slate-900 dark:text-white">{vm.ip_address ?? '—'}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500">Status</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{vm.status}</dd>
            </div>
          </dl>
        </div>
      </div>

      <section className="space-y-3">
        <h2 className="text-lg font-semibold text-slate-900 dark:text-white">Snapshots</h2>
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Size</th>
                <th className="px-4 py-3 font-medium">Created</th>
              </tr>
            </thead>
            <tbody>
              {snapshots.map((s) => (
                <tr key={s.name} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 font-mono text-xs">{s.name}</td>
                  <td className="px-4 py-3">{formatBytes(s.size_bytes)}</td>
                  <td className="px-4 py-3 text-slate-500">{new Date(s.created_at).toLocaleString()}</td>
                </tr>
              ))}
              {snapshots.length === 0 && (
                <tr>
                  <td colSpan={3} className="px-4 py-8 text-center text-slate-400">
                    No snapshots yet — click Snapshot above
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
