/* ferros-rarity v1.0.0
 *
 * Rarity data structures and probability engine.
 * Pure functions — no DOM dependencies.
 *
 * Uses token variables from _tokens.css via RARITY_COLORS mapping.
 * Consumers: loot-box-3d (directly), any future loot/gacha asset.
 *
 * See: ADR-008 (docs/adr/ADR-008-modular-rendering-system.md)
 */

var RARITY_WEIGHTS = {
  COMMON:    40,
  UNCOMMON:  25,
  RARE:      20,
  EPIC:      10,
  LEGENDARY:  4,
  MYTHIC:     1
};

var RARITY_COLORS = {
  COMMON:    'var(--rarity-common)',
  UNCOMMON:  'var(--rarity-uncommon)',
  RARE:      'var(--rarity-rare)',
  EPIC:      'var(--rarity-epic)',
  LEGENDARY: 'var(--rarity-legendary)',
  MYTHIC:    'var(--rarity-mythic)'
};

var RARITY_ORDER = ['MYTHIC', 'LEGENDARY', 'EPIC', 'RARE', 'UNCOMMON', 'COMMON'];

function computeWeightTotal() {
  var total = 0;
  for (var key in RARITY_WEIGHTS) { total += RARITY_WEIGHTS[key]; }
  return total;
}

function rollRarity() {
  var total = computeWeightTotal();
  var roll = Math.random() * total;
  var cumulative = 0;
  for (var i = 0; i < RARITY_ORDER.length; i++) {
    var r = RARITY_ORDER[i];
    if (RARITY_WEIGHTS[r] === undefined || RARITY_WEIGHTS[r] <= 0) continue;
    cumulative += RARITY_WEIGHTS[r];
    if (roll < cumulative) return r;
  }
  return 'COMMON';
}

function getRarityPct(rarity) {
  var total = computeWeightTotal();
  var w = RARITY_WEIGHTS[rarity] || 0;
  return (w / total) * 100;
}
