import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import { API_URL } from './api-url';

export interface AuthUser {
  id?: string;
  username: string;
  email?: string;
  roles?: string[];
}

interface AuthContextValue {
  token: string | null;
  user: AuthUser | null;
  isAuthenticated: boolean;
  login: (token: string, user?: AuthUser | null) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

function userFromToken(token: string): AuthUser | null {
  try {
    const payload = JSON.parse(atob(token.split('.')[1] ?? ''));
    return {
      id: payload.sub,
      username: payload.username ?? 'user',
      roles: payload.roles ?? [],
    };
  } catch {
    return null;
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('auth_token'));
  const [user, setUser] = useState<AuthUser | null>(() => {
    const raw = localStorage.getItem('auth_user');
    if (raw) {
      try {
        return JSON.parse(raw) as AuthUser;
      } catch {
        /* fall through */
      }
    }
    const existing = localStorage.getItem('auth_token');
    return existing ? userFromToken(existing) : null;
  });

  const login = useCallback((newToken: string, newUser?: AuthUser | null) => {
    localStorage.setItem('auth_token', newToken);
    setToken(newToken);
    const resolved = newUser ?? userFromToken(newToken) ?? { username: 'admin' };
    localStorage.setItem('auth_user', JSON.stringify(resolved));
    setUser(resolved);
  }, []);

  const logout = useCallback(() => {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('auth_user');
    setToken(null);
    setUser(null);
  }, []);

  const value = useMemo(
    () => ({
      token,
      user,
      isAuthenticated: Boolean(token),
      login,
      logout,
    }),
    [token, user, login, logout],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return ctx;
}

export function authHeaders(): Record<string, string> {
  const token = localStorage.getItem('auth_token');
  return token
    ? { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' }
    : { 'Content-Type': 'application/json' };
}

export async function apiJson<T>(path: string, init?: RequestInit): Promise<T> {
  const url =
    path.startsWith('http') || path.startsWith('/api')
      ? path
      : `${API_URL}${path.startsWith('/') ? path : `/${path}`}`;
  const res = await fetch(url, {
    ...init,
    headers: { ...authHeaders(), ...(init?.headers ?? {}) },
  });
  if (!res.ok) {
    const body = await res.text().catch(() => '');
    throw new Error(body || `Request failed (${res.status})`);
  }
  if (res.status === 204) return undefined as T;
  const text = await res.text();
  if (!text) return undefined as T;
  return JSON.parse(text) as T;
}
