# FERROS ACC Bridge + Operator Monitor

## Status

**BRIDGE-WORKAROUND**: Pre-auth, pre-ADR, pre-safety. This component will be deleted when the native Rust bridge is built.

## What this is

A throwaway launcher and standalone operator monitoring panel for early-stage FERROS agent testing. The bridge launches the ferros shell on LAN and serves a minimal HTML monitoring UI.

## What this is NOT

- Not part of FERROS ACC proper (ACC is being developed separately)
- Not part of the ferros-node crate
- Not following FERROS architectural directives (Card/Deck layout, ADR requirement, style conventions are all lifted)
- Not production-ready
- Not persistent or stateful

## How to run

```bash
cd tools/acc-bridge
node bridge.js [options]
```

### Options

- `--port <n>` — target ferros shell port (default: 4317)
- `--monitor-port <n>` — local monitor HTTP port (default: 4318)
- `--no-open` — do not auto-open browser

### Examples

```bash
# Start shell and open monitor UI
node bridge.js

# Start shell on custom port, monitor on 4320, don't auto-open
node bridge.js --port 4317 --monitor-port 4320 --no-open

# Check if shell is already running, then open monitor
node bridge.js --no-open
```

### What it does

1. Checks if ferros shell is already running on the target port
2. If not, spawns `ferros shell --bind 0.0.0.0 --port <port>`
3. Waits up to 30 seconds for the shell to be ready (polls every 500ms)
4. Starts a simple Node.js HTTP server serving `monitor.html` on a separate port (default: 4318)
5. Attempts to open the monitor URL in the browser (VS Code first, then system default browser)
6. Prints both shell and monitor URLs to stdout
7. Pipes ferros stdout/stderr to the terminal
8. On Ctrl+C or SIGTERM, cleanly shuts down the shell and monitor server

## Monitor UI sections

The monitor.html panel displays:

- **System Status Bar**: Checkpoint state, agent count, deny count, live timestamp, refresh and auto-toggle buttons
- **Agent Roster**: Agent name, version, status (color-coded), and Run/Stop buttons
- **Runway Checklist**: Visual status of runway checkpoint items
- **Deny Log**: Last 20 deny log entries in reverse chronological order
- **RPC Console** (collapsible): Free-form JSON-RPC test harness
- **Profile Quick Actions** (collapsible): Profile init and show shortcuts

## Configuration

Edit the config block at the top of `monitor.html` (lines after `BRIDGE-WORKAROUND CONFIG`):

```js
const FERROS_SHELL_URL = 'http://127.0.0.1:4317'; // Change this if shell is on LAN
const POLL_INTERVAL_MS = 3000;
```

To point at a LAN machine, change `127.0.0.1` to the target IP:

```js
const FERROS_SHELL_URL = 'http://192.168.1.100:4317';
```

## Dependencies

**Zero npm dependencies.** This component uses only Node.js built-in modules (`child_process`, `http`, `fs`, `path`, `net`, `os`, `process`) and vanilla HTML/CSS/JavaScript. No package installation required.

## Scope limitations

This component:
- Does NOT proxy HTTP requests (browser fetches directly from ferros shell)
- Does NOT implement authentication or token validation
- Does NOT use TLS or any security layer
- Does NOT follow FERROS UI conventions or architecture
- Does NOT require an ADR (it is a launch script, not an architectural decision)
- Does NOT get imported into `site/`, `docs/`, `crates/`, or `Cargo.toml`

## Error handling

If the ferros shell becomes unreachable, the monitor displays a red banner at the top and stops polling. Restart the bridge or the shell to resume monitoring.

## When will this be deleted?

This component will be replaced when:

1. A native Rust bridge is designed and implemented
2. The FERROS ACC reaches feature parity with this monitoring panel
3. VS Code or FERROS gains native extension support for embedding web views

Timeline: expected in Q2–Q3 2026.

## What happens on exit?

- Ctrl+C or SIGTERM cleanly shuts down the monitor server and any ferros shell spawned by the bridge
- If the ferros shell was already running before the bridge started, it is left untouched
- No cleanup files are left behind

## Troubleshooting

**"Shell did not start within 30s"**  
Check that `ferros` or `ferros-node` binary is available in `PATH` or `./target/release/`. The bridge tries to find it automatically.

**"Cannot reach ferros shell at http://127.0.0.1:4317 — is it running?"**  
The monitor is running but the shell is not reachable. Start it manually or stop the bridge, start the shell separately, and then start the bridge with `--no-open`.

**"Could not auto-open browser"**  
The bridge tries VS Code, then the system default browser, then prints the URL for manual opening. Copy the printed URL and open it manually in your browser.

## License

Same as FERROS.
