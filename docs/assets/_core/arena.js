/* arena.js — The Arena v3.0.0
 *
 * FERROS 3D asset renderer runtime.
 * Loads part geometry via iframe (works on file:// and http://).
 * Controls the part via postMessage to embedded arena-guest script.
 * Provides: drag-rotate, shift+drag pan, zoom (iso + perspective),
 * wireframe toggle, normals debug, angle HUD, reset.
 *
 * Usage:
 *   <script src="arena.js"></script>
 *   — auto-initializes from ?part= query param
 *   — or call: arena.loadPart('relative/path/to/part.html')
 */
(function() {
  'use strict';

  var mql = window.matchMedia('(prefers-reduced-motion: reduce)');

  /* ── State ── */
  var rx = 0, ry = 0, zoom = 1;
  var panX = 0, panY = 0;
  var initScale = 1;
  var baseRx = -20, baseRy = 30;
  var projMode = 'perspective'; /* 'perspective' | 'iso' */
  var dragging = false, dragMode = 'rotate', lastX = 0, lastY = 0;
  var touchStartDist = 0;
  var wireframeOn = false;
  var debugNormalsOn = false;

  /* ── DOM refs (set on init) ── */
  var scene, overlay, hud, projToggle, projLabel, debugToggle, iframe;

  /* ── Send command to part iframe ── */
  function send(cmd) {
    if (iframe && iframe.contentWindow) {
      iframe.contentWindow.postMessage(cmd, '*');
    }
  }

  /* ── Apply transforms — sent to iframe guest ── */
  function apply() {
    send({
      type: 'arena:update',
      rx: baseRx + rx,
      ry: baseRy + ry,
      zoom: zoom,
      panX: panX,
      panY: panY,
      projMode: projMode
    });

    if (hud) {
      hud.textContent =
        'X: ' + (baseRx + rx).toFixed(1) + '\u00b0  ' +
        'Y: ' + (baseRy + ry).toFixed(1) + '\u00b0  ' +
        'Zoom: ' + zoom.toFixed(1) + 'x  ' +
        (projMode === 'iso' ? 'ISO' : 'PERSP');
    }
  }

  /* ── Mouse drag (rotate) / shift+drag (pan) ── */
  function onMouseDown(e) {
    if (mql.matches) return;
    dragging = true;
    dragMode = e.shiftKey ? 'pan' : 'rotate';
    lastX = e.clientX;
    lastY = e.clientY;
    send({ type: 'arena:transition', value: 'none' });
  }
  function onMouseMove(e) {
    if (!dragging) return;
    var dx = e.clientX - lastX;
    var dy = e.clientY - lastY;
    if (dragMode === 'pan') {
      panX += dx;
      panY += dy;
    } else {
      ry += dx * 0.5;
      rx -= dy * 0.5;
    }
    lastX = e.clientX;
    lastY = e.clientY;
    apply();
  }
  function onMouseUp() {
    if (!dragging) return;
    dragging = false;
    send({ type: 'arena:transition', value: '' });
  }

  /* ── Scroll zoom ── */
  function onWheel(e) {
    e.preventDefault();
    var step = zoom * 0.08;
    zoom += e.deltaY > 0 ? -step : step;
    zoom = Math.max(0.3, Math.min(24.0, zoom));
    apply();
  }

  /* ── Touch ── */
  function onTouchStart(e) {
    if (mql.matches) return;
    if (e.touches.length === 1) {
      dragging = true;
      dragMode = 'rotate';
      lastX = e.touches[0].clientX;
      lastY = e.touches[0].clientY;
      send({ type: 'arena:transition', value: 'none' });
    } else if (e.touches.length === 2) {
      dragging = false;
      var dx = e.touches[1].clientX - e.touches[0].clientX;
      var dy = e.touches[1].clientY - e.touches[0].clientY;
      touchStartDist = Math.sqrt(dx * dx + dy * dy);
    }
  }
  function onTouchMove(e) {
    if (e.touches.length === 1 && dragging) {
      e.preventDefault();
      ry += (e.touches[0].clientX - lastX) * 0.5;
      rx -= (e.touches[0].clientY - lastY) * 0.5;
      lastX = e.touches[0].clientX;
      lastY = e.touches[0].clientY;
      apply();
    } else if (e.touches.length === 2 && touchStartDist > 0) {
      e.preventDefault();
      var dx = e.touches[1].clientX - e.touches[0].clientX;
      var dy = e.touches[1].clientY - e.touches[0].clientY;
      var dist = Math.sqrt(dx * dx + dy * dy);
      zoom *= dist / touchStartDist;
      zoom = Math.max(0.3, Math.min(24.0, zoom));
      touchStartDist = dist;
      apply();
    }
  }
  function onTouchEnd() {
    dragging = false;
    touchStartDist = 0;
    send({ type: 'arena:transition', value: '' });
  }

  /* ── Load part into iframe ── */
  function loadPart(partPath) {
    /* Reset state for new part */
    rx = 0; ry = 0; zoom = 1;
    panX = 0; panY = 0;
    initScale = 1; baseRx = -20; baseRy = 30;
    wireframeOn = false;
    debugNormalsOn = false;

    var wireBtn = document.getElementById('arena-wireframe');
    if (wireBtn) wireBtn.textContent = 'Wireframe';
    if (debugToggle) debugToggle.setAttribute('data-enabled', 'false');

    iframe.style.display = 'block';
    iframe.src = partPath;

    var loadingEl = document.querySelector('.arena-loading');
    if (loadingEl) loadingEl.style.display = 'none';
  }

  /* ── Listen for messages from guest iframe ── */
  function onMessage(e) {
    var d = e.data;
    if (!d || !d.type) return;

    if (d.type === 'arena:ready') {
      /* Part reports its metadata */
      if (d.scale) { initScale = d.scale; zoom = initScale; }
      if (d.rx != null) baseRx = d.rx;
      if (d.ry != null) baseRy = d.ry;
      if (d.title) {
        var titleEl = document.querySelector('.arena-title');
        if (titleEl) titleEl.textContent = d.title;
      }
      /* Apply initial state */
      apply();
      /* Re-apply wireframe/projection state to new part */
      if (wireframeOn) send({ type: 'arena:wireframe', value: true });
      if (debugNormalsOn) send({ type: 'arena:debug-normals', value: true });
    }
  }

  /* ── Init ── */
  function init() {
    scene = document.querySelector('.arena-scene');
    overlay = document.querySelector('.arena-overlay');
    hud = document.querySelector('.arena-hud');
    projToggle = document.querySelector('.arena-proj-toggle');
    projLabel = document.querySelector('.arena-proj-label');
    debugToggle = document.querySelector('.arena-debug-toggle');
    iframe = document.getElementById('arena-frame');

    if (!scene || !iframe) return;

    /* Drag/pan — on overlay */
    overlay.addEventListener('mousedown', onMouseDown);
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);

    /* Zoom — on overlay */
    overlay.addEventListener('wheel', onWheel, { passive: false });

    /* Touch — on overlay */
    overlay.addEventListener('touchstart', onTouchStart, { passive: true });
    overlay.addEventListener('touchmove', onTouchMove, { passive: false });
    overlay.addEventListener('touchend', onTouchEnd);

    /* postMessage listener */
    window.addEventListener('message', onMessage);

    /* Reset */
    var resetBtn = document.getElementById('arena-reset');
    if (resetBtn) {
      resetBtn.addEventListener('click', function() {
        rx = 0; ry = 0; zoom = initScale;
        panX = 0; panY = 0;
        send({ type: 'arena:transition', value: 'transform 0.4s ease-out' });
        apply();
        setTimeout(function() { send({ type: 'arena:transition', value: '' }); }, 400);
      });
    }

    /* Wireframe */
    var wireBtn = document.getElementById('arena-wireframe');
    if (wireBtn) {
      wireBtn.addEventListener('click', function() {
        wireframeOn = !wireframeOn;
        send({ type: 'arena:wireframe', value: wireframeOn });
        wireBtn.textContent = wireframeOn ? 'Solid' : 'Wireframe';
      });
    }

    /* Projection toggle */
    if (projToggle) {
      projToggle.addEventListener('click', function() {
        projMode = projMode === 'perspective' ? 'iso' : 'perspective';
        projToggle.setAttribute('data-mode', projMode);
        projToggle.setAttribute('data-enabled', projMode === 'iso' ? 'true' : 'false');
        if (projLabel) projLabel.textContent = projMode === 'iso' ? 'ISO' : 'PERSP';
        apply();
      });
      projToggle.setAttribute('data-enabled', 'false');
    }

    /* Normals debug toggle */
    if (debugToggle) {
      debugToggle.addEventListener('click', function() {
        debugNormalsOn = !debugNormalsOn;
        debugToggle.setAttribute('data-enabled', debugNormalsOn ? 'true' : 'false');
        send({ type: 'arena:debug-normals', value: debugNormalsOn });
      });
    }

    /* Auto-load from query param ?part=path */
    var params = new URLSearchParams(window.location.search);
    var partPath = params.get('part');
    if (partPath) {
      loadPart(partPath);
    }
  }

  /* ── Export ── */
  window.arena = {
    init: init,
    loadPart: loadPart
  };

  /* Auto-init on DOMContentLoaded */
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
