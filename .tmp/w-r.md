ADR-019 Input: Structured Audit — Maangled/workpace-rust
Prefatory Note
The name "workpace-rust" is the product name of a web-based OS shell, not a workspace-management library in the tmux/tiling-WM sense. The repository does not implement workspace switching, spatial pane management, or persistent session topologies. What it does contain is a clean, security-focused architecture for a browser-native OS shell built on Rust/WASM + actix-web, with distinct patterns around module delivery, sandboxing, HUD composition, inter-frame messaging, and session authentication. The following audit extracts those patterns for FERROS relevance.

1. Workspace / Session Model
1.1 Authentication Session (Server-Side)
Files: modules/database/src/sessions.rs (lines 1–135), modules/shared/src/app_state.rs

Sessions are UUID-keyed rows in a PostgreSQL sessions table with a corresponding session_tokens table for bearer tokens:

Code
sessions: (session_id UUID, username TEXT)
session_tokens: (token_id UUID, session_id UUID)
AppState is a single Arc<Mutex<tokio_postgres::Client>> — a global singleton with no per-user in-memory state (shared/src/app_state.rs, lines 5–8). Sessions are retrieved by UUID or by Authorization header key (lines 78–88). On logout, the row is deleted; there is no "workspace snapshot" or checkpoint.

Key observation: There is no concept of workspace-state persistence. Session represents authentication identity, not context/workspace position. The session table carries no layout, no open-module list, no scroll position, no split configuration.

1.2 Client-Side Module State (PageState)
Files: modules/pagedata/src/pagedata.rs (lines 32–45), modules/pagedata/wasm/src/pagedata.rs (lines 38–52)

Each module payload carries a PageState struct:

Rust
pub struct PageState {
    pub user_prefs:    HashMap<String, String>,
    pub form_data:     HashMap<String, String>,
    pub custom_state:  HashMap<String, serde_json::Value>,
}
This is attached to every PageData parcel (module_id, payload, page_state, hash_b64, sig_b64). In practice it is constructed empty on every server response (os/src/main.rs, lines 52–56: all HashMap::new()). The struct is designed for extensibility but is not yet populated. There is no round-trip that restores previous PageState on reconnect.

Key observation: PageState is a declared-but-unused workspace-context carrier. It is the embryo of a workspace context model.

1.3 Module Cache / Version Ledger (IndexedDB)
Files: modules/indexeddb/wasm/src/indexeddb.rs (lines 97–200, 210–271)

Store::open("my-app") opens an IndexedDB database. The modules object store maps module_id → PageData. On each module request:

Client reads cached version string.
Sends REQUEST_MODULE { module_id, version? } over the data WebSocket.
Server responds either { status: "VERIFIED" } (cache fresh) or includes full fresh PageData.
Client updates local cache on receipt.
A PENDING registry (thread_local HashMap<request_id, oneshot::Sender>) correlates responses to outstanding requests. This is the closest mechanism to a "workspace restore" — the local IndexedDB survives tab refresh and provides module continuity across sessions, but only for rendering assets, not for user-interaction state.

Key observation: This is a versioned content-delivery cache, not a workspace persistence layer. Switching between modules is stateless beyond the cached WASM/HTML blob.

1.4 WebSocket Session Registry (Server)
Files: modules/websockets/src/websocket_service.rs (lines 15–36)

Rust
pub struct WebSocketServer {
    sessions: Arc<Mutex<HashMap<usize, actix_ws::Session>>>,
}
Each WS connection is registered by a random usize key. No concept of rooms, named workspaces, or workspace membership. No session eviction, no workspace-linked session groups.

2. Shell Composition
2.1 OS Shell Layout Architecture
Files: modules/os/static/os.html (lines 1–42), modules/os/static/os.css

The shell is a four-layer stack rendered as a full-viewport browser document:

Layer	DOM id	Z-position	Purpose
Background	#bg (img)	z=-1	Blurred SVG backdrop
Page container	#page-container	default	Active module iframe
Window space	#window_layer > #window_space	stacking	Floating popups/windows
Security layer	#security_layer	above windows	System alerts, auth overlays
HUD layer	#hud_layer	z=1000 (fixed)	Persistent chrome
The page-container is positioned absolutely, sized to calc(100vw - 200px) × calc(100vh - 175px) to accommodate the HUD chrome (os.css lines 40–51). When the HUD is hidden, page-container expands to 100vw × 100vh (hud/wasm/src/wasm.rs lines 30–37).

