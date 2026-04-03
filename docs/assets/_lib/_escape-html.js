/* ferros-escape-html v1.0.0
 *
 * XSS-safe HTML escaping via DOM textContent trick.
 *
 * Consumers: trading-card, hand-fan, deck-builder, ticket-gallery
 * See: ADR-008 (docs/adr/ADR-008-modular-rendering-system.md)
 */

function esc(str) {
  var div = document.createElement('div');
  div.textContent = str;
  return div.innerHTML;
}
