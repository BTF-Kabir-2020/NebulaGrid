import { useEffect, useState, useCallback } from 'react';
import { useParams, Link } from 'react-router-dom';
import { Container as ContainerIcon, ArrowLeft, Play, Square, RotateCcw, Trash2, CheckCircle, XCircle, Loader2 } from 'lucide-react';
import type { ContainerPort } from '../types/container';
import { useContainers } from '../hooks/useContainers';

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

function formatPort(p: string | ContainerPort): string {
  if (typeof p === 'string') return p;
  return p.public_port ? `${p.ip}:${p.public_port} → ${p.private_port}` : `${p.private_port}/${p.ip}`;
}

export function ContainerDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { container, loading, error, fetchContainer } = useContainers();
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [actionMsg, setActionMsg] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  useEffect(() => {
    if (id) fetchContainer(id);
  }, [id, fetchContainer]);

  const doAction = useCallback(async (action: string) => {
    if (!id) return;
    setActionLoading(action);
    setActionMsg(null);
    try {
      const res = await fetch(`/api/containers/${id}/${action}`, { method: 'POST', headers: authHeaders() });
      if (res.ok) {
        setActionMsg({ type: 'success', text: `Container ${action} successful` });
        fetchContainer(id);
      } else {
        const err = await res.text();
        setActionMsg({ type: 'error', text: `Failed: ${err}` });
      }
    } catch {
      setActionMsg({ type: 'error', text: 'Network error' });
    } finally {
      setActionLoading(null);
    }
  }, [id, fetchContainer]);

  const doDelete = useCallback(async () => {
    if (!id) return;
    setActionLoading('delete');
    setActionMsg(null);
    try {
      const res = await fetch(`/api/containers/${id}`, { method: 'DELETE', headers: authHeaders() });
      if (res.ok) {
        setActionMsg({ type: 'success', text: 'Container removed' });
      } else {
        const err = await res.text();
        setActionMsg({ type: 'error', text: `Failed: ${err}` });
      }
    } catch {
      setActionMsg({ type: 'error', text: 'Network error' });
    } finally {
      setActionLoading(null);
    }
  }, [id]);

  const statusColors: Record<string, string> = {
    running: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400',
    stopped: 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400',
    paused: 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400',
  };

  if (loading && !container) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Container Details</h1>
        </div>
        <div className="rounded-xl border border-slate-200 bg-white p-12 dark:border-slate-800 dark:bg-slate-900">
          <div className="flex flex-col items-center justify-center text-slate-400 dark:text-slate-500">
            <ContainerIcon className="mb-4 h-12 w-12 animate-pulse" />
            <p className="text-lg font-medium">Loading container...</p>
          </div>
        </div>
      </div>
    );
  }

  if (error && !container) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Container Details</h1>
        </div>
        <div className="rounded-xl border border-red-200 bg-red-50 p-12 dark:border-red-800 dark:bg-red-900/20">
          <div className="flex flex-col items-center justify-center text-red-500">
            <ContainerIcon className="mb-4 h-12 w-12" />
            <p className="text-lg font-medium">Error loading container</p>
            <p className="mt-1 text-sm">{error}</p>
          </div>
        </div>
      </div>
    );
  }

  if (!container) {
    return (
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Container Details</h1>
        </div>
        <div className="rounded-xl border border-slate-200 bg-white p-12 dark:border-slate-800 dark:bg-slate-900">
          <div className="flex flex-col items-center justify-center text-slate-400 dark:text-slate-500">
            <ContainerIcon className="mb-4 h-12 w-12" />
            <p className="text-lg font-medium">Container not found</p>
          </div>
        </div>
      </div>
    );
  }

  const isRunning = container.status === 'running';
  const statusColor = statusColors[container.status.toLowerCase()] ?? 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-400';

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link
          to="/containers"
          className="flex items-center gap-2 rounded-lg border border-slate-200 px-4 py-2 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-50 dark:border-slate-700 dark:text-slate-400 dark:hover:bg-slate-800"
        >
          <ArrowLeft className="h-4 w-4" />
          Back to Containers
        </Link>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
        <div className="border-b border-slate-200 px-6 py-4 dark:border-slate-800">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <ContainerIcon className="h-6 w-6 text-indigo-500" />
              <h1 className="text-xl font-bold text-slate-900 dark:text-white">{container.name}</h1>
              <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${statusColor}`}>
                {container.status}
              </span>
            </div>
            <div className="flex items-center gap-2">
              {!isRunning && (
                <button
                  onClick={() => doAction('start')}
                  disabled={actionLoading !== null}
                  className="flex items-center gap-1.5 rounded-lg bg-emerald-500 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-emerald-600 disabled:opacity-50"
                >
                  {actionLoading === 'start' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Play className="h-3.5 w-3.5" />}
                  Start
                </button>
              )}
              {isRunning && (
                <button
                  onClick={() => doAction('stop')}
                  disabled={actionLoading !== null}
                  className="flex items-center gap-1.5 rounded-lg bg-red-500 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-red-600 disabled:opacity-50"
                >
                  {actionLoading === 'stop' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Square className="h-3.5 w-3.5" />}
                  Stop
                </button>
              )}
              <button
                onClick={() => doAction('restart')}
                disabled={actionLoading !== null}
                className="flex items-center gap-1.5 rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 transition-colors hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 dark:hover:bg-slate-800 disabled:opacity-50"
              >
                {actionLoading === 'restart' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <RotateCcw className="h-3.5 w-3.5" />}
                Restart
              </button>
              <button
                onClick={doDelete}
                disabled={actionLoading !== null}
                className="flex items-center gap-1.5 rounded-lg bg-red-500 px-3 py-1.5 text-xs font-medium text-white transition-colors hover:bg-red-600 disabled:opacity-50"
              >
                {actionLoading === 'delete' ? <Loader2 className="h-3.5 w-3.5 animate-spin" /> : <Trash2 className="h-3.5 w-3.5" />}
                Delete
              </button>
            </div>
          </div>
        </div>

        {actionMsg && (
          <div className={`mx-6 mt-4 flex items-center gap-2 rounded-lg p-3 text-sm ${
            actionMsg.type === 'success' ? 'bg-emerald-50 text-emerald-700 dark:bg-emerald-900/20 dark:text-emerald-400' : 'bg-red-50 text-red-700 dark:bg-red-900/20 dark:text-red-400'
          }`}>
            {actionMsg.type === 'success' ? <CheckCircle className="h-4 w-4" /> : <XCircle className="h-4 w-4" />}
            {actionMsg.text}
          </div>
        )}

        <div className="p-6">
          <dl className="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Container ID</dt>
              <dd className="mt-1 font-mono text-sm text-slate-900 dark:text-white">{container.container_id}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Image</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{container.image}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Status</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{container.status}</dd>
            </div>
            <div>
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Created</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">{container.created_at ? new Date(container.created_at).toLocaleString() : '-'}</dd>
            </div>
            <div className="sm:col-span-2">
              <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">Ports</dt>
              <dd className="mt-1 text-sm text-slate-900 dark:text-white">
                {container.ports && container.ports.length > 0 ? (
                  <div className="flex flex-wrap gap-2">
                    {container.ports.map((p, i) => (
                      <span key={i} className="inline-flex items-center rounded-md bg-slate-100 px-2 py-1 font-mono text-xs dark:bg-slate-800">
                        {formatPort(p)}
                      </span>
                    ))}
                  </div>
                ) : (
                  <span className="text-slate-400">No ports exposed</span>
                )}
              </dd>
            </div>
          </dl>
        </div>
      </div>
    </div>
  );
}