2.2 HUD Zones
Files: modules/hud/static/hud.html, modules/hud/static/hud.css, modules/hud/wasm/src/wasm.rs

The HUD defines four fixed chrome zones:

#top-hud: nav links, announcement banner, diamond (currency) count
#bottom-hud: task list, hand/cards, assistant bar
#left-hud: 4-slot icon navigation (home, messages, profile, about)
#right-hud: cart, store popup, favorites, feed popup
Each zone is position: fixed, managed independently. Show/hide is driven by CSS class assignment. The transitions are defined in hud.css (lines 144–167):

CSS
.hidden-top-hud    { transform: translateY(-200%); }
.hidden-left-hud   { transform: translateX(-200%); }
/* visible-* resets to transform: translate(0) with 0.5s ease */
WASM functions hide_hud() and show_hud() in hud/wasm/src/wasm.rs (lines 7–72) toggle classes on all four zones simultaneously and resize the page container. There is no partial-hide or zone-specific focus management.

2.3 Module Composition via Sandboxed Iframes
Files: modules/pagedata/wasm/src/pagedata.rs (lines 71–107, 112–186), modules/os/wasm/src/rendering.rs, modules/server/static/boot.js, modules/server/src/boot/sandbox.rs

Each module is delivered as a self-contained PageData bundle: { module_id, payload: { version, html, css, js, wasm, assets }, page_state, hash_b64, sig_b64 }. The rendering pipeline:

build_iframe() (pagedata/wasm/src/pagedata.rs:72): Creates a sandboxed iframe using a blob URL. The iframe's srcdoc contains a <script type="module"> that listens for a render postMessage to call render_pagedata().

render_pagedata() (pagedata/wasm/src/pagedata.rs:114): Inside the iframe:

Decodes CSS/JS/WASM from base64 into blob URLs
Patches [data-asset] elements with blob-URL asset URLs
Injects HTML into #root div
Dynamically import(jsUrl).then(m => m.init(wasmUrl))
Revokes blob URLs on window.unload
render_child_page() (rendering.rs:15): Retrieves or creates an iframe by DOM id, loads the PageData from IndexedDB cache (via Store::module()), builds the iframe.

CSP isolation: The OS srcdoc page has its own CSP (os_srcdoc.html lines 6–11):

Code
connect-src wss: blob:; frame-src blob:; script-src 'unsafe-inline' 'wasm-unsafe-eval' blob: data:
The init page has a stricter script-src 'self' CSP.

2.4 Boot Sequence
Files: modules/server/src/main.rs, modules/server/static/init.html, modules/server/static/os_srcdoc.html, modules/server/static/boot.js

The full boot chain:

Code
HTTP GET / → init.html
  └─ <iframe src="/static/OS_SRCDOC.html">
       └─ os_srcdoc.html (atob base64 blobs, import OS JS)
            └─ os_wasm.js init() → main_js()
                 └─ init_ws_bridge() → start_ws_runtime()
                      └─ init_hud() → Store::module("hud") → build_iframe("hud-iframe")
The OS PageData is assembled server-side at startup in os/src/main.rs → generate_os_pagedata() → seal_payload() (Blake3+Ed25519). The signed blob is template-injected into OS_SRCDOC.html via process_pagedata() (server/src/boot/process_pagedata.rs).

2.5 WebSocket Command Surface
Files: modules/websockets/wasm/src/protocol.rs, modules/websockets/wasm/src/runtime.rs, modules/websockets/wasm/src/pipe.rs, modules/websockets/src/data_socket/app_socket.rs

The client command surface is a typed enum:

Rust
pub enum WsCmd {
    AuthLogin { user: String, pass: String },
    AuthLogout,
    DataTx { json: serde_json::Value },
}
pub enum WsResp {
    AuthOk { jwt: String },
    AuthErr(String),
    DataRx { text: String },
    Error(String),
    Raw(serde_json::Value),
}
The runtime (runtime.rs:12) creates:

One rx_cmd dispatcher task (routes by WsCmd variant)
Two WsPipe tasks (auth pipe at /ws/auth/, data pipe at /ws/data/)
One resp_router task that fan-outs all responses to all child iframes via win.post_message(*)
Server-side module routing (app_socket.rs:39–91) is a hardcoded match module_id with arms for "home", "hud", "smart-home". Module discovery is not dynamic.

