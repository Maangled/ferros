#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

const repoRoot = path.resolve(__dirname, '..');
const agentsRoot = path.join(repoRoot, 'agents');
const sourceRoot = path.join(agentsRoot, 'source');
const manifestPath = path.join(agentsRoot, 'manifest.json');
const mirrorRoot = path.join(repoRoot, '.github', 'agents');

const SOURCE_LAYOUT = Object.freeze({
  'ferros-agent.agent.md': 'ferros/ferros-agent.agent.md',
  'ferros-agent-architect.agent.md': 'agent_architect/ferros-agent-architect.agent.md',
  'ferros-prompt-architect.agent.md': 'ferros/architects/ferros-prompt-architect.agent.md',
  'ferros-orchestration-architect.agent.md': 'ferros/architects/ferros-orchestration-architect.agent.md',
  'ferros-documentation-architect.agent.md': 'ferros/architects/ferros-documentation-architect.agent.md',
  'ferros-backup-officer.agent.md': 'ferros/officers/ferros-backup-officer.agent.md',
  'ferros-audit-recovery-officer.agent.md': 'ferros/officers/ferros-audit-recovery-officer.agent.md',
  'ferros-software-architect.agent.md': 'coding/ferros-software-architect.agent.md',
  'ferros-coding-agent-architect.agent.md': 'coding/architects/ferros-coding-agent-architect.agent.md',
  'ferros-core-lane-architect.agent.md': 'coding/architects/ferros-core-lane-architect.agent.md',
  'ferros-subcore-lane-architect.agent.md': 'coding/architects/ferros-subcore-lane-architect.agent.md',
  'ferros-core.agent.md': 'coding/execution/ferros-core.agent.md',
  'ferros-subcore.agent.md': 'coding/execution/ferros-subcore.agent.md',
  'ferros-coding-continuity.agent.md': 'coding/incubation/ferros-coding-continuity.agent.md',
  'ferros-coding-packet-validator.agent.md': 'coding/incubation/ferros-coding-packet-validator.agent.md',
  'ferros-coding-malformed-response.agent.md': 'coding/incubation/ferros-coding-malformed-response.agent.md',
  'ferros-business-agent.agent.md': 'business/ferros-business-agent.agent.md',
  'ferros-business-agent-architect.agent.md': 'business/architects/ferros-business-agent-architect.agent.md',
});

const RETIRED_AGENT_FILES = new Set([
  'ferros-coding-agent.agent.md',
]);

