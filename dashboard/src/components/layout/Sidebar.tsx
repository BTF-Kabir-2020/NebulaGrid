import {
  LayoutDashboard,
  Server,
  Container,
  Monitor,
  Boxes,
  Network,
  HardDrive,
  ListTodo,
  Package,
  Shield,
  KeyRound,
  SlidersHorizontal,
  Bell,
  Settings,
  BookOpen,
  ChevronLeft,
  ChevronRight,
  Activity,
  ScrollText,
  DatabaseBackup,
  Puzzle,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { useState } from 'react';
import { NavLink } from 'react-router-dom';

const navItems = [
  { to: '/dashboard', icon: LayoutDashboard, label: 'Dashboard' },
  { to: '/servers', icon: Server, label: 'Servers' },
  { to: '/containers', icon: Container, label: 'Containers' },
  { to: '/vms', icon: Monitor, label: 'VMs' },
  { to: '/kubernetes', icon: Boxes, label: 'Kubernetes' },
  { to: '/networks', icon: Network, label: 'Networks' },
  { to: '/storage', icon: HardDrive, label: 'Storage' },
  { to: '/inventory', icon: Package, label: 'Inventory' },
  { to: '/policies', icon: Shield, label: 'Policies' },
  { to: '/certificates', icon: KeyRound, label: 'Certificates' },
  { to: '/config', icon: SlidersHorizontal, label: 'Config' },
  { to: '/jobs', icon: ListTodo, label: 'Jobs' },
  { to: '/monitoring', icon: Activity, label: 'Monitoring' },
  { to: '/logs', icon: ScrollText, label: 'Logs' },
  { to: '/backups', icon: DatabaseBackup, label: 'Backups' },
  { to: '/plugins', icon: Puzzle, label: 'Plugins' },
  { to: '/alerts', icon: Bell, label: 'Alerts' },
  { to: '/settings', icon: Settings, label: 'Settings' },
  { to: '/api-docs', icon: BookOpen, label: 'API Docs' },
];

export function Sidebar() {
  const [collapsed, setCollapsed] = useState(false);

  return (
    <aside
      className={cn(
        'flex flex-col border-r border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 transition-all duration-200',
        collapsed ? 'w-16' : 'w-60',
      )}
    >
      <div className="flex h-14 items-center justify-between border-b px-4 dark:border-slate-800">
        {!collapsed && (
          <span className="text-lg font-bold text-nebula-600 dark:text-nebula-400">
            NebulaGrid
          </span>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="rounded p-1 hover:bg-slate-100 dark:hover:bg-slate-800"
        >
          {collapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
        </button>
      </div>

      <nav className="flex-1 space-y-1 overflow-y-auto p-2">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              cn(
                'flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors',
                isActive
                  ? 'bg-nebula-50 text-nebula-700 dark:bg-nebula-950 dark:text-nebula-300'
                  : 'text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800',
                collapsed && 'justify-center px-2',
              )
            }
          >
            <item.icon size={20} />
            {!collapsed && <span>{item.label}</span>}
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