2.6 postMessage ACL Bridge
Files: modules/os/wasm/src/ws_bridge.rs (lines 14–93)

The OS maintains a IFACE_REGISTRY: thread_local HashMap<u64, &'static str> mapping child Window instance pointers (as u64 addresses) to module name strings. The ACL table is:

Rust
fn acl(module: &str, cmd: &WsCmd) -> bool {
    matches!((module, cmd),
        ("hud",  WsCmd::AuthLogin{..} | WsCmd::AuthLogout) |
        ("main", WsCmd::DataTx{..})
    )
}
Only registered child windows with approved module IDs can emit allowed command types. Unregistered or mismatched sources are logged and dropped. This is a lightweight capability-based access control for the iframe message bus.

2.7 UI Component Primitives
Files: modules/ui_components/src/tabs/tabs.rs, modules/ui_components/src/popups/popup.rs, modules/ui_components/src/cards/

Server-side HTML generation using include_str! templates and str::replace() substitution:

generate_tabs(Vec<TabHeader>, Vec<TabContent>) → HTML string
generate_popup(id, content) → HTML string with id-prefixed controls
Cards: card_types.rs, tab_cards/checklist_card.rs
All component generation is synchronous Rust on the server, producing static HTML fragments. There is no client-side component framework or reactive binding.

2.8 Payload Integrity
Files: modules/pagedata/src/pagedata.rs (lines 62–118)

seal_payload() generates a Blake3 digest over a canonical length-prefixed encoding of all payload fields (css, html, js, version, wasm, assets), then Ed25519-signs the digest. verify_page() recomputes and checks both hash and signature. Tests at lines 122–160 validate the round-trip and tamper-detection. The signing key is generated at startup and written to /tmp/ed25519.secret — a development-mode placeholder.

3. UX-Shell Patterns
3.1 Autohide Overlay Chrome
HUD zones slide off-screen when modules receive focus; explicit hide_hud()/show_hud() API. The page container auto-resizes to reclaim HUD chrome area. This is a "focus mode" affordance: the content surface expands to fill the screen when the user engages a module.

3.2 Layered Z-Stack with Named Slots
Five named z-layers (background, page, window, security, HUD) provide predictable compositional contracts. Each layer has independent visibility, sizing, and overflow semantics. The security/alert layer sits above the window layer — system messages cannot be obscured by application popups.

3.3 Module-as-Payload Delivery Contract
Every presentable unit has the same envelope: { module_id, version, html, css, js, wasm, assets, page_state, hash, sig }. The OS treats this as an opaque signed blob to be rendered in a sandbox. There is no special protocol or lifecycle hook — the module just needs to export a WASM main_js() function.

3.4 Version-Keyed Cache-on-Client
Client-side IndexedDB holds rendered modules keyed by module_id. On next load, the client sends its cached version; the server either confirms (VERIFIED) or pushes a diff. This avoids re-downloading large WASM blobs on every session.

3.5 Bottom Bar as Multi-Surface Dock
#bottom-hud contains task list, hand/cards widget, and AI assistant — three distinct interaction surfaces in a horizontal dock. Each is independently templated (HUD html: {{ task_list_html }}, {{ hand_html }}, {{ assistant_html }}). This supports extension by adding new template slots without changing the underlying layout grid.

3.6 Capability-Scoped postMessage Bus
The postMessage bridge is the only inter-module communication channel. Every message is source-validated against the iframe registry, then checked against a per-module ACL. This enforces least-privilege communication: modules can only emit the commands their role allows.

