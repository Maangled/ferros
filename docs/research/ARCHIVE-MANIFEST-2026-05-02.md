# Archive Manifest — 2026-05-02

**Archive root:** `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation`  
**Prepared from:** `/home/homelab001/apps`  
**Prepared by:** GitHub Copilot agent pass  
**Purpose:** final inventory and operational state at archive time

---

## Archive Decisions

| Folder | Decision | Operational state at archive time | Restart policy status | Archive location |
|--------|----------|-----------------------------------|-----------------------|------------------|
| `botgen-rust` | Archived after checkpoint commit | No running botgen service; one exited test DB container remained (`botgen-test-postgres`) | Local compose changed to `restart: "no"` | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/botgen-rust` |
| `workpace-rust` | Archived | No running workpace container detected | DB service compose changed to `restart: "no"` | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/workpace-rust` |
| `sheetgen-rust` | Archived | No running sheetgen container detected | No active runtime observed at archive moment | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/sheetgen-rust` |
| `workspace-old` | Archived | No running workspace-old container detected | Compose services changed to `restart: "no"` | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/workspace-old` |
| `tunes-bot-js` | Archived (reference-only harvest) | No running tunes container detected | Compose service changed to `restart: "no"` | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/tunes-bot-js` |
| `palworld-server` | Archived (out-of-scope project) | No running palworld container detected | Compose already changed to `restart: "no"` | `/home/homelab001/apps/_archive/2026-05-02-ferros-consolidation/palworld-server` |
| `home-browser` | Removed earlier (empty directory) | N/A | N/A | Removed from apps root prior to manifest |
| `home-assistant` | **Retained in place** | Active live stack still running | `unless-stopped` (intentionally unchanged) | `/home/homelab001/apps/home-assistant` |
| `ferros` | **Retained in place** | Active working repo | N/A | `/home/homelab001/apps/ferros` |

---

## Runtime Snapshot

At manifest capture time, running or restarting containers were Home Assistant stack components:

- `homeassistant`
- `mosquitto`
- `localai`
- `fasterwhisper`
- `openwakeword`
- `floorplan-api` (restarting)
- `pipertts` (restarting)
- `llama` (restarting)

No legacy project container from archived folders was running at capture time.

---

## Systemd Snapshot

No matching systemd unit files were found for archived project names (`botgen`, `workpace`,
`workspace`, `sheetgen`, `tunes`, `palworld`, `ferros`, `homeassistant`) at capture time.

---

## Secret-Sensitive Files in Archive

Archive path contains environment files and should be treated as private/internal material.
Observed examples include:

- `.../tunes-bot-js/.env`
- `.../sheetgen-rust/.env`
- `.../botgen-rust/.env`
- `.../workspace-old/workspace-old/.env`

Do not publish this archive to public storage without secret review and redaction.

---

## Related Records

- `docs/research/S6-legacy-archive-sweep-2026-05.md`
- `docs/adr/ADR-027-service-parity-broker-and-assurance-tiers.md`
- `docs/adr/ADR-023-onramp-policy.md`
- `docs/adr/_RESEARCH-NOTES/RN-2026-05-voting-decision-models.md`