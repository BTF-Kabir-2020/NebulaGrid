import { useState } from 'react';
import { BookOpen, ChevronDown, ChevronRight, Copy, Check } from 'lucide-react';

interface Endpoint {
  method: string;
  path: string;
  auth: string;
  description: string;
  body?: string;
  response?: string;
}

const categories: { title: string; endpoints: Endpoint[] }[] = [
  {
    title: 'Health',
    endpoints: [
      { method: 'GET', path: '/api/health', auth: 'No', description: 'API health check', response: '{"status":"ok","version":"0.1.0","uptime_seconds":42}' },
      { method: 'GET', path: '/api/health/ready', auth: 'No', description: 'Readiness check', response: '{"status":"ready","version":"0.1.0","uptime_seconds":42}' },
    ],
  },
  {
    title: 'Authentication',
    endpoints: [
      { method: 'POST', path: '/api/auth/login', auth: 'No', description: 'Login with username/password', body: '{"username":"admin","password":"admin123"}', response: '{"token":"eyJ...","refresh_token":"uuid","expires_in":86400}' },
      { method: 'POST', path: '/api/auth/refresh', auth: 'No', description: 'Refresh JWT token', body: '{"refresh_token":"uuid"}', response: '{"token":"eyJ...","refresh_token":"uuid","expires_in":86400}' },
      { method: 'POST', path: '/api/auth/logout', auth: 'JWT', description: 'Logout and clear refresh tokens' },
      { method: 'GET', path: '/api/auth/me', auth: 'JWT', description: 'Get current user profile', response: '{"id":"uuid","username":"admin","email":"admin@nebula.local","roles":["admin"]}' },
      { method: 'POST', path: '/api/auth/change-password', auth: 'JWT', description: 'Change password', body: '{"current_password":"old","new_password":"new"}' },
      { method: 'GET', path: '/api/auth/tokens', auth: 'JWT', description: 'List API tokens' },
      { method: 'POST', path: '/api/auth/tokens', auth: 'JWT', description: 'Create API token', body: '{"name":"CI/CD Token"}', response: '{"id":"uuid","name":"CI/CD Token","token":"ng_abc...","created_at":"..."}' },
      { method: 'DELETE', path: '/api/auth/tokens/:id', auth: 'JWT', description: 'Revoke API token' },
    ],
  },
  {
    title: 'Users',
    endpoints: [
      { method: 'GET', path: '/api/users', auth: 'JWT', description: 'List all users' },
      { method: 'POST', path: '/api/users', auth: 'JWT', description: 'Create a new user', body: '{"username":"jdoe","email":"j@e.com","password":"..."}' },
      { method: 'GET', path: '/api/users/:id', auth: 'JWT', description: 'Get user by ID' },
      { method: 'PUT', path: '/api/users/:id', auth: 'JWT', description: 'Update user email', body: '{"email":"new@e.com"}' },
      { method: 'DELETE', path: '/api/users/:id', auth: 'JWT', description: 'Delete user' },
    ],
  },
  {
    title: 'Nodes',
    endpoints: [
      { method: 'GET', path: '/api/nodes', auth: 'JWT', description: 'List all registered nodes' },
      { method: 'POST', path: '/api/nodes/register', auth: 'JWT', description: 'Register a new compute node', body: '{"hostname":"node-1","ip_address":"10.0.1.5","os_name":"Ubuntu 22.04","cpu_cores":8,"ram_total_bytes":17179869184,"disk_total_bytes":536870912000}' },
      { method: 'GET', path: '/api/nodes/:id', auth: 'JWT', description: 'Get node details' },
      { method: 'PUT', path: '/api/nodes/:id', auth: 'JWT', description: 'Update node hostname/status' },
      { method: 'DELETE', path: '/api/nodes/:id', auth: 'JWT', description: 'Delete a node' },
      { method: 'GET', path: '/api/nodes/:id/metrics', auth: 'JWT', description: 'Get node metrics history' },
      { method: 'POST', path: '/api/nodes/:id/metrics', auth: 'JWT', description: 'Submit metrics for a node', body: '{"cpu_percent":45.2,"ram_percent":62.8,"disk_percent":71.3,"net_rx_bytes":1048576,"net_tx_bytes":524288}' },
      { method: 'GET', path: '/api/nodes/:id/metrics/latest', auth: 'JWT', description: 'Get latest node metrics' },
      { method: 'POST', path: '/api/nodes/:id/command', auth: 'JWT', description: 'Execute command on node', body: '{"command":"uptime"}' },
    ],
  },
  {
    title: 'Containers',
    endpoints: [
      { method: 'GET', path: '/api/containers', auth: 'JWT', description: 'List all containers' },
      { method: 'GET', path: '/api/containers/:id', auth: 'JWT', description: 'Get container details' },
      { method: 'POST', path: '/api/containers/:id/start', auth: 'JWT', description: 'Start a container' },
      { method: 'POST', path: '/api/containers/:id/stop', auth: 'JWT', description: 'Stop a container' },
      { method: 'POST', path: '/api/containers/:id/restart', auth: 'JWT', description: 'Restart a container' },
      { method: 'DELETE', path: '/api/containers/:id', auth: 'JWT', description: 'Delete a container' },
      { method: 'GET', path: '/api/containers/:id/logs', auth: 'JWT', description: 'Get container logs (mock)' },
    ],
  },
  {
    title: 'Virtual Machines',
    endpoints: [
      { method: 'GET', path: '/api/vms', auth: 'JWT', description: 'List all VMs' },
      { method: 'POST', path: '/api/vms', auth: 'JWT', description: 'Create a VM', body: '{"name":"web-01","os_type":"ubuntu-22.04","cpu_cores":4,"ram_mb":8192,"disk_gb":100}' },
      { method: 'GET', path: '/api/vms/:id', auth: 'JWT', description: 'Get VM details' },
      { method: 'PUT', path: '/api/vms/:id', auth: 'JWT', description: 'Update VM' },
      { method: 'DELETE', path: '/api/vms/:id', auth: 'JWT', description: 'Delete VM' },
      { method: 'POST', path: '/api/vms/:id/start', auth: 'JWT', description: 'Start a VM' },
      { method: 'POST', path: '/api/vms/:id/stop', auth: 'JWT', description: 'Stop a VM' },
      { method: 'POST', path: '/api/vms/:id/restart', auth: 'JWT', description: 'Restart a VM' },
      { method: 'POST', path: '/api/vms/:id/snapshot', auth: 'JWT', description: 'Create a VM snapshot' },
      { method: 'GET', path: '/api/vms/:id/snapshots', auth: 'JWT', description: 'List VM snapshots' },
      { method: 'POST', path: '/api/vms/:id/backup', auth: 'JWT', description: 'Initiate VM backup' },
    ],
  },
  {
    title: 'Kubernetes',
    endpoints: [
      { method: 'GET', path: '/api/k8s/nodes', auth: 'JWT', description: 'List K8s cluster nodes' },
      { method: 'GET', path: '/api/k8s/pods', auth: 'JWT', description: 'List K8s pods' },
      { method: 'GET', path: '/api/k8s/deployments', auth: 'JWT', description: 'List K8s deployments' },
      { method: 'GET', path: '/api/k8s/services', auth: 'JWT', description: 'List K8s services' },
      { method: 'GET', path: '/api/k8s/namespaces', auth: 'JWT', description: 'List K8s namespaces' },
    ],
  },
  {
    title: 'Monitoring',
    endpoints: [
      { method: 'GET', path: '/api/monitoring/overview', auth: 'JWT', description: 'Aggregate overview (node stats, avg CPU/RAM)' },
      { method: 'GET', path: '/api/monitoring/prometheus/query', auth: 'JWT', description: 'Prometheus-compatible query' },
      { method: 'GET', path: '/api/monitoring/alerts', auth: 'JWT', description: 'List alerts (filter by severity/ack status)' },
      { method: 'POST', path: '/api/monitoring/alerts/:id/ack', auth: 'JWT', description: 'Acknowledge an alert' },
    ],
  },
  {
    title: 'Networks',
    endpoints: [
      { method: 'GET', path: '/api/networks', auth: 'JWT', description: 'List networks' },
      { method: 'POST', path: '/api/networks', auth: 'JWT', description: 'Create network', body: '{"name":"app-net","subnet":"10.20.0.0/16","gateway":"10.20.0.1","vlan_id":200,"network_type":"bridge"}' },
      { method: 'GET', path: '/api/networks/:id', auth: 'JWT', description: 'Get network by ID' },
      { method: 'DELETE', path: '/api/networks/:id', auth: 'JWT', description: 'Delete network' },
    ],
  },
  {
    title: 'Storage',
    endpoints: [
      { method: 'GET', path: '/api/storage/pools', auth: 'JWT', description: 'List storage pools' },
      { method: 'POST', path: '/api/storage/pools', auth: 'JWT', description: 'Create storage pool', body: '{"name":"nvme-pool","pool_type":"lvm","total_bytes":1099511627776}' },
      { method: 'GET', path: '/api/storage/pools/:id', auth: 'JWT', description: 'Get storage pool' },
      { method: 'PUT', path: '/api/storage/pools/:id', auth: 'JWT', description: 'Update storage pool', body: '{"name":"nvme-pool","status":"online","total_bytes":2199023255552}' },
      { method: 'DELETE', path: '/api/storage/pools/:id', auth: 'JWT', description: 'Delete pool (409 if volumes remain)' },
      { method: 'GET', path: '/api/storage/volumes', auth: 'JWT', description: 'List volumes' },
      { method: 'POST', path: '/api/storage/volumes', auth: 'JWT', description: 'Create volume', body: '{"pool_id":"uuid","name":"data-1","size_bytes":21474836480}' },
      { method: 'PUT', path: '/api/storage/volumes/:id', auth: 'JWT', description: 'Update volume', body: '{"name":"data-1","size_bytes":42949672960,"status":"available"}' },
      { method: 'DELETE', path: '/api/storage/volumes/:id', auth: 'JWT', description: 'Delete volume' },
    ],
  },
  {
    title: 'Jobs',
    endpoints: [
      { method: 'GET', path: '/api/jobs', auth: 'JWT', description: 'List automation jobs' },
      { method: 'POST', path: '/api/jobs', auth: 'JWT', description: 'Create and run a job', body: '{"name":"health-check","job_type":"playbook","target_nodes":["compute-1"],"params":{}}' },
      { method: 'GET', path: '/api/jobs/:id', auth: 'JWT', description: 'Get job by ID' },
    ],
  },
  {
    title: 'Inventory',
    endpoints: [
      { method: 'GET', path: '/api/inventory', auth: 'JWT', description: 'List inventory assets' },
      { method: 'POST', path: '/api/inventory', auth: 'JWT', description: 'Register inventory item', body: '{"name":"Server X","item_type":"server","location":"DC1"}' },
      { method: 'GET', path: '/api/inventory/:id', auth: 'JWT', description: 'Get inventory item' },
      { method: 'PUT', path: '/api/inventory/:id', auth: 'JWT', description: 'Update inventory item', body: '{"name":"Server X","location":"DC2"}' },
      { method: 'DELETE', path: '/api/inventory/:id', auth: 'JWT', description: 'Delete inventory item' },
    ],
  },
  {
    title: 'Policies',
    endpoints: [
      { method: 'GET', path: '/api/policies', auth: 'JWT', description: 'List policies' },
      { method: 'POST', path: '/api/policies', auth: 'JWT', description: 'Create policy', body: '{"name":"deny-root","policy_type":"security","scope":"cluster"}' },
      { method: 'GET', path: '/api/policies/:id', auth: 'JWT', description: 'Get policy' },
      { method: 'PUT', path: '/api/policies/:id', auth: 'JWT', description: 'Update policy', body: '{"enabled":false}' },
      { method: 'DELETE', path: '/api/policies/:id', auth: 'JWT', description: 'Delete policy' },
    ],
  },
  {
    title: 'Certificates',
    endpoints: [
      { method: 'GET', path: '/api/certificates', auth: 'JWT', description: 'List certificates' },
      { method: 'POST', path: '/api/certificates', auth: 'JWT', description: 'Issue certificate', body: '{"name":"api","common_name":"api.local","validity_days":365}' },
      { method: 'POST', path: '/api/certificates/:id/revoke', auth: 'JWT', description: 'Revoke certificate' },
    ],
  },
  {
    title: 'Configuration',
    endpoints: [
      { method: 'GET', path: '/api/config', auth: 'JWT', description: 'List config entries' },
      { method: 'POST', path: '/api/config', auth: 'JWT', description: 'Set config key', body: '{"key":"alerts.cpu_threshold","value":85,"group":"monitoring"}' },
      { method: 'GET', path: '/api/config/:key', auth: 'JWT', description: 'Get config by key' },
    ],
  },
  {
    title: 'Backups & Audit',
    endpoints: [
      { method: 'GET', path: '/api/backups', auth: 'JWT', description: 'List control-plane backups' },
      { method: 'POST', path: '/api/backups', auth: 'JWT', description: 'Create backup', body: '{"name":"manual","backup_type":"full"}' },
      { method: 'POST', path: '/api/backups/:id/restore', auth: 'JWT', description: 'Restore a backup' },
      { method: 'DELETE', path: '/api/backups/:id', auth: 'JWT', description: 'Delete a backup' },
      { method: 'GET', path: '/api/audit', auth: 'JWT', description: 'List audit events' },
    ],
  },
  {
    title: 'Plugins',
    endpoints: [
      { method: 'GET', path: '/api/plugins', auth: 'JWT', description: 'List installed plugins' },
      { method: 'POST', path: '/api/plugins', auth: 'JWT', description: 'Install plugin', body: '{"name":"custom-hook","version":"1.0.0","category":"automation"}' },
      { method: 'PUT', path: '/api/plugins/:id', auth: 'JWT', description: 'Enable/disable or update config', body: '{"enabled":true}' },
      { method: 'DELETE', path: '/api/plugins/:id', auth: 'JWT', description: 'Uninstall plugin' },
    ],
  },
  {
    title: 'Notifications',
    endpoints: [
      { method: 'GET', path: '/api/notifications/preferences', auth: 'JWT', description: 'Get notification preferences' },
      { method: 'PUT', path: '/api/notifications/preferences', auth: 'JWT', description: 'Update notification preferences', body: '{"email_alerts":true,"email_digest":false,"browser_alerts":true,"slack_webhook":null}' },
    ],
  },
];

