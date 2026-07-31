import { useEffect, useState } from 'react';
import { User, Shield, Bell, Key, Moon, Sun, Copy, Check, X, Plus, Trash2, Eye, EyeOff, Loader2, Save } from 'lucide-react';

interface UserProfile {
  id: string; username: string; email: string; roles: string[];
}

interface ApiToken {
  id: string; name: string; token: string; created_at: string; last_used_at: string | null;
}

interface NotifPrefs {
  email_alerts: boolean; email_digest: boolean; browser_alerts: boolean; slack_webhook: string | null;
}

function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
}

function SettingsSection({ title, icon: Icon, children }: { title: string; icon: React.ElementType; children: React.ReactNode }) {
  return (
    <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900">
      <div className="border-b border-slate-200 px-6 py-4 dark:border-slate-800">
        <div className="flex items-center gap-2">
          <Icon className="h-5 w-5 text-indigo-500" />
          <h2 className="text-lg font-semibold text-slate-900 dark:text-white">{title}</h2>
        </div>
      </div>
      <div className="p-6">{children}</div>
    </div>
  );
}

function InfoRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="grid grid-cols-1 gap-1 sm:grid-cols-3 sm:gap-4">
      <dt className="text-sm font-medium text-slate-500 dark:text-slate-400">{label}</dt>
      <dd className="text-sm text-slate-900 dark:text-white sm:col-span-2">{value}</dd>
    </div>
  );
}

function Toast({ message, type, onClose }: { message: string; type: 'success' | 'error'; onClose: () => void }) {
  useEffect(() => { const t = setTimeout(onClose, 4000); return () => clearTimeout(t); }, [onClose]);
  return (
    <div className={`fixed bottom-4 right-4 z-50 flex items-center gap-2 rounded-lg px-4 py-3 text-sm shadow-lg ${
      type === 'success' ? 'bg-emerald-600 text-white' : 'bg-red-600 text-white'
    }`}>
      {type === 'success' ? <Check className="h-4 w-4" /> : <X className="h-4 w-4" />}
      {message}
    </div>
  );
}

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <button type="button" onClick={() => onChange(!checked)}
      className={`relative inline-flex h-6 w-11 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors ${checked ? 'bg-indigo-500' : 'bg-slate-300 dark:bg-slate-600'}`}>
      <span className={`inline-block h-5 w-5 transform rounded-full bg-white shadow transition-transform ${checked ? 'translate-x-5' : 'translate-x-0'}`} />
    </button>
  );
}

