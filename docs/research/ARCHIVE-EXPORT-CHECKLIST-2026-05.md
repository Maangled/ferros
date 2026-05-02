# Archive Export Checklist — 2026-05

**Archive root:** `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation`  
**Primary manifest:** `docs/research/ARCHIVE-MANIFEST-2026-05-02.md`

Use this checklist before exporting the archive to external or long-term storage.

---

## 1) Preflight

- [ ] Confirm archive root exists and expected folders are present.
- [ ] Confirm active project roots remain in place (`/home/homelab001/apps/ferros`, `/home/homelab001/apps/home-assistant`).
- [ ] Confirm no additional folders were moved into archive after manifest generation.
- [ ] Confirm local disk space is sufficient for checksum and compressed export artifacts.

Suggested commands:

```bash
ls -la /home/homelab001/apps
ls -la /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation
df -h
```

---

## 2) Repository Integrity

- [ ] For each archived git repo, capture `git status --short` and `git log --oneline -n 5`.
- [ ] Confirm pushed checkpoint commits are present on `origin/main`.
- [ ] Record any intentionally retained dirty state.

Known retained dirty state at archive time:

- `workspace-old/workspace-old`: untracked `wonderland_db_backup_1`

Suggested commands:

```bash
for r in \
  /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/botgen-rust \
  /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/palworld-server/palworld-server \
  /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/workpace-rust \
  /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/workspace-old/workspace-old \
  /home/homelab001/apps/ferros
do
  echo "repo:$r"
  git -C "$r" status --short
  git -C "$r" log --oneline -n 5
done
```

---

## 3) Runtime and Autostart Safety

- [ ] Verify archived projects have no running containers.
- [ ] Verify archived project compose files use `restart: "no"` where applicable.
- [ ] Verify Home Assistant remains intentionally live and unarchived.
- [ ] Verify no matching systemd service units for archived projects.

Suggested commands:

```bash
docker ps --format '{{.Names}}\t{{.Status}}\t{{.Image}}'
docker inspect $(docker ps -aq) --format '{{.Name}}\t{{.HostConfig.RestartPolicy.Name}}\t{{.State.Status}}\t{{.Config.Image}}' | sort
systemctl list-unit-files --type=service --no-pager | grep -Ei 'botgen|workpace|workspace|sheetgen|tunes|palworld' || true
```

---

## 4) Secret and Privacy Review

- [ ] Inventory `.env` and secret-like files in archive.
- [ ] Decide export mode:
  - private encrypted export (full archive), or
  - sanitized public export (remove/redact secrets).
- [ ] If public export, remove `.env` files and credentials before packaging.
- [ ] Record redaction decisions in an addendum note.

Suggested command:

```bash
find /home/homelab001/apps/_archive/2026-05-02-ferros-consolidation -maxdepth 5 -type f \( -name '.env' -o -name '*.env' -o -name '.env.*' \)
```

---

## 5) Package and Checksum

- [ ] Create archive package (tar.zst or tar.gz).
- [ ] Generate SHA-256 checksum.
- [ ] Verify checksum immediately after generation.
- [ ] Store checksum file next to package and in a second location.

Example:

```bash
cd /home/homelab001/apps/_archive
tar -I 'zstd -19' -cf 2026-05-02-ferros-consolidation.tar.zst 2026-05-02-ferros-consolidation
sha256sum 2026-05-02-ferros-consolidation.tar.zst > 2026-05-02-ferros-consolidation.tar.zst.sha256
sha256sum -c 2026-05-02-ferros-consolidation.tar.zst.sha256
```

---

## 6) Export Destinations

- [ ] Copy package to primary backup destination.
- [ ] Copy package to secondary backup destination.
- [ ] Validate checksum at each destination.
- [ ] Record destination URIs/paths and verification timestamp.

---

## 7) Documentation Closeout

- [ ] Update `docs/research/ARCHIVE-MANIFEST-2026-05-02.md` if export altered archive contents.
- [ ] Add an export completion note (date, operator, package name, checksum, destinations).
- [ ] Link completion note from `docs/research/S6-legacy-archive-sweep-2026-05.md` if material changes occurred.

---

## 8) Gate to Delete Originals (Optional, Later)

Only consider deleting local archived folders after all are true:

- [ ] Two verified backup copies exist.
- [ ] Checksums verified at both destinations.
- [ ] Secret handling decision recorded.
- [ ] Owner explicitly approves deletion.
