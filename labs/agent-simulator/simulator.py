#!/usr/bin/env python3
import os
import sys
import json
import time
import random
import urllib.request
import urllib.error

GATEWAY = os.environ.get('GATEWAY_URL', 'http://gateway:8080')
API_USER = os.environ.get('API_USER', 'admin')
API_PASS = os.environ.get('API_PASS', 'admin123')
HOSTNAME = os.environ.get('AGENT_HOSTNAME', 'sim-agent')
INTERVAL = int(os.environ.get('INTERVAL_SECONDS', '10'))
VERBOSE = os.environ.get('VERBOSE', 'false').lower() == 'true'

TOKEN = None

def login():
    global TOKEN
    payload = {'username': API_USER, 'password': API_PASS}
    data = json.dumps(payload).encode()
    headers = {'Content-Type': 'application/json'}
    r = urllib.request.Request(f'{GATEWAY}/api/auth/login', data=data, headers=headers, method='POST')
    try:
        resp = urllib.request.urlopen(r)
        result = json.loads(resp.read().decode())
        TOKEN = result['token']
        if VERBOSE:
            print(f'Logged in as {API_USER}')
        return True
    except Exception as e:
        print(f'Login failed: {e}', file=sys.stderr)
        return False

def req(method, path, body=None):
    if not TOKEN:
        print('No token available', file=sys.stderr)
        return None
    url = f'{GATEWAY}{path}'
    data = json.dumps(body).encode() if body else None
    headers = {'Authorization': f'Bearer {TOKEN}', 'Content-Type': 'application/json'}
    r = urllib.request.Request(url, data=data, headers=headers, method=method)
    try:
        resp = urllib.request.urlopen(r)
        return json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        print(f'HTTP {e.code}: {e.read().decode()}', file=sys.stderr)
        return None
    except Exception as e:
        print(f'Error: {e}', file=sys.stderr)
        return None

def register():
    payload = {
        'hostname': HOSTNAME,
        'ip_address': f'10.0.1.{random.randint(20, 50)}',
        'os_name': 'NebulaOS 3.0',
        'os_version': '3.0.1',
        'cpu_cores': random.randint(2, 16),
        'ram_total_bytes': random.randint(8, 64) * 1073741824,
        'disk_total_bytes': random.randint(100, 1000) * 1073741824,
    }
    result = req('POST', '/api/nodes/register', payload)
    if result and 'id' in result:
        return result['id']
    return None

def send_metrics(node_id, snapshot):
    result = req('POST', f'/api/nodes/{node_id}/metrics', snapshot)
    if VERBOSE and result:
        print(f'Sent: CPU={snapshot["cpu_percent"]:.1f}% RAM={snapshot["ram_percent"]:.1f}%')

def generate_snapshot():
    return {
        'cpu_percent': round(random.uniform(10, 95), 1),
        'ram_percent': round(random.uniform(20, 90), 1),
        'ram_used_bytes': random.randint(2, 32) * 1073741824,
        'ram_total_bytes': 32 * 1073741824,
        'disk_percent': round(random.uniform(30, 95), 1),
        'disk_used_bytes': random.randint(50, 500) * 1073741824,
        'disk_total_bytes': 500 * 1073741824,
        'net_rx_bytes': random.randint(1048576, 104857600),
        'net_tx_bytes': random.randint(524288, 52428800),
        'process_count': random.randint(50, 500),
        'load_avg_1min': round(random.uniform(0.5, 8.0), 2),
        'load_avg_5min': round(random.uniform(0.3, 6.0), 2),
        'load_avg_15min': round(random.uniform(0.2, 4.0), 2),
        'collected_at': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
    }

def main():
    print(f'Agent simulator starting — gateway: {GATEWAY}, hostname: {HOSTNAME}, interval: {INTERVAL}s')
    if not login():
        print('Login failed, exiting')
        sys.exit(1)
    node_id = register()
    if not node_id:
        print('Registration failed, exiting')
        sys.exit(1)
    print(f'Registered as node: {node_id}')
    while True:
        snapshot = generate_snapshot()
        send_metrics(node_id, snapshot)
        time.sleep(INTERVAL)

if __name__ == '__main__':
    main()
