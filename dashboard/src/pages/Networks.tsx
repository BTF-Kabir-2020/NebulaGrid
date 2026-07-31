import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Network {
  id: string;
  name: string;
  subnet: string;
  gateway?: string | null;
  vlan_id?: number | null;
  network_type: string;
  status: string;
  created_at: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

const emptyForm = {
  name: '',
  subnet: '10.30.0.0/16',
  gateway: '10.30.0.1',
  vlan_id: '',
  network_type: 'bridge',
};

export function NetworksPage() {
  const [networks, setNetworks] = useState<Network[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState<string | null>(null);
  const [form, setForm] = useState(emptyForm);

  const fetchNetworks = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setNetworks(await apiJson<Network[]>('/networks'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchNetworks();
  }, [fetchNetworks]);

  const create = async () => {
    if (!form.name.trim() || !form.subnet.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/networks', {
        method: 'POST',
        body: JSON.stringify({
          name: form.name.trim(),
          subnet: form.subnet.trim(),
          gateway: form.gateway.trim() || null,
          vlan_id: form.vlan_id ? Number(form.vlan_id) : null,
          network_type: form.network_type,
        }),
      });
      setOpen(false);
      setForm(emptyForm);
      await fetchNetworks();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Create failed');
    } finally {
      setSaving(false);
    }
  };

  const remove = async (id: string) => {
    setDeleting(id);
    try {
      await apiJson(`/networks/${id}`, { method: 'DELETE' });
      await fetchNetworks();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Delete failed');
    } finally {
      setDeleting(null);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Networks</h1>
          <p className="mt-1 text-sm text-slate-500">Virtual networks and VLANs</p>
        </div>
        <Button onClick={() => setOpen(true)}>Create network</Button>
      </div>

      {loading && <p className="text-slate-400">Loading networks...</p>}
      {error && <p className="text-red-500">Error: {error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Subnet</th>
                <th className="px-4 py-3 font-medium">Gateway</th>
                <th className="px-4 py-3 font-medium">VLAN</th>
                <th className="px-4 py-3 font-medium">Type</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {networks.map((n) => (
                <tr key={n.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{n.name}</td>
                  <td className="px-4 py-3 font-mono text-slate-600 dark:text-slate-300">{n.subnet}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{n.gateway ?? '—'}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{n.vlan_id ?? '—'}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{n.network_type}</td>
                  <td className="px-4 py-3">
                    <span className="rounded-full bg-emerald-50 px-2 py-0.5 text-xs text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300">
                      {n.status}
                    </span>
                  </td>
                  <td className="px-4 py-3 text-right">
                    <Button size="sm" variant="danger" loading={deleting === n.id} onClick={() => remove(n.id)}>
                      Delete
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title="Create network">
        <div className="space-y-4">
          <Input label="Name" value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
          <Input
            label="Subnet (CIDR)"
            value={form.subnet}
            onChange={(e) => setForm((f) => ({ ...f, subnet: e.target.value }))}
          />
          <Input
            label="Gateway"
            value={form.gateway}
            onChange={(e) => setForm((f) => ({ ...f, gateway: e.target.value }))}
          />
          <Input
            label="VLAN ID"
            type="number"
            value={form.vlan_id}
            onChange={(e) => setForm((f) => ({ ...f, vlan_id: e.target.value }))}
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className={fieldClass}
              value={form.network_type}
              onChange={(e) => setForm((f) => ({ ...f, network_type: e.target.value }))}
            >
              <option value="bridge">bridge</option>
              <option value="overlay">overlay</option>
              <option value="macvlan">macvlan</option>
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
