/**
 * FERROS Embed Helper v2.0 — with Event Router
 * Include inline in any parent page that embeds asset iframes.
 *
 * Usage:
 *   <iframe class="ferros-embed"
 *           src="./assets/cards/trading-card.html"
 *           data-ferros-asset="trading-card"
 *           data-ferros-control="demo"
 *           data-ferros-data='{"ticker":"NVDA","rarity":"epic"}'
 *           style="border:none;width:100%;overflow:hidden;">
 *   </iframe>
 *
 * Event routing (optional):
 *   <iframe ... data-ferros-routes='["hand-fan","game-board"]'>
 *   Routes all ferros:event messages from this asset to the listed target assets.
 *
 * Routing table (built-in defaults):
 *   deck-changed   → hand-fan          (deck builder changed the deck)
 *   card-played    → game-board        (hand-fan played a card to a lane)
 *   card-dragged   → game-board        (hand-fan started a drag)
 *   turn-ended     → pvp-arena         (game-board ended a turn)
 *   ability-used   → pvp-arena         (hand-fan used an ability)
 *   loot-opened    → profile-card      (loot-box revealed an item)
 *   item-purchased → profile-card      (store purchased something)
 *   score          → profile-card      (snake/scratcher sent a score)
 *   navigate       → hero-card-fan     (infographic/card-fan navigation)
 *   creature-evolved → profile-card    (creature hit a new stage)
 */
