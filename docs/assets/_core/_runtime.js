/* ferros-runtime v1.0.0
 *
 * Asset-side IIFE scaffold for all FERROS render assets.
 * Provides messaging, control modes, reduced-motion, accessibility, resize.
 *
 * Contract:
 *   - Assets set data-ferros-asset on <html> tag
 *   - Assets provide #ferros-asset-root element
 *   - Assets optionally define window.onFerrosInit(config) and window.onFerrosUpdate(payload)
 *   - Assets optionally provide #ferros-sr-announce element for screen-reader announcements
 *   - Runtime exposes window.ferrosRuntime namespace — assets NEVER redefine it
 *   - Runtime OWNS root.dataset.control — assets read via CSS [data-control] selectors
 *
 * See: ADR-008 (docs/adr/ADR-008-modular-rendering-system.md)
 */
(function() {
  'use strict';

  /* ═══ BOOTSTRAPPING ═══ */
  var ASSET_NAME = document.documentElement.dataset.ferrosAsset || 'unknown-asset';
  var root       = document.getElementById('ferros-asset-root');
  var isEmbedded = window.parent !== window;
  var params     = new URLSearchParams(window.location.search);
  var control    = params.get('control') || 'demo';
  var _runtimeNonce = null; // A4: nonce from ferros:init, included in all outbound messages

  /* ═══ REDUCED MOTION ═══ */
  var mql = window.matchMedia('(prefers-reduced-motion: reduce)');
  function isReducedMotion() { return mql.matches; }

  /* ═══ MESSAGING — OUTBOUND ═══ */
  function emit(event, payload) {
    var msg = {
      type: 'ferros:event',
      asset: ASSET_NAME,
      event: event,
      payload: payload,
      nonce: _runtimeNonce
    };
    if (isEmbedded) {
      window.parent.postMessage(msg, '*');
    }
    if (root) {
      root.dispatchEvent(new CustomEvent('ferros:event', {
        detail: msg,
        bubbles: true
      }));
    }
  }

  function reportSize() {
    if (!isEmbedded || !root) return;
    window.parent.postMessage({
      type: 'ferros:resize',
      asset: ASSET_NAME,
      width: root.scrollWidth,
      height: root.scrollHeight
    }, '*');
  }

  /* ═══ MESSAGING — INBOUND ═══ */
  window.addEventListener('message', function(e) {
    if (!e.data || !e.data.type) return;

    if (e.data.type === 'ferros:init') {
      var config = e.data.config || {};
      if (config.control) control = config.control;
      if (e.data.nonce) _runtimeNonce = e.data.nonce; // A4: store nonce for outbound messages
      applyControl();
      if (typeof window.onFerrosInit === 'function') {
        window.onFerrosInit(config);
      }
    }

    if (e.data.type === 'ferros:update') {
      /* Normalize: _embed.js wraps in e.data.payload, but some legacy
         callers put data directly on e.data. Try .payload first. */
      var payload = e.data.payload || e.data;
      if (payload.control) {
        control = payload.control;
        applyControl();
      }
      if (typeof window.onFerrosUpdate === 'function') {
        window.onFerrosUpdate(payload);
      }
    }
  });

  /* ═══ CONTROL MODES ═══ */
  function applyControl() {
    if (root) root.dataset.control = control;
  }

  /* ═══ ACCESSIBILITY ═══ */
  function announceToScreenReader(text) {
    var el = document.getElementById('ferros-sr-announce');
    if (!el) return;
    el.textContent = '';
    void el.offsetWidth; /* force reflow to re-trigger aria-live */
    el.textContent = text;
  }

  /* ═══ AUTO-INIT ═══ */
  applyControl();
  reportSize();
  if (typeof ResizeObserver !== 'undefined' && root) {
    new ResizeObserver(function() { reportSize(); }).observe(root);
  }

  /* ═══ EXPORTS ═══ */
  window.ferrosRuntime = {
    root: root,
    isEmbedded: isEmbedded,
    control: function() { return control; },
    emit: emit,
    reportSize: reportSize,
    isReducedMotion: isReducedMotion,
    announceToScreenReader: announceToScreenReader
  };
})();
