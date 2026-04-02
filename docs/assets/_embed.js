/**
 * FERROS Embed Helper v1.0
 * Include inline in any parent page that embeds asset iframes.
 *
 * Usage:
 *   <iframe class="ferros-embed"
 *           src="./assets/cards/trading-card.html"
 *           data-ferros-control="demo"
 *           data-ferros-data='{"ticker":"NVDA","rarity":"epic"}'
 *           style="border:none;width:100%;overflow:hidden;">
 *   </iframe>
 */
(function() {
  'use strict';

  // Auto-initialize all ferros-embed iframes once they load
  document.querySelectorAll('iframe.ferros-embed').forEach(function(iframe) {
    iframe.addEventListener('load', function() {
      var config = {
        control: iframe.dataset.ferrosControl || 'demo',
        theme: 'dark'
      };
      if (iframe.dataset.ferrosData) {
        try { config.data = JSON.parse(iframe.dataset.ferrosData); } catch(e) {}
      }
      iframe.contentWindow.postMessage({ type: 'ferros:init', config: config }, '*');
    });
  });

  // Listen for messages from embedded assets
  window.addEventListener('message', function(e) {
    if (!e.data || !e.data.type) return;

    // Auto-resize iframes to match content height
    if (e.data.type === 'ferros:resize') {
      document.querySelectorAll('iframe.ferros-embed').forEach(function(iframe) {
        if (iframe.contentWindow === e.source) {
          iframe.style.height = e.data.height + 'px';
        }
      });
    }

    // Bubble asset events as CustomEvents on the parent document
    if (e.data.type === 'ferros:event') {
      document.dispatchEvent(new CustomEvent('ferros:' + e.data.event, {
        detail: { asset: e.data.asset, payload: e.data.payload }
      }));
    }
  });
})();
