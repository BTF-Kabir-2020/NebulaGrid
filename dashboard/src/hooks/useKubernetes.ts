import { useState, useCallback } from 'react';
import type { K8sPod, K8sDeployment, K8sService, K8sNode } from '../types/kubernetes';
import { API_URL } from '../lib/api-url';
import { authHeaders } from '../lib/auth';

function mapPod(raw: Record<string, unknown>): K8sPod {
  return {
    name: String(raw.name ?? ''),
    namespace: String(raw.namespace ?? ''),
    status: String(raw.status ?? ''),
    node: String(raw.node ?? ''),
    ip: String(raw.pod_ip ?? raw.ip ?? ''),
    created_at: String(raw.created_at ?? ''),
    containers: Array.isArray(raw.containers) ? (raw.containers as string[]) : [],
  };
}

function mapDeployment(raw: Record<string, unknown>): K8sDeployment {
  return {
    name: String(raw.name ?? ''),
    namespace: String(raw.namespace ?? ''),
    replicas: Number(raw.replicas ?? 0),
    available: Number(raw.available_replicas ?? raw.available ?? 0),
    image: String(raw.image ?? '—'),
    created_at: String(raw.created_at ?? ''),
  };
}

function mapService(raw: Record<string, unknown>): K8sService {
  return {
    name: String(raw.name ?? ''),
    namespace: String(raw.namespace ?? ''),
    cluster_ip: String(raw.cluster_ip ?? ''),
    ports: Array.isArray(raw.ports) ? (raw.ports as string[]) : [],
    type: String(raw.type ?? raw.type_ ?? 'ClusterIP'),
  };
}

function mapNode(raw: Record<string, unknown>): K8sNode {
  return {
    name: String(raw.name ?? ''),
    status: String(raw.status ?? ''),
    version: String(raw.version ?? ''),
    cpu_capacity: String(raw.cpu_capacity ?? '—'),
    memory_capacity: String(raw.memory_capacity ?? '—'),
    pod_count: Number(raw.pod_count ?? 0),
  };
}

export function useKubernetes() {
  const [pods, setPods] = useState<K8sPod[]>([]);
  const [deployments, setDeployments] = useState<K8sDeployment[]>([]);
  const [services, setServices] = useState<K8sService[]>([]);
  const [nodes, setNodes] = useState<K8sNode[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const headers = authHeaders();
      const [podsRes, depsRes, svcRes, nodesRes] = await Promise.all([
        fetch(`${API_URL}/k8s/pods`, { headers }),
        fetch(`${API_URL}/k8s/deployments`, { headers }),
        fetch(`${API_URL}/k8s/services`, { headers }),
        fetch(`${API_URL}/k8s/nodes`, { headers }),
      ]);
      if (!podsRes.ok || !depsRes.ok || !svcRes.ok || !nodesRes.ok) {
        throw new Error('Failed to load Kubernetes resources');
      }
      const [podsData, depsData, svcData, nodesData] = await Promise.all([
        podsRes.json(),
        depsRes.json(),
        svcRes.json(),
        nodesRes.json(),
      ]);
      setPods((Array.isArray(podsData) ? podsData : []).map(mapPod));
      setDeployments((Array.isArray(depsData) ? depsData : []).map(mapDeployment));
      setServices((Array.isArray(svcData) ? svcData : []).map(mapService));
      setNodes((Array.isArray(nodesData) ? nodesData : []).map(mapNode));
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, []);

  return {
    pods,
    deployments,
    services,
    nodes,
    loading,
    error,
    refresh,
    fetchPods: refresh,
    fetchDeployments: refresh,
    fetchServices: refresh,
    fetchNodes: refresh,
  };
}
