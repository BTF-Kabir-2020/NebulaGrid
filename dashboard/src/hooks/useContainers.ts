import { useState, useCallback } from 'react';
import type { Container } from '../types/container';
import { API_URL } from '../lib/api-url';

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

export function useContainers() {
  const [containers, setContainers] = useState<Container[]>([]);
  const [container, setContainer] = useState<Container | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchContainers = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/containers`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch containers (${res.status})`);
      const data = await res.json();
      setContainers(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchContainer = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/containers/${id}`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch container (${res.status})`);
      const data = await res.json();
      setContainer(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  return { containers, container, loading, error, fetchContainers, fetchContainer };
}
