import { Link } from 'react-router-dom';
import { TriangleAlert } from 'lucide-react';

export function NotFoundPage() {
  return (
    <div className="flex min-h-screen items-center justify-center bg-slate-50 dark:bg-slate-950">
      <div className="flex flex-col items-center text-center">
        <TriangleAlert className="mb-6 h-16 w-16 text-amber-500" />
        <h1 className="text-4xl font-bold text-slate-900 dark:text-white">404</h1>
        <p className="mt-2 text-lg text-slate-600 dark:text-slate-400">Page not found</p>
        <Link
          to="/dashboard"
          className="mt-6 rounded-lg bg-indigo-600 px-6 py-2.5 text-sm font-medium text-white transition-colors hover:bg-indigo-700"
        >
          Back to Dashboard
        </Link>
      </div>
    </div>
  );
}
