import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface ConfigEntry {
  id: string;
  key: string;
  value: unknown;
  group: string;
  description?: string | null;
  version: number;
  updated_at: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

function stringifyValue(v: unknown): string {
  if (typeof v === 'string') return v;
  return JSON.stringify(v);
}

function parseValue(raw: string): unknown {
  const trimmed = raw.trim();
  try {
    return JSON.parse(trimmed);
  } catch {
    return trimmed;
  }
}

export function ConfigPage() {
  const [entries, setEntries] = useState<ConfigEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<ConfigEntry | null>(null);
  const [saving, setSaving] = useState(false);
  const [form, setForm] = useState({ key: '', value: '', group: 'default', description: '' });

  const fetchEntries = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setEntries(await apiJson<ConfigEntry[]>('/config'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchEntries();
  }, [fetchEntries]);

  const openCreate = () => {
    setEditing(null);
    setForm({ key: '', value: '', group: 'default', description: '' });
    setOpen(true);
  };

  const openEdit = (entry: ConfigEntry) => {
    setEditing(entry);
    setForm({
      key: entry.key,
      value: stringifyValue(entry.value),
      group: entry.group,
      description: entry.description ?? '',
    });
    setOpen(true);
  };

  const save = async () => {
    if (!form.key.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/config', {
        method: 'POST',
        body: JSON.stringify({
          key: form.key.trim(),
          value: parseValue(form.value),
          group: form.group.trim() || 'default',
          description: form.description.trim() || null,
        }),
      });
      setOpen(false);
      await fetchEntries();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Save failed');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Configuration</h1>
          <p className="mt-1 text-sm text-slate-500">Cluster settings stored in the control plane</p>
        </div>
        <Button onClick={openCreate}>Add setting</Button>
      </div>

      {loading && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Key</th>
                <th className="px-4 py-3 font-medium">Value</th>
                <th className="px-4 py-3 font-medium">Group</th>
                <th className="px-4 py-3 font-medium">Version</th>
                <th className="px-4 py-3 font-medium">Updated</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {entries.map((e) => (
                <tr key={e.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3">
                    <div className="font-mono text-slate-900 dark:text-white">{e.key}</div>
                    {e.description && <div className="text-xs text-slate-500">{e.description}</div>}
                  </td>
                  <td className="px-4 py-3 font-mono text-slate-600 dark:text-slate-300">
                    {stringifyValue(e.value)}
                  </td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{e.group}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{e.version}</td>
                  <td className="px-4 py-3 text-slate-500">{new Date(e.updated_at).toLocaleString()}</td>
                  <td className="px-4 py-3 text-right">
                    <Button size="sm" variant="secondary" onClick={() => openEdit(e)}>
                      Edit
                    </Button>
                  </td>
                </tr>
              ))}
              {entries.length === 0 && (
                <tr>
                  <td colSpan={6} className="px-4 py-8 text-center text-slate-400">
                    No settings yet
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title={editing ? 'Edit setting' : 'Add setting'}>
        <div className="space-y-4">
          <Input
            label="Key"
            value={form.key}
            disabled={!!editing}
            onChange={(ev) => setForm((f) => ({ ...f, key: ev.target.value }))}
            placeholder="alerts.cpu_threshold"
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Value (JSON or text)</label>
            <textarea
              className={`${fieldClass} min-h-[80px] font-mono`}
              value={form.value}
              onChange={(ev) => setForm((f) => ({ ...f, value: ev.target.value }))}
              placeholder='90 or "enabled" or {"a":1}'
            />
          </div>
          <Input
            label="Group"
            value={form.group}
            onChange={(ev) => setForm((f) => ({ ...f, group: ev.target.value }))}
          />
          <Input
            label="Description"
            value={form.description}
            onChange={(ev) => setForm((f) => ({ ...f, description: ev.target.value }))}
          />
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={save}>
              Save
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