function methodColor(method: string): string {
  switch (method) {
    case 'GET': return 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400';
    case 'POST': return 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-400';
    case 'PUT': return 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400';
    case 'DELETE': return 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400';
    default: return 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-400';
  }
}

function EndpointRow({ endpoint }: { endpoint: Endpoint }) {
  const [copied, setCopied] = useState(false);

  const copyToClipboard = async (text: string) => {
    try { await navigator.clipboard.writeText(text); setCopied(true); setTimeout(() => setCopied(false), 2000); } catch { /* ignore */ }
  };

  return (
    <div className="rounded-lg border border-slate-200 bg-white p-4 dark:border-slate-700 dark:bg-slate-800/50">
      <div className="flex items-start gap-3">
        <span className={`shrink-0 rounded px-2 py-0.5 text-xs font-semibold font-mono ${methodColor(endpoint.method)}`}>
          {endpoint.method}
        </span>
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2">
            <code className="text-sm font-mono text-slate-900 dark:text-white break-all">{endpoint.path}</code>
            <button onClick={() => copyToClipboard(`${endpoint.method} ${endpoint.path}`)} className="shrink-0 rounded p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-600 dark:hover:bg-slate-700">
              {copied ? <Check className="h-3.5 w-3.5 text-emerald-500" /> : <Copy className="h-3.5 w-3.5" />}
            </button>
          </div>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">{endpoint.description}</p>
          <div className="mt-1 flex items-center gap-2">
            <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
              endpoint.auth === 'No' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' : 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400'
            }`}>
              {endpoint.auth === 'No' ? 'Public' : 'JWT'}
            </span>
          </div>
          {endpoint.body && (
            <div className="mt-2">
              <p className="text-xs font-medium text-slate-500 dark:text-slate-400 mb-1">Request Body:</p>
              <pre className="rounded bg-slate-100 p-2 text-xs font-mono text-slate-800 dark:bg-slate-900 dark:text-slate-300 overflow-x-auto">{endpoint.body}</pre>
            </div>
          )}
          {endpoint.response && (
            <div className="mt-2">
              <p className="text-xs font-medium text-slate-500 dark:text-slate-400 mb-1">Response:</p>
              <pre className="rounded bg-slate-100 p-2 text-xs font-mono text-slate-800 dark:bg-slate-900 dark:text-slate-300 overflow-x-auto">{endpoint.response}</pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

function CategorySection({ category, defaultOpen }: { category: typeof categories[0]; defaultOpen: boolean }) {
  const [open, setOpen] = useState(defaultOpen);

  return (
    <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
      <button onClick={() => setOpen(!open)} className="flex w-full items-center justify-between px-6 py-4 hover:bg-slate-50 dark:hover:bg-slate-800/50">
        <div className="flex items-center gap-2">
          {open ? <ChevronDown className="h-4 w-4 text-slate-400" /> : <ChevronRight className="h-4 w-4 text-slate-400" />}
          <h2 className="text-lg font-semibold text-slate-900 dark:text-white">{category.title}</h2>
          <span className="rounded-full bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">{category.endpoints.length}</span>
        </div>
      </button>
      {open && (
        <div className="border-t border-slate-200 px-6 py-4 space-y-3 dark:border-slate-800">
          {category.endpoints.map((ep, i) => (
            <EndpointRow key={i} endpoint={ep} />
          ))}
        </div>
      )}
    </div>
  );
}

export function ApiDocsPage() {
  return (
    <div className="space-y-6">
      <div className="flex items-center gap-3">
        <BookOpen className="h-6 w-6 text-indigo-500" />
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">API Documentation</h1>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
        <p className="text-sm text-slate-600 dark:text-slate-400 mb-4">
          Base URL: <code className="rounded bg-slate-100 px-2 py-0.5 text-sm font-mono text-indigo-600 dark:bg-slate-800 dark:text-indigo-400">http://localhost:8080</code>
        </p>
        <p className="text-sm text-slate-600 dark:text-slate-400">
          All protected endpoints require a <code className="rounded bg-slate-100 px-2 py-0.5 text-xs font-mono dark:bg-slate-800">Authorization: Bearer &lt;token&gt;</code> header.
          Get a token by calling <code className="rounded bg-slate-100 px-2 py-0.5 text-xs font-mono dark:bg-slate-800">POST /api/auth/login</code> with <code className="rounded bg-slate-100 px-2 py-0.5 text-xs font-mono dark:bg-slate-800">admin/admin123</code>.
        </p>
      </div>

      <div className="space-y-4">
        {categories.map((cat, i) => (
          <CategorySection key={i} category={cat} defaultOpen={i < 3} />
        ))}
      </div>
    </div>
  );
}
