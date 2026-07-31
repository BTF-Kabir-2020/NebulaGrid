import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Policy {
  id: string;
  name: string;
  description?: string | null;
  policy_type: string;
  scope: string;
  enforcement: string;
  priority: number;
  enabled: boolean;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

const emptyForm = {
  name: '',
  description: '',
  policy_type: 'security',
  scope: 'cluster',
  enforcement: 'enforce',
  priority: '100',
};

export function PoliciesPage() {
  const [policies, setPolicies] = useState<Policy[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Policy | null>(null);
  const [saving, setSaving] = useState(false);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [form, setForm] = useState(emptyForm);

  const fetchPolicies = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setPolicies(await apiJson<Policy[]>('/policies'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchPolicies();
  }, [fetchPolicies]);

  const create = async () => {
    if (!form.name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      if (editing) {
        await apiJson(`/policies/${editing.id}`, {
          method: 'PUT',
          body: JSON.stringify({
            name: form.name.trim(),
            description: form.description.trim() || null,
            policy_type: form.policy_type,
            scope: form.scope,
            enforcement: form.enforcement,
            priority: Number(form.priority) || 100,
          }),
        });
      } else {
        await apiJson('/policies', {
          method: 'POST',
          body: JSON.stringify({
            name: form.name.trim(),
            description: form.description.trim() || null,
            policy_type: form.policy_type,
            scope: form.scope,
            enforcement: form.enforcement,
            priority: Number(form.priority) || 100,
          }),
        });
      }
      setOpen(false);
      setEditing(null);
      setForm(emptyForm);
      await fetchPolicies();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Save failed');
    } finally {
      setSaving(false);
    }
  };

  const openCreate = () => {
    setEditing(null);
    setForm(emptyForm);
    setOpen(true);
  };

  const openEdit = (p: Policy) => {
    setEditing(p);
    setForm({
      name: p.name,
      description: p.description ?? '',
      policy_type: p.policy_type,
      scope: p.scope,
      enforcement: p.enforcement,
      priority: String(p.priority),
    });
    setOpen(true);
  };

  const toggle = async (p: Policy) => {
    setBusyId(p.id);
    try {
      await apiJson(`/policies/${p.id}`, {
        method: 'PUT',
        body: JSON.stringify({ enabled: !p.enabled }),
      });
      await fetchPolicies();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Update failed');
    } finally {
      setBusyId(null);
    }
  };

  const remove = async (id: string) => {
    setBusyId(id);
    try {
      await apiJson(`/policies/${id}`, { method: 'DELETE' });
      await fetchPolicies();
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
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Policies</h1>
          <p className="mt-1 text-sm text-slate-500">Security and operational policy rules</p>
        </div>
        <Button onClick={openCreate}>Create policy</Button>
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
                <th className="px-4 py-3 font-medium">Scope</th>
                <th className="px-4 py-3 font-medium">Enforcement</th>
                <th className="px-4 py-3 font-medium">Priority</th>
                <th className="px-4 py-3 font-medium">Enabled</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {policies.map((p) => (
                <tr key={p.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3">
                    <div className="font-medium text-slate-900 dark:text-white">{p.name}</div>
                    {p.description && <div className="text-xs text-slate-500">{p.description}</div>}
                  </td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{p.policy_type}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{p.scope}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{p.enforcement}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{p.priority}</td>
                  <td className="px-4 py-3">
                    <Button size="sm" variant="secondary" loading={busyId === p.id} onClick={() => toggle(p)}>
                      {p.enabled ? 'On' : 'Off'}
                    </Button>
                  </td>
                  <td className="px-4 py-3 text-right space-x-2">
                    <Button size="sm" variant="secondary" onClick={() => openEdit(p)}>
                      Edit
                    </Button>
                    <Button size="sm" variant="danger" loading={busyId === p.id} onClick={() => remove(p.id)}>
                      Delete
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => { setOpen(false); setEditing(null); }} title={editing ? 'Edit policy' : 'Create policy'}>
        <div className="space-y-4">
          <Input label="Name" value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
          <Input
            label="Description"
            value={form.description}
            onChange={(e) => setForm((f) => ({ ...f, description: e.target.value }))}
          />
          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
              <select
                className={fieldClass}
                value={form.policy_type}
                onChange={(e) => setForm((f) => ({ ...f, policy_type: e.target.value }))}
              >
                <option value="security">security</option>
                <option value="compliance">compliance</option>
                <option value="ops">ops</option>
              </select>
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Scope</label>
              <select
                className={fieldClass}
                value={form.scope}
                onChange={(e) => setForm((f) => ({ ...f, scope: e.target.value }))}
              >
                <option value="cluster">cluster</option>
                <option value="node">node</option>
                <option value="namespace">namespace</option>
              </select>
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Enforcement</label>
              <select
                className={fieldClass}
                value={form.enforcement}
                onChange={(e) => setForm((f) => ({ ...f, enforcement: e.target.value }))}
              >
                <option value="enforce">enforce</option>
                <option value="audit">audit</option>
              </select>
            </div>
            <Input
              label="Priority"
              type="number"
              value={form.priority}
              onChange={(e) => setForm((f) => ({ ...f, priority: e.target.value }))}
            />
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={create}>
              {editing ? 'Save' : 'Create'}
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
