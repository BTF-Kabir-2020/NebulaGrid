import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface StoragePool {
  id: string;
  name: string;
  pool_type: string;
  total_bytes: number;
  used_bytes: number;
  free_bytes: number;
  mount_path?: string | null;
  status: string;
  created_at: string;
}

interface Volume {
  id: string;
  pool_id: string;
  name: string;
  size_bytes: number;
  volume_type: string;
  format: string;
  status: string;
  created_at: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

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

export function StoragePage() {
  const [pools, setPools] = useState<StoragePool[]>([]);
  const [volumes, setVolumes] = useState<Volume[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [poolOpen, setPoolOpen] = useState(false);
  const [volOpen, setVolOpen] = useState(false);
  const [editPool, setEditPool] = useState<StoragePool | null>(null);
  const [editVol, setEditVol] = useState<Volume | null>(null);
  const [saving, setSaving] = useState(false);
  const [busyId, setBusyId] = useState<string | null>(null);
  const [poolForm, setPoolForm] = useState({
    name: '',
    pool_type: 'lvm',
    total_gb: '500',
    mount_path: '',
  });
  const [volForm, setVolForm] = useState({
    pool_id: '',
    name: '',
    size_gb: '20',
    volume_type: 'raw',
    format: 'ext4',
  });
  const [poolEditForm, setPoolEditForm] = useState({
    name: '',
    pool_type: 'lvm',
    total_gb: '500',
    mount_path: '',
    status: 'online',
  });
  const [volEditForm, setVolEditForm] = useState({
    name: '',
    size_gb: '20',
    volume_type: 'raw',
    format: 'ext4',
    status: 'available',
  });

  const fetchAll = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const [p, v] = await Promise.all([
        apiJson<StoragePool[]>('/storage/pools'),
        apiJson<Volume[]>('/storage/volumes'),
      ]);
      setPools(p);
      setVolumes(v);
      setVolForm((f) => (f.pool_id || p.length === 0 ? f : { ...f, pool_id: p[0].id }));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void fetchAll();
  }, [fetchAll]);

