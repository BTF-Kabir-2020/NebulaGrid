export interface K8sPod {
  name: string;
  namespace: string;
  status: string;
  node: string;
  ip: string;
  created_at: string;
  containers: string[];
}

export interface K8sDeployment {
  name: string;
  namespace: string;
  replicas: number;
  available: number;
  image: string;
  created_at: string;
}

export interface K8sService {
  name: string;
  namespace: string;
  cluster_ip: string;
  ports: string[];
  type: string;
}

export interface K8sNode {
  name: string;
  status: string;
  version: string;
  cpu_capacity: string;
  memory_capacity: string;
  pod_count: number;
}
