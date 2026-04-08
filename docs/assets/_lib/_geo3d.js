/* _geo3d.js — FERROS Vertex Geometry Renderer v2.0.0
 *
 * Reads window.FERROS_GEO (array of face definitions with vertex positions)
 * and creates CSS 3D divs with matrix3d transforms.
 *
 * Faces are projected into their own local 2D plane first, then clipped
 * to the actual quad polygon. This lets the renderer handle arbitrary
 * planar quads instead of only perfect parallelograms.
 *
 * Each unclipped face is inflated by BLEED px in its own plane and nudged
 * BIAS px along its normal. This hides the hairline anti-aliasing gaps
 * that CSS 3D creates between perfectly edge-to-edge faces.
 *
 * Face definition:
 *   { id, verts: [[x,y,z]x4], color, cls, clip }
 *   verts order: [topLeft, topRight, bottomRight, bottomLeft]
 *   as seen from the OUTSIDE of the face (CCW winding → outward normal).
 *
 * Coordinates: origin at center of bounding box.
 *   X = right, Y = down (CSS convention), Z = toward viewer.
 *   Units = CSS pixels.
 *
 * Public API (window.FERROS_GEO3D):
 *   .createScene(container)         → origin node (preserve-3d, zero-size)
 *   .render(faces, origin, opts?)   → array of face divs
 *   .ensureDebugStyle()             → injects debug-normal CSS once
 *
 * Legacy auto-boot: if window.FERROS_GEO and .part-geo exist at load time,
 * renders automatically (backward-compatible with all existing part files).
 */
