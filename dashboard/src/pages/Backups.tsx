import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Backup {
  id: string;
  name: string;
  backup_type: string;
  status: string;
  size_bytes: number;
  created_at: string;
  restored_at?: string | null;
}

function formatBytes(n: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let v = n;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) {
    v /= 1024;
    i += 1;
  }
  return `${v.toFixed(1)} ${units[i]}`;
}

export function BackupsPage() {
  const [backups, setBackups] = useState<Backup[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [saving, setSaving] = useState(false);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [form, setForm] = useState({ name: '', backup_type: 'full' });

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setBackups(await apiJson<Backup[]>('/backups'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load backups');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const create = async () => {
    setSaving(true);
    setError(null);
    try {
      await apiJson('/backups', {
        method: 'POST',
        body: JSON.stringify({
          name: form.name.trim() || null,
          backup_type: form.backup_type,
        }),
      });
      setOpen(false);
      setForm({ name: '', backup_type: 'full' });
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Create failed');
    } finally {
      setSaving(false);
    }
  };

  const restore = async (id: string) => {
    setBusyId(id);
    try {
      await apiJson(`/backups/${id}/restore`, { method: 'POST' });
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Restore failed');
    } finally {
      setBusyId(null);
    }
  };

  const remove = async (id: string) => {
    if (!confirm('Delete this backup?')) return;
    setBusyId(`del-${id}`);
    try {
      await apiJson(`/backups/${id}`, { method: 'DELETE' });
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Delete failed');
    } finally {
      setBusyId(null);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Backups</h1>
          <p className="mt-1 text-sm text-slate-500">Control-plane backup and restore</p>
        </div>
        <Button onClick={() => setOpen(true)}>Create backup</Button>
      </div>

      {loading && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Type</th>
                <th className="px-4 py-3 font-medium">Size</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium">Created</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {backups.map((b) => (
                <tr key={b.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{b.name}</td>
                  <td className="px-4 py-3">{b.backup_type}</td>
                  <td className="px-4 py-3">{formatBytes(b.size_bytes)}</td>
                  <td className="px-4 py-3">{b.status}</td>
                  <td className="px-4 py-3 text-slate-500">{new Date(b.created_at).toLocaleString()}</td>
                  <td className="px-4 py-3 text-right">
                    <div className="flex justify-end gap-2">
                      <Button size="sm" variant="secondary" loading={busyId === b.id} onClick={() => restore(b.id)}>
                        Restore
                      </Button>
                      <Button
                        size="sm"
                        variant="secondary"
                        loading={busyId === `del-${b.id}`}
                        onClick={() => void remove(b.id)}
                      >
                        Delete
                      </Button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title="Create backup">
        <div className="space-y-4">
          <Input
            label="Name (optional)"
            value={form.name}
            onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))}
            placeholder="manual-full"
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white"
              value={form.backup_type}
              onChange={(e) => setForm((f) => ({ ...f, backup_type: e.target.value }))}
            >
              <option value="full">full</option>
              <option value="config">config</option>
              <option value="incremental">incremental</option>
            </select>
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={create}>
              Create
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