4. Candidate Classification
ADOPT
Candidate	Classification	Rationale	Primary File(s)
WsCmd/WsResp typed enum protocol	ADOPT	Clean, serializable command surface; maps well to any typed IPC channel regardless of transport. The enum discipline enforces exhaustive handling.	websockets/wasm/src/protocol.rs
Dispatcher + dual-pipe topology	ADOPT	One inbound channel, fan-out to typed pipes, broadcast response fan-in. This pattern is transport-agnostic and directly applicable to FERROS inter-subsystem messaging.	websockets/wasm/src/runtime.rs:12–82
Blake3 + Ed25519 payload seal/verify	ADOPT	Cryptographically tamper-evident module delivery. seal_payload()/verify_page() provide a well-tested integrity primitive with no browser dependencies.	pagedata/src/pagedata.rs:91–118
postMessage capability ACL pattern	ADOPT	Source registry + per-module ACL on a shared message bus is directly applicable to FERROS inter-pane or inter-worklet communication. Prevents privilege escalation through the message bus.	os/wasm/src/ws_bridge.rs:22–41
ADAPT
Candidate	Classification	Rationale	Primary File(s)
PageData as module delivery envelope	ADAPT	The { module_id, version, payload: {html,css,js,wasm,assets}, page_state, hash, sig } structure is a sound module identity + content + integrity contract. FERROS would strip browser-specific fields (html, css, blob assets) and replace them with its own content model, but the outer envelope (versioned, signed, state-carrying) is a strong pattern.	pagedata/src/pagedata.rs:38–45
PageState context carrier	ADAPT	{ user_prefs, form_data, custom_state } is a reasonable session-context schema. FERROS would extend custom_state for workspace topology (open panes, split geometry, cursor position) rather than leaving it empty. This needs to actually be populated and round-tripped, which workspace-rust does not yet do.	pagedata/src/pagedata.rs:32–35
HUD four-zone layout model	ADAPT	Top/bottom/left/right fixed chrome zones with auto-hide are relevant to FERROS if it has a rendered display layer. The concepts survive transport to non-browser contexts (e.g., a terminal-mode HUD could have similar north/south/east/west chrome bands). The CSS implementation is browser-only.	hud/static/hud.html, hud/static/hud.css
Version-keyed module cache with server VERIFIED handshake	ADAPT	The client-version → server-VERIFIED protocol is a clean lazy-update pattern. Applicable to FERROS worklet/module caching with a FERROS-native persistence layer (not IndexedDB).	indexeddb/wasm/src/indexeddb.rs:171–199
Named-slot HUD template composition	ADAPT	Bottom bar with {{ task_list }}, {{ hand }}, {{ assistant }} slots is a declarative extensibility model. FERROS could adopt a similar slot-declaration pattern for shell surface composition without coupling to HTML templates.	hud/static/hud.html
REFERENCE
Candidate	Classification	Rationale	Primary File(s)
Sandboxed iframe module isolation	REFERENCE	The two-level CSP + blob-URL iframe sandbox is a sophisticated browser-native isolation model. It is inspiration-only for FERROS — the relevant principle is "render untrusted module content inside an enforced boundary" — but the mechanism (iframe/CSP/blob URL) is entirely browser-specific.	pagedata/wasm/src/pagedata.rs:72–107, server/static/boot.js
Autohide HUD CSS transitions	REFERENCE	translateY(-200%) slide-off with ease-in-out transitions is a UX precedent for focus-mode expansion. Reference for FERROS UX specs if it has an animated overlay model.	hud/static/hud.css:144–167
Five-layer z-stack (bg/page/window/security/hud)	REFERENCE	The named-layer model with a fixed security-layer above app-layer is a useful UX safety pattern. Reference for FERROS shell surface architecture.	os/static/os.html
Signed cookie with HMAC verification	REFERENCE	CookieManager + Signer provides a session token that does not require a DB round-trip to validate. Precedent for FERROS lightweight session-token design.	websockets/src/cookie_utils/cookie_manager.rs
Server-side HTML component generation	REFERENCE	generate_tabs(), generate_popup() use include_str! + .replace() for template composition. Simple but brittle; reference as a "minimum viable shell template" model, not as an architectural pattern to carry forward.	ui_components/src/tabs/tabs.rs, ui_components/src/popups/popup.rs
DOMPurify HTML sanitization at boot	REFERENCE	HTML is sanitized before sandbox injection (boot.js:20). Principle (sanitize before render in any boundary) is reference for FERROS content ingestion, but DOMPurify is browser-only.	server/static/boot.js:20
Workspace layout HTML comments/intent	REFERENCE	os.html comments describe design intent: "window_space is like pinned popups", "page-container is the window lock for full-page modules", "zooming out zooms only the window layer". These are articulate spatial UX design decisions worth studying for FERROS S5B UX spec.	os/static/os.html:lines 17–40
DISCARD
Candidate	Classification	Rationale	Primary File(s)
IndexedDB module store	DISCARD	Browser-only API. No relevance to FERROS unless FERROS targets WebAssembly-in-browser deployment.	indexeddb/wasm/src/indexeddb.rs
Blob URL asset delivery	DISCARD	URL.createObjectURL() is browser-native. All asset encoding/delivery logic depending on this is browser-only.	pagedata/wasm/src/pagedata.rs:119–132
actix-session cookie middleware	DISCARD	Express-style server-side session cookies are specific to actix-web HTTP.	websockets/src/auth_socket/auth_socket.rs:29–34
Hardcoded match module_id routing	DISCARD	app_socket.rs match arms for "home", "hud", "smart-home" are manual and non-extensible. This is a placeholder not worth carrying forward.	websockets/src/data_socket/app_socket.rs:39–91
Ed25519 key written to /tmp	DISCARD	Startup key-gen writing to /tmp/ed25519.secret is a dev-mode artifact (server/src/main.rs:25–31). Discard the key-management approach; retain the sign/verify logic.	server/src/main.rs:25–31
js_sys::eval for dynamic JS import	DISCARD	eval(import('...')) in render_pagedata() is a browser-WASM bridge hack. No relevance outside the WASM-in-browser context.	pagedata/wasm/src/pagedata.rs:168
HTML/CSS static component templates	DISCARD	include_str! + .replace() for tabs and popups has no server-side rendering or reactivity story. Produces fragile string-surgery HTML.	ui_components/src/tabs/tabs.rs
5. Findings That Should Remain Inspiration-Only
Sandboxed iframe model: The principle of rendering module content inside a hard security boundary (with enforced capability restrictions) is architecturally sound inspiration. But the mechanism — iframe + CSP + blob URL + DOMPurify — is wholly browser-native. Do not attempt to translate this mechanism; translate only the principle (signed payload, enforced render boundary, no shared address space between host shell and module).