  const createPool = async () => {
    if (!poolForm.name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/storage/pools', {
        method: 'POST',
        body: JSON.stringify({
          name: poolForm.name.trim(),
          pool_type: poolForm.pool_type,
          total_bytes: Math.round(Number(poolForm.total_gb) * 1024 ** 3),
          mount_path: poolForm.mount_path.trim() || null,
        }),
      });
      setPoolOpen(false);
      setPoolForm({ name: '', pool_type: 'lvm', total_gb: '500', mount_path: '' });
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Create pool failed');
    } finally {
      setSaving(false);
    }
  };

  const createVolume = async () => {
    if (!volForm.name.trim() || !volForm.pool_id) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/storage/volumes', {
        method: 'POST',
        body: JSON.stringify({
          pool_id: volForm.pool_id,
          name: volForm.name.trim(),
          size_bytes: Math.round(Number(volForm.size_gb) * 1024 ** 3),
          volume_type: volForm.volume_type,
          format: volForm.format,
        }),
      });
      setVolOpen(false);
      setVolForm((f) => ({ ...f, name: '', size_gb: '20' }));
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Create volume failed');
    } finally {
      setSaving(false);
    }
  };

  const openEditPool = (p: StoragePool) => {
    setEditPool(p);
    setPoolEditForm({
      name: p.name,
      pool_type: p.pool_type,
      total_gb: String(Math.round(p.total_bytes / 1024 ** 3) || 1),
      mount_path: p.mount_path ?? '',
      status: p.status,
    });
  };

  const savePool = async () => {
    if (!editPool) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson(`/storage/pools/${editPool.id}`, {
        method: 'PUT',
        body: JSON.stringify({
          name: poolEditForm.name.trim(),
          pool_type: poolEditForm.pool_type,
          total_bytes: Math.round(Number(poolEditForm.total_gb) * 1024 ** 3),
          mount_path: poolEditForm.mount_path.trim() || null,
          status: poolEditForm.status,
        }),
      });
      setEditPool(null);
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Update pool failed');
    } finally {
      setSaving(false);
    }
  };

  const deletePool = async (id: string) => {
    if (!confirm('Delete this pool? Remove volumes first if any exist.')) return;
    setBusyId(id);
    setError(null);
    try {
      await apiJson(`/storage/pools/${id}`, { method: 'DELETE' });
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Delete pool failed (pool may still have volumes)');
    } finally {
      setBusyId(null);
    }
  };

  const openEditVol = (v: Volume) => {
    setEditVol(v);
    setVolEditForm({
      name: v.name,
      size_gb: String(Math.round(v.size_bytes / 1024 ** 3) || 1),
      volume_type: v.volume_type,
      format: v.format,
      status: v.status,
    });
  };

  const saveVolume = async () => {
    if (!editVol) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson(`/storage/volumes/${editVol.id}`, {
        method: 'PUT',
        body: JSON.stringify({
          name: volEditForm.name.trim(),
          size_bytes: Math.round(Number(volEditForm.size_gb) * 1024 ** 3),
          volume_type: volEditForm.volume_type,
          format: volEditForm.format,
          status: volEditForm.status,
        }),
      });
      setEditVol(null);
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Update volume failed');
    } finally {
      setSaving(false);
    }
  };

  const deleteVolume = async (id: string) => {
    if (!confirm('Delete this volume?')) return;
    setBusyId(id);
    setError(null);
    try {
      await apiJson(`/storage/volumes/${id}`, { method: 'DELETE' });
      await fetchAll();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Delete volume failed');
    } finally {
      setBusyId(null);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Storage</h1>
          <p className="mt-1 text-sm text-slate-500">Pools and volumes — create, edit, delete</p>
        </div>
        <div className="flex gap-2">
          <Button variant="secondary" onClick={() => setVolOpen(true)} disabled={pools.length === 0}>
            Create volume
          </Button>
          <Button onClick={() => setPoolOpen(true)}>Create pool</Button>
        </div>
      </div>

      {loading && <p className="text-slate-400">Loading storage...</p>}
      {error && <p className="text-red-500">Error: {error}</p>}

      {!loading && (
        <>
          <section className="space-y-3">
            <h2 className="text-lg font-semibold text-slate-900 dark:text-white">Pools</h2>
            <div className="grid gap-4 md:grid-cols-2">
              {pools.map((p) => {
                const pct = p.total_bytes > 0 ? (p.used_bytes / p.total_bytes) * 100 : 0;
                return (
                  <div
                    key={p.id}
                    className="rounded-xl border border-slate-200 bg-white p-5 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div>
                        <p className="font-semibold text-slate-900 dark:text-white">{p.name}</p>
                        <p className="text-sm text-slate-500">
                          {p.pool_type}
                          {p.mount_path ? ` · ${p.mount_path}` : ''}
                        </p>
                      </div>
                      <span className="rounded-full bg-emerald-50 px-2 py-0.5 text-xs text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300">
                        {p.status}
                      </span>
                    </div>
                    <div className="mt-4">
                      <div className="mb-1 flex justify-between text-xs text-slate-500">
                        <span>{formatBytes(p.used_bytes)} used</span>
                        <span>{formatBytes(p.free_bytes)} free</span>
                      </div>
                      <div className="h-2 rounded-full bg-slate-200 dark:bg-slate-700">
                        <div
                          className="h-2 rounded-full bg-nebula-500"
                          style={{ width: `${Math.min(pct, 100)}%` }}
                        />
                      </div>
                    </div>
                    <div className="mt-4 flex gap-2">
                      <Button size="sm" variant="secondary" onClick={() => openEditPool(p)}>
                        Edit
                      </Button>
                      <Button
                        size="sm"
                        variant="secondary"
                        loading={busyId === p.id}
                        onClick={() => void deletePool(p.id)}
                      >
                        Delete
                      </Button>
                    </div>
                  </div>
                );
              })}
              {pools.length === 0 && <p className="text-slate-400">No pools yet</p>}
            </div>
          </section>

          <section className="space-y-3">
            <h2 className="text-lg font-semibold text-slate-900 dark:text-white">Volumes</h2>
            <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
              <table className="w-full text-left text-sm">
                <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
                  <tr>
                    <th className="px-4 py-3 font-medium">Name</th>
                    <th className="px-4 py-3 font-medium">Size</th>
                    <th className="px-4 py-3 font-medium">Type</th>
                    <th className="px-4 py-3 font-medium">Format</th>
                    <th className="px-4 py-3 font-medium">Status</th>
                    <th className="px-4 py-3 font-medium" />
                  </tr>
                </thead>
                <tbody>
                  {volumes.map((v) => (
                    <tr key={v.id} className="border-b border-slate-100 dark:border-slate-800">
                      <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{v.name}</td>
                      <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{formatBytes(v.size_bytes)}</td>
                      <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{v.volume_type}</td>
                      <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{v.format}</td>
                      <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{v.status}</td>
                      <td className="px-4 py-3 text-right">
                        <div className="flex justify-end gap-2">
                          <Button size="sm" variant="secondary" onClick={() => openEditVol(v)}>
                            Edit
                          </Button>
                          <Button
                            size="sm"
                            variant="secondary"
                            loading={busyId === v.id}
                            onClick={() => void deleteVolume(v.id)}
                          >
                            Delete
                          </Button>
                        </div>
                      </td>
                    </tr>
                  ))}
                  {volumes.length === 0 && (
                    <tr>
                      <td colSpan={6} className="px-4 py-8 text-center text-slate-400">
                        No volumes yet
                      </td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </section>
        </>
      )}

      <Modal open={poolOpen} onClose={() => setPoolOpen(false)} title="Create storage pool">
        <div className="space-y-4">
          <Input
            label="Name"
            value={poolForm.name}
            onChange={(e) => setPoolForm((f) => ({ ...f, name: e.target.value }))}
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className={fieldClass}
              value={poolForm.pool_type}
              onChange={(e) => setPoolForm((f) => ({ ...f, pool_type: e.target.value }))}
            >
              <option value="lvm">lvm</option>
              <option value="zfs">zfs</option>
              <option value="ceph">ceph</option>
            </select>
          </div>
          <Input
            label="Capacity (GB)"
            type="number"
            value={poolForm.total_gb}
            onChange={(e) => setPoolForm((f) => ({ ...f, total_gb: e.target.value }))}
          />
          <Input
            label="Mount path"
            value={poolForm.mount_path}
            onChange={(e) => setPoolForm((f) => ({ ...f, mount_path: e.target.value }))}
            placeholder="/mnt/pool"
          />
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setPoolOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={createPool}>
              Create
            </Button>
          </div>
        </div>
      </Modal>

      <Modal open={volOpen} onClose={() => setVolOpen(false)} title="Create volume">
        <div className="space-y-4">
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Pool</label>
            <select
              className={fieldClass}
              value={volForm.pool_id}
              onChange={(e) => setVolForm((f) => ({ ...f, pool_id: e.target.value }))}
            >
              {pools.map((p) => (
                <option key={p.id} value={p.id}>
                  {p.name}
                </option>
              ))}
            </select>
          </div>
          <Input
            label="Name"
            value={volForm.name}
            onChange={(e) => setVolForm((f) => ({ ...f, name: e.target.value }))}
          />
          <Input
            label="Size (GB)"
            type="number"
            value={volForm.size_gb}
            onChange={(e) => setVolForm((f) => ({ ...f, size_gb: e.target.value }))}
          />
          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
              <select
                className={fieldClass}
                value={volForm.volume_type}
                onChange={(e) => setVolForm((f) => ({ ...f, volume_type: e.target.value }))}
              >
                <option value="raw">raw</option>
                <option value="qcow2">qcow2</option>
              </select>
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Format</label>
              <select
                className={fieldClass}
                value={volForm.format}
                onChange={(e) => setVolForm((f) => ({ ...f, format: e.target.value }))}
              >
                <option value="ext4">ext4</option>
                <option value="xfs">xfs</option>
                <option value="btrfs">btrfs</option>
              </select>
            </div>
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setVolOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={createVolume}>
              Create
            </Button>
          </div>
        </div>
      </Modal>

      <Modal open={!!editPool} onClose={() => setEditPool(null)} title="Edit storage pool">
        <div className="space-y-4">
          <Input
            label="Name"
            value={poolEditForm.name}
            onChange={(e) => setPoolEditForm((f) => ({ ...f, name: e.target.value }))}
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
            <select
              className={fieldClass}
              value={poolEditForm.pool_type}
              onChange={(e) => setPoolEditForm((f) => ({ ...f, pool_type: e.target.value }))}
            >
              <option value="lvm">lvm</option>
              <option value="zfs">zfs</option>
              <option value="ceph">ceph</option>
            </select>
          </div>
          <Input
            label="Capacity (GB)"
            type="number"
            value={poolEditForm.total_gb}
            onChange={(e) => setPoolEditForm((f) => ({ ...f, total_gb: e.target.value }))}
          />
          <Input
            label="Mount path"
            value={poolEditForm.mount_path}
            onChange={(e) => setPoolEditForm((f) => ({ ...f, mount_path: e.target.value }))}
          />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Status</label>
            <select
              className={fieldClass}
              value={poolEditForm.status}
              onChange={(e) => setPoolEditForm((f) => ({ ...f, status: e.target.value }))}
            >
              <option value="online">online</option>
              <option value="degraded">degraded</option>
              <option value="offline">offline</option>
            </select>
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setEditPool(null)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={() => void savePool()}>
              Save
            </Button>
          </div>
        </div>
      </Modal>

      <Modal open={!!editVol} onClose={() => setEditVol(null)} title="Edit volume">
        <div className="space-y-4">
          <Input
            label="Name"
            value={volEditForm.name}
            onChange={(e) => setVolEditForm((f) => ({ ...f, name: e.target.value }))}
          />
          <Input
            label="Size (GB)"
            type="number"
            value={volEditForm.size_gb}
            onChange={(e) => setVolEditForm((f) => ({ ...f, size_gb: e.target.value }))}
          />
          <div className="grid grid-cols-2 gap-3">
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Type</label>
              <select
                className={fieldClass}
                value={volEditForm.volume_type}
                onChange={(e) => setVolEditForm((f) => ({ ...f, volume_type: e.target.value }))}
              >
                <option value="raw">raw</option>
                <option value="qcow2">qcow2</option>
              </select>
            </div>
            <div className="space-y-1">
              <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Format</label>
              <select
                className={fieldClass}
                value={volEditForm.format}
                onChange={(e) => setVolEditForm((f) => ({ ...f, format: e.target.value }))}
              >
                <option value="ext4">ext4</option>
                <option value="xfs">xfs</option>
                <option value="btrfs">btrfs</option>
              </select>
            </div>
          </div>
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Status</label>
            <select
              className={fieldClass}
              value={volEditForm.status}
              onChange={(e) => setVolEditForm((f) => ({ ...f, status: e.target.value }))}
            >
              <option value="available">available</option>
              <option value="in-use">in-use</option>
              <option value="error">error</option>
            </select>
          </div>
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setEditVol(null)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={() => void saveVolume()}>
              Save
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
