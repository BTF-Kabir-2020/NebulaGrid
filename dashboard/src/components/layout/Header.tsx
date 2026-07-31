import { useAuth } from '@/lib/auth';
import { LogOut, User, ChevronRight } from 'lucide-react';
import { useLocation, Link } from 'react-router-dom';

const breadcrumbMap: Record<string, string> = {
  '/dashboard': 'Dashboard',
  '/servers': 'Servers',
  '/containers': 'Containers',
  '/vms': 'Virtual Machines',
  '/kubernetes': 'Kubernetes',
  '/networks': 'Networks',
  '/storage': 'Storage',
  '/inventory': 'Inventory',
  '/policies': 'Policies',
  '/certificates': 'Certificates',
  '/config': 'Configuration',
  '/jobs': 'Jobs',
  '/monitoring': 'Monitoring',
  '/logs': 'Logs',
  '/backups': 'Backups',
  '/plugins': 'Plugins',
  '/alerts': 'Alerts',
  '/settings': 'Settings',
  '/api-docs': 'API Docs',
};

export function Header() {
  const { user, logout } = useAuth();
  const location = useLocation();
  const pathParts = location.pathname.split('/').filter(Boolean);
  const basePath = '/' + (pathParts[0] ?? '');
  const label = breadcrumbMap[basePath] || '';

  return (
    <header className="flex h-14 items-center justify-between border-b border-slate-200 bg-white px-6 dark:border-slate-800 dark:bg-slate-900">
      <div className="flex items-center gap-1 text-sm text-slate-500 dark:text-slate-400">
        <Link to="/dashboard" className="hover:text-slate-700 dark:hover:text-slate-300">Dashboard</Link>
        {pathParts.length > 1 && label && (
          <>
            <ChevronRight size={14} />
            <Link to={basePath} className="hover:text-slate-700 dark:hover:text-slate-300">{label}</Link>
          </>
        )}
        {pathParts.length > 1 && (
          <>
            <ChevronRight size={14} />
            <span className="text-slate-700 dark:text-slate-300">{pathParts[pathParts.length - 1]}</span>
          </>
        )}
      </div>
      <div className="flex items-center gap-4">
        {user && (
          <div className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400">
            <User size={16} />
            {user.username}
          </div>
        )}
        <button
          onClick={logout}
          className="flex items-center gap-1 text-sm text-slate-500 hover:text-red-500 dark:text-slate-400"
        >
          <LogOut size={16} />
          Logout
        </button>
      </div>
    </header>
  );
}
