import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useVms } from '../hooks/useVms';
import { VmTable } from '../components/tables/VmTable';
import type { VirtualMachine } from '../types/vm';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface NodeOption {
  id: string;
  hostname: string;
}

const fieldClass =
  'w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm dark:border-slate-700 dark:bg-slate-800 dark:text-white';

export function VmsPage() {
  const navigate = useNavigate();
  const { vms, loading, error, fetchVms } = useVms();
  const [open, setOpen] = useState(false);
  const [saving, setSaving] = useState(false);
  const [formError, setFormError] = useState<string | null>(null);
  const [nodes, setNodes] = useState<NodeOption[]>([]);
  const [form, setForm] = useState({
    name: '',
    os_type: 'ubuntu-22.04',
    cpu_cores: '2',
    ram_mb: '4096',
    disk_gb: '40',
    node_id: '',
  });

  useEffect(() => {
    fetchVms();
  }, [fetchVms]);

  useEffect(() => {
    void apiJson<NodeOption[]>('/nodes')
      .then((list) => {
        setNodes(list);
        if (list.length > 0) {
          setForm((f) => ({ ...f, node_id: f.node_id || list[0].id }));
        }
      })
      .catch(() => undefined);
  }, []);

  const handleSelect = (vm: VirtualMachine) => {
    navigate(`/vms/${vm.id}`);
  };

  const create = async () => {
    if (!form.name.trim() || !form.node_id) {
      setFormError('Name and node are required');
      return;
    }
    setSaving(true);
    setFormError(null);
    try {
      await apiJson('/vms', {
        method: 'POST',
        body: JSON.stringify({
          name: form.name.trim(),
          os_type: form.os_type,
          cpu_cores: Number(form.cpu_cores) || 2,
          ram_mb: Number(form.ram_mb) || 4096,
          disk_gb: Number(form.disk_gb) || 40,
          node_id: form.node_id,
        }),
      });
      setOpen(false);
      setForm((f) => ({ ...f, name: '' }));
      await fetchVms();
    } catch (e) {
      setFormError(e instanceof Error ? e.message : 'Create failed');
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Virtual Machines</h1>
          <p className="mt-1 text-sm text-slate-500">Provision and manage VMs</p>
        </div>
        <Button onClick={() => setOpen(true)}>Create VM</Button>
      </div>

      {loading && vms.length === 0 && (
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <p className="text-slate-400">Loading virtual machines...</p>
        </div>
      )}

      {(error || formError) && (
        <div className="rounded-xl border border-red-200 bg-red-50 p-6 dark:border-red-800 dark:bg-red-900/20">
          <p className="text-red-600 dark:text-red-400">Error: {error || formError}</p>
        </div>
      )}

      {!loading && !error && (
        <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <VmTable vms={vms} onSelect={handleSelect} />
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title="Create virtual machine">
        <div className="space-y-4">
          <Input label="Name" value={form.name} onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))} />
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">OS</label>
            <select
              className={fieldClass}
              value={form.os_type}
              onChange={(e) => setForm((f) => ({ ...f, os_type: e.target.value }))}
            >
              <option value="ubuntu-22.04">ubuntu-22.04</option>
              <option value="debian-12">debian-12</option>
              <option value="rocky-9">rocky-9</option>
            </select>
          </div>
          <div className="space-y-1">
            <label className="text-sm font-medium text-slate-700 dark:text-slate-300">Node</label>
            <select
              className={fieldClass}
              value={form.node_id}
              onChange={(e) => setForm((f) => ({ ...f, node_id: e.target.value }))}
            >
              {nodes.map((n) => (
                <option key={n.id} value={n.id}>
                  {n.hostname}
                </option>
              ))}
            </select>
          </div>
          <div className="grid grid-cols-3 gap-3">
            <Input
              label="vCPU"
              type="number"
              value={form.cpu_cores}
              onChange={(e) => setForm((f) => ({ ...f, cpu_cores: e.target.value }))}
            />
            <Input
              label="RAM (MB)"
              type="number"
              value={form.ram_mb}
              onChange={(e) => setForm((f) => ({ ...f, ram_mb: e.target.value }))}
            />
            <Input
              label="Disk (GB)"
              type="number"
              value={form.disk_gb}
              onChange={(e) => setForm((f) => ({ ...f, disk_gb: e.target.value }))}
            />
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