function ensureDirectory(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

function readFileIfExists(filePath) {
  return fs.existsSync(filePath) ? fs.readFileSync(filePath, 'utf8') : null;
}

function writeFileIfChanged(filePath, content) {
  const current = readFileIfExists(filePath);
  if (current === content) {
    return false;
  }

  ensureDirectory(path.dirname(filePath));
  fs.writeFileSync(filePath, content, 'utf8');
  return true;
}

function listMirrorAgentFiles() {
  if (!fs.existsSync(mirrorRoot)) {
    return [];
  }

  return fs.readdirSync(mirrorRoot, { withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.agent.md'))
    .map((entry) => entry.name)
    .sort();
}

function listSourceAgentFiles(dirPath = sourceRoot) {
  if (!fs.existsSync(dirPath)) {
    return [];
  }

  return fs.readdirSync(dirPath, { withFileTypes: true }).flatMap((entry) => {
    const fullPath = path.join(dirPath, entry.name);
    if (entry.isDirectory()) {
      return listSourceAgentFiles(fullPath);
    }

    return entry.name.endsWith('.agent.md') ? [fullPath] : [];
  });
}

function parseFrontmatter(content) {
  const lines = content.split(/\r?\n/);
  if (lines[0] !== '---') {
    return {};
  }

  const fields = {};
  let currentKey = null;
  let listValues = [];

  const flushList = () => {
    if (!currentKey) {
      return;
    }
    fields[currentKey] = listValues.join('\n');
    currentKey = null;
    listValues = [];
  };

  for (let index = 1; index < lines.length; index += 1) {
    const line = lines[index];
    if (line.trim() === '---') {
      flushList();
      break;
    }

    const trimmed = line.trimEnd();
    const keyValueMatch = trimmed.match(/^([A-Za-z0-9_-]+):(.*)$/);
    if (keyValueMatch) {
      flushList();
      const [, rawKey, rawValue] = keyValueMatch;
      const value = rawValue.trim();
      if (value.length === 0) {
        currentKey = rawKey.trim();
      } else {
        fields[rawKey.trim()] = value;
      }
      continue;
    }

    if (currentKey && trimmed.trimStart().startsWith('-')) {
      listValues.push(trimmed.trim().replace(/^-\s*/, ''));
    }
  }

  return fields;
}

function parseFrontmatterList(value) {
  if (!value) {
    return [];
  }

  const trimmed = String(value).trim();
  if (!trimmed) {
    return [];
  }

  if (trimmed.startsWith('[') && trimmed.endsWith(']')) {
    return trimmed.slice(1, -1)
      .split(',')
      .map((item) => item.trim().replace(/^['"]|['"]$/g, ''))
      .filter(Boolean);
  }

  return trimmed
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean);
}

function normalizeBoolean(value) {
  return String(value).trim().toLowerCase() === 'true';
}

function familyForDisplayName(displayName) {
  const value = displayName.toLowerCase();
  if (value.includes('business') || value.includes('operations') || value.includes('sales') || value.includes('finance') || value.includes('hr')) {
    return 'business';
  }
  if (value.includes('software architect') || value.includes('core') || value.includes('subcore') || value.includes('coding')) {
    return 'software';
  }
  if (value.includes('architect')) {
    return 'architect';
  }
  if (value.includes('ferros') || value.includes('officer')) {
    return 'administration';
  }
  return 'service';
}

function roleForPath(relativeSourcePath, displayName) {
  const normalizedPath = relativeSourcePath.replace(/\\/g, '/');
  const value = displayName.toLowerCase();
  if (normalizedPath.includes('/architects/') || value.includes('architect')) {
    return 'architect';
  }
  if (normalizedPath.includes('/officers/') || value.includes('officer')) {
    return 'officer';
  }
  if (normalizedPath.includes('/execution/')) {
    return 'execution';
  }
  if (normalizedPath.includes('/incubation/')) {
    return 'incubation';
  }
  return 'agent';
}

function laneForPath(relativeSourcePath, role) {
  const parts = relativeSourcePath.replace(/\\/g, '/').split('/');
  if (parts.length >= 3) {
    const segment = parts[1];
    if (segment === 'architects') {
      return 'architect';
    }
    if (segment === 'officers') {
      return 'officer';
    }
    if (segment === 'execution') {
      return 'execution';
    }
    if (segment === 'incubation') {
      return 'incubation';
    }
  }
  return role;
}

function sourceRelativePathForMirror(filename) {
  if (SOURCE_LAYOUT[filename]) {
    return SOURCE_LAYOUT[filename];
  }

  if (filename.includes('business')) {
    return path.posix.join('business', filename);
  }
  if (filename.includes('coding') || filename.includes('software') || filename.includes('core') || filename.includes('subcore')) {
    return path.posix.join('coding', filename);
  }
  return path.posix.join('ferros', filename);
}

function pruneStaleMirrors(entries) {
  if (!fs.existsSync(mirrorRoot)) {
    return [];
  }

  const keep = new Set(entries.map((entry) => path.basename(entry.mirror_path)));
  const removed = [];

  for (const dirEntry of fs.readdirSync(mirrorRoot, { withFileTypes: true })) {
    if (!dirEntry.isFile() || !dirEntry.name.endsWith('.agent.md')) {
      continue;
    }
    if (keep.has(dirEntry.name)) {
      continue;
    }

    fs.unlinkSync(path.join(mirrorRoot, dirEntry.name));
    removed.push(path.posix.join('.github/agents', dirEntry.name));
  }

  return removed;
}

function importMirrorIntoSource() {
  const imported = [];
  for (const filename of listMirrorAgentFiles()) {
    if (RETIRED_AGENT_FILES.has(filename)) {
      continue;
    }

    const sourceRelativePath = sourceRelativePathForMirror(filename);
    const sourcePath = path.join(sourceRoot, sourceRelativePath);
    if (fs.existsSync(sourcePath)) {
      continue;
    }

    const mirrorPath = path.join(mirrorRoot, filename);
    const content = readFileIfExists(mirrorPath);
    if (!content) {
      continue;
    }

    writeFileIfChanged(sourcePath, content);
    imported.push(sourceRelativePath.replace(/\\/g, '/'));
  }
  return imported;
}

function buildManifestEntries() {
  const entries = [];
  for (const sourceFile of listSourceAgentFiles()) {
    const filename = path.basename(sourceFile);
    if (RETIRED_AGENT_FILES.has(filename)) {
      continue;
    }

    const content = readFileIfExists(sourceFile);
    if (!content) {
      continue;
    }

    const frontmatter = parseFrontmatter(content);
    const relativeSourcePath = path.relative(repoRoot, sourceFile).replace(/\\/g, '/');
    const mirrorPath = `.github/agents/${filename}`;
    const displayName = frontmatter.name || filename.replace(/\.agent\.md$/, '');
    const role = roleForPath(relativeSourcePath, displayName);
    const lane = laneForPath(relativeSourcePath, role);

    entries.push({
      id: filename.replace(/\.agent\.md$/, ''),
      display_name: displayName,
      description: frontmatter.description || 'No description published.',
      family: familyForDisplayName(displayName),
      role,
      lane,
      source_path: relativeSourcePath,
      mirror_path: mirrorPath,
      user_invocable: normalizeBoolean(frontmatter['user-invocable']),
      tools: parseFrontmatterList(frontmatter.tools),
      child_agents: parseFrontmatterList(frontmatter.agents),
    });
  }

  return entries.sort((left, right) => left.display_name.localeCompare(right.display_name));
}

function writeManifest(entries) {
  const manifest = { entries };
  const content = `${JSON.stringify(manifest, null, 2)}\n`;
  return writeFileIfChanged(manifestPath, content);
}

function mirrorSource(entries) {
  const mirrored = [];
  for (const entry of entries) {
    const sourcePath = path.join(repoRoot, entry.source_path);
    const mirrorPath = path.join(repoRoot, entry.mirror_path);
    const content = readFileIfExists(sourcePath);
    if (!content) {
      throw new Error(`Missing source file for mirror entry: ${entry.source_path}`);
    }

    if (writeFileIfChanged(mirrorPath, content)) {
      mirrored.push(entry.mirror_path);
    }
  }
  return mirrored;
}

function main() {
  ensureDirectory(sourceRoot);

  const imported = importMirrorIntoSource();
  const entries = buildManifestEntries();
  if (entries.length === 0) {
    throw new Error('No source agent definitions were found under agents/source');
  }

  const manifestChanged = writeManifest(entries);
  const mirrored = mirrorSource(entries);
  const pruned = pruneStaleMirrors(entries);

  const summary = {
    imported: imported.length,
    manifestChanged,
    mirrored: mirrored.length,
    pruned: pruned.length,
    entries: entries.length,
  };

  console.log('[agent-source-sync] summary');
  console.log(JSON.stringify(summary, null, 2));
  if (imported.length > 0) {
    console.log('[agent-source-sync] imported source files:');
    imported.forEach((entry) => console.log(`- ${entry}`));
  }
  if (mirrored.length > 0) {
    console.log('[agent-source-sync] mirrored files:');
    mirrored.forEach((entry) => console.log(`- ${entry}`));
  }
}

main();