#!/usr/bin/env node
/**
 * ferros-inline v1.0.0
 *
 * Reads FERROS asset HTML files, replaces @inline marker content with
 * fresh copies from canonical source modules, writes to dist/.
 *
 * Dev files have compressed inlined content (no module headers, no internal
 * blank lines). The build output includes FULL module content (with headers).
 * Blank lines between modules act as block separators.
 *
 * Zero npm dependencies. Node.js >= 14.
 *
 * Usage:
 *   node docs/assets/_build/inline.js              Build all → dist/
 *   node docs/assets/_build/inline.js --check       Validate only (no writes)
 *   node docs/assets/_build/inline.js --verbose      Per-marker detail
 *   node docs/assets/_build/inline.js <file>         Build single asset
 *
 * See: ADR-008 (docs/adr/ADR-008-modular-rendering-system.md)
 */
'use strict';

var fs   = require('fs');
var path = require('path');

var ASSETS_DIR = path.resolve(__dirname, '..');
var DIST_DIR   = path.join(ASSETS_DIR, 'dist');

/* ──────────────────────────────────────────────
   Marker detection
   ────────────────────────────────────────────── */
var MARKER_RE = /^\s*(?:\/\*\s*@inline\s+(\S+)\s*\*\/|<!--\s*@inline\s+(\S+)\s*-->)\s*$/;

function parseMarker(line) {
  var m = line.match(MARKER_RE);
  return m ? (m[1] || m[2]) : null;
}

function isBlank(line) {
  return /^\s*$/.test(line);
}

/* ──────────────────────────────────────────────
   Module loader (cached)
   ────────────────────────────────────────────── */
var moduleCache = {};

function loadModule(relPath) {
  if (moduleCache[relPath]) return moduleCache[relPath];

  var fullPath = path.join(ASSETS_DIR, relPath);
  if (!fs.existsSync(fullPath)) {
    throw new Error('Module not found: ' + relPath + ' → ' + fullPath);
  }

  var raw = fs.readFileSync(fullPath, 'utf8');
  var ver = (raw.match(/v(\d+\.\d+\.\d+)/) || [null, '?'])[1];

  /* Strip single trailing newline so split doesn't create an empty last element */
  var content = raw.replace(/\n$/, '');

  moduleCache[relPath] = { content: content, version: ver, path: relPath };
  return moduleCache[relPath];
}

/* ──────────────────────────────────────────────
   Asset discovery
   ────────────────────────────────────────────── */
function findAssets(singleFile) {
  if (singleFile) {
    var abs = path.resolve(singleFile);
    if (!fs.existsSync(abs)) throw new Error('File not found: ' + abs);
    return [abs];
  }

  var files = [];
  (function walk(dir) {
    fs.readdirSync(dir, { withFileTypes: true }).forEach(function(entry) {
      if (entry.name.startsWith('_') || entry.name === 'dist') return;
      var full = path.join(dir, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.endsWith('.html')) files.push(full);
    });
  })(ASSETS_DIR);

  return files.sort();
}

/* ──────────────────────────────────────────────
   Process one asset file

   Algorithm:
   1. Walk lines sequentially
   2. When an @inline marker is found:
      a. Keep the marker line in output
      b. Skip forward through non-blank lines (old compressed content)
      c. Insert fresh FULL module content (with headers)
      d. The blank separator line remains in the original array
         and will be copied on the next loop iteration
   3. Non-marker lines are copied verbatim (asset-specific code)
   ────────────────────────────────────────────── */
function processFile(filePath) {
  var lines   = fs.readFileSync(filePath, 'utf8').split('\n');
  var out     = [];
  var modules = [];
  var i       = 0;

  while (i < lines.length) {
    var modPath = parseMarker(lines[i]);

    if (!modPath) {
      out.push(lines[i]);
      i++;
      continue;
    }

    /* ── @inline marker found ── */
    var mod = loadModule(modPath);
    modules.push({ module: modPath, version: mod.version, line: i + 1 });

    /* Keep the marker */
    out.push(lines[i]);
    i++;

    /* Skip old inlined content (continuous non-blank lines) */
    while (i < lines.length && !isBlank(lines[i])) {
      i++;
    }

    /* Insert fresh full module content */
    mod.content.split('\n').forEach(function(l) {
      out.push(l);
    });
  }

  return { output: out.join('\n'), modules: modules };
}

/* ──────────────────────────────────────────────
   CLI
   ────────────────────────────────────────────── */
function main() {
  var args    = process.argv.slice(2);
  var check   = args.indexOf('--check')   !== -1;
  var verbose = args.indexOf('--verbose')  !== -1;

  /* Positional arg = single file */
  var singleFile = null;
  args.forEach(function(a) {
    if (!a.startsWith('--') && fs.existsSync(a)) singleFile = a;
  });

  var assets   = findAssets(singleFile);
  var totalMod = 0;
  var errors   = 0;

  console.log('ferros-inline v1.0.0  (%s)', check ? 'check' : 'build');
  console.log('Source : %s', ASSETS_DIR);
  console.log('Output : %s\n', check ? '(none)' : DIST_DIR);

  if (!check) {
    fs.mkdirSync(DIST_DIR, { recursive: true });
  }

  assets.forEach(function(fp) {
    var rel = path.relative(ASSETS_DIR, fp);
    try {
      var result = processFile(fp);
      totalMod += result.modules.length;

      var tag = result.modules.length > 0
        ? result.modules.length + ' modules'
        : 'no @inline markers';

      console.log('  %s  %s  (%s)', result.modules.length > 0 ? 'OK' : '--', rel, tag);

      if (verbose) {
        result.modules.forEach(function(m) {
          var pad = m.module + Array(Math.max(1, 30 - m.module.length)).join(' ');
          console.log('       @inline %s v%s  line %d', pad, m.version, m.line);
        });
      }

      if (!check) {
        var distPath = path.join(DIST_DIR, rel);
        fs.mkdirSync(path.dirname(distPath), { recursive: true });
        fs.writeFileSync(distPath, result.output);
      }
    } catch (err) {
      console.error('  ERR  %s  %s', rel, err.message);
      errors++;
    }
  });

  console.log('\n%d asset(s), %d module block(s), %d error(s)',
    assets.length, totalMod, errors);

  if (errors > 0) {
    console.log('FAILED');
    process.exit(1);
  }
  console.log('Done.');
}

main();
