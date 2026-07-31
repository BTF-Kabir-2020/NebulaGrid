import { useState, useCallback } from 'react';
import type { Node, NodeMetrics } from '../types/node';
import { API_URL } from '../lib/api-url';

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

export function useNodes() {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [node, setNode] = useState<Node | null>(null);
  const [nodeMetrics, setNodeMetrics] = useState<NodeMetrics[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchNodes = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/nodes`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch nodes (${res.status})`);
      const data = await res.json();
      setNodes(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchNode = useCallback(async (id: string) => {
    setLoading(true);
    setError(null);
    try {
      const res = await fetch(`${API_URL}/nodes/${id}`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch node (${res.status})`);
      const data = await res.json();
      setNode(data);
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
      const res = await fetch(`${API_URL}/nodes/${nodeId}/metrics`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`Failed to fetch node metrics (${res.status})`);
      const data = await res.json();
      setNodeMetrics(Array.isArray(data) ? data : data.value ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  return { nodes, node, nodeMetrics, loading, error, fetchNodes, fetchNode, fetchNodeMetrics };
}
