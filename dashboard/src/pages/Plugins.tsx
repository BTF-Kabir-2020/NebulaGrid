import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Plugin {
  id: string;
  name: string;
  version: string;
  description: string;
  category: string;
  enabled: boolean;
  config: unknown;
  installed_at: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

const emptyForm = {
  name: '',
  version: '1.0.0',
  description: '',
  category: 'general',
  enabled: true,
};

export function PluginsPage() {
  const [plugins, setPlugins] = useState<Plugin[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [saving, setSaving] = useState(false);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [form, setForm] = useState(emptyForm);

  const load = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setPlugins(await apiJson<Plugin[]>('/plugins'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load plugins');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void load();
  }, [load]);

  const install = async () => {
    if (!form.name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/plugins', {
        method: 'POST',
        body: JSON.stringify({
          name: form.name.trim(),
          version: form.version.trim() || '1.0.0',
          description: form.description.trim(),
          category: form.category,
          enabled: form.enabled,
          config: {},
        }),
      });
      setOpen(false);
      setForm(emptyForm);
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Install failed');
    } finally {
      setSaving(false);
    }
  };

  const toggle = async (p: Plugin) => {
    setBusyId(p.id);
    try {
      await apiJson(`/plugins/${p.id}`, {
        method: 'PUT',
        body: JSON.stringify({ enabled: !p.enabled }),
      });
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Update failed');
    } finally {
      setBusyId(null);
    }
  };

  const uninstall = async (id: string) => {
    setBusyId(id);
    try {
      await apiJson(`/plugins/${id}`, { method: 'DELETE' });
      await load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Uninstall failed');
    } finally {
      setBusyId(null);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Plugins</h1>
          <p className="mt-1 text-sm text-slate-500">
            Extensibility hooks — enable/disable integrations for labs and future production adapters
          </p>
        </div>
        <Button onClick={() => setOpen(true)}>Install plugin</Button>
      </div>

      {loading && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Version</th>
                <th className="px-4 py-3 font-medium">Category</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium">Installed</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {plugins.map((p) => (
                <tr key={p.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3">
                    <div className="font-medium text-slate-900 dark:text-white">{p.name}</div>
                    <div className="text-xs text-slate-500">{p.description}</div>
                  </td>
                  <td className="px-4 py-3 font-mono text-xs">{p.version}</td>
                  <td className="px-4 py-3">{p.category}</td>
                  <td className="px-4 py-3">
                    <Button size="sm" variant="secondary" loading={busyId === p.id} onClick={() => toggle(p)}>
                      {p.enabled ? 'Enabled' : 'Disabled'}
                    </Button>
                  </td>
                  <td className="px-4 py-3 text-slate-500">{new Date(p.installed_at).toLocaleDateString()}</td>
                  <td className="px-4 py-3 text-right">
                    <Button size="sm" variant="danger" loading={busyId === p.id} onClick={() => uninstall(p.id)}>
                      Uninstall
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title="Install plugin">
        <div className="space-y-4">
          <Input label="Name" value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
          <Input
            label="Version"
            value={form.version}
            onChange={(e) => setForm((f) => ({ ...f, version: e.target.value }))}
          />
          <Input
            label="Description"
            value={form.description}
            onChange={(e) => setForm((f) => ({ ...f, description: e.target.value }))}
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Category</label>
            <select
              className={fieldClass}
              value={form.category}
              onChange={(e) => setForm((f) => ({ ...f, category: e.target.value }))}
            >
              <option value="general">general</option>
              <option value="observability">observability</option>
              <option value="notifications">notifications</option>
              <option value="automation">automation</option>
              <option value="security">security</option>
            </select>
          </div>
          <label className="flex items-center gap-2 text-sm text-slate-700 dark:text-slate-300">
            <input
              type="checkbox"
              checked={form.enabled}
              onChange={(e) => setForm((f) => ({ ...f, enabled: e.target.checked }))}
            />
            Enable after install
          </label>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={install}>
              Install
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
