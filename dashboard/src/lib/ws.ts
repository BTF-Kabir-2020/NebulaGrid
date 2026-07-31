type MessageHandler = (data: unknown) => void;

function defaultWsUrl(): string {
  if (typeof window === 'undefined') return 'ws://localhost:8080/ws';
  const proto = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
  return `${proto}//${window.location.host}/ws`;
}

class WebSocketClient {
  private ws: WebSocket | null = null;
  private handlers = new Map<string, Set<MessageHandler>>();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private url = '';
  private token = '';

  connect(url: string, token: string): void {
    this.url = url;
    this.token = token;
    this.disconnect();
    const sep = url.includes('?') ? '&' : '?';
    this.ws = new WebSocket(`${url}${sep}token=${encodeURIComponent(token)}`);

    this.ws.onopen = () => {
      if (this.reconnectTimer) {
        clearTimeout(this.reconnectTimer);
        this.reconnectTimer = null;
      }
    };

    this.ws.onmessage = (event) => {
      try {
        const msg = JSON.parse(event.data) as { type?: string; data?: unknown };
        const type = msg.type;
        if (type && this.handlers.has(type)) {
          this.handlers.get(type)!.forEach((h) => h(msg.data ?? msg));
        }
      } catch {
        /* ignore parse errors */
      }
    };

    this.ws.onclose = () => {
      this.scheduleReconnect();
    };

    this.ws.onerror = () => {
      this.ws?.close();
    };
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      if (this.url && this.token) this.connect(this.url, this.token);
    }, 5000);
  }

  disconnect(): void {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    if (this.ws) {
      this.ws.onclose = null;
      this.ws.close();
      this.ws = null;
    }
  }

  on(type: string, handler: MessageHandler): () => void {
    if (!this.handlers.has(type)) this.handlers.set(type, new Set());
    this.handlers.get(type)!.add(handler);
    return () => this.handlers.get(type)?.delete(handler);
  }

  get connected(): boolean {
    return this.ws?.readyState === WebSocket.OPEN;
  }
}

export const wsClient = new WebSocketClient();
export { defaultWsUrl };
