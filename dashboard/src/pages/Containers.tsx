import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useContainers } from '../hooks/useContainers';
import { ContainerTable } from '../components/tables/ContainerTable';
import type { Container } from '../types/container';

export function ContainersPage() {
  const navigate = useNavigate();
  const { containers, loading, error, fetchContainers } = useContainers();

  useEffect(() => {
    fetchContainers();
  }, [fetchContainers]);

  const handleSelect = (container: Container) => {
    navigate(`/containers/${container.id}`);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Containers</h1>
      </div>

      {loading && containers.length === 0 && (
        <div className="rounded-xl border border-slate-200 bg-white p-12 text-center dark:border-slate-800 dark:bg-slate-900">
          <p className="text-slate-400">Loading containers...</p>
        </div>
      )}

      {error && (
        <div className="rounded-xl border border-red-200 bg-red-50 p-6 dark:border-red-800 dark:bg-red-900/20">
          <p className="text-red-600 dark:text-red-400">Error: {error}</p>
        </div>
      )}

      {!loading && !error && (
        <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
          <ContainerTable containers={containers} onSelect={handleSelect} />
        </div>
      )}
    </div>
  );
}
