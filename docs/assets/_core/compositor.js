/* compositor.js — FERROS Manifest-Driven Scene Compositor
 *
 * Reads a project manifest (loot-box-3d.json), loads geometry from
 * part files via FERROS_GEO_SOURCE, and renders the full assembly
 * into a single CSS 3D scene using FERROS_GEO3D.
 *
 * Dependencies:
 *   - window.FERROS_GEO3D   (from _geo3d.js)
 *   - window.FERROS_GEO_SOURCE (from geo-source.js)
 *
 * Public API (window.FERROS_COMPOSITOR):
 *   .compose(manifest, container)  → Promise<CompositorScene>
 *
 * CompositorScene:
 *   .root           — the scene origin node
 *   .groups         — Map<groupId, { node, def }>
 *   .components     — Map<componentId, { node, faces, def }>
 *   .setState(label) — apply group stateTransforms for a state
 *   .destroy()      — clean up
 */
(function() {
  'use strict';

  var GEO3D = null;    /* resolved lazily */
  var SOURCE = null;

  function ensureDeps() {
    GEO3D = window.FERROS_GEO3D;
    SOURCE = window.FERROS_GEO_SOURCE;
    if (!GEO3D) throw new Error('compositor: FERROS_GEO3D not found');
    if (!SOURCE) throw new Error('compositor: FERROS_GEO_SOURCE not found');
  }

  /* ─── transform helpers ──────────────────────────────── */

  function applyTransform(node, t) {
    if (!t) return;
    var parts = [];
    if (t.tx || t.ty || t.tz) {
      parts.push('translate3d(' +
        (t.tx || 0) + 'px,' +
        (t.ty || 0) + 'px,' +
        (t.tz || 0) + 'px)');
    }
    if (t.rx) parts.push('rotateX(' + t.rx + 'deg)');
    if (t.ry) parts.push('rotateY(' + t.ry + 'deg)');
    if (t.rz) parts.push('rotateZ(' + t.rz + 'deg)');
    if (t.scale && t.scale !== 1) parts.push('scale3d(' + t.scale + ',' + t.scale + ',' + t.scale + ')');

    if (parts.length) {
      node.style.transform = parts.join(' ');
    }
  }

  function applyPivot(node, p) {
    if (!p) return;
    node.style.transformOrigin =
      (p.x || 0) + 'px ' + (p.y || 0) + 'px ' + (p.z || 0) + 'px';
  }

  function scaleFaces(faces, s) {
    if (!s || s === 1) return faces;
    return faces.map(function(f) {
      return {
        id: f.id,
        verts: f.verts.map(function(v) {
          return [v[0] * s, v[1] * s, v[2] * s];
        }),
        color: f.color,
        cls: f.cls,
        clip: f.clip
      };
    });
  }

  /* ─── compose ────────────────────────────────────────── */

  function compose(manifest, container) {
    ensureDeps();

    /* Resolve base URL for part files */
    var manifestUrl = manifest._baseUrl || '';

    /* Create the root scene origin */
    var root = GEO3D.createScene(container);

    /* Build group nodes */
    var groupNodes = {};   /* groupId → DOM node */
    var groupDefs = {};    /* groupId → manifest group def */

    if (manifest.groups) {
      manifest.groups.forEach(function(g) {
        var node = document.createElement('div');
        node.style.cssText =
          'position:absolute;left:0;top:0;width:0;height:0;transform-style:preserve-3d;';
        node.dataset.group = g.id;

        applyTransform(node, g.transform);
        applyPivot(node, g.pivot);

        /* Attach to parent group or root */
        var parentNode = g.parent ? groupNodes[g.parent] : root;
        if (!parentNode) parentNode = root;
        parentNode.appendChild(node);

        groupNodes[g.id] = node;
        groupDefs[g.id] = g;
      });
    }

    /* Deduplicate file loads — collect unique files */
    var fileMap = {};  /* file → [componentDef, ...] */
    manifest.components.forEach(function(c) {
      if (!fileMap[c.file]) fileMap[c.file] = [];
      fileMap[c.file].push(c);
    });

    var componentResults = {};  /* componentId → { node, faces, def } */

    /* Load all unique geometry files */
    var loadPromises = Object.keys(fileMap).map(function(file) {
      var url = manifestUrl + file;
      return SOURCE.load(url).then(function(faces) {
        /* Render each component that uses this file */
        fileMap[file].forEach(function(comp) {
          var parentNode = comp.group ? groupNodes[comp.group] : root;
          if (!parentNode) parentNode = root;

          /* Create a transform wrapper for this component */
          var wrapper = document.createElement('div');
          wrapper.style.cssText =
            'position:absolute;left:0;top:0;width:0;height:0;transform-style:preserve-3d;';
          wrapper.dataset.component = comp.id;

          /* Apply component transform (which includes scale for assembly fit) */
          var compTransform = comp.transform || {};

          /* Separate scale from positional transform:
             scale is applied to geometry vertices, not CSS transform,
             so that face normals and lighting remain correct. */
          var geoScale = compTransform.scale || 1;
          var posTransform = {
            tx: compTransform.tx,
            ty: compTransform.ty,
            tz: compTransform.tz,
            rx: compTransform.rx,
            ry: compTransform.ry,
            rz: compTransform.rz
          };
          applyTransform(wrapper, posTransform);

          parentNode.appendChild(wrapper);

          /* Scale vertices and render */
          var scaledFaces = scaleFaces(faces, geoScale);
          var origin = GEO3D.createScene(wrapper);
          var divs = GEO3D.render(scaledFaces, origin);

          componentResults[comp.id] = {
            node: wrapper,
            origin: origin,
            faces: scaledFaces,
            divs: divs,
            def: comp
          };
        });
      });
    });

    return Promise.all(loadPromises).then(function() {
      /* ─── build the scene object ───────────────────── */

      var scene = {
        root: root,
        groups: groupDefs,
        groupNodes: groupNodes,
        components: componentResults,

        setState: function(label) {
          Object.keys(groupDefs).forEach(function(gid) {
            var g = groupDefs[gid];
            var node = groupNodes[gid];
            if (!node) return;

            /* Start from the group's base transform */
            var base = g.transform || {};
            var stateT = (g.stateTransforms && g.stateTransforms[label]) || {};

            /* Merge: state overrides base for rotation/translation */
            var merged = {
              tx: stateT.tx !== undefined ? stateT.tx : base.tx,
              ty: stateT.ty !== undefined ? stateT.ty : base.ty,
              tz: stateT.tz !== undefined ? stateT.tz : base.tz,
              rx: (base.rx || 0) + (stateT.rx || 0),
              ry: (base.ry || 0) + (stateT.ry || 0),
              rz: (base.rz || 0) + (stateT.rz || 0),
              scale: stateT.scale !== undefined ? stateT.scale : base.scale
            };

            applyTransform(node, merged);
          });
        },

        destroy: function() {
          if (root.parentNode) root.parentNode.removeChild(root);
        }
      };

      return scene;
    });
  }

  /* ─── expose ─────────────────────────────────────────── */

  window.FERROS_COMPOSITOR = {
    compose: compose
  };
})();
