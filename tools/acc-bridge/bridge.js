#!/usr/bin/env node
// BRIDGE-WORKAROUND
// Status: pre-auth, pre-ADR, pre-safety
// Purpose: bridge launcher + monitor server — NOT part of ferros-node
// Zero npm dependencies — Node built-ins only
// Superseded by: native Rust bridge (not yet designed)
// Do not expand scope without operator approval

const { spawn } = require('child_process');
const http = require('http');
const { createReadStream } = require('fs');
const { resolve } = require('path');
const { execSync } = require('child_process');
const net = require('net');
const os = require('os');

// Configuration
const FERROS_PORT = 4317;
const MONITOR_PORT = 4318;
const SHELL_POLL_TIMEOUT_MS = 30000;
const SHELL_POLL_INTERVAL_MS = 500;

// Parse command line arguments
let shellPort = FERROS_PORT;
let monitorPort = MONITOR_PORT;
let noOpen = false;
let lanMode = false;

for (let i = 2; i < process.argv.length; i++) {
  if (process.argv[i] === '--port' && i + 1 < process.argv.length) {
    shellPort = parseInt(process.argv[++i], 10);
  } else if (process.argv[i] === '--monitor-port' && i + 1 < process.argv.length) {
    monitorPort = parseInt(process.argv[++i], 10);
  } else if (process.argv[i] === '--no-open') {
    noOpen = true;
  } else if (process.argv[i] === '--lan') {
    lanMode = true;
  }
}

/**
 * Check if shell is already running on target port
 */
function isShellRunning(port) {
  return new Promise((resolve) => {
    const socket = net.createConnection({ port, host: '127.0.0.1' });
    socket.on('connect', () => {
      socket.destroy();
      resolve(true);
    });
    socket.on('error', () => {
      resolve(false);
    });
  });
}

/**
 * Poll until shell is ready (max 30s)
 */
async function waitForShell(port) {
  const start = Date.now();
  while (Date.now() - start < SHELL_POLL_TIMEOUT_MS) {
    if (await isShellRunning(port)) {
      return true;
    }
    process.stdout.write('.');
    await new Promise((r) => setTimeout(r, SHELL_POLL_INTERVAL_MS));
  }
  return false;
}

/**
 * Attempt to open URL in browser
 */
async function openBrowser(url) {
  try {
    // Try VS Code first
    try {
      execSync(`code --open-url ${url}`, { stdio: 'ignore' });
      console.log(`[bridge] Opened in VS Code: ${url}`);
      return;
    } catch {
      // Ignore VS Code failure, try system open
    }

    // Try system open command
    const platform = os.platform();
    let openCmd;
    if (platform === 'darwin') {
      openCmd = 'open';
    } else if (platform === 'linux') {
      openCmd = 'xdg-open';
    } else if (platform === 'win32') {
      openCmd = 'start';
    }

    if (openCmd) {
      execSync(`${openCmd} ${url}`, { stdio: 'ignore' });
      console.log(`[bridge] Opened in default browser: ${url}`);
    }
  } catch (e) {
    console.log(`[bridge] Could not auto-open browser. Open manually: ${url}`);
  }
}

/**
 * Serve monitor.html on separate port
 */
function startMonitorServer(port, lanMode = false) {
  const monitorPath = resolve(__dirname, 'monitor.html');
  const bindAddr = lanMode ? '0.0.0.0' : '127.0.0.1';

  const server = http.createServer((req, res) => {
    if (req.url === '/' || req.url === '') {
      res.writeHead(200, {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'no-store',
      });
      createReadStream(monitorPath).pipe(res);
    } else {
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('Not found');
    }
  });

  return new Promise((resolve, reject) => {
    server.listen(port, bindAddr, () => {
      console.log(`[bridge] Monitor UI serving on http://${bindAddr}:${port}/`);
      resolve(server);
    });
    server.on('error', reject);
  });
}

/**
 * Spawn ferros shell
 */
function spawnShell(port) {
  // Try release binary first, fall back to cargo run
  let cmd = 'ferros-node';
  let args = ['shell', '--bind', '0.0.0.0', '--port', port.toString()];

  try {
    // Check if release binary exists
    const { execSync: exec } = require('child_process');
    try {
      exec('./target/release/ferros-node --help', { stdio: 'ignore' });
      cmd = './target/release/ferros-node';
    } catch {
      // Fall back to cargo run
      cmd = 'cargo';
      args = ['run', '--bin', 'ferros-node', '--', ...args];
    }
  } catch {
    // Use ferros-node as fallback
    cmd = 'ferros-node';
  }

  console.log(`[bridge] Spawning: ${cmd} ${args.join(' ')}`);
  const child = spawn(cmd, args, {
    stdio: ['ignore', 'inherit', 'inherit'],
    cwd: process.cwd(),
  });

  return child;
}

/**
 * Main
 */
async function main() {
  console.log('[bridge] ACC Bridge + Operator Monitor');
  console.log(`[bridge] Shell target: http://127.0.0.1:${shellPort}/`);

  // Check if shell is already running
  const alreadyRunning = await isShellRunning(shellPort);
  let shellChild = null;

  if (alreadyRunning) {
    console.log(`[bridge] Shell already running on port ${shellPort}`);
  } else {
    console.log(`[bridge] Shell not running, starting...`);
    shellChild = spawnShell(shellPort);

    process.stdout.write('[bridge] Waiting for shell to be ready');
    const ready = await waitForShell(shellPort);
    console.log('');

    if (!ready) {
      console.error('[bridge] Shell did not start within 30s');
      if (shellChild) shellChild.kill();
      process.exit(1);
    }
    console.log('[bridge] Shell is ready');
  }

  // Start monitor server
  const monitorServer = await startMonitorServer(monitorPort, lanMode);

  const monitorBindAddr = lanMode ? '0.0.0.0' : '127.0.0.1';
  const monitorUrl = `http://${monitorBindAddr}:${monitorPort}/`;
  console.log(`[bridge] Monitor URL: ${monitorUrl}`);
  console.log(`[bridge] Shell URL: http://${lanMode ? '0.0.0.0' : '127.0.0.1'}:${shellPort}/`);

  // Open in browser unless --no-open
  if (!noOpen) {
    await openBrowser(monitorUrl);
  }

  // Handle graceful shutdown
  const handleExit = () => {
    console.log('[bridge] Shutting down...');
    if (shellChild) {
      shellChild.kill();
    }
    if (monitorServer) {
      monitorServer.close();
    }
    process.exit(0);
  };

  process.on('SIGINT', handleExit);
  process.on('SIGTERM', handleExit);
}

main().catch((err) => {
  console.error('[bridge] Error:', err);
  process.exit(1);
});
