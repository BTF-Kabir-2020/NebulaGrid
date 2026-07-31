import { Routes, Route, Navigate } from 'react-router-dom';
import { LoginPage } from './pages/Login';
import { DashboardPage } from './pages/Dashboard';
import { ServersPage } from './pages/Servers';
import { ServerDetailPage } from './pages/ServerDetail';
import { ContainersPage } from './pages/Containers';
import { ContainerDetailPage } from './pages/ContainerDetail';
import { VmsPage } from './pages/Vms';
import { VmDetailPage } from './pages/VmDetail';
import { KubernetesPage } from './pages/Kubernetes';
import { NetworksPage } from './pages/Networks';
import { StoragePage } from './pages/Storage';
import { JobsPage } from './pages/Jobs';
import { InventoryPage } from './pages/Inventory';
import { PoliciesPage } from './pages/Policies';
import { CertificatesPage } from './pages/Certificates';
import { ConfigPage } from './pages/Config';
import { AlertsPage } from './pages/Alerts';
import { SettingsPage } from './pages/Settings';
import { ApiDocsPage } from './pages/ApiDocs';
import { MonitoringPage } from './pages/Monitoring';
import { LogsPage } from './pages/Logs';
import { BackupsPage } from './pages/Backups';
import { PluginsPage } from './pages/Plugins';
import { NotFoundPage } from './pages/NotFound';
import { AppLayout } from './components/layout/AppLayout';
import { AuthGuard } from './components/guard/AuthGuard';

export default function App() {
  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route element={<AuthGuard />}>
        <Route element={<AppLayout />}>
          <Route path="/dashboard" element={<DashboardPage />} />
          <Route path="/servers" element={<ServersPage />} />
          <Route path="/servers/:id" element={<ServerDetailPage />} />
          <Route path="/containers" element={<ContainersPage />} />
          <Route path="/containers/:id" element={<ContainerDetailPage />} />
          <Route path="/vms" element={<VmsPage />} />
          <Route path="/vms/:id" element={<VmDetailPage />} />
          <Route path="/kubernetes" element={<KubernetesPage />} />
          <Route path="/networks" element={<NetworksPage />} />
          <Route path="/storage" element={<StoragePage />} />
          <Route path="/inventory" element={<InventoryPage />} />
          <Route path="/policies" element={<PoliciesPage />} />
          <Route path="/certificates" element={<CertificatesPage />} />
          <Route path="/config" element={<ConfigPage />} />
          <Route path="/jobs" element={<JobsPage />} />
          <Route path="/monitoring" element={<MonitoringPage />} />
          <Route path="/logs" element={<LogsPage />} />
          <Route path="/backups" element={<BackupsPage />} />
          <Route path="/plugins" element={<PluginsPage />} />
          <Route path="/alerts" element={<AlertsPage />} />
          <Route path="/settings" element={<SettingsPage />} />
          <Route path="/api-docs" element={<ApiDocsPage />} />
          <Route path="/" element={<Navigate to="/dashboard" replace />} />
        </Route>
      </Route>
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
