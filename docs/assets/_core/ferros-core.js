/**
 * FERROS Core — Shared Runtime Functions
 * ========================================
 * Classic-script IIFE bundle. Exposes window.FerrosCore.
 * Phase 0 only — no ESM, no fetch, no external deps.
 *
 * Functions extracted from personal-profile.html monolith
 * to eliminate duplication across harnesses and pages.
 *
 * Load via: <script src="assets/_core/ferros-core.js"></script>
 */
(function(root) {
  'use strict';

  var FerrosCore = {};

  // ── Version ────────────────────────────────────────────────────────────────
  FerrosCore.VERSION = '1.0';

  // ── computeHash ────────────────────────────────────────────────────────────
  // Returns { hash: string, algorithm: 'sha256' | 'djb2' }
  FerrosCore.computeHash = async function computeHash(data) {
    if (root.crypto && root.crypto.subtle) {
      var enc = new TextEncoder().encode(data);
      var buf = await root.crypto.subtle.digest('SHA-256', enc);
      return {
        hash: Array.from(new Uint8Array(buf)).map(function(b) {
          return b.toString(16).padStart(2, '0');
        }).join(''),
        algorithm: 'sha256'
      };
    }
    // Fallback: djb2 for file:// where crypto.subtle is unavailable
    var h = 5381;
    for (var i = 0; i < data.length; i++) {
      h = ((h << 5) + h) ^ data.charCodeAt(i);
      h = h >>> 0;
    }
    return { hash: 'local-' + h.toString(16).padStart(8, '0'), algorithm: 'djb2' };
  };

  // ── hashWithAlgorithm ─────────────────────────────────────────────────────
  // Deterministic hash helper used by full-chain verification.
  // Returns { ok: true, hash } or { ok: false, code }.
  FerrosCore.hashWithAlgorithm = async function hashWithAlgorithm(data, algorithm) {
    if (algorithm === 'sha256') {
      if (!(root.crypto && root.crypto.subtle)) {
        return { ok: false, code: 'SHA256_UNAVAILABLE' };
      }
      var enc = new TextEncoder().encode(data);
      var buf = await root.crypto.subtle.digest('SHA-256', enc);
      return {
        ok: true,
        hash: Array.from(new Uint8Array(buf)).map(function(b) {
          return b.toString(16).padStart(2, '0');
        }).join('')
      };
    }
    if (algorithm === 'djb2') {
      var h = 5381;
      for (var i = 0; i < data.length; i++) {
        h = ((h << 5) + h) ^ data.charCodeAt(i);
        h = h >>> 0;
      }
      return { ok: true, hash: 'local-' + h.toString(16).padStart(8, '0') };
    }
    return { ok: false, code: 'UNKNOWN_HASH_ALGORITHM' };
  };

  // ── createSealEntry ────────────────────────────────────────────────────────
  // Generates a complete seal entry with nonce, hashAlgorithm, and timestamp.
  // Returns { taskId, seal, previousSeal, timestamp, data, hashAlgorithm, nonce }
  FerrosCore.createSealEntry = async function createSealEntry(taskId, taskData, previousSeal) {
    var prev = previousSeal || 'genesis';
    var nonce = (root.crypto && root.crypto.getRandomValues)
      ? root.crypto.getRandomValues(new Uint32Array(1))[0]
      : Math.floor(Math.random() * 0xFFFFFFFF);
    var ts = new Date().toISOString();
    var payload = JSON.stringify({
      taskId: taskId,
      data: taskData,
      previousSeal: prev,
      timestamp: ts,
      nonce: nonce
    });
    var result = await FerrosCore.computeHash(payload);
    return {
      taskId: taskId,
      seal: result.hash,
      previousSeal: prev,
      timestamp: ts,
      data: taskData,
      hashAlgorithm: result.algorithm,
      nonce: nonce
    };
  };

  // ── verifyChain ────────────────────────────────────────────────────────────
  // Linkage-only verification: checks previousSeal → seal chain integrity.
  // Returns { valid: boolean, brokenAt?: number }
  FerrosCore.verifyChain = function verifyChain(chain) {
    if (!chain || chain.length === 0) return { valid: true };
    if (chain[0].previousSeal !== 'genesis') return { valid: false, brokenAt: 0 };
    for (var i = 1; i < chain.length; i++) {
      if (chain[i].previousSeal !== chain[i - 1].seal) return { valid: false, brokenAt: i };
    }
    return { valid: true };
  };

  // ── verifyChainFull ───────────────────────────────────────────────────────
  // Full verification: linkage + per-entry rehash using stored metadata.
  // Returns { valid: boolean, brokenAt?: number, reason?: string }
  FerrosCore.verifyChainFull = async function verifyChainFull(chain) {
    var linkage = FerrosCore.verifyChain(chain);
    if (!linkage.valid) {
      return { valid: false, brokenAt: linkage.brokenAt, reason: 'LINKAGE_BROKEN' };
    }
    if (!chain || chain.length === 0) {
      return { valid: true };
    }
    for (var i = 0; i < chain.length; i++) {
      var e = chain[i] || {};
      if (!e.taskId || !e.seal || !e.previousSeal || !e.timestamp || e.data === undefined || e.nonce === undefined || !e.hashAlgorithm) {
        return { valid: false, brokenAt: i, reason: 'SEAL_FIELDS_MISSING' };
      }
      var payload = JSON.stringify({
        taskId: e.taskId,
        data: e.data,
        previousSeal: e.previousSeal,
        timestamp: e.timestamp,
        nonce: e.nonce
      });
      var hashResult = await FerrosCore.hashWithAlgorithm(payload, e.hashAlgorithm);
      if (!hashResult.ok) {
        return { valid: false, brokenAt: i, reason: hashResult.code };
      }
      if (hashResult.hash !== e.seal) {
        return { valid: false, brokenAt: i, reason: 'HASH_MISMATCH' };
      }
    }
    return { valid: true };
  };

  // ── canMutateDurableState ──────────────────────────────────────────────────
  // Unified write predicate. Takes flags object so it's free of global state.
  // flags: { tradeWindowAccepted, sessionMode, aliasMode, recoveryMode }
  FerrosCore.canMutateDurableState = function canMutateDurableState(flags) {
    return !!(flags.tradeWindowAccepted && !flags.sessionMode && !flags.aliasMode && !flags.recoveryMode);
  };

  // ── validateImport ─────────────────────────────────────────────────────────
  // Reference import validator (C9-compliant).
  // Returns { ok: boolean, code: string|null, detail?: string }
  FerrosCore.validateImport = function validateImport(raw) {
    if (!raw || typeof raw !== 'object' || Array.isArray(raw)) {
      return { ok: false, code: 'STORAGE_JSON_INVALID', detail: 'not a JSON object' };
    }
    if (!raw.profile) {
      return { ok: false, code: 'STORAGE_SCHEMA_INCOMPLETE', detail: 'profile absent from envelope' };
    }
    var p = raw.profile;
    if (!p.meta) {
      return { ok: false, code: 'STORAGE_SCHEMA_INCOMPLETE', detail: 'profile.meta absent' };
    }
    // Version check — fail fast
    if (p.meta.version === undefined || p.meta.version === null || p.meta.version === '') {
      return { ok: false, code: 'STORAGE_VERSION_MISSING' };
    }
    var importMajor = parseInt(String(p.meta.version).split('.')[0], 10);
    var currentMajor = parseInt(String(FerrosCore.VERSION).split('.')[0], 10);
    if (isNaN(importMajor) || importMajor !== currentMajor) {
      return { ok: false, code: 'STORAGE_VERSION_MISMATCH',
        detail: 'import=' + p.meta.version + ' current=' + FerrosCore.VERSION };
    }
    // genesisHash
    if (!p.meta.genesisHash) {
      return { ok: false, code: 'STORAGE_GENESIS_MISSING' };
    }
    // sealChain
    var chain = (raw.sealChain !== undefined) ? raw.sealChain : (p.sealChain || null);
    if (!chain || !Array.isArray(chain)) {
      return { ok: false, code: 'STORAGE_SEAL_CHAIN_MISSING' };
    }
    // Chain root check
    if (chain.length > 0 && chain[0].previousSeal !== 'genesis') {
      return { ok: false, code: 'STORAGE_SEAL_CHAIN_INVALID_ROOT',
        detail: 'sealChain[0].previousSeal="' + chain[0].previousSeal + '"' };
    }
    // Chain linkage
    for (var i = 1; i < chain.length; i++) {
      if (chain[i].previousSeal !== chain[i - 1].seal) {
        return { ok: false, code: 'STORAGE_SEAL_CHAIN_BROKEN',
          detail: 'at index ' + i + ': previousSeal="' + chain[i].previousSeal + '" expected="' + chain[i - 1].seal + '"' };
      }
    }
    // Identity
    if (!p.identity || !p.identity.name || !String(p.identity.name).trim()) {
      return { ok: false, code: 'STORAGE_IDENTITY_MISSING' };
    }
    // Required profile fields
    var req = ['meta', 'identity', 'attributes', 'skills', 'achievements', 'journal', 'credentials'];
    for (var ri = 0; ri < req.length; ri++) {
      if (!p[req[ri]]) {
        return { ok: false, code: 'STORAGE_SCHEMA_INCOMPLETE', detail: req[ri] + ' absent from profile' };
      }
    }
    return { ok: true, code: null };
  };

  // ── serializeExport ────────────────────────────────────────────────────────
  // Builds canonical export envelope.
  FerrosCore.serializeExport = function serializeExport(profile, sealChain) {
    return {
      ferrosVersion: FerrosCore.VERSION,
      exportedAt: new Date().toISOString(),
      profile: profile,
      sealChain: sealChain
    };
  };

  // ── TEMPLATE_PROFILES ──────────────────────────────────────────────────────
  // Populated by generate-ferros-core.ps1 from templates.json.
  // Inline fallback preserved for standalone loading.
  FerrosCore.TEMPLATE_PROFILES = [{"id":"tesla","name":"Nikola Tesla","icon":"⚡","aliasClass":"Engineer","tagline":"Worked obsessively. Slept rarely. Changed everything.","stream":"A","archetype":"deep-work-nocturnal","templateSchedule":{"blocks":[{"time":"10:00","label":"Rise \u0026 Coffee"},{"time":"11:00","label":"Laboratory Work"},{"time":"14:00","label":"Correspondence"},{"time":"18:00","label":"Dinner (often skipped)"},{"time":"20:00","label":"Deep Experiment Block"},{"time":"02:00","label":"Rest (minimal)"}]}},{"id":"kahlo","name":"Frida Kahlo","icon":"🎨","aliasClass":"Artisan","tagline":"Pain became pigment. Every canvas, a self-portrait.","stream":"B","archetype":"pain-driven-creative","templateSchedule":{"blocks":[{"time":"08:00","label":"Morning Pain Management"},{"time":"10:00","label":"Studio Setup"},{"time":"11:00","label":"Painting Session"},{"time":"15:00","label":"Rest \u0026 Recovery"},{"time":"17:00","label":"Writing \u0026 Reflection"},{"time":"20:00","label":"Social / Diego"}]}},{"id":"curie","name":"Marie Curie","icon":"☢️","aliasClass":"Scholar","tagline":"Two Nobel Prizes. Zero shortcuts.","stream":"A","archetype":"structured-research","templateSchedule":{"blocks":[{"time":"06:00","label":"Rise \u0026 Breakfast"},{"time":"07:00","label":"Laboratory"},{"time":"12:00","label":"Lunch \u0026 Brief Rest"},{"time":"13:00","label":"Continued Research"},{"time":"17:00","label":"Reading \u0026 Papers"},{"time":"20:00","label":"Evening Study"},{"time":"22:00","label":"Sleep"}]}},{"id":"aurelius","name":"Marcus Aurelius","icon":"🏛️","aliasClass":"Guardian","tagline":"Rule an empire. Master yourself first.","stream":"A","archetype":"stoic-morning-ruler","templateSchedule":{"blocks":[{"time":"04:00","label":"Rise"},{"time":"04:30","label":"Journaling \u0026 Meditations"},{"time":"06:00","label":"Physical Training"},{"time":"08:00","label":"Council \u0026 Governance"},{"time":"12:00","label":"Lunch \u0026 Walk"},{"time":"14:00","label":"Correspondence \u0026 Edicts"},{"time":"17:00","label":"Philosophy Study"},{"time":"20:00","label":"Family \u0026 Rest"}]}},{"id":"fry","name":"Philip J. Fry","icon":"📺","aliasClass":"Guided","tagline":"Slept 1000 years. Still figuring it out.","stream":"B","archetype":"flexible-chaotic","templateSchedule":{"blocks":[{"time":"11:00","label":"Wake Up (usually)"},{"time":"12:00","label":"Food (whatever\u0027s available)"},{"time":"14:00","label":"Something happens"},{"time":"17:00","label":"Hang out with Bender"},{"time":"20:00","label":"Pizza \u0026 TV"},{"time":"23:00","label":"Eventually sleep"}]}},{"id":"nightingale","name":"Florence Nightingale","icon":"🕯️","aliasClass":"Healer","tagline":"Data-driven compassion before it had a name.","stream":"C","archetype":"systems-care-rotational","templateSchedule":{"blocks":[{"time":"05:00","label":"Rise \u0026 Ward Review"},{"time":"06:00","label":"Patient Rounds"},{"time":"10:00","label":"Statistical Analysis"},{"time":"13:00","label":"Lunch \u0026 Admin"},{"time":"14:00","label":"Staff Coordination"},{"time":"17:00","label":"Evening Rounds"},{"time":"20:00","label":"Report Writing"},{"time":"22:00","label":"Rest"}]}},{"id":"lovelace","name":"Ada Lovelace","icon":"🔢","aliasClass":"Architect","tagline":"Wrote the first algorithm. The machine wasn\u0027t built yet.","stream":"A","archetype":"analytical-visionary","templateSchedule":{"blocks":[{"time":"09:00","label":"Rise (health permitting)"},{"time":"10:00","label":"Mathematical Correspondence"},{"time":"12:00","label":"Algorithm Development"},{"time":"15:00","label":"Tea \u0026 Social"},{"time":"16:00","label":"Writing \u0026 Notes"},{"time":"19:00","label":"Evening Calculations"},{"time":"22:00","label":"Rest"}]}},{"id":"malone","name":"Sam Malone","icon":"🍺","aliasClass":"Community","tagline":"Everyone knows your name. That\u0027s the whole point.","stream":"B","archetype":"service-flexible-shift","templateSchedule":{"blocks":[{"time":"10:00","label":"Open Cheers"},{"time":"12:00","label":"Lunch Rush"},{"time":"15:00","label":"Quiet Afternoon Prep"},{"time":"17:00","label":"Happy Hour Begins"},{"time":"21:00","label":"Evening Service"},{"time":"00:00","label":"Close \u0026 Clean"},{"time":"02:00","label":"Sleep"}]}},{"id":"turing","name":"Alan Turing","icon":"🖥️","aliasClass":"Engineer","tagline":"Broke codes. Built machines. Ran marathons.","stream":"A","archetype":"structured-research","templateSchedule":{"blocks":[{"time":"08:00","label":"Morning Run (5–10 miles)"},{"time":"09:30","label":"Codebreaking / Logic Work"},{"time":"12:00","label":"Lunch"},{"time":"13:00","label":"Chess \u0026 Puzzles"},{"time":"14:00","label":"Machine Design / Theory"},{"time":"18:00","label":"Lecture / Paper Writing"},{"time":"22:00","label":"Sleep"}]}},{"id":"darwin","name":"Charles Darwin","icon":"🦕","aliasClass":"Scholar","tagline":"Observed everything. Concluded slowly. Changed everything.","stream":"C","archetype":"structured-research","templateSchedule":{"blocks":[{"time":"07:00","label":"Early Walk"},{"time":"08:00","label":"Breakfast \u0026 Post"},{"time":"09:30","label":"Science Work (prime hours)"},{"time":"12:00","label":"Rest \u0026 Lunch"},{"time":"15:00","label":"Letters \u0026 Reading"},{"time":"16:00","label":"Walk (thinking time)"},{"time":"18:00","label":"Family Time"},{"time":"22:00","label":"Sleep"}]}},{"id":"woolf","name":"Virginia Woolf","icon":"📖","aliasClass":"Artisan","tagline":"Three hours of writing per day. The rest was living.","stream":"B","archetype":"pain-driven-creative","templateSchedule":{"blocks":[{"time":"09:30","label":"Writing (3 hrs, no interruptions)"},{"time":"12:30","label":"Lunch \u0026 Walk"},{"time":"14:00","label":"Rest / Reading"},{"time":"16:00","label":"Letters \u0026 Social"},{"time":"18:00","label":"Tea \u0026 Conversation"},{"time":"22:00","label":"Sleep"}]}},{"id":"jobs","name":"Steve Jobs","icon":"🍏","aliasClass":"Architect","tagline":"Design first. Ship second. Repeat.","stream":"B","archetype":"stoic-morning-ruler","templateSchedule":{"blocks":[{"time":"06:00","label":"Wake \u0026 Reflect"},{"time":"07:00","label":"Family Breakfast"},{"time":"09:00","label":"Design Review"},{"time":"12:00","label":"Walk Meeting"},{"time":"14:00","label":"Product Decisions"},{"time":"17:00","label":"Email \u0026 Calls"},{"time":"19:00","label":"Family Dinner"},{"time":"22:00","label":"Sleep"}]}}];

  // ── Expose ─────────────────────────────────────────────────────────────────
  root.FerrosCore = FerrosCore;

})(typeof window !== 'undefined' ? window : this);
