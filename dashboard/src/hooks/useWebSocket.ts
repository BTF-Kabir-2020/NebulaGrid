import { useState, useEffect, useCallback } from 'react';
import { wsClient, defaultWsUrl } from '../lib/ws';

const WS_URL = import.meta.env.VITE_WS_URL ?? defaultWsUrl();

type MessageHandler = (data: unknown) => void;

export function useWebSocket() {
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    if (!token) return;

    wsClient.connect(WS_URL, token);

    const interval = setInterval(() => {
      setConnected(wsClient.connected);
    }, 1000);

    return () => {
      clearInterval(interval);
      wsClient.disconnect();
    };
  }, []);

  const subscribe = useCallback((type: string, handler: MessageHandler) => {
    return wsClient.on(type, handler);
  }, []);

  return { connected, subscribe };
}