HUD CSS animation model: The slide-off transitions are a UX precedent, not a code pattern. Useful as a UX reference document for S5B interaction design, but nothing to extract technically.

os.html layer comment intent: The inline HTML comments in os/static/os.html (lines 13–40) are an unusually well-articulated spatial UX design document embedded in markup. Worth reading in full for S5B. Treat as a design spec, not as code.

6. Findings Worth Translating Only After G3
PageState round-trip persistence: The struct exists but is never populated or restored. A full implementation (serialize on navigate-away, restore on navigate-to) would require the server to store per-user per-module state alongside the session, which requires a schema decision. This is only worth implementing in FERROS after the workspace context model is finalized post-G3.

Module version ledger: The VERIFIED handshake is correct in principle but currently embeds static version strings ("1.0.1" in app_socket.rs:42). A real implementation needs dynamic version management tied to deployment. Translate this pattern only after FERROS has a module deployment and versioning strategy.

postMessage ACL extension: The current ACL (hud → Auth*, main → DataTx) is two entries. Extending this to a policy engine would be non-trivial. Worth translating only once FERROS has identified its inter-module permission surface.

7. Constraints, Mismatches, and Coupling Risks
"Workspace" is a product name, not a WM concept. The repository does not implement workspace switching, session save/restore, or multi-workspace coordination in any sense relevant to a tiling WM or terminal multiplexer model. FERROS must not assume workspace-rust provides that primitive; it does not.

All runtime is in WASM-in-browser. The client-side codebase (*/wasm/src/) is entirely dependent on web_sys, wasm_bindgen, js_sys. Anything relying on Window, Document, IdbDatabase, WebSocket, MessageEvent, Blob, URL cannot be reused outside a browser context. The dependency surface is deep and pervasive.

Single-database singleton state. AppState holds one Arc<Mutex<Client>> shared across all request handlers. There is no per-user, per-workspace, or per-session memory. All state is either in the database or on the client. FERROS should not model its workspace state on this architecture if it needs in-memory workspace coordination between subsystems.

Auth and data are on separate WebSocket endpoints but share a response bus. resp_router in runtime.rs:84 broadcasts all responses with post_message("*") — no filtering by session or module. All iframes receive all responses. This is acceptable for a small hardcoded module set but creates an information-disclosure surface that would need rearchitecting for a FERROS multi-tenant or multi-workspace environment.

Module routing is manual and centralized. The server's app_socket.rs match module_id block means every new module requires a server code change. There is no plugin registry, manifest, or dynamic dispatch. Direct translation to FERROS would create the same maintenance bottleneck.

