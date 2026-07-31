import type { VirtualMachine } from '@/types/vm';
import { Badge } from '@/components/common/Badge';
import { cn } from '@/lib/utils';

interface VmTableProps {
  vms: VirtualMachine[];
  onSelect?: (vm: VirtualMachine) => void;
}

const statusVariant: Record<string, 'success' | 'warning' | 'danger' | 'default'> = {
  running: 'success',
  paused: 'warning',
  stopped: 'default',
};

export function VmTable({ vms, onSelect }: VmTableProps) {
  if (!vms.length) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-slate-400 dark:text-slate-500">
        <p className="text-sm">No virtual machines found</p>
      </div>
    );
  }
  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-slate-200 dark:border-slate-700">
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Name</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">OS</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Status</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">CPU Cores</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">RAM</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">Disk</th>
            <th className="px-4 py-3 text-left font-medium text-slate-500 dark:text-slate-400">IP</th>
          </tr>
        </thead>
        <tbody>
          {vms.map((vm) => (
            <tr
              key={vm.id}
              onClick={() => onSelect?.(vm)}
              className={cn(
                'border-b border-slate-100 transition-colors dark:border-slate-800',
                onSelect && 'cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50',
              )}
            >
              <td className="px-4 py-3 font-medium text-slate-900 dark:text-white">{vm.name}</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400">{vm.os_type}</td>
              <td className="px-4 py-3">
                <Badge variant={statusVariant[vm.status] || 'default'}>
                  {vm.status}
                </Badge>
              </td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400">{vm.cpu_cores}</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400">{vm.ram_mb} MB</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400">{vm.disk_gb} GB</td>
              <td className="px-4 py-3 text-slate-600 dark:text-slate-400 font-mono text-xs">
                {vm.ip_address || '-'}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
