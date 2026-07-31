import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Server, Container, Monitor, Bell, Activity, MemoryStick as Memory, Wifi } from 'lucide-react';
import { useMetrics } from '../hooks/useMetrics';
import { useNodes } from '../hooks/useNodes';
import { useWebSocket } from '../hooks/useWebSocket';
import type { NodeMetrics } from '../types/node';

export function DashboardPage() {
  const navigate = useNavigate();
  const { overview, loading, fetchOverview } = useMetrics();
  const { nodes, nodeMetrics, fetchNodes, fetchNodeMetrics } = useNodes();
  const { connected, subscribe } = useWebSocket();
  const [liveMetrics, setLiveMetrics] = useState<NodeMetrics[]>([]);

  useEffect(() => {
    fetchOverview();
    fetchNodes();
  }, [fetchOverview, fetchNodes]);

  useEffect(() => {
    if (nodes.length > 0) {
      fetchNodeMetrics(nodes[0].id);
    }
  }, [nodes, fetchNodeMetrics]);

  useEffect(() => {
    setLiveMetrics(nodeMetrics);
  }, [nodeMetrics]);

  useEffect(() => {
    return subscribe('metrics', (data) => {
      const m = data as NodeMetrics;
      if (!m?.node_id) return;
      setLiveMetrics((prev) => {
        const next = [m, ...prev.filter((x) => x.collected_at !== m.collected_at)].slice(0, 20);
        return next;
      });
      fetchOverview();
    });
  }, [subscribe, fetchOverview]);

  const stats = [
    {
      title: 'Nodes Online',
      value: loading ? '-' : String(overview?.online_nodes ?? 0),
      total: overview?.total_nodes,
      icon: Server,
      color: 'text-blue-500',
      bg: 'bg-blue-50 dark:bg-blue-900/20',
      link: '/servers',
    },
    {
      title: 'Containers',
      value: loading ? '-' : String(overview?.total_containers ?? 0),
      icon: Container,
      color: 'text-emerald-500',
      bg: 'bg-emerald-50 dark:bg-emerald-900/20',
      link: '/containers',
    },
    {
      title: 'VMs',
      value: loading ? '-' : String(overview?.total_vms ?? 0),
      icon: Monitor,
      color: 'text-violet-500',
      bg: 'bg-violet-50 dark:bg-violet-900/20',
      link: '/vms',
    },
    {
      title: 'Alerts',
      value: loading ? '-' : String(overview?.active_alerts ?? 0),
      icon: Bell,
      color: 'text-rose-500',
      bg: 'bg-rose-50 dark:bg-rose-900/20',
      link: '/alerts',
    },
  ];

  const chartSource = liveMetrics.length > 0 ? liveMetrics : nodeMetrics;
  const cpuData = chartSource
    .map((m) => ({
      time: new Date(m.collected_at).toLocaleTimeString(),
      value: m.cpu_percent,
    }))
    .reverse();

  const ramData = chartSource
    .map((m) => ({
      time: new Date(m.collected_at).toLocaleTimeString(),
      value: m.ram_percent,
    }))
    .reverse();

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Dashboard</h1>
        <span
          className={`inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-xs font-medium ${
            connected
              ? 'bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-300'
              : 'bg-slate-100 text-slate-500 dark:bg-slate-800 dark:text-slate-400'
          }`}
        >
          <Wifi className="h-3.5 w-3.5" />
          {connected ? 'Live' : 'Polling'}
        </span>
      </div>

      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <button
            key={stat.title}
            onClick={() => navigate(stat.link)}
            className="rounded-xl border border-slate-200 bg-white p-6 text-left transition-all hover:shadow-md dark:border-slate-800 dark:bg-slate-900"
          >
            <div className="flex items-start justify-between">
              <div>
                <p className="text-sm text-slate-500 dark:text-slate-400">{stat.title}</p>
                <p className="mt-1 text-3xl font-bold text-slate-900 dark:text-white">
                  {stat.value}
                </p>
                {stat.total !== undefined && (
                  <p className="mt-0.5 text-xs text-slate-400">of {stat.total} total</p>
                )}
              </div>
              <div className={`rounded-lg p-2 ${stat.bg}`}>
                <stat.icon className={`h-5 w-5 ${stat.color}`} />
              </div>
            </div>
          </button>
        ))}
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
          <div className="mb-4 flex items-center gap-2">
            <Activity className="h-5 w-5 text-blue-500" />
            <h2 className="text-lg font-semibold text-slate-900 dark:text-white">CPU Usage</h2>
            <span className="ml-auto text-2xl font-bold text-blue-500">
              {overview ? `${overview.avg_cpu_percent.toFixed(1)}%` : ''}
            </span>
          </div>
          {cpuData.length > 0 ? (
            <div className="space-y-2">
              {cpuData.map((d, i) => (
                <div key={i} className="flex items-center gap-2 text-sm">
                  <span className="w-20 text-slate-500">{d.time}</span>
                  <div className="h-2 flex-1 rounded-full bg-slate-200 dark:bg-slate-700">
                    <div className="h-2 rounded-full bg-blue-500" style={{ width: `${Math.min(d.value, 100)}%` }} />
                  </div>
                  <span className="w-10 text-right text-slate-600 dark:text-slate-400">{d.value.toFixed(0)}%</span>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex h-32 items-center justify-center text-sm text-slate-400">
              No data available. Connect agents to see metrics.
            </div>
          )}
        </div>

        <div className="rounded-xl border border-slate-200 bg-white p-6 dark:border-slate-800 dark:bg-slate-900">
          <div className="mb-4 flex items-center gap-2">
            <Memory className="h-5 w-5 text-emerald-500" />
            <h2 className="text-lg font-semibold text-slate-900 dark:text-white">RAM Usage</h2>
            <span className="ml-auto text-2xl font-bold text-emerald-500">
              {overview ? `${overview.avg_ram_percent.toFixed(1)}%` : ''}
            </span>
          </div>
          {ramData.length > 0 ? (
            <div className="space-y-2">
              {ramData.map((d, i) => (
                <div key={i} className="flex items-center gap-2 text-sm">
                  <span className="w-20 text-slate-500">{d.time}</span>
                  <div className="h-2 flex-1 rounded-full bg-slate-200 dark:bg-slate-700">
                    <div className="h-2 rounded-full bg-emerald-500" style={{ width: `${Math.min(d.value, 100)}%` }} />
                  </div>
                  <span className="w-10 text-right text-slate-600 dark:text-slate-400">{d.value.toFixed(0)}%</span>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex h-32 items-center justify-center text-sm text-slate-400">
              No data available. Connect agents to see metrics.
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
