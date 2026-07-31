import { useCallback, useEffect, useState } from 'react';
import { Button } from '../components/common';
import { apiJson } from '../lib/auth';

interface ContainerOption {
  id: string;
  name: string;
  image?: string;
}

interface LogsResponse {
  logs: string;
}

export function LogsPage() {
  const [containers, setContainers] = useState<ContainerOption[]>([]);
  const [selected, setSelected] = useState('');
  const [logs, setLogs] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    void apiJson<ContainerOption[]>('/containers')
      .then((list) => {
        setContainers(list);
        if (list[0]) setSelected(list[0].id);
      })
      .catch((e) => setError(e instanceof Error ? e.message : 'Failed to list containers'));
  }, []);

  const fetchLogs = useCallback(async () => {
    if (!selected) return;
    setLoading(true);
    setError(null);
    try {
      const data = await apiJson<LogsResponse>(`/containers/${selected}/logs`);
      setLogs(data.logs ?? '');
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load logs');
    } finally {
      setLoading(false);
    }
  }, [selected]);

  useEffect(() => {
    void fetchLogs();
  }, [fetchLogs]);

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Logs</h1>
          <p className="mt-1 text-sm text-slate-500">
            Per-container runtime logs (labs: mock streams that change by container)
          </p>
        </div>
        <div className="flex items-center gap-2">
          <select
            className="rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white"
            value={selected}
            onChange={(e) => setSelected(e.target.value)}
          >
            {containers.map((c) => (
              <option key={c.id} value={c.id}>
                {c.name}
                {c.image ? ` (${c.image})` : ''}
              </option>
            ))}
          </select>
          <Button variant="secondary" loading={loading} onClick={() => void fetchLogs()}>
            Refresh
          </Button>
        </div>
      </div>

      {error && <p className="text-red-500">{error}</p>}

      <pre className="max-h-[70vh] overflow-auto rounded-xl border border-slate-200 bg-slate-950 p-4 text-xs leading-relaxed text-emerald-300 dark:border-slate-800">
        {logs || (loading ? 'Loading…' : 'No logs')}
      </pre>
    </div>
  );
}
