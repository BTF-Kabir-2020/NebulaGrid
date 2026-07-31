import { useEffect, useState } from 'react';
import { Box, Layers, Globe, Server } from 'lucide-react';
import { useKubernetes } from '../hooks/useKubernetes';

type Tab = 'pods' | 'deployments' | 'services' | 'nodes';

const tabs: { key: Tab; label: string; icon: typeof Box }[] = [
  { key: 'pods', label: 'Pods', icon: Box },
  { key: 'deployments', label: 'Deployments', icon: Layers },
  { key: 'services', label: 'Services', icon: Globe },
  { key: 'nodes', label: 'Nodes', icon: Server },
];

export function KubernetesPage() {
  const [activeTab, setActiveTab] = useState<Tab>('pods');
  const { pods, deployments, services, nodes, loading, error, refresh } = useKubernetes();

  useEffect(() => {
    void refresh();
  }, [refresh]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Kubernetes</h1>
        <button
          type="button"
          onClick={() => void refresh()}
          className="rounded-lg border border-slate-300 px-3 py-1.5 text-sm text-slate-700 hover:bg-slate-50 dark:border-slate-600 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          Refresh
        </button>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
        <div className="border-b border-slate-200 dark:border-slate-800">
          <div className="flex">
            {tabs.map(({ key, label, icon: Icon }) => (
              <button
                key={key}
                onClick={() => setActiveTab(key)}
                className={`flex items-center gap-2 px-6 py-3 text-sm font-medium transition-colors ${
                  activeTab === key
                    ? 'border-b-2 border-indigo-500 text-indigo-600 dark:text-indigo-400'
                    : 'text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-300'
                }`}
              >
                <Icon className="h-4 w-4" />
                {label}
              </button>
            ))}
          </div>
        </div>

        <div className="p-6">
          {loading && (
            <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
              <Server className="mb-4 h-12 w-12 animate-pulse" />
              <p className="text-lg font-medium">Loading...</p>
            </div>
          )}

          {error && (
            <div className="flex flex-col items-center justify-center py-12 text-red-500">
              <Server className="mb-4 h-12 w-12" />
              <p className="text-lg font-medium">Error</p>
              <p className="mt-1 text-sm">{error}</p>
            </div>
          )}

          {!loading && !error && activeTab === 'pods' && (
            <div>
              {pods.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
                  <Box className="mb-4 h-12 w-12" />
                  <p className="text-lg font-medium">No pods found</p>
                </div>
              ) : (
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-slate-200 dark:border-slate-800">
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Name</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Namespace</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Status</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Node</th>
                        <th className="pb-3 font-medium text-slate-500 dark:text-slate-400">IP</th>
                      </tr>
                    </thead>
                    <tbody>
                      {pods.map((pod) => (
                        <tr key={pod.name} className="border-b border-slate-100 dark:border-slate-800">
                          <td className="py-3 pr-4 text-slate-900 dark:text-white">{pod.name}</td>
                          <td className="py-3 pr-4 text-slate-600 dark:text-slate-400">{pod.namespace}</td>
                          <td className="py-3 pr-4">
                            <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                              pod.status === 'Running' ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400' :
                              pod.status === 'Pending' ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400' :
                              'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                            }`}>
                              {pod.status}
                            </span>
                          </td>
                          <td className="py-3 pr-4 font-mono text-xs text-slate-600 dark:text-slate-400">{pod.node}</td>
                          <td className="py-3 font-mono text-xs text-slate-600 dark:text-slate-400">{pod.ip}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          )}

          {!loading && !error && activeTab === 'deployments' && (
            <div>
              {deployments.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
                  <Layers className="mb-4 h-12 w-12" />
                  <p className="text-lg font-medium">No deployments found</p>
                </div>
              ) : (
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-slate-200 dark:border-slate-800">
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Name</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Namespace</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Replicas</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Available</th>
                        <th className="pb-3 font-medium text-slate-500 dark:text-slate-400">Image</th>
                      </tr>
                    </thead>
                    <tbody>
                      {deployments.map((dep) => (
                        <tr key={dep.name} className="border-b border-slate-100 dark:border-slate-800">
                          <td className="py-3 pr-4 text-slate-900 dark:text-white">{dep.name}</td>
                          <td className="py-3 pr-4 text-slate-600 dark:text-slate-400">{dep.namespace}</td>
                          <td className="py-3 pr-4 text-slate-900 dark:text-white">{dep.replicas}</td>
                          <td className="py-3 pr-4">
                            <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                              dep.available === dep.replicas
                                ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400'
                                : 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-400'
                            }`}>
                              {dep.available}
                            </span>
                          </td>
                          <td className="py-3 font-mono text-xs text-slate-600 dark:text-slate-400">{dep.image}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          )}

          {!loading && !error && activeTab === 'services' && (
            <div>
              {services.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
                  <Globe className="mb-4 h-12 w-12" />
                  <p className="text-lg font-medium">No services found</p>
                </div>
              ) : (
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-slate-200 dark:border-slate-800">
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Name</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Namespace</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Type</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Cluster IP</th>
                        <th className="pb-3 font-medium text-slate-500 dark:text-slate-400">Ports</th>
                      </tr>
                    </thead>
                    <tbody>
                      {services.map((svc) => (
                        <tr key={svc.name} className="border-b border-slate-100 dark:border-slate-800">
                          <td className="py-3 pr-4 text-slate-900 dark:text-white">{svc.name}</td>
                          <td className="py-3 pr-4 text-slate-600 dark:text-slate-400">{svc.namespace}</td>
                          <td className="py-3 pr-4">
                            <span className="inline-flex items-center rounded-full bg-indigo-100 px-2 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-900/30 dark:text-indigo-400">
                              {svc.type}
                            </span>
                          </td>
                          <td className="py-3 pr-4 font-mono text-xs text-slate-600 dark:text-slate-400">{svc.cluster_ip}</td>
                          <td className="py-3 font-mono text-xs text-slate-600 dark:text-slate-400">{svc.ports.join(', ')}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          )}

          {!loading && !error && activeTab === 'nodes' && (
            <div>
              {nodes.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
                  <Server className="mb-4 h-12 w-12" />
                  <p className="text-lg font-medium">No nodes found</p>
                </div>
              ) : (
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-slate-200 dark:border-slate-800">
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Name</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Status</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Version</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">CPU</th>
                        <th className="pb-3 pr-4 font-medium text-slate-500 dark:text-slate-400">Memory</th>
                        <th className="pb-3 font-medium text-slate-500 dark:text-slate-400">Pods</th>
                      </tr>
                    </thead>
                    <tbody>
                      {nodes.map((node) => (
                        <tr key={node.name} className="border-b border-slate-100 dark:border-slate-800">
                          <td className="py-3 pr-4 text-slate-900 dark:text-white">{node.name}</td>
                          <td className="py-3 pr-4">
                            <span className={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                              node.status === 'Ready' ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400' :
                              'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-400'
                            }`}>
                              {node.status}
                            </span>
                          </td>
                          <td className="py-3 pr-4 text-xs text-slate-600 dark:text-slate-400">{node.version}</td>
                          <td className="py-3 pr-4 text-sm text-slate-900 dark:text-white">{node.cpu_capacity}</td>
                          <td className="py-3 pr-4 text-sm text-slate-900 dark:text-white">{node.memory_capacity}</td>
                          <td className="py-3 text-sm text-slate-900 dark:text-white">{node.pod_count}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
