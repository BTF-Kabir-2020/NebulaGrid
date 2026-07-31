import type { Container, ContainerPort } from '@/types/container';
import { Badge } from '@/components/common/Badge';
import { cn } from '@/lib/utils';

interface ContainerTableProps {
  containers: Container[];
  onSelect?: (container: Container) => void;
}

function statusVariant(status: string): 'success' | 'warning' | 'danger' | 'info' | 'default' {
  const s = status.toLowerCase();
  if (s === 'running') return 'success';
  if (s === 'paused') return 'warning';
  if (s === 'restarting') return 'info';
  if (s === 'exited' || s === 'dead') return 'danger';
  return 'default';
}

function formatPorts(ports: string[] | ContainerPort[] | undefined): string {
  if (!ports || ports.length === 0) return '-';
  return ports
    .map((p) => {
      if (typeof p === 'string') return p;
      return p.public_port ? `${p.ip}:${p.public_port}` : `${p.private_port}`;
    })
    .join(', ');
}

export function ContainerTable({ containers, onSelect }: ContainerTableProps) {
  if (!containers.length) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
        <p className="text-sm">No containers found</p>
      </div>
    );
  }
  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-slate-200 dark:border-slate-700">
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Name</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Image</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Status</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Ports</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Created</th>
          </tr>
        </thead>
        <tbody>
          {containers.map((container) => (
            <tr
              key={container.id}
              onClick={() => onSelect?.(container)}
              className={cn(
                'border-b border-slate-100 transition-colors dark:border-slate-800',
                onSelect && 'cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50',
              )}
            >
              <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{container.name}</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400 font-mono text-xs">{container.image}</td>
              <td className="px-4 py-3">
                <Badge variant={statusVariant(container.status)}>
                  {container.status}
                </Badge>
              </td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400 text-xs">
                {formatPorts(container.ports)}
              </td>
              <td className="px-4 py-3 text-slate-500 dark:text-slate-400 text-xs">
                {container.created_at ? new Date(container.created_at).toLocaleDateString() : '-'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
