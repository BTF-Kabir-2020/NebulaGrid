import { useState, useCallback } from 'react';
import type { VirtualMachine, Snapshot } from '../types/vm';
import { API_URL } from '../lib/api-url';

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

export function useVms() {
  const [vms, setVms] = useState<VirtualMachine[]>([]);
  const [vm, setVm] = useState<VirtualMachine | null>(null);
  const [snapshots, setSnapshots] = useState<Snapshot[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchVms = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/vms`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch VMs (${res.status})`);
      const data = await res.json();
      setVms(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchVm = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/vms/${id}`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch VM (${res.status})`);
      const data = await res.json();
      setVm(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchSnapshots = useCallback(async (vmId: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/vms/${vmId}/snapshots`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch snapshots (${res.status})`);
      const data = await res.json();
      setSnapshots(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  return { vms, vm, snapshots, loading, error, fetchVms, fetchVm, fetchSnapshots };
}
