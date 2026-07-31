import type { Node } from '@/types/node';
import { Badge } from '@/components/common/Badge';
import { cn } from '@/lib/utils';

interface NodeTableProps {
  nodes: Node[];
  onSelect?: (node: Node) => void;
}

const statusVariant: Record<string, 'success' | 'warning' | 'danger' | 'default'> = {
  online: 'success',
  warning: 'warning',
  error: 'danger',
  offline: 'default',
};

function Bar({ value, color }: { value: number; color: string }) {
  return (
    <div className="flex items-center gap-2">
      <div className="h-2 flex-1 rounded-full bg-slate-200 dark:bg-slate-700">
        <div
          className={cn('h-2 rounded-full transition-all', color)}
          style={{ width: `${Math.min(value, 100)}%` }}
        />
      </div>
      <span className="w-10 text-right text-xs text-slate-500 dark:text-slate-400">
        {value.toFixed(0)}%
      </span>
    </div>
  );
}

export function NodeTable({ nodes, onSelect }: NodeTableProps) {
  if (!nodes.length) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
        <p className="text-sm">No nodes registered</p>
      </div>
    );
  }
  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-slate-200 dark:border-slate-700">
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Hostname</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">IP</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Status</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">CPU</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">RAM</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Disk</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Last Seen</th>
          </tr>
        </thead>
        <tbody>
          {nodes.map((node) => (
            <tr
              key={node.id}
              onClick={() => onSelect?.(node)}
              className={cn(
                'border-b border-slate-100 transition-colors dark:border-slate-800',
                onSelect && 'cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50',
              )}
            >
              <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{node.hostname}</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400 font-mono">{node.ip_address}</td>
              <td className="px-4 py-3">
                <Badge variant={statusVariant[node.status] || 'default'}>
                  {node.status}
                </Badge>
              </td>
              <td className="px-4 py-3">
                <Bar value={node.cpu_percent} color="bg-blue-500" />
              </td>
              <td className="px-4 py-3">
                <Bar value={node.ram_percent} color="bg-emerald-500" />
              </td>
              <td className="px-4 py-3">
                <Bar value={node.disk_percent} color="bg-violet-500" />
              </td>
              <td className="px-4 py-3 text-slate-500 dark:text-slate-400 text-xs">
                {new Date(node.last_seen_at).toLocaleString()}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
