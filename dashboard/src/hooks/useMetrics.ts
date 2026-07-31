import { useState, useCallback } from 'react';
import type { MonitoringOverview, Alert, MetricsSnapshot } from '../types/metrics';
import { API_URL } from '../lib/api-url';

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

export function useMetrics() {
  const [overview, setOverview] = useState<MonitoringOverview | null>(null);
  const [alerts, setAlerts] = useState<Alert[]>([]);
  const [nodeMetrics, setNodeMetrics] = useState<MetricsSnapshot | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchOverview = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/monitoring/overview`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch overview (${res.status})`);
      const data = await res.json();
      setOverview(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchAlerts = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/monitoring/alerts`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch alerts (${res.status})`);
      const data = await res.json();
      setAlerts(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchNodeMetrics = useCallback(async (nodeId: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/nodes/${nodeId}/metrics/latest`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch node metrics (${res.status})`);
      const data = await res.json();
      setNodeMetrics(data);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  return { overview, alerts, nodeMetrics, loading, error, fetchOverview, fetchAlerts, fetchNodeMetrics };
}
