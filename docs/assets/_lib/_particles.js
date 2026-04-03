/* ferros-particles v1.0.0
 *
 * Generic radial particle burst.
 * Requires a container element and the lootParticle keyframe CSS.
 *
 * CSS dependency (include in your asset's <style>):
 *   .ferros-particle {
 *     position: absolute; width: 8px; height: 8px; border-radius: 50%;
 *     animation: ferrosParticleBurst 0.8s ease-out forwards;
 *   }
 *   @keyframes ferrosParticleBurst {
 *     0%   { transform: translate(0, 0) scale(1); opacity: 1; }
 *     100% { transform: translate(var(--lp-x), var(--lp-y)) scale(0); opacity: 0; }
 *   }
 *
 * See: ADR-008 (docs/adr/ADR-008-modular-rendering-system.md)
 */

var PARTICLE_DEFAULTS = {
  count: 20,
  radius: 80,
  colors: ['#f59e0b', '#a78bfa', '#2dd4bf', '#f472b6', '#4a90d9', '#34d399', '#f87171'],
  className: 'ferros-particle'
};

/**
 * Emit a burst of particles from the center of a container.
 * @param {HTMLElement} container  — element to append particles to
 * @param {Object}      [opts]    — { count, radius, colors, className }
 */
function spawnParticles(container, opts) {
  if (!container) return;
  if (window.ferrosRuntime && window.ferrosRuntime.isReducedMotion()) return;
  opts = opts || {};
  var count  = opts.count  || PARTICLE_DEFAULTS.count;
  var radius = opts.radius || PARTICLE_DEFAULTS.radius;
  var colors = opts.colors || PARTICLE_DEFAULTS.colors;
  var cls    = opts.className || PARTICLE_DEFAULTS.className;

  container.innerHTML = '';
  for (var i = 0; i < count; i++) {
    var p = document.createElement('div');
    p.classList.add(cls);
    var angle = (Math.PI * 2 / count) * i;
    var dist  = (radius * 0.5) + Math.random() * (radius * 0.5);
    p.style.setProperty('--lp-x', (Math.cos(angle) * dist) + 'px');
    p.style.setProperty('--lp-y', (Math.sin(angle) * dist) + 'px');
    p.style.left = '50%';
    p.style.top  = '50%';
    p.style.background = colors[Math.floor(Math.random() * colors.length)];
    p.style.animationDelay = (Math.random() * 0.2) + 's';
    container.appendChild(p);
  }
}
