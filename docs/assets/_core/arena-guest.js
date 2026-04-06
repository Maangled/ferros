/* arena-guest.js — The Arena Guest v2.0.0
 *
 * Embedded in each FERROS part file.
 * Listens for postMessage commands from The Arena (parent frame)
 * and applies rotation, zoom, wireframe, projection, and debug changes.
 *
 * Perspective zoom uses translateZ (camera distance) to stay sharp.
 * ISO zoom uses scale on part-geo (inside 3D context).
 * Pan is always on part-root (outside 3D context).
 */
(function() {
  'use strict';

  /* ── Part metadata from body attributes ── */
  var body = document.body;
  var scale = parseFloat(body.getAttribute('data-arena-scale')) || 1;
  var initRx = parseFloat(body.getAttribute('data-arena-rx')) || -20;
  var initRy = parseFloat(body.getAttribute('data-arena-ry')) || 30;

  /* ── DOM refs ── */
  var root = document.querySelector('.part-root');
  var geo = document.querySelector('.part-geo');
  if (!root || !geo) return;

  /* ── Wireframe style (injected on demand) ── */
  var wireStyle = null;

  function setDebugNormals(on) {
    geo.setAttribute('data-debug-normals', on ? 'true' : 'false');
  }

  /* ── Report ready to parent ── */
  function reportReady() {
    if (window.parent && window.parent !== window) {
      window.parent.postMessage({
        type: 'arena:ready',
        scale: scale,
        rx: initRx,
        ry: initRy,
        title: document.title || ''
      }, '*');
    }
  }

  /* ── Listen for commands ── */
  window.addEventListener('message', function(e) {
    var d = e.data;
    if (!d || !d.type) return;

    if (d.type === 'arena:update') {
      var totalRx = d.rx || 0;
      var totalRy = d.ry || 0;
      var z = d.zoom || scale || 1;
      var px = d.panX || 0;
      var py = d.panY || 0;

      /* zoom relative to this part's natural starting size */
      var rel = z / (scale || 1);
      if (!isFinite(rel) || rel <= 0) rel = 1;

      root.style.transform = 'translate(' + px + 'px,' + py + 'px)';

      if (d.projMode === 'perspective') {
        root.style.perspective = '1400px';

        /* logarithmic camera zoom: preserves feel across large range */
        var camZ = Math.log(rel) * 900;

        geo.style.transform =
          'translateZ(' + camZ.toFixed(3) + 'px) ' +
          'rotateX(' + totalRx + 'deg) ' +
          'rotateY(' + totalRy + 'deg)';
      } else {
        root.style.perspective = '8000px';

        geo.style.transform =
          'scale(' + rel + ') ' +
          'rotateX(' + totalRx + 'deg) ' +
          'rotateY(' + totalRy + 'deg)';
      }
    }

    if (d.type === 'arena:transition') {
      geo.style.transition = d.value || '';
      root.style.transition = d.value || '';
    }

    if (d.type === 'arena:wireframe') {
      if (d.value && !wireStyle) {
        wireStyle = document.createElement('style');
        wireStyle.textContent =
          '.part-geo * { background: transparent !important; ' +
          'box-shadow: inset 0 0 0 1px rgba(74,144,217,0.35) !important; ' +
          'filter: none !important; border: none !important; }\n' +
          '.part-geo *::before, .part-geo *::after { display: none !important; }';
        document.head.appendChild(wireStyle);
      } else if (!d.value && wireStyle) {
        wireStyle.remove();
        wireStyle = null;
      }
    }

    if (d.type === 'arena:debug-normals') {
      setDebugNormals(!!d.value);
    }
  });

  setDebugNormals(false);

  /* Report ready once DOM is loaded */
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', reportReady);
  } else {
    reportReady();
  }
})();
