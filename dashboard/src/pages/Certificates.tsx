import { useCallback, useEffect, useState } from 'react';
import { Button, Input, Modal } from '../components/common';
import { apiJson } from '../lib/auth';

interface Certificate {
  id: string;
  name: string;
  common_name: string;
  issuer: string;
  status: string;
  not_after: string;
  fingerprint: string;
}

export function CertificatesPage() {
  const [certs, setCerts] = useState<Certificate[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [open, setOpen] = useState(false);
  const [saving, setSaving] = useState(false);
  const [revoking, setRevoking] = useState<string | null>(null);
  const [form, setForm] = useState({ name: '', common_name: '', validity_days: '365' });

  const fetchCerts = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      setCerts(await apiJson<Certificate[]>('/certificates'));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchCerts();
  }, [fetchCerts]);

  const issue = async () => {
    if (!form.name.trim() || !form.common_name.trim()) return;
    setSaving(true);
    setError(null);
    try {
      await apiJson('/certificates', {
        method: 'POST',
        body: JSON.stringify({
          name: form.name.trim(),
          common_name: form.common_name.trim(),
          validity_days: Number(form.validity_days) || 365,
        }),
      });
      setOpen(false);
      setForm({ name: '', common_name: '', validity_days: '365' });
      await fetchCerts();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Issue failed');
    } finally {
      setSaving(false);
    }
  };

  const revoke = async (id: string) => {
    setRevoking(id);
    try {
      await apiJson(`/certificates/${id}/revoke`, { method: 'POST' });
      await fetchCerts();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Revoke failed');
    } finally {
      setRevoking(null);
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Certificates</h1>
          <p className="mt-1 text-sm text-slate-500">Issue and revoke TLS certificates</p>
        </div>
        <Button onClick={() => setOpen(true)}>Issue certificate</Button>
      </div>

      {loading && <p className="text-slate-400">Loading...</p>}
      {error && <p className="text-red-500">{error}</p>}

      {!loading && (
        <div className="overflow-hidden rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <table className="w-full text-left text-sm">
            <thead className="border-b border-slate-200 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <tr>
                <th className="px-4 py-3 font-medium">Name</th>
                <th className="px-4 py-3 font-medium">CN</th>
                <th className="px-4 py-3 font-medium">Issuer</th>
                <th className="px-4 py-3 font-medium">Expires</th>
                <th className="px-4 py-3 font-medium">Status</th>
                <th className="px-4 py-3 font-medium" />
              </tr>
            </thead>
            <tbody>
              {certs.map((c) => (
                <tr key={c.id} className="border-b border-slate-100 dark:border-slate-800">
                  <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{c.name}</td>
                  <td className="px-4 py-3 font-mono text-slate-600 dark:text-slate-300">{c.common_name}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{c.issuer}</td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">
                    {new Date(c.not_after).toLocaleDateString()}
                  </td>
                  <td className="px-4 py-3 text-slate-600 dark:text-slate-300">{c.status}</td>
                  <td className="px-4 py-3 text-right">
                    {c.status !== 'revoked' && (
                      <Button
                        size="sm"
                        variant="danger"
                        loading={revoking === c.id}
                        onClick={() => revoke(c.id)}
                      >
                        Revoke
                      </Button>
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <Modal open={open} onClose={() => setOpen(false)} title="Issue certificate">
        <div className="space-y-4">
          <Input
            label="Name"
            value={form.name}
            onChange={(e) => setForm((f) => ({ ...f, name: e.target.value }))}
            placeholder="api-tls"
          />
          <Input
            label="Common name"
            value={form.common_name}
            onChange={(e) => setForm((f) => ({ ...f, common_name: e.target.value }))}
            placeholder="api.nebula.local"
          />
          <Input
            label="Validity (days)"
            type="number"
            value={form.validity_days}
            onChange={(e) => setForm((f) => ({ ...f, validity_days: e.target.value }))}
          />
          <div className="flex justify-end gap-2">
            <Button variant="secondary" onClick={() => setOpen(false)}>
              Cancel
            </Button>
            <Button loading={saving} onClick={issue}>
              Issue
            </Button>
          </div>
        </div>
      </Modal>
    </div>
  );
}