(function() {
  'use strict';

  /* === BUILT-IN ROUTING TABLE === */
  var defaultRoutes = {
    'deck-changed':     ['hand-fan'],
    'deck-validated':   ['hand-fan'],
    'card-played':      ['game-board', 'pvp-arena'],
    'card-dragged':     ['game-board'],
    'turn-ended':       ['pvp-arena'],
    'ability-used':     ['pvp-arena', 'game-board'],
    'loot-opened':      ['profile-card'],
    'item-purchased':   ['profile-card', 'store-grid'],
    'item-equipped':    ['profile-card'],
    'score':            ['profile-card'],
    'navigate':         ['hero-card-fan', 'hero-infographic'],
    'creature-evolved': ['profile-card'],
    'resource-gained':  ['profile-card'],
    'building-upgraded':['idle-town'],
    'gallery-updated':  ['profile-card'],
    'agent-selected':   ['ai-float-panel'],
    'match-result':     ['profile-card']
  };

  /* === IFRAME REGISTRY === */
  var iframes = {};       // assetName → iframe element
  var pendingQueue = {};  // assetName → [messages] (queued before iframe is ready)
  var readyAssets = {};   // assetName → true (set after first message received or load event)
  var assetNonces = {};   // A4: assetName → nonce (generated on init, validated on events)

  function registerIframe(iframe) {
    var name = iframe.dataset.ferrosAsset;
    if (!name) {
      var src = iframe.getAttribute('src') || '';
      name = src.replace(/.*\//, '').replace('.html', '');
    }
    if (name) {
      iframes[name] = iframe;
      pendingQueue[name] = pendingQueue[name] || [];
    }
    return name;
  }

  function flushQueue(name) {
    if (!pendingQueue[name] || !iframes[name]) return;
    var queue = pendingQueue[name];
    pendingQueue[name] = [];
    queue.forEach(function(msg) {
      iframes[name].contentWindow.postMessage(msg, '*');
    });
  }

  function sendToAsset(name, message) {
    if (readyAssets[name] && iframes[name]) {
      iframes[name].contentWindow.postMessage(message, '*');
    } else {
      pendingQueue[name] = pendingQueue[name] || [];
      pendingQueue[name].push(message);
    }
  }

  /* === ROUTE RESOLUTION === */
  function getRoutesForEvent(sourceAsset, eventName, iframe) {
    var targets = [];

    // 1. Check iframe-specific routes (data-ferros-routes attribute)
    if (iframe && iframe.dataset.ferrosRoutes) {
      try { targets = targets.concat(JSON.parse(iframe.dataset.ferrosRoutes)); } catch(e) {}
    }

    // 2. Check built-in routing table
    if (defaultRoutes[eventName]) {
      defaultRoutes[eventName].forEach(function(t) {
        if (targets.indexOf(t) === -1) targets.push(t);
      });
    }

    // Don't route back to the source
    return targets.filter(function(t) { return t !== sourceAsset; });
  }

  /* === INIT ALL IFRAMES === */
  function initAll() {
    document.querySelectorAll('iframe.ferros-embed').forEach(function(iframe) {
      var name = registerIframe(iframe);

      iframe.addEventListener('load', function() {
        var config = {
          control: iframe.dataset.ferrosControl || 'demo',
          theme: 'dark'
        };
        if (iframe.dataset.ferrosData) {
          try { config.data = JSON.parse(iframe.dataset.ferrosData); } catch(e) {}
        }
        // A4: generate nonce for this asset's communication channel
        var nonce = (typeof FerrosCore !== 'undefined' && FerrosCore.generateRuntimeNonce)
          ? FerrosCore.generateRuntimeNonce()
          : Math.random().toString(36).slice(2);
        if (name) assetNonces[name] = nonce;
        iframe.contentWindow.postMessage({ type: 'ferros:init', config: config, nonce: nonce }, '*');

        // Mark as ready and flush any queued messages
        if (name) {
          readyAssets[name] = true;
          flushQueue(name);
        }
      });
    });
  }

  /* === MESSAGE HANDLER === */
  window.addEventListener('message', function(e) {
    if (!e.data || !e.data.type) return;

    // Auto-resize iframes to match content height
    if (e.data.type === 'ferros:resize') {
      document.querySelectorAll('iframe.ferros-embed').forEach(function(iframe) {
        if (iframe.contentWindow === e.source) {
          iframe.style.height = e.data.height + 'px';
        }
      });
      return;
    }

    // Handle asset events
    if (e.data.type === 'ferros:event') {
      var sourceAsset = e.data.asset;
      var eventName = e.data.event;
      var payload = e.data.payload;

      // A4: validate nonce — reject messages without valid nonce
      if (sourceAsset && assetNonces[sourceAsset] && e.data.nonce !== assetNonces[sourceAsset]) {
        console.warn('[ferros-embed] Rejected message from "' + sourceAsset + '": invalid nonce');
        return;
      }

      // Mark source as ready (it sent us a message, so it's loaded)
      if (sourceAsset && !readyAssets[sourceAsset]) {
        readyAssets[sourceAsset] = true;
        flushQueue(sourceAsset);
      }

      // Bubble as CustomEvent on parent document
      document.dispatchEvent(new CustomEvent('ferros:' + eventName, {
        detail: { asset: sourceAsset, payload: payload }
      }));

      // Find source iframe for per-iframe route lookup
      var sourceIframe = null;
      Object.keys(iframes).forEach(function(name) {
        if (iframes[name].contentWindow === e.source) sourceIframe = iframes[name];
      });

      // Route to target assets
      var targets = getRoutesForEvent(sourceAsset, eventName, sourceIframe);
      targets.forEach(function(targetName) {
        sendToAsset(targetName, {
          type: 'ferros:update',
          payload: {
            fromAsset: sourceAsset,
            fromEvent: eventName,
            data: payload
          }
        });
      });
    }
  });

  /* === BOOT === */
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initAll);
  } else {
    initAll();
  }

  // Also catch dynamically added iframes
  if (typeof MutationObserver !== 'undefined') {
    new MutationObserver(function(mutations) {
      mutations.forEach(function(m) {
        m.addedNodes.forEach(function(node) {
          if (node.tagName === 'IFRAME' && node.classList.contains('ferros-embed')) {
            registerIframe(node);
          }
        });
      });
    }).observe(document.body || document.documentElement, { childList: true, subtree: true });
  }
})();