(function() {
  'use strict';

  var BLEED = 0.6;   /* local px overlap to hide AA seams */
  var BIAS  = 0.01;  /* tiny normal nudge to reduce z-fight */

  var _debugNormals = false; /* set true only for debug; default OFF avoids ~520 extra DOM nodes */

  /* ─── helpers ─────────────────────────────────────────── */

  function dot(ax, ay, az, bx, by, bz) {
    return ax * bx + ay * by + az * bz;
  }

  function fmt(n) {
    return Number(n.toFixed(3));
  }

  /* ─── debug-normal CSS (injected once) ───────────────── */

  var debugStyleId = 'ferros-geo3d-debug-style';

  function ensureDebugStyle() {
    if (document.getElementById(debugStyleId)) return;
    var s = document.createElement('style');
    s.id = debugStyleId;
    s.textContent =
      '.geo3d-normal{opacity:0;pointer-events:none;transition:opacity 0.18s ease;}' +
      '[data-debug-normals="true"] .geo3d-normal{opacity:0.95;}' +
      '[data-debug-normals="true"] .geo3d-normal::after{opacity:1;}' +
      '.geo3d-normal::after{' +
        'content:"";position:absolute;right:-6px;top:50%;margin-top:-4px;' +
        'border-top:4px solid transparent;border-bottom:4px solid transparent;' +
        'border-left:7px solid rgba(255,108,108,0.95);opacity:0;' +
      '}';
    document.head.appendChild(s);
  }

  /* ─── normal-indicator line ──────────────────────────── */

  function addNormalIndicator(originNode, center, nx, ny, nz, ux, uy, uz, faceW, faceH) {
    var lineLen = Math.max(18, Math.min(42, Math.min(faceW, faceH) * 0.35));
    var thickness = 4;
    var tx = uy * nz - uz * ny;
    var ty = uz * nx - ux * nz;
    var tz = ux * ny - uy * nx;
    var tl = Math.sqrt(tx * tx + ty * ty + tz * tz) || 1;

    tx /= tl;
    ty /= tl;
    tz /= tl;

    var bx = ny * tz - nz * ty;
    var by = nz * tx - nx * tz;
    var bz = nx * ty - ny * tx;

    var lineOriginX = center[0] - tx * (thickness / 2) + nx * 1.25;
    var lineOriginY = center[1] - ty * (thickness / 2) + ny * 1.25;
    var lineOriginZ = center[2] - tz * (thickness / 2) + nz * 1.25;

    var lineMatrix = [
      nx, ny, nz, 0,
      tx, ty, tz, 0,
      bx, by, bz, 0,
      lineOriginX, lineOriginY, lineOriginZ, 1
    ];

    var line = document.createElement('div');
    line.className = 'geo3d-normal';
    line.style.cssText =
      'position:absolute;left:0;top:0;' +
      'width:' + fmt(lineLen) + 'px;height:' + thickness + 'px;' +
      'transform-origin:0 0;' +
      'backface-visibility:visible;' +
      'background:linear-gradient(90deg, rgba(255,59,59,0.3) 0%, rgba(255,59,59,1) 100%);' +
      'box-shadow:0 0 18px rgba(255,59,59,0.9);' +
      'transform:matrix3d(' + lineMatrix.join(',') + ');';
    originNode.appendChild(line);
  }

  /* ─── core: render one face into an origin node ──────── */

  function renderFace(originNode, f) {
    var v = f.verts;

    /* Edge vectors: e = v1-v0 (width dir), g = v3-v0 (height dir) */
    var ex = v[1][0] - v[0][0];
    var ey = v[1][1] - v[0][1];
    var ez = v[1][2] - v[0][2];

    var gx = v[3][0] - v[0][0];
    var gy = v[3][1] - v[0][1];
    var gz = v[3][2] - v[0][2];

    /* Face basis vectors */
    var w = Math.sqrt(ex * ex + ey * ey + ez * ez) || 1;

    /* Unit width vector */
    var ux = ex / w, uy = ey / w, uz = ez / w;

    /* Normal = cross(e, g), normalized */
    var nx = ey * gz - ez * gy;
    var ny = ez * gx - ex * gz;
    var nz = ex * gy - ey * gx;
    var nl = Math.sqrt(nx * nx + ny * ny + nz * nz) || 1;
    nx /= nl; ny /= nl; nz /= nl;

    /* Unit height vector, orthogonalized in the face plane */
    var vx = ny * uz - nz * uy;
    var vy = nz * ux - nx * uz;
    var vz = nx * uy - ny * ux;

    /* Project the quad into local 2D face-space */
    var minX = Infinity, maxX = -Infinity;
    var minY = Infinity, maxY = -Infinity;
    var projected = [];
    var i;

    for (i = 0; i < v.length; i++) {
      var px = v[i][0] - v[0][0];
      var py = v[i][1] - v[0][1];
      var pz = v[i][2] - v[0][2];
      var lx = dot(px, py, pz, ux, uy, uz);
      var ly = dot(px, py, pz, vx, vy, vz);

      projected.push([lx, ly]);
      if (lx < minX) minX = lx;
      if (lx > maxX) maxX = lx;
      if (ly < minY) minY = ly;
      if (ly > maxY) maxY = ly;
    }

    var localBleed = f.clip ? 0 : BLEED;
    var faceW = Math.max(maxX - minX, 0.001);
    var faceH = Math.max(maxY - minY, 0.001);

    /* Inflated dimensions and shifted origin */
    var w2 = faceW + localBleed * 2;
    var h2 = faceH + localBleed * 2;

    var tx = v[0][0] + ux * (minX - localBleed) + vx * (minY - localBleed) + nx * BIAS;
    var ty = v[0][1] + uy * (minX - localBleed) + vy * (minY - localBleed) + ny * BIAS;
    var tz = v[0][2] + uz * (minX - localBleed) + vz * (minY - localBleed) + nz * BIAS;

    var clip = f.clip;
    if (!clip) {
      clip = 'polygon(' + projected.map(function(p) {
        return fmt(p[0] - minX + localBleed) + 'px ' +
          fmt(p[1] - minY + localBleed) + 'px';
      }).join(',') + ')';
    }

    /* matrix3d — column-major for CSS:
       col1 = u  (unit width dir)
       col2 = v  (unit height dir)
       col3 = N  (face normal)
       col4 = t  (inflated origin with bias) */
    var m = [
      ux, uy, uz, 0,
      vx, vy, vz, 0,
      nx, ny, nz, 0,
      tx, ty, tz, 1
    ];

    var div = document.createElement('div');
    div.className = 'face' + (f.cls ? ' ' + f.cls : '');
    if (f.id) div.id = f.id;

    div.style.cssText =
      'position:absolute;left:0;top:0;' +
      'width:' + w2 + 'px;height:' + h2 + 'px;' +
      'transform-origin:0 0;' +
      'backface-visibility:hidden;' +
      'transform:matrix3d(' + m.join(',') + ');' +
      (f.color ? 'background:' + f.color + ';' : '') +
      (clip ? 'clip-path:' + clip + ';' : '');

    /* Store params for lazy normal-indicator creation when debug is toggled on.
       Lightweight object kept per-face to enable setDebugNormals() without re-render. */
    div._normalParams = {
      center: [
        (v[0][0] + v[1][0] + v[2][0] + v[3][0]) / 4,
        (v[0][1] + v[1][1] + v[2][1] + v[3][1]) / 4,
        (v[0][2] + v[1][2] + v[2][2] + v[3][2]) / 4
      ],
      nx: nx, ny: ny, nz: nz,
      ux: ux, uy: uy, uz: uz,
      faceW: faceW, faceH: faceH
    };

    originNode.appendChild(div);

    if (_debugNormals) {
      addNormalIndicator(originNode, div._normalParams.center,
        nx, ny, nz, ux, uy, uz, faceW, faceH);
    }

    return div;
  }

  /* ─── public API ─────────────────────────────────────── */

  /**
   * Create a zero-size preserve-3d origin node inside a container.
   * All geometry rendered into this origin shares a single 3D space.
   * @param {HTMLElement} container — must have transform-style: preserve-3d
   * @returns {HTMLElement} the origin node
   */
  function createScene(container) {
    var origin = document.createElement('div');
    origin.style.cssText =
      'width:0;height:0;position:relative;transform-style:preserve-3d;';
    container.appendChild(origin);
    return origin;
  }

  /**
   * Render an array of FERROS_GEO faces into an origin node.
   * @param {Array} faces — FERROS_GEO face definitions
   * @param {HTMLElement} origin — the scene origin node from createScene()
   * @returns {Array<HTMLElement>} the created face divs
   */
  function render(faces, origin) {
    if (!faces || !faces.length) return [];
    ensureDebugStyle();
    var divs = [];
    faces.forEach(function(f) {
      divs.push(renderFace(origin, f));
    });
    return divs;
  }

  /* ─── expose namespace ───────────────────────────────── */

  /**
   * Enable or disable debug normal indicators at runtime.
   * When enabling, lazily creates indicators for faces already in the DOM.
   * @param {boolean} enabled
   * @param {HTMLElement} [origin] — origin node to create missing indicators in
   */
  function setDebugNormals(enabled, origin) {
    _debugNormals = !!enabled;
    if (_debugNormals && origin) {
      ensureDebugStyle();
      var children = origin.children;
      for (var i = 0; i < children.length; i++) {
        var child = children[i];
        if (!child._normalParams) continue;
        /* Skip if a .geo3d-normal sibling already follows this face */
        var next = child.nextElementSibling;
        if (next && next.classList && next.classList.contains('geo3d-normal')) continue;
        var p = child._normalParams;
        addNormalIndicator(origin, p.center,
          p.nx, p.ny, p.nz, p.ux, p.uy, p.uz, p.faceW, p.faceH);
      }
    }
  }

  window.FERROS_GEO3D = {
    createScene: createScene,
    render: render,
    ensureDebugStyle: ensureDebugStyle,
    setDebugNormals: setDebugNormals
  };

  /* ─── geometry extraction protocol ─────────────────── */
  /* Parent compositor can request raw FERROS_GEO data from
     part iframes via postMessage. This avoids DOM parsing. */

  window.addEventListener('message', function(e) {
    if (e.data && e.data.type === 'ferros:geo-request') {
      var geo = window.FERROS_GEO;
      if (geo) {
        e.source.postMessage({
          type: 'ferros:geo-response',
          requestId: e.data.requestId,
          faces: geo
        }, e.origin === 'null' ? '*' : e.origin);
      }
    }
  });

  /* ─── legacy auto-boot ───────────────────────────────── */

  var faces = window.FERROS_GEO;
  if (!faces || !faces.length) return;

  var geo = document.querySelector('.part-geo');
  if (!geo) return;

  var origin = createScene(geo);
  render(faces, origin);
})();