export function SettingsPage() {
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [loading, setLoading] = useState(true);
  const [darkMode, setDarkMode] = useState(() => document.documentElement.classList.contains('dark'));

  const [toast, setToast] = useState<{ message: string; type: 'success' | 'error' } | null>(null);

  // Change password
  const [cpCurrent, setCpCurrent] = useState('');
  const [cpNew, setCpNew] = useState('');
  const [cpConfirm, setCpConfirm] = useState('');
  const [cpSaving, setCpSaving] = useState(false);
  const [showPasswords, setShowPasswords] = useState(false);

  // API tokens
  const [tokens, setTokens] = useState<ApiToken[]>([]);
  const [tokensLoading, setTokensLoading] = useState(false);
  const [newTokenName, setNewTokenName] = useState('');
  const [creatingToken, setCreatingToken] = useState(false);
  const [createdTokenValue, setCreatedTokenValue] = useState<string | null>(null);

  // Notification prefs
  const [notifPrefs, setNotifPrefs] = useState<NotifPrefs>({ email_alerts: true, email_digest: false, browser_alerts: true, slack_webhook: null });
  const [notifSaving, setNotifSaving] = useState(false);
  const [notifLoaded, setNotifLoaded] = useState(false);

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const res = await fetch('/api/auth/me', { headers: authHeaders() });
        if (res.ok) setProfile(await res.json());
      } catch { /* ignore */ } finally { setLoading(false); }
    };
    fetchProfile();
  }, []);

  const fetchTokens = async () => {
    setTokensLoading(true);
    try {
      const res = await fetch('/api/auth/tokens', { headers: authHeaders() });
      if (res.ok) setTokens(await res.json());
    } catch { /* ignore */ } finally { setTokensLoading(false); }
  };

  const fetchNotifPrefs = async () => {
    if (notifLoaded) return;
    try {
      const res = await fetch('/api/notifications/preferences', { headers: authHeaders() });
      if (res.ok) { setNotifPrefs(await res.json()); setNotifLoaded(true); }
    } catch { /* ignore */ }
  };

  useEffect(() => { fetchTokens(); fetchNotifPrefs(); }, []);

  const toggleDarkMode = () => {
    const next = !darkMode; setDarkMode(next);
    document.documentElement.classList.toggle('dark', next);
    localStorage.setItem('theme', next ? 'dark' : 'light');
  };

  const handleChangePassword = async (e: React.FormEvent) => {
    e.preventDefault();
    if (cpNew !== cpConfirm) { setToast({ message: 'New passwords do not match', type: 'error' }); return; }
    if (cpNew.length < 6) { setToast({ message: 'New password must be at least 6 characters', type: 'error' }); return; }
    setCpSaving(true);
    try {
      const res = await fetch('/api/auth/change-password', {
        method: 'POST', headers: authHeaders(),
        body: JSON.stringify({ current_password: cpCurrent, new_password: cpNew }),
      });
      if (res.ok) {
        setToast({ message: 'Password changed successfully', type: 'success' });
        setCpCurrent(''); setCpNew(''); setCpConfirm('');
      } else {
        const err = res.status === 401 ? 'Current password is incorrect' : 'Failed to change password';
        setToast({ message: err, type: 'error' });
      }
    } catch { setToast({ message: 'Network error', type: 'error' }); } finally { setCpSaving(false); }
  };

  const handleCreateToken = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTokenName.trim()) { setToast({ message: 'Token name is required', type: 'error' }); return; }
    setCreatingToken(true);
    try {
      const res = await fetch('/api/auth/tokens', {
        method: 'POST', headers: authHeaders(),
        body: JSON.stringify({ name: newTokenName.trim() }),
      });
      if (res.ok) {
        const data = await res.json();
        setCreatedTokenValue(data.token);
        setNewTokenName('');
        fetchTokens();
      } else {
        setToast({ message: 'Failed to create token', type: 'error' });
      }
    } catch { setToast({ message: 'Network error', type: 'error' }); } finally { setCreatingToken(false); }
  };

  const handleRevokeToken = async (id: string) => {
    try {
      const res = await fetch(`/api/auth/tokens/${id}`, { method: 'DELETE', headers: authHeaders() });
      if (res.ok) { setToast({ message: 'Token revoked', type: 'success' }); fetchTokens(); }
      else { setToast({ message: 'Failed to revoke token', type: 'error' }); }
    } catch { setToast({ message: 'Network error', type: 'error' }); }
  };

  const handleCopyToken = async (token: string) => {
    try {
      await navigator.clipboard.writeText(token);
      setToast({ message: 'Token copied to clipboard', type: 'success' });
    } catch { setToast({ message: 'Failed to copy', type: 'error' }); }
  };

  const handleSaveNotifPrefs = async () => {
    setNotifSaving(true);
    try {
      const res = await fetch('/api/notifications/preferences', {
        method: 'PUT', headers: authHeaders(),
        body: JSON.stringify({
          ...notifPrefs,
          slack_webhook: notifPrefs.slack_webhook?.trim() ? notifPrefs.slack_webhook.trim() : null,
        }),
      });
      if (res.ok) setToast({ message: 'Notification preferences saved', type: 'success' });
      else setToast({ message: 'Failed to save preferences', type: 'error' });
    } catch { setToast({ message: 'Network error', type: 'error' }); } finally { setNotifSaving(false); }
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-slate-900 dark:text-white">Settings</h1>

      {toast && <Toast message={toast.message} type={toast.type} onClose={() => setToast(null)} />}

      <SettingsSection title="Profile" icon={User}>
        {loading ? <p className="text-sm text-slate-400">Loading profile...</p> : profile ? (
          <dl className="space-y-4">
            <InfoRow label="Username" value={profile.username} />
            <InfoRow label="Email" value={profile.email} />
            <InfoRow label="Roles" value={profile.roles.join(', ') || '—'} />
            <InfoRow label="User ID" value={profile.id} />
          </dl>
        ) : <p className="text-sm text-slate-400">Could not load profile.</p>}
      </SettingsSection>

      <SettingsSection title="Appearance" icon={darkMode ? Moon : Sun}>
        <div className="flex items-center justify-between">
          <div>
            <p className="text-sm font-medium text-slate-900 dark:text-white">Dark Mode</p>
            <p className="text-xs text-slate-500 dark:text-slate-400">Toggle between light and dark theme</p>
          </div>
          <Toggle checked={darkMode} onChange={toggleDarkMode} />
        </div>
      </SettingsSection>

      <SettingsSection title="Security" icon={Shield}>
        <form onSubmit={handleChangePassword} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Current Password</label>
            <div className="relative">
              <input type={showPasswords ? 'text' : 'password'} value={cpCurrent} onChange={e => setCpCurrent(e.target.value)}
                className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 pr-10 text-sm text-slate-900 dark:border-slate-600 dark:bg-slate-800 dark:text-white" required />
              <button type="button" onClick={() => setShowPasswords(!showPasswords)} className="absolute right-2 top-1/2 -translate-y-1/2 text-slate-400 hover:text-slate-600">
                {showPasswords ? <EyeOff className="h-4 w-4" /> : <Eye className="h-4 w-4" />}
              </button>
            </div>
          </div>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">New Password</label>
              <input type={showPasswords ? 'text' : 'password'} value={cpNew} onChange={e => setCpNew(e.target.value)}
                className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 dark:border-slate-600 dark:bg-slate-800 dark:text-white" required minLength={6} />
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Confirm New Password</label>
              <input type={showPasswords ? 'text' : 'password'} value={cpConfirm} onChange={e => setCpConfirm(e.target.value)}
                className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 dark:border-slate-600 dark:bg-slate-800 dark:text-white" required minLength={6} />
            </div>
          </div>
          <button type="submit" disabled={cpSaving}
            className="inline-flex items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50">
            {cpSaving && <Loader2 className="h-4 w-4 animate-spin" />}
            <Save className="h-4 w-4" /> Change Password
          </button>
        </form>
      </SettingsSection>

      <SettingsSection title="Notifications" icon={Bell}>
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div><p className="text-sm font-medium text-slate-900 dark:text-white">Email Alerts</p><p className="text-xs text-slate-500 dark:text-slate-400">Receive alert emails for critical issues</p></div>
            <Toggle checked={notifPrefs.email_alerts} onChange={v => setNotifPrefs(p => ({ ...p, email_alerts: v }))} />
          </div>
          <div className="flex items-center justify-between">
            <div><p className="text-sm font-medium text-slate-900 dark:text-white">Email Digest</p><p className="text-xs text-slate-500 dark:text-slate-400">Weekly summary of all alerts and activity</p></div>
            <Toggle checked={notifPrefs.email_digest} onChange={v => setNotifPrefs(p => ({ ...p, email_digest: v }))} />
          </div>
          <div className="flex items-center justify-between">
            <div><p className="text-sm font-medium text-slate-900 dark:text-white">Browser Notifications</p><p className="text-xs text-slate-500 dark:text-slate-400">Show notifications in the dashboard</p></div>
            <Toggle checked={notifPrefs.browser_alerts} onChange={v => setNotifPrefs(p => ({ ...p, browser_alerts: v }))} />
          </div>
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm font-medium text-slate-900 dark:text-white">Slack notifications (optional)</p>
              <p className="text-xs text-slate-500 dark:text-slate-400">Leave off unless you have a webhook URL</p>
            </div>
            <Toggle
              checked={Boolean(notifPrefs.slack_webhook)}
              onChange={(v) =>
                setNotifPrefs((p) => ({
                  ...p,
                  slack_webhook: v ? p.slack_webhook || '' : null,
                }))
              }
            />
          </div>
          {notifPrefs.slack_webhook !== null && (
            <div>
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">Slack Webhook URL</label>
              <input
                type="url"
                value={notifPrefs.slack_webhook || ''}
                onChange={(e) => setNotifPrefs((p) => ({ ...p, slack_webhook: e.target.value || '' }))}
                placeholder="https://hooks.slack.com/services/..."
                className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 dark:border-slate-600 dark:bg-slate-800 dark:text-white"
              />
              <p className="mt-1 text-xs text-slate-500">Optional — clear and turn off to disable Slack</p>
            </div>
          )}
          <button onClick={handleSaveNotifPrefs} disabled={notifSaving}
            className="inline-flex items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50">
            {notifSaving && <Loader2 className="h-4 w-4 animate-spin" />}
            <Save className="h-4 w-4" /> Save Preferences
          </button>
        </div>
      </SettingsSection>

      <SettingsSection title="API Access" icon={Key}>
        <div className="space-y-4">
          {createdTokenValue && (
            <div className="rounded-lg border border-emerald-200 bg-emerald-50 p-4 dark:border-emerald-800 dark:bg-emerald-900/20">
              <p className="text-sm font-medium text-emerald-800 dark:text-emerald-300 mb-2">Token created! Copy it now — it won't be shown again.</p>
              <div className="flex items-center gap-2">
                <code className="flex-1 rounded bg-white px-3 py-2 text-xs font-mono text-slate-900 dark:bg-slate-800 dark:text-white border border-emerald-200 dark:border-emerald-800 break-all">{createdTokenValue}</code>
                <button onClick={() => handleCopyToken(createdTokenValue)} className="shrink-0 rounded-lg bg-emerald-600 p-2 text-white hover:bg-emerald-700">
                  <Copy className="h-4 w-4" />
                </button>
                <button onClick={() => setCreatedTokenValue(null)} className="shrink-0 rounded-lg bg-slate-600 p-2 text-white hover:bg-slate-700">
                  <X className="h-4 w-4" />
                </button>
              </div>
            </div>
          )}

          <form onSubmit={handleCreateToken} className="flex items-end gap-2">
            <div className="flex-1">
              <label className="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-1">New Token Name</label>
              <input type="text" value={newTokenName} onChange={e => setNewTokenName(e.target.value)}
                placeholder="e.g. CI/CD Pipeline"
                className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 dark:border-slate-600 dark:bg-slate-800 dark:text-white" />
            </div>
            <button type="submit" disabled={creatingToken}
              className="inline-flex items-center gap-2 rounded-lg bg-indigo-600 px-4 py-2 text-sm font-medium text-white hover:bg-indigo-700 disabled:opacity-50">
              {creatingToken ? <Loader2 className="h-4 w-4 animate-spin" /> : <Plus className="h-4 w-4" />}
              Create Token
            </button>
          </form>

          {tokensLoading ? (
            <p className="text-sm text-slate-400">Loading tokens...</p>
          ) : tokens.length === 0 ? (
            <p className="text-sm text-slate-500 dark:text-slate-400">No API tokens yet. Create one above.</p>
          ) : (
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead>
                  <tr className="border-b border-slate-200 dark:border-slate-700">
                    <th className="px-3 py-2 text-left font-medium text-slate-500 dark:text-slate-400">Name</th>
                    <th className="px-3 py-2 text-left font-medium text-slate-500 dark:text-slate-400">Token</th>
                    <th className="px-3 py-2 text-left font-medium text-slate-500 dark:text-slate-400">Created</th>
                    <th className="px-3 py-2 text-left font-medium text-slate-500 dark:text-slate-400">Last Used</th>
                    <th className="px-3 py-2 text-right font-medium text-slate-500 dark:text-slate-400">Action</th>
                  </tr>
                </thead>
                <tbody>
                  {tokens.map(t => (
                    <tr key={t.id} className="border-b border-slate-100 dark:border-slate-800">
                      <td className="px-3 py-2 font-medium text-slate-900 dark:text-white">{t.name}</td>
                      <td className="px-3 py-2 font-mono text-xs text-slate-500">ng_{t.id.slice(0, 8)}...</td>
                      <td className="px-3 py-2 text-xs text-slate-500">{new Date(t.created_at).toLocaleDateString()}</td>
                      <td className="px-3 py-2 text-xs text-slate-500">{t.last_used_at ? new Date(t.last_used_at).toLocaleDateString() : 'Never'}</td>
                      <td className="px-3 py-2 text-right">
                        <button onClick={() => handleRevokeToken(t.id)} className="rounded-lg p-1.5 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20" title="Revoke token">
                          <Trash2 className="h-4 w-4" />
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </SettingsSection>
    </div>
  );
}
