import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Job {
  id: string;
  name: string;
  job_type: string;
  status: string;
  target_nodes: string[];
  created_at: string;
  started_at?: string | null;
  completed_at?: string | null;
  error?: string | null;
  result?: { message?: string } | null;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

function statusClass(status: string): string {
  switch (status) {
    case 'completed':
      return 'bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300';
    case 'running':
      return 'bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300';
    case 'failed':
      return 'bg-rose-50 text-rose-700 dark:bg-rose-900/30 dark:text-rose-300';
    default:
      return 'bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-300';
  }
}

export function JobsPage() {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [creating, setCreating] = useState(false);
  const [form, setForm] = useState({
    name: '',
    job_type: 'playbook',
    target_nodes: 'compute-1.nebula.internal',
    playbook: 'health-check',
  });

  const fetchJobs = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setJobs(await apiJson<Job[]>('/jobs'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchJobs();
  }, [fetchJobs]);

  useEffect(() => {
    const hasActive = jobs.some((j) => j.status === 'pending' || j.status === 'running');
    if (!hasActive) return;
    const t = setInterval(() => {
      void fetchJobs();
    }, 1500);
    return () => clearInterval(t);
  }, [jobs, fetchJobs]);

  const createJob = async () => {
    setCreating(true);
    setError(null);
    try {
      const name = form.name.trim() || `manual-job-${Date.now()}`;
      await apiJson('/jobs', {
        method: 'POST',
        body: JSON.stringify({
          name,
          job_type: form.job_type,
          target_nodes: form.target_nodes
            .split(',')
            .map((s) => s.trim())
            .filter(Boolean),
          params: { playbook: form.playbook },
        }),
      });
      setOpen(false);
      setForm({
        name: '',
        job_type: 'playbook',
        target_nodes: 'compute-1.nebula.internal',
        playbook: 'health-check',
      });
      await fetchJobs();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Create failed');
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Jobs</h1>
          <p className="mt-1 text-sm text-slate-500">Automation playbooks and scheduled work</p>
        </div>
        <Button onClick={() => setOpen(true)}>Run Job</Button>
      </div>

      {loading && jobs.length === 0 && <p className="text-slate-400">Loading jobs...</p>}
      {error && <p className="text-red-500">Error: {error}</p>}

      <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
        <table className="w-full text-left text-sm">
          <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
            <tr>
              <th className="px-4 py-3 font-medium">Name</th>
              <th className="px-4 py-3 font-medium">Type</th>
              <th className="px-4 py-3 font-medium">Status</th>
              <th className="px-4 py-3 font-medium">Targets</th>
              <th className="px-4 py-3 font-medium">Created</th>
            </tr>
          </thead>
          <tbody>
            {jobs.map((j) => (
              <tr key={j.id} className="border-b border-slate-100 dark:border-slate-800">
                <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{j.name}</td>
                <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{j.job_type}</td>
                <td className="px-4 py-3">
                  <span className={`rounded-full px-2 py-0.5 text-xs ${statusClass(j.status)}`}>
                    {j.status}
                  </span>
                </td>
                <td className="px-4 py-3 text-slate-600 dark:text-slate-300">
                  {j.target_nodes.join(', ') || '—'}
                </td>
                <td className="px-4 py-3 text-slate-500">{new Date(j.created_at).toLocaleString()}</td>
              </tr>
            ))}
            {jobs.length === 0 && !loading && (
              <tr>
                <td colSpan={5} className="px-4 py-8 text-center text-slate-400">
                  No jobs yet
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      <Modal open={open} onClose={() => setOpen(false)} title="Run job">
        <div className="space-y-4">
          <Input
            label="Name"
            value={form.name}
            onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))}
            placeholder="health-check-manual"
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className={fieldClass}
              value={form.job_type}
              onChange={(e) => setForm((f) => ({ ...f, job_type: e.target.value }))}
            >
              <option value="playbook">playbook</option>
              <option value="script">script</option>
              <option value="backup">backup</option>
            </select>
          </div>
          <Input
            label="Target nodes (comma-separated)"
            value={form.target_nodes}
            onChange={(e) => setForm((f) => ({ ...f, target_nodes: e.target.value }))}
          />
          <Input
            label="Playbook / params"
            value={form.playbook}
            onChange={(e) => setForm((f) => ({ ...f, playbook: e.target.value }))}
          />
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={creating} onClick={createJob}>
              Run
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