Ed25519 key lifecycle is incomplete. Keys are generated per-server-startup and are not distributed to clients for verification. verify_page() exists client-side but there is no bootstrapping of the verifying key to the client. The integrity model is aspirationally sound but practically unenforced until key distribution is implemented.

window_layer / window management is a stub. os.html comments describe a floating window system ("items in the window space can be pinned next to or on top of the main container") but no implementation exists. There is no window manager, no drag/resize, no z-order management beyond what the CSS stacking context provides. FERROS should not expect workspace-rust to be a source of window management patterns.

8. Notable Implementation Assumptions Requiring FERROS Caution
Assumption	Risk for FERROS
Browser as runtime: all module execution assumes a DOM, blob URLs, and WASM-in-browser	FERROS must not carry over any web_sys-dependent module execution path
Opaque-origin iframe as the security boundary	No equivalent in non-browser environments; FERROS must define its own module isolation boundary independently
DOMPurify as the trust boundary at render time	Browser HTML sanitization is not a substitute for actual capability restriction; FERROS should not equate sanitization with isolation
window.post_message("*") response broadcast	The "*" targetOrigin is a security antipattern in general postMessage usage; FERROS messaging should use explicit target addressing
PageState.custom_state as an escape hatch	The arbitrary HashMap<String, serde_json::Value> signals that the state model is not finished. FERROS should define a typed workspace state schema, not inherit an open bag.
Boot sequence requires server deliver a signed OS blob at startup	The entire OS is delivered as a single signed payload at startup. This is a "fat client" model that has no equivalent in FERROS unless FERROS defines a similar host-delivered signed worklet bootstrap.
No workspace save/restore path exists	FERROS cannot look to workspace-rust for guidance on workspace checkpoint/restore; it must design that mechanism de novo.
9. Decision-Ready Summary Bullets (FERROS Architecture Review)
Adopt the typed WsCmd/WsResp enum IPC surface. It is transport-agnostic, serializable, and enforces exhaustive dispatch handling. Replace the WebSocket transport with FERROS's native IPC but keep the protocol shape.

Adopt the dispatcher + typed-pipe topology. A single inbound command channel fanning out to purpose-built subsystem pipes (auth, data) with a unified response fan-in is a clean concurrency model applicable to FERROS subsystem routing.

Adopt seal_payload() / verify_page() as the module integrity primitive. Blake3 + Ed25519 over a canonical length-prefixed encoding is well-implemented and tested. Strip the browser-specific PageData fields and apply the seal/verify pattern to FERROS's own module manifest.

Adopt the postMessage ACL architecture as a principle. A source-identity registry + per-source command capability table on a shared message bus is directly relevant to FERROS inter-pane messaging. Reimplement using FERROS-native identity and capability primitives.

Adapt PageData envelope into a FERROS Module Manifest. Keep the outer structure: { module_id, version, payload, context_state, hash, sig }. Strip HTML/CSS/JS/blob assets. Replace page_state with a typed FERROS workspace-context struct (open splits, active focus, scroll/cursor state). Mandate actual population and round-trip from day one — do not leave it as HashMap::new().

Adapt the VERIFIED cache handshake. The { client_version → VERIFIED | fresh_payload } protocol avoids redundant module delivery. Apply to FERROS worklet loading with a FERROS-native persistence layer.

Reference the five-layer z-stack model for FERROS display surface spec. Named layers with predictable stacking contracts (background / content / window / security / chrome) are a sound UX architecture. Translate to FERROS rendering layer semantics if applicable.

Do not translate any client-side browser code. The entire */wasm/src/ tree is not portable. Treat all web_sys, wasm_bindgen, IndexedDB, Blob URL, and DOM manipulation code as browser-only prior art with no direct carry-over value.

Design workspace persistence from scratch. workspace-rust has no workspace save/restore, no multi-workspace coordination, and no spatial layout management. These must be designed de novo for FERROS S6. workspace-rust should not be treated as a prior art source for this feature area.

Flag post_message("*") broadcast as a pattern to avoid. FERROS inter-module responses must be addressed to specific recipients, not broadcast. The unrestricted broadcast in resp_router is a known information-disclosure antipattern.

Treat signed payload delivery (OS-as-blob) as a potential G3 input. The model of delivering the entire OS shell as a server-signed payload at startup is architecturally interesting for FERROS if it needs a hardware-attested worklet bootstrap. Worth revisiting post-G3, not before.