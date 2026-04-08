/* geo-source.js — FERROS Geometry Source (iframe extraction)
 *
 * Loads part HTML files in hidden iframes, requests their FERROS_GEO
 * vertex data via postMessage, and caches the results.
 *
 * Protocol:
 *   Parent → iframe:  { type: 'ferros:geo-request', requestId }
 *   iframe → Parent:  { type: 'ferros:geo-response', requestId, faces }
 *
 * The iframe's _geo3d.js handles the response side.
 *
 * Public API (window.FERROS_GEO_SOURCE):
 *   .load(url)          → Promise<Array<face>>  (cached)
 *   .preload(urls)      → Promise<void>
 *   .clearCache()       → void
 */
(function() {
  'use strict';

  var cache = {};        /* url → Promise<faces> */
  var pending = {};      /* requestId → { resolve, reject, iframe, timer } */
  var nextId = 1;
  var TIMEOUT = 8000;    /* ms to wait for a geo-response */

  /* ─── message listener ───────────────────────────────── */

  window.addEventListener('message', function(e) {
    if (!e.data || e.data.type !== 'ferros:geo-response') return;
    var id = e.data.requestId;
    var p = pending[id];
    if (!p) return;

    clearTimeout(p.timer);
    delete pending[id];

    /* Clean up hidden iframe */
    if (p.iframe && p.iframe.parentNode) {
      p.iframe.parentNode.removeChild(p.iframe);
    }

    p.resolve(e.data.faces || []);
  });

  /* ─── load a single part URL ─────────────────────────── */

  function load(url) {
    if (cache[url]) return cache[url];

    var promise = new Promise(function(resolve, reject) {
      var iframe = document.createElement('iframe');
      iframe.style.cssText =
        'position:absolute;width:1px;height:1px;opacity:0;pointer-events:none;' +
        'left:-9999px;top:-9999px;border:none;';

      iframe.onload = function() {
        var timer = setTimeout(function() {
          if (iframe.parentNode) iframe.parentNode.removeChild(iframe);
          reject(new Error('geo-source: timeout loading ' + url));
        }, TIMEOUT);

        function tryResolveFromIframe() {
          try {
            var faces = iframe.contentWindow && iframe.contentWindow.FERROS_GEO;
            if (Array.isArray(faces) && faces.length) {
              clearTimeout(timer);
              if (iframe.parentNode) iframe.parentNode.removeChild(iframe);
              resolve(faces);
              return;
            }
          } catch (error) {
            clearTimeout(timer);
            if (iframe.parentNode) iframe.parentNode.removeChild(iframe);
            reject(error);
            return;
          }

          setTimeout(tryResolveFromIframe, 50);
        }

        tryResolveFromIframe();
      };

      iframe.onerror = function() {
        if (iframe.parentNode) iframe.parentNode.removeChild(iframe);
        reject(new Error('geo-source: failed to load ' + url));
      };

      document.body.appendChild(iframe);
      iframe.src = url;
    });

    cache[url] = promise;
    return promise;
  }

  /* ─── preload multiple URLs in parallel ──────────────── */

  function preload(urls) {
    return Promise.all(urls.map(function(u) { return load(u); }))
      .then(function() { /* resolve void */ });
  }

  /* ─── clear cache (forces re-fetch on next load) ─────── */

  function clearCache() {
    /* Abort any pending requests */
    Object.keys(pending).forEach(function(id) {
      clearTimeout(pending[id].timer);
      var iframe = pending[id].iframe;
      if (iframe && iframe.parentNode) iframe.parentNode.removeChild(iframe);
      pending[id].reject(new Error('geo-source: cache cleared'));
    });
    pending = {};
    cache = {};
  }

  /* ─── expose ─────────────────────────────────────────── */

  window.FERROS_GEO_SOURCE = {
    load: load,
    preload: preload,
    clearCache: clearCache
  };
})();
