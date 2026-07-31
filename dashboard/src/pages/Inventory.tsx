import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Item {
  id: string;
  asset_tag: string;
  name: string;
  item_type: string;
  manufacturer?: string | null;
  model?: string | null;
  location?: string | null;
  status: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

const emptyForm = {
  name: '',
  item_type: 'server',
  asset_tag: '',
  manufacturer: '',
  model: '',
  location: '',
};

export function InventoryPage() {
  const [items, setItems] = useState<Item[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [editing, setEditing] = useState<Item | null>(null);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState<string | null>(null);
  const [form, setForm] = useState(emptyForm);

  const fetchItems = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setItems(await apiJson<Item[]>('/inventory'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchItems();
  }, [fetchItems]);

  const create = async () => {
    if (!form.name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      if (editing) {
        await apiJson(`/inventory/${editing.id}`, {
          method: 'PUT',
          body: JSON.stringify({
            name: form.name.trim(),
            item_type: form.item_type,
            manufacturer: form.manufacturer.trim() || null,
            model: form.model.trim() || null,
            location: form.location.trim() || null,
          }),
        });
      } else {
        await apiJson('/inventory', {
          method: 'POST',
          body: JSON.stringify({
            name: form.name.trim(),
            item_type: form.item_type,
            asset_tag: form.asset_tag.trim() || null,
            manufacturer: form.manufacturer.trim() || null,
            model: form.model.trim() || null,
            location: form.location.trim() || null,
          }),
        });
      }
      setOpen(false);
      setEditing(null);
      setForm(emptyForm);
      await fetchItems();
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

  const openEdit = (i: Item) => {
    setEditing(i);
    setForm({
      name: i.name,
      item_type: i.item_type,
      asset_tag: i.asset_tag,
      manufacturer: i.manufacturer ?? '',
      model: i.model ?? '',
      location: i.location ?? '',
    });
    setOpen(true);
  };

  const remove = async (id: string) => {
    setDeleting(id);
    try {
      await apiJson(`/inventory/${id}`, { method: 'DELETE' });
      await fetchItems();
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
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Inventory</h1>
          <p className="mt-1 text-sm text-slate-500">Hardware and asset registry</p>
        </div>
        <Button onClick={openCreate}>Add asset</Button>
      </div>

      {loading && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Asset Tag</th>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">Type</th>
                <th className="px-4 py-3 font-medium">Manufacturer</th>
                <th className="px-4 py-3 font-medium">Location</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {items.map((i) => (
                <tr key={i.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 font-mono text-slate-600 dark:text-slate-300">{i.asset_tag}</td>
                  <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{i.name}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{i.item_type}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{i.manufacturer ?? '—'}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{i.location ?? '—'}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{i.status}</td>
                  <td className="px-4 py-3 text-right space-x-2">
                    <Button size="sm" variant="secondary" onClick={() => openEdit(i)}>
                      Edit
                    </Button>
                    <Button size="sm" variant="danger" loading={deleting === i.id} onClick={() => remove(i.id)}>
                      Delete
                    </Button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => { setOpen(false); setEditing(null); }} title={editing ? 'Edit asset' : 'Add inventory asset'}>
        <div className="space-y-4">
          <Input label="Name" value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className={fieldClass}
              value={form.item_type}
              onChange={(e) => setForm((f) => ({ ...f, item_type: e.target.value }))}
            >
              <option value="server">server</option>
              <option value="switch">switch</option>
              <option value="storage">storage</option>
              <option value="other">other</option>
            </select>
          </div>
          {!editing && (
            <Input
              label="Asset tag (optional)"
              value={form.asset_tag}
              onChange={(e) => setForm((f) => ({ ...f, asset_tag: e.target.value }))}
            />
          )}
          <Input
            label="Manufacturer"
            value={form.manufacturer}
            onChange={(e) => setForm((f) => ({ ...f, manufacturer: e.target.value }))}
          />
          <Input label="Model" value={form.model} onChange={(e) => setForm((f) => ({ ...f, model: e.target.value }))} />
          <Input
            label="Location"
            value={form.location}
            onChange={(e) => setForm((f) => ({ ...f, location: e.target.value }))}
          />
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => { setOpen(false); setEditing(null); }}>
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
