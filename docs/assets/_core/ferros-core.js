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

  var PROFILE_STORAGE_KEY = 'ferros_profile';
  var SEAL_CHAIN_STORAGE_KEY = 'ferros_seal_chain';
  var ALIAS_SESSION_STORAGE_KEY = 'ferros_alias_session';
  var AUDIT_TRAIL_CAP = 1000;
  var AUDIT_ACTIONS = ['seal-added', 'profile-saved', 'profile-imported', 'alias-claimed', 'recovery-claimed'];
  var PORTABLE_LOG_XP_PER_ENTRY = 15;

  FerrosCore.ALIAS_SESSION_STORAGE_KEY = ALIAS_SESSION_STORAGE_KEY;
  FerrosCore.PORTABLE_LOG_XP_PER_ENTRY = PORTABLE_LOG_XP_PER_ENTRY;

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

  function getDurableStorage() {
    try {
      return root.localStorage || null;
    } catch (error) {
      return null;
    }
  }

  function cloneJson(value) {
    if (value === undefined) return undefined;
    return JSON.parse(JSON.stringify(value));
  }

  function isPlainObject(value) {
    return !!value && typeof value === 'object' && !Array.isArray(value);
  }

  function buildPortableEntryPayload(entry) {
    return JSON.stringify({
      ts: entry.ts,
      text: entry.text,
      type: entry.type || 'activity'
    });
  }

  function computePortableEntrySeal(entry) {
    var payload = buildPortableEntryPayload(entry);
    var h = 5381;
    for (var i = 0; i < payload.length; i++) {
      h = ((h << 5) + h) ^ payload.charCodeAt(i);
      h = h >>> 0;
    }
    return 'local-' + h.toString(16).padStart(8, '0');
  }

  function makePortableSessionId(baseId, sessionStart) {
    return String(baseId || 'session') + '-' + String(sessionStart || '');
  }

  function normalizePortableEntry(entry, index) {
    if (!isPlainObject(entry)) {
      return { ok: false, code: 'PORTABLE_LOG_ENTRY_INVALID', detail: 'entry[' + index + '] is not an object' };
    }
    if (!entry.ts || typeof entry.ts !== 'string') {
      return { ok: false, code: 'PORTABLE_LOG_ENTRY_INVALID', detail: 'entry[' + index + '].ts missing' };
    }
    if (typeof entry.text !== 'string' || !entry.text.trim()) {
      return { ok: false, code: 'PORTABLE_LOG_ENTRY_INVALID', detail: 'entry[' + index + '].text missing' };
    }
    return {
      ok: true,
      entry: {
        ts: entry.ts,
        text: entry.text,
        type: entry.type || 'activity',
        seal: entry.seal === undefined ? null : entry.seal
      }
    };
  }

  function normalizeRecoveryInfo(log) {
    if (isPlainObject(log.recovery)) {
      return {
        ok: true,
        recovery: {
          profileName: log.recovery.profileName || null,
          genesisHash: log.recovery.genesisHash || null,
          attribution: log.recovery.attribution || 'self',
          integrityWarning: !!log.recovery.integrityWarning
        }
      };
    }

    if (isPlainObject(log.profile)) {
      return {
        ok: true,
        recovery: {
          profileName: log.profile.id || log.profile.profileName || null,
          genesisHash: log.profile.genesisHash || null,
          attribution: log.profile.attribution || 'self',
          integrityWarning: !!log.profile.integrityWarning
        }
      };
    }

    return { ok: false, code: 'PORTABLE_LOG_RECOVERY_REQUIRED', detail: 'recovery info missing' };
  }

  async function verifyPortableEntrySeal(entry) {
    if (!entry.seal) {
      return { ok: false, warningCode: 'PORTABLE_LOG_ENTRY_SEAL_MISSING' };
    }

    var portableSeal = computePortableEntrySeal(entry);
    if (portableSeal === entry.seal) {
      return { ok: true, algorithm: 'djb2' };
    }

    if (/^[0-9a-f]{64}$/i.test(entry.seal)) {
      var shaResult = await FerrosCore.hashWithAlgorithm(buildPortableEntryPayload(entry), 'sha256');
      if (shaResult.ok && shaResult.hash === entry.seal) {
        return { ok: true, algorithm: 'sha256' };
      }
      if (!shaResult.ok) {
        return { ok: false, warningCode: shaResult.code };
      }
    }

    return { ok: false, warningCode: 'PORTABLE_LOG_ENTRY_SEAL_MISMATCH' };
  }

  function normalizeLoadedProfile(profile, sealChain) {
    if (profile.meta) {
      if (typeof profile.meta.revision !== 'number') profile.meta.revision = 0;
      if (typeof profile.meta.xp !== 'number') profile.meta.xp = 0;
      if (!Array.isArray(profile.meta.claimedAliasSessions)) profile.meta.claimedAliasSessions = [];
      if (typeof profile.meta.sealBroken !== 'boolean') profile.meta.sealBroken = false;
      if (typeof profile.meta.schemaVersion !== 'number') profile.meta.schemaVersion = 1;
    }
    profile.sealChain = Array.isArray(sealChain) ? sealChain : [];
    if (!Array.isArray(profile.auditTrail)) profile.auditTrail = [];
    if (!profile.schedule || typeof profile.schedule !== 'object' || Array.isArray(profile.schedule)) {
      profile.schedule = {};
    }
    if (profile.schedule.archetype === undefined) {
      profile.schedule.archetype = profile.identity && profile.identity.archetype ? profile.identity.archetype : null;
    }
    if (profile.schedule.activeDeck === undefined) profile.schedule.activeDeck = null;
    if (profile.schedule.wakeTime === undefined) {
      profile.schedule.wakeTime = profile.identity && profile.identity.wakeTime ? profile.identity.wakeTime : '07:00';
    }
    if (profile.schedule.sleepTime === undefined) {
      profile.schedule.sleepTime = profile.identity && profile.identity.sleepTime ? profile.identity.sleepTime : '23:00';
    }
    if (!Array.isArray(profile.schedule.slots)) profile.schedule.slots = [];
    if (!profile.completions || typeof profile.completions !== 'object' || Array.isArray(profile.completions)) {
      profile.completions = {};
    }
    if (!profile.creditLog || typeof profile.creditLog !== 'object' || Array.isArray(profile.creditLog)) {
      profile.creditLog = {};
    }
    if (!Array.isArray(profile.bag)) profile.bag = [];
    return profile;
  }

  // ── loadProfile ────────────────────────────────────────────────────────────
  // Shared localStorage loader for consumer surfaces.
  // Returns { ok, code?, detail?, profile?, sealChain? }.
  FerrosCore.loadProfile = function loadProfile() {
    var storage = getDurableStorage();
    if (!storage) {
      return { ok: false, code: 'STORAGE_UNAVAILABLE' };
    }
    var raw = storage.getItem(PROFILE_STORAGE_KEY);
    if (!raw) {
      return { ok: false, code: 'PROFILE_NOT_FOUND' };
    }

    var parsed;
    try {
      parsed = JSON.parse(raw);
    } catch (error) {
      return { ok: false, code: 'STORAGE_JSON_INVALID', detail: 'Profile data is not valid JSON.' };
    }

    var chain = parsed.sealChain;
    if (!Array.isArray(chain)) {
      var fallbackRaw = storage.getItem(SEAL_CHAIN_STORAGE_KEY);
      if (fallbackRaw) {
        try {
          chain = JSON.parse(fallbackRaw);
        } catch (fallbackError) {
          chain = [];
        }
      } else {
        chain = [];
      }
    }

    if (parsed.meta && parsed.meta.genesisHash === null && parsed.meta.stage > 0) {
      return { ok: false, code: 'STORAGE_GENESIS_STAGE_MISMATCH', detail: 'Profile has stage > 0 but no genesis hash.' };
    }
    if (parsed.meta && typeof parsed.meta.sealCount === 'number' && chain.length !== parsed.meta.sealCount) {
      return {
        ok: false,
        code: 'STORAGE_SEAL_COUNT_MISMATCH',
        detail: 'Seal chain length (' + chain.length + ') does not match meta.sealCount (' + parsed.meta.sealCount + ').'
      };
    }
    if (parsed.meta && parsed.meta.currentSeal && chain.length > 0 && chain[chain.length - 1].seal !== parsed.meta.currentSeal) {
      return { ok: false, code: 'STORAGE_LAST_SEAL_MISMATCH', detail: 'Last seal in chain does not match meta.currentSeal.' };
    }

    normalizeLoadedProfile(parsed, chain);
    return { ok: true, code: null, profile: parsed, sealChain: chain };
  };

  // ── pushAuditEntry ─────────────────────────────────────────────────────────
  // Appends a bounded audit entry to the profile-scoped audit trail.
  // Returns { ok, code?, entry?, profile }.
  FerrosCore.pushAuditEntry = function pushAuditEntry(profile, action, detail) {
    if (!profile || typeof profile !== 'object' || Array.isArray(profile)) {
      return { ok: false, code: 'PROFILE_REQUIRED' };
    }
    if (AUDIT_ACTIONS.indexOf(action) === -1) {
      return { ok: false, code: 'AUDIT_ACTION_INVALID', detail: action };
    }
    if (!Array.isArray(profile.auditTrail)) profile.auditTrail = [];
    var entry = {
      ts: new Date().toISOString(),
      action: action,
      detail: detail === undefined ? null : detail
    };
    profile.auditTrail.push(entry);
    if (profile.auditTrail.length > AUDIT_TRAIL_CAP) {
      profile.auditTrail = profile.auditTrail.slice(profile.auditTrail.length - AUDIT_TRAIL_CAP);
    }
    return { ok: true, code: null, entry: entry, profile: profile };
  };

  // ── saveProfile ────────────────────────────────────────────────────────────
  // Shared localStorage writer for consumer surfaces.
  // options: { sealChain?, flags, skipAudit?, auditDetail? }
  // Returns { ok, code?, detail?, profile?, sealChain? }.
  FerrosCore.saveProfile = function saveProfile(profile, options) {
    if (!profile || typeof profile !== 'object' || Array.isArray(profile)) {
      return { ok: false, code: 'PROFILE_REQUIRED' };
    }

    var storage = getDurableStorage();
    if (!storage) {
      return { ok: false, code: 'STORAGE_UNAVAILABLE' };
    }

    var opts = options || {};
    if (!opts.flags) {
      return { ok: false, code: 'DURABLE_WRITE_FLAGS_REQUIRED' };
    }
    if (!FerrosCore.canMutateDurableState(opts.flags)) {
      return { ok: false, code: 'DURABLE_WRITE_FORBIDDEN' };
    }

    var sealChain = Array.isArray(opts.sealChain)
      ? opts.sealChain
      : (Array.isArray(profile.sealChain) ? profile.sealChain : []);

    normalizeLoadedProfile(profile, sealChain);

    var shapeCheck = FerrosCore.validateProfileShape(profile);
    if (!shapeCheck.ok) {
      return { ok: false, code: shapeCheck.code, detail: shapeCheck.detail };
    }

    profile.meta.lastModified = new Date().toISOString();
    profile.meta.revision = (profile.meta.revision || 0) + 1;
    profile.meta.sealCount = sealChain.length;
    profile.meta.currentSeal = sealChain.length ? sealChain[sealChain.length - 1].seal : null;
    if (sealChain.length === 1 && !profile.meta.genesisHash) {
      profile.meta.genesisHash = sealChain[0].seal;
    }
    profile.sealChain = sealChain;

    if (!opts.skipAudit) {
      var auditResult = FerrosCore.pushAuditEntry(
        profile,
        'profile-saved',
        opts.auditDetail || { sealCount: sealChain.length }
      );
      if (!auditResult.ok) {
        return auditResult;
      }
    }

    try {
      storage.setItem(PROFILE_STORAGE_KEY, JSON.stringify(profile));
      storage.setItem(SEAL_CHAIN_STORAGE_KEY, JSON.stringify(sealChain));
    } catch (error) {
      if (error.name === 'QuotaExceededError' || error.code === 22) {
        return { ok: false, code: 'STORAGE_QUOTA_EXCEEDED' };
      }
      throw error;
    }

    return { ok: true, code: null, profile: profile, sealChain: sealChain };
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

  // ── validateProfileShape ────────────────────────────────────────────────────
  // Write-time shape validator. Rejects profiles with undeclared fields.
  // Returns { ok: boolean, code: string|null, detail?: string }
  FerrosCore.validateProfileShape = function validateProfileShape(p) {
    if (!p || typeof p !== 'object' || Array.isArray(p)) {
      return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'not a plain object' };
    }

    // Top-level allowed properties (profile.schema.json)
    var topAllowed = ['meta','identity','attributes','skills','achievements','journal',
      'credentials','sealChain','auditTrail','schedule','completions','creditLog','bag'];
    var topKeys = Object.keys(p);
    for (var i = 0; i < topKeys.length; i++) {
      if (topAllowed.indexOf(topKeys[i]) === -1) {
        return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'undeclared top-level field: ' + topKeys[i] };
      }
    }

    // Required top-level fields
    var topRequired = ['meta','identity','attributes','skills','achievements','journal','credentials','sealChain'];
    for (var r = 0; r < topRequired.length; r++) {
      if (p[topRequired[r]] === undefined || p[topRequired[r]] === null) {
        return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'missing required field: ' + topRequired[r] };
      }
    }

    // meta allowed properties
    if (typeof p.meta !== 'object' || Array.isArray(p.meta)) {
      return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'meta is not an object' };
    }
    var metaAllowed = ['version','created','lastModified','assistanceLevel','genesisHash',
      'currentSeal','sealCount','stage','anchoredToLedger','ledgerTxHash',
      'schemaVersion','claimedAliasSessions','xp','sealBroken','revision'];
    var metaKeys = Object.keys(p.meta);
    for (var m = 0; m < metaKeys.length; m++) {
      if (metaAllowed.indexOf(metaKeys[m]) === -1) {
        return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'undeclared meta field: ' + metaKeys[m] };
      }
    }

    // meta required fields
    var metaRequired = ['version','created','lastModified','assistanceLevel','genesisHash','currentSeal','sealCount','stage'];
    for (var mr = 0; mr < metaRequired.length; mr++) {
      if (p.meta[metaRequired[mr]] === undefined) {
        return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'missing required meta field: ' + metaRequired[mr] };
      }
    }

    // journal entry type validation (spot-check — validate each entry type is in enum)
    var journalTypeEnum = ['activity','journal','system','claim-event','claimed-alias','claimed-recovery'];
    if (Array.isArray(p.journal)) {
      for (var j = 0; j < p.journal.length; j++) {
        var jt = p.journal[j].type;
        if (jt !== undefined && journalTypeEnum.indexOf(jt) === -1) {
          return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'invalid journal entry type at index ' + j + ': ' + jt };
        }
      }
    }

    if (Array.isArray(p.auditTrail)) {
      for (var at = 0; at < p.auditTrail.length; at++) {
        var auditEntry = p.auditTrail[at];
        if (!auditEntry || !auditEntry.ts || !auditEntry.action) {
          return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'auditTrail entry at index ' + at + ' missing ts or action' };
        }
        if (AUDIT_ACTIONS.indexOf(auditEntry.action) === -1) {
          return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'invalid auditTrail action at index ' + at + ': ' + auditEntry.action };
        }
      }
    }

    // achievement required fields
    if (Array.isArray(p.achievements)) {
      for (var a = 0; a < p.achievements.length; a++) {
        var ach = p.achievements[a];
        if (!ach.id || !ach.name) {
          return { ok: false, code: 'PROFILE_SHAPE_INVALID', detail: 'achievement at index ' + a + ' missing id or name' };
        }
      }
    }

    return { ok: true, code: null };
  };

  // ── generateRuntimeNonce (A4) ──────────────────────────────────────────────
  // Generates a random nonce string for message authentication.
  // Uses crypto.getRandomValues when available, falls back to Math.random.
  FerrosCore.generateRuntimeNonce = function generateRuntimeNonce() {
    if (root.crypto && root.crypto.getRandomValues) {
      var arr = new Uint8Array(16);
      root.crypto.getRandomValues(arr);
      return Array.from(arr).map(function(b) { return b.toString(16).padStart(2, '0'); }).join('');
    }
    // Fallback for file:// without crypto
    var s = '';
    for (var i = 0; i < 32; i++) { s += Math.floor(Math.random() * 16).toString(16); }
    return s;
  };

  // ── validateRuntimeMessage (A4) ────────────────────────────────────────────
  // Returns true if the message contains a valid nonce matching the expected one.
  // msg: the postMessage data object. expectedNonce: the nonce from ferros:init.
  FerrosCore.validateRuntimeMessage = function validateRuntimeMessage(msg, expectedNonce) {
    if (!msg || typeof msg !== 'object') return false;
    if (!expectedNonce) return false;
    return msg.nonce === expectedNonce;
  };

  // ── templateBlockToEvent (C1) ──────────────────────────────────────────────
  // Transforms a template schedule block ({time, label}) into a C6 schedule-event.
  // templateId: the template's ID (e.g. 'tesla'). blockIndex: 0-based index for ID gen.
  // stream (optional): template's stream attribute.
  FerrosCore.templateBlockToEvent = function templateBlockToEvent(block, templateId, blockIndex, stream) {
    var evt = {
      id: (templateId || 'tpl') + '-block-' + blockIndex,
      kind: 'block',
      label: block.label,
      time: block.time,
      source: { type: 'template', templateId: templateId || 'unknown' }
    };
    if (stream) evt.stream = stream;
    return evt;
  };

  // ── templateToEvents (C1) ──────────────────────────────────────────────────
  // Transforms an entire template's schedule blocks into an array of C6 events.
  // template: object with {id, stream?, templateSchedule: {blocks: [{time, label}]}}.
  FerrosCore.templateToEvents = function templateToEvents(template) {
    if (!template || !template.templateSchedule || !Array.isArray(template.templateSchedule.blocks)) {
      return [];
    }
    return template.templateSchedule.blocks.map(function(block, i) {
      return FerrosCore.templateBlockToEvent(block, template.id, i, template.stream);
    });
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

  // ── createAliasSession ────────────────────────────────────────────────────
  // Creates a canonical alias session object for sessionStorage-backed flows.
  FerrosCore.createAliasSession = function createAliasSession(alias, options) {
    if (!isPlainObject(alias) || !alias.id || !alias.name) {
      return { ok: false, code: 'ALIAS_SESSION_ALIAS_REQUIRED', detail: 'alias.id and alias.name are required' };
    }

    var opts = options || {};
    var sessionStart = opts.sessionStart || new Date().toISOString();
    var session = {
      sessionId: makePortableSessionId(alias.id, sessionStart),
      alias: {
        id: alias.id,
        name: alias.name,
        icon: alias.icon || '',
        class: alias.aliasClass || alias.class || '',
        attribution: 'unlinked'
      },
      sessionStart: sessionStart,
      entries: []
    };

    return { ok: true, code: null, session: session };
  };

  // ── appendAliasSessionEntry ───────────────────────────────────────────────
  // Appends a portable-log entry to an alias session.
  FerrosCore.appendAliasSessionEntry = function appendAliasSessionEntry(session, entry) {
    if (!isPlainObject(session) || !Array.isArray(session.entries)) {
      return { ok: false, code: 'ALIAS_SESSION_REQUIRED', detail: 'session.entries missing' };
    }

    var normalized = normalizePortableEntry(entry, session.entries.length);
    if (!normalized.ok) return normalized;

    session.entries.push({
      ts: normalized.entry.ts,
      text: normalized.entry.text,
      type: normalized.entry.type,
      seal: null
    });

    return { ok: true, code: null, session: session };
  };

  // ── serializeAliasSessionLog ──────────────────────────────────────────────
  // Builds the canonical .ferros-log alias envelope with sessionId and seals.
  FerrosCore.serializeAliasSessionLog = function serializeAliasSessionLog(session, options) {
    if (!isPlainObject(session) || !isPlainObject(session.alias) || !Array.isArray(session.entries)) {
      return { ok: false, code: 'ALIAS_SESSION_REQUIRED', detail: 'alias session missing alias or entries' };
    }

    if (!session.sessionId) {
      return { ok: false, code: 'PORTABLE_LOG_SESSION_ID_REQUIRED', detail: 'alias session missing sessionId' };
    }

    var opts = options || {};
    var entries = [];
    for (var i = 0; i < session.entries.length; i++) {
      var normalized = normalizePortableEntry(session.entries[i], i);
      if (!normalized.ok) return normalized;
      entries.push({
        ts: normalized.entry.ts,
        text: normalized.entry.text,
        type: normalized.entry.type,
        seal: computePortableEntrySeal(normalized.entry)
      });
    }

    return {
      ok: true,
      code: null,
      log: {
        ferrosVersion: FerrosCore.VERSION,
        logType: 'alias-session',
        sessionId: session.sessionId,
        alias: {
          id: session.alias.id,
          name: session.alias.name,
          icon: session.alias.icon || '',
          class: session.alias.class || '',
          attribution: 'unlinked'
        },
        sessionStart: session.sessionStart,
        sessionEnd: opts.sessionEnd || new Date().toISOString(),
        entries: entries,
        entryCount: entries.length,
        claimInstructions: 'Import this file on your home FERROS instance to claim these logs and merge them into your profile. Unlinked until claimed.'
      }
    };
  };

  // ── serializeRecoverySessionLog ───────────────────────────────────────────
  // Builds the canonical .ferros-log recovery envelope with sessionId and seals.
  FerrosCore.serializeRecoverySessionLog = function serializeRecoverySessionLog(session, options) {
    if (!isPlainObject(session) || !isPlainObject(session.recovery) || !Array.isArray(session.entries)) {
      return { ok: false, code: 'RECOVERY_SESSION_REQUIRED', detail: 'recovery session missing recovery or entries' };
    }

    if (!session.sessionId) {
      return { ok: false, code: 'PORTABLE_LOG_SESSION_ID_REQUIRED', detail: 'recovery session missing sessionId' };
    }

    var opts = options || {};
    var entries = [];
    for (var i = 0; i < session.entries.length; i++) {
      var normalized = normalizePortableEntry(session.entries[i], i);
      if (!normalized.ok) return normalized;
      entries.push({
        ts: normalized.entry.ts,
        text: normalized.entry.text,
        type: normalized.entry.type,
        seal: computePortableEntrySeal(normalized.entry)
      });
    }

    return {
      ok: true,
      code: null,
      log: {
        ferrosVersion: FerrosCore.VERSION,
        logType: 'recovery-session',
        sessionId: session.sessionId,
        recovery: {
          profileName: session.recovery.profileName || null,
          genesisHash: session.recovery.genesisHash || null,
          attribution: 'self',
          integrityWarning: !!session.recovery.integrityWarning
        },
        sessionStart: session.sessionStart,
        sessionEnd: opts.sessionEnd || new Date().toISOString(),
        entries: entries,
        entryCount: entries.length,
        claimInstructions: 'Import this file on your home FERROS instance to claim these logs and merge them into your profile.'
      }
    };
  };

  // ── validatePortableLog ───────────────────────────────────────────────────
  // Validates and normalizes canonical .ferros-log envelopes for claim flows.
  FerrosCore.validatePortableLog = async function validatePortableLog(raw) {
    if (!isPlainObject(raw)) {
      return { ok: false, code: 'PORTABLE_LOG_REQUIRED', detail: 'portable log must be an object' };
    }

    if (!raw.ferrosVersion || typeof raw.ferrosVersion !== 'string') {
      return { ok: false, code: 'PORTABLE_LOG_VERSION_REQUIRED', detail: 'ferrosVersion missing' };
    }

    if (raw.logType !== 'alias-session' && raw.logType !== 'recovery-session') {
      return { ok: false, code: 'PORTABLE_LOG_TYPE_INVALID', detail: 'logType must be alias-session or recovery-session' };
    }

    if (!raw.sessionId || typeof raw.sessionId !== 'string') {
      return { ok: false, code: 'PORTABLE_LOG_SESSION_ID_REQUIRED', detail: 'sessionId missing' };
    }

    if (!Array.isArray(raw.entries)) {
      return { ok: false, code: 'PORTABLE_LOG_ENTRIES_REQUIRED', detail: 'entries array missing' };
    }

    if (typeof raw.entryCount !== 'number' || raw.entryCount < 0) {
      return { ok: false, code: 'PORTABLE_LOG_ENTRY_COUNT_INVALID', detail: 'entryCount missing or invalid' };
    }

    if (raw.entryCount !== raw.entries.length) {
      return { ok: false, code: 'PORTABLE_LOG_ENTRY_COUNT_MISMATCH', detail: 'entryCount=' + raw.entryCount + ' entries.length=' + raw.entries.length };
    }

    var normalizedLog = {
      ferrosVersion: raw.ferrosVersion,
      logType: raw.logType,
      sessionId: raw.sessionId,
      sessionStart: raw.sessionStart || null,
      sessionEnd: raw.sessionEnd || null,
      entries: [],
      entryCount: raw.entryCount,
      claimInstructions: raw.claimInstructions || ''
    };

    if (raw.logType === 'alias-session') {
      if (!isPlainObject(raw.alias) || !raw.alias.id || !raw.alias.name) {
        return { ok: false, code: 'PORTABLE_LOG_ALIAS_REQUIRED', detail: 'alias identity missing' };
      }
      normalizedLog.alias = {
        id: raw.alias.id,
        name: raw.alias.name,
        icon: raw.alias.icon || '',
        class: raw.alias.class || '',
        attribution: raw.alias.attribution || 'unlinked'
      };
    } else {
      var recoveryInfo = normalizeRecoveryInfo(raw);
      if (!recoveryInfo.ok) return recoveryInfo;
      if (!recoveryInfo.recovery.genesisHash) {
        return { ok: false, code: 'PORTABLE_LOG_RECOVERY_REQUIRED', detail: 'recovery.genesisHash missing' };
      }
      normalizedLog.recovery = recoveryInfo.recovery;
    }

    var warnings = [];
    var integrityWarning = !!(normalizedLog.recovery && normalizedLog.recovery.integrityWarning);

    for (var i = 0; i < raw.entries.length; i++) {
      var normalizedEntry = normalizePortableEntry(raw.entries[i], i);
      if (!normalizedEntry.ok) return normalizedEntry;
      normalizedLog.entries.push(normalizedEntry.entry);

      var sealCheck = await verifyPortableEntrySeal(normalizedEntry.entry);
      if (!sealCheck.ok) {
        integrityWarning = true;
        warnings.push({ index: i, code: sealCheck.warningCode });
      }
    }

    if (normalizedLog.recovery) {
      normalizedLog.recovery.integrityWarning = integrityWarning;
    }

    return {
      ok: true,
      code: null,
      log: normalizedLog,
      sessionId: normalizedLog.sessionId,
      entryCount: normalizedLog.entries.length,
      xpGain: normalizedLog.entries.length * PORTABLE_LOG_XP_PER_ENTRY,
      integrityWarning: integrityWarning,
      warnings: warnings
    };
  };

  // ── applyPortableLogClaim ─────────────────────────────────────────────────
  // Canonical claim path: validate, dedupe, merge, seal, audit, and optionally persist.
  FerrosCore.applyPortableLogClaim = async function applyPortableLogClaim(profile, rawLog, options) {
    if (!isPlainObject(profile)) {
      return { ok: false, code: 'PROFILE_REQUIRED', detail: 'profile must be an object' };
    }

    var opts = options || {};
    if (opts.persist) {
      if (!opts.flags) {
        return { ok: false, code: 'DURABLE_WRITE_FLAGS_REQUIRED' };
      }
      if (!FerrosCore.canMutateDurableState(opts.flags)) {
        return { ok: false, code: 'DURABLE_WRITE_FORBIDDEN' };
      }
    }

    var validated = await FerrosCore.validatePortableLog(rawLog);
    if (!validated.ok) return validated;

    var log = validated.log;
    var isRecovery = log.logType === 'recovery-session';
    var mergedProfile = cloneJson(profile);
    normalizeLoadedProfile(mergedProfile, Array.isArray(mergedProfile.sealChain) ? mergedProfile.sealChain : []);

    var claimed = Array.isArray(mergedProfile.meta.claimedAliasSessions)
      ? mergedProfile.meta.claimedAliasSessions.slice()
      : [];
    if (claimed.indexOf(log.sessionId) !== -1) {
      return { ok: false, code: 'CLAIM_DUPLICATE_SESSION', detail: 'sessionId already claimed: ' + log.sessionId };
    }

    var now = opts.now || new Date().toISOString();
    var genesisHash = mergedProfile.meta && mergedProfile.meta.genesisHash ? mergedProfile.meta.genesisHash : null;
    var summaryText;
    var mergedEntries = [];

    if (isRecovery) {
      var recoveryName = log.recovery.profileName || 'Recovery';
      summaryText = 'Claimed ' + log.entries.length + ' recovery log ' + (log.entries.length === 1 ? 'entry' : 'entries') + ' from ' + recoveryName + '.';
      if (validated.integrityWarning) summaryText += ' Integrity warning recorded.';
      for (var i = 0; i < log.entries.length; i++) {
        mergedEntries.push({
          ts: log.entries[i].ts || now,
          text: log.entries[i].text,
          type: 'claimed-recovery',
          linkedTo: genesisHash,
          claimId: log.sessionId,
          sealBroken: !!validated.integrityWarning
        });
      }
    } else {
      summaryText = 'Claimed ' + log.entries.length + ' alias log ' + (log.entries.length === 1 ? 'entry' : 'entries') + ' from ' + log.alias.name + ' (' + log.alias.id + ').';
      if (validated.integrityWarning) summaryText += ' Integrity warning recorded.';
      for (var j = 0; j < log.entries.length; j++) {
        mergedEntries.push({
          ts: log.entries[j].ts || now,
          text: log.entries[j].text,
          type: 'claimed-alias',
          aliasId: log.alias.id,
          aliasName: log.alias.name,
          linkedTo: genesisHash,
          claimId: log.sessionId,
          sealBroken: !!validated.integrityWarning
        });
      }
    }

    var summaryEntry = {
      ts: now,
      text: summaryText,
      type: 'claim-event',
      aliasId: isRecovery ? 'recovery' : log.alias.id,
      aliasName: isRecovery ? (log.recovery.profileName || 'Recovery') : log.alias.name,
      claimId: log.sessionId
    };

    mergedProfile.journal = [summaryEntry].concat(mergedEntries).concat(Array.isArray(mergedProfile.journal) ? mergedProfile.journal : []);
    mergedProfile.meta.xp = (mergedProfile.meta.xp || 0) + validated.xpGain;
    mergedProfile.meta.claimedAliasSessions = claimed.concat([log.sessionId]);
    mergedProfile.meta.sealBroken = !!mergedProfile.meta.sealBroken || !!validated.integrityWarning;

    if (Array.isArray(mergedProfile.achievements)) {
      var claimAchievementId = isRecovery ? 'claimed-recovery' : 'claimed-alias';
      var hasClaimAchievement = mergedProfile.achievements.some(function(achievement) {
        return achievement && achievement.id === claimAchievementId;
      });
      if (!hasClaimAchievement) {
        mergedProfile.achievements.push(isRecovery
          ? { id: 'claimed-recovery', name: 'Recovery Claimed', desc: 'Claimed a recovery session log', icon: '🔑', unlocked: true, unlockedAt: now }
          : { id: 'claimed-alias', name: 'Alias Claimed', desc: 'Claimed an alias session log', icon: '📎', unlocked: true, unlockedAt: now });
      }
    }

    var claimTaskData = isRecovery
      ? { claimId: log.sessionId, count: log.entries.length, aliasId: log.recovery.genesisHash }
      : { claimId: log.sessionId, count: log.entries.length, aliasId: log.alias.id };
    var previousSeal = mergedProfile.sealChain.length ? mergedProfile.sealChain[mergedProfile.sealChain.length - 1].seal : null;
    var claimSealEntry = await FerrosCore.createSealEntry(isRecovery ? 'recovery-claim' : 'alias-claim', claimTaskData, previousSeal);
    mergedProfile.sealChain.push(claimSealEntry);

    var sealAudit = FerrosCore.pushAuditEntry(mergedProfile, 'seal-added', {
      taskId: claimSealEntry.taskId,
      seal: claimSealEntry.seal
    });
    if (!sealAudit.ok) return sealAudit;

    var claimAudit = FerrosCore.pushAuditEntry(mergedProfile, isRecovery ? 'recovery-claimed' : 'alias-claimed', {
      claimId: log.sessionId,
      entryCount: log.entries.length,
      xpGain: validated.xpGain,
      integrityWarning: !!validated.integrityWarning
    });
    if (!claimAudit.ok) return claimAudit;

    var saveResult = { ok: true, profile: mergedProfile, sealChain: mergedProfile.sealChain };
    if (opts.persist) {
      saveResult = FerrosCore.saveProfile(mergedProfile, {
        flags: opts.flags,
        sealChain: mergedProfile.sealChain,
        auditDetail: {
          sealCount: mergedProfile.sealChain.length,
          reason: 'portable-log-claim',
          claimId: log.sessionId
        }
      });
      if (!saveResult.ok) return saveResult;
      mergedProfile = saveResult.profile;
    }

    return {
      ok: true,
      code: null,
      profile: mergedProfile,
      sealChain: saveResult.sealChain || mergedProfile.sealChain,
      sessionId: log.sessionId,
      claimId: log.sessionId,
      logType: log.logType,
      xpGain: validated.xpGain,
      integrityWarning: !!validated.integrityWarning,
      warnings: validated.warnings,
      auditDetail: claimAudit.entry ? claimAudit.entry.detail : null
    };
  };

  // ── TEMPLATE_PROFILES ──────────────────────────────────────────────────────
  // Populated by generate-ferros-core.ps1 from templates.json.
  // Inline fallback preserved for standalone loading.
  FerrosCore.TEMPLATE_PROFILES = [
  {
    "id": "tesla",
    "name": "Nikola Tesla",
    "icon": "⚡",
    "aliasClass": "Engineer",
    "tagline": "Worked obsessively. Slept rarely. Changed everything.",
    "stream": "A",
    "archetype": "deep-work-nocturnal",
    "templateSchedule": {
      "blocks": [
        { "time": "10:00", "label": "Rise & Coffee" },
        { "time": "11:00", "label": "Laboratory Work" },
        { "time": "14:00", "label": "Correspondence" },
        { "time": "18:00", "label": "Dinner (often skipped)" },
        { "time": "20:00", "label": "Deep Experiment Block" },
        { "time": "02:00", "label": "Rest (minimal)" }
      ]
    }
  },
  {
    "id": "kahlo",
    "name": "Frida Kahlo",
    "icon": "🎨",
    "aliasClass": "Artisan",
    "tagline": "Pain became pigment. Every canvas, a self-portrait.",
    "stream": "B",
    "archetype": "pain-driven-creative",
    "templateSchedule": {
      "blocks": [
        { "time": "08:00", "label": "Morning Pain Management" },
        { "time": "10:00", "label": "Studio Setup" },
        { "time": "11:00", "label": "Painting Session" },
        { "time": "15:00", "label": "Rest & Recovery" },
        { "time": "17:00", "label": "Writing & Reflection" },
        { "time": "20:00", "label": "Social / Diego" }
      ]
    }
  },
  {
    "id": "curie",
    "name": "Marie Curie",
    "icon": "☢️",
    "aliasClass": "Scholar",
    "tagline": "Two Nobel Prizes. Zero shortcuts.",
    "stream": "A",
    "archetype": "structured-research",
    "templateSchedule": {
      "blocks": [
        { "time": "06:00", "label": "Rise & Breakfast" },
        { "time": "07:00", "label": "Laboratory" },
        { "time": "12:00", "label": "Lunch & Brief Rest" },
        { "time": "13:00", "label": "Continued Research" },
        { "time": "17:00", "label": "Reading & Papers" },
        { "time": "20:00", "label": "Evening Study" },
        { "time": "22:00", "label": "Sleep" }
      ]
    }
  },
  {
    "id": "aurelius",
    "name": "Marcus Aurelius",
    "icon": "🏛️",
    "aliasClass": "Guardian",
    "tagline": "Rule an empire. Master yourself first.",
    "stream": "A",
    "archetype": "stoic-morning-ruler",
    "templateSchedule": {
      "blocks": [
        { "time": "04:00", "label": "Rise" },
        { "time": "04:30", "label": "Journaling & Meditations" },
        { "time": "06:00", "label": "Physical Training" },
        { "time": "08:00", "label": "Council & Governance" },
        { "time": "12:00", "label": "Lunch & Walk" },
        { "time": "14:00", "label": "Correspondence & Edicts" },
        { "time": "17:00", "label": "Philosophy Study" },
        { "time": "20:00", "label": "Family & Rest" }
      ]
    }
  },
  {
    "id": "fry",
    "name": "Philip J. Fry",
    "icon": "📺",
    "aliasClass": "Guided",
    "tagline": "Slept 1000 years. Still figuring it out.",
    "stream": "B",
    "archetype": "flexible-chaotic",
    "templateSchedule": {
      "blocks": [
        { "time": "11:00", "label": "Wake Up (usually)" },
        { "time": "12:00", "label": "Food (whatever's available)" },
        { "time": "14:00", "label": "Something happens" },
        { "time": "17:00", "label": "Hang out with Bender" },
        { "time": "20:00", "label": "Pizza & TV" },
        { "time": "23:00", "label": "Eventually sleep" }
      ]
    }
  },
  {
    "id": "nightingale",
    "name": "Florence Nightingale",
    "icon": "🕯️",
    "aliasClass": "Healer",
    "tagline": "Data-driven compassion before it had a name.",
    "stream": "C",
    "archetype": "systems-care-rotational",
    "templateSchedule": {
      "blocks": [
        { "time": "05:00", "label": "Rise & Ward Review" },
        { "time": "06:00", "label": "Patient Rounds" },
        { "time": "10:00", "label": "Statistical Analysis" },
        { "time": "13:00", "label": "Lunch & Admin" },
        { "time": "14:00", "label": "Staff Coordination" },
        { "time": "17:00", "label": "Evening Rounds" },
        { "time": "20:00", "label": "Report Writing" },
        { "time": "22:00", "label": "Rest" }
      ]
    }
  },
  {
    "id": "lovelace",
    "name": "Ada Lovelace",
    "icon": "🔢",
    "aliasClass": "Architect",
    "tagline": "Wrote the first algorithm. The machine wasn't built yet.",
    "stream": "A",
    "archetype": "analytical-visionary",
    "templateSchedule": {
      "blocks": [
        { "time": "09:00", "label": "Rise (health permitting)" },
        { "time": "10:00", "label": "Mathematical Correspondence" },
        { "time": "12:00", "label": "Algorithm Development" },
        { "time": "15:00", "label": "Tea & Social" },
        { "time": "16:00", "label": "Writing & Notes" },
        { "time": "19:00", "label": "Evening Calculations" },
        { "time": "22:00", "label": "Rest" }
      ]
    }
  },
  {
    "id": "malone",
    "name": "Sam Malone",
    "icon": "🍺",
    "aliasClass": "Community",
    "tagline": "Everyone knows your name. That's the whole point.",
    "stream": "B",
    "archetype": "service-flexible-shift",
    "templateSchedule": {
      "blocks": [
        { "time": "10:00", "label": "Open Cheers" },
        { "time": "12:00", "label": "Lunch Rush" },
        { "time": "15:00", "label": "Quiet Afternoon Prep" },
        { "time": "17:00", "label": "Happy Hour Begins" },
        { "time": "21:00", "label": "Evening Service" },
        { "time": "00:00", "label": "Close & Clean" },
        { "time": "02:00", "label": "Sleep" }
      ]
    }
  },
  {
    "id": "turing",
    "name": "Alan Turing",
    "icon": "🖥️",
    "aliasClass": "Engineer",
    "tagline": "Broke codes. Built machines. Ran marathons.",
    "stream": "A",
    "archetype": "structured-research",
    "templateSchedule": {
      "blocks": [
        { "time": "08:00", "label": "Morning Run (5–10 miles)" },
        { "time": "09:30", "label": "Codebreaking / Logic Work" },
        { "time": "12:00", "label": "Lunch" },
        { "time": "13:00", "label": "Chess & Puzzles" },
        { "time": "14:00", "label": "Machine Design / Theory" },
        { "time": "18:00", "label": "Lecture / Paper Writing" },
        { "time": "22:00", "label": "Sleep" }
      ]
    }
  },
  {
    "id": "darwin",
    "name": "Charles Darwin",
    "icon": "🦕",
    "aliasClass": "Scholar",
    "tagline": "Observed everything. Concluded slowly. Changed everything.",
    "stream": "C",
    "archetype": "structured-research",
    "templateSchedule": {
      "blocks": [
        { "time": "07:00", "label": "Early Walk" },
        { "time": "08:00", "label": "Breakfast & Post" },
        { "time": "09:30", "label": "Science Work (prime hours)" },
        { "time": "12:00", "label": "Rest & Lunch" },
        { "time": "15:00", "label": "Letters & Reading" },
        { "time": "16:00", "label": "Walk (thinking time)" },
        { "time": "18:00", "label": "Family Time" },
        { "time": "22:00", "label": "Sleep" }
      ]
    }
  },
  {
    "id": "woolf",
    "name": "Virginia Woolf",
    "icon": "📖",
    "aliasClass": "Artisan",
    "tagline": "Three hours of writing per day. The rest was living.",
    "stream": "B",
    "archetype": "pain-driven-creative",
    "templateSchedule": {
      "blocks": [
        { "time": "09:30", "label": "Writing (3 hrs, no interruptions)" },
        { "time": "12:30", "label": "Lunch & Walk" },
        { "time": "14:00", "label": "Rest / Reading" },
        { "time": "16:00", "label": "Letters & Social" },
        { "time": "18:00", "label": "Tea & Conversation" },
        { "time": "22:00", "label": "Sleep" }
      ]
    }
  },
  {
    "id": "jobs",
    "name": "Steve Jobs",
    "icon": "🍏",
    "aliasClass": "Architect",
    "tagline": "Design first. Ship second. Repeat.",
    "stream": "B",
    "archetype": "stoic-morning-ruler",
    "templateSchedule": {
      "blocks": [
        { "time": "06:00", "label": "Wake & Reflect" },
        { "time": "07:00", "label": "Family Breakfast" },
        { "time": "09:00", "label": "Design Review" },
        { "time": "12:00", "label": "Walk Meeting" },
        { "time": "14:00", "label": "Product Decisions" },
        { "time": "17:00", "label": "Email & Calls" },
        { "time": "19:00", "label": "Family Dinner" },
        { "time": "22:00", "label": "Sleep" }
      ]
    }
  }
];

  // ── Expose ─────────────────────────────────────────────────────────────────
  root.FerrosCore = FerrosCore;

})(typeof window !== 'undefined' ? window : this);
