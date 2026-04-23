# FERROS Glossary

Living vocabulary for repo docs. Prefer these terms over ad-hoc synonyms, and update this file when stream docs start using a term in a narrower or different way.

---

| Term | Meaning |
|------|---------|
| Agent | A FERROS unit of work exposed through S3 with a manifest, lifecycle, and declared capability needs. |
| Agent Center | The S3 control surface for registering, inspecting, authorizing, and running agents. |
| Capability | A named permission FERROS checks at runtime before allowing an action. |
| CapabilityGrant | The S2 authorization artifact that binds a capability to a profile and is evaluated by policy code. |
| Consent / deny-by-default | FERROS rule that privileged actions are denied unless an explicit grant allows them. |
| Gate | A project checkpoint such as G1, G2, G3, or G4 that unlocks later implementation claims. |
| Hub | The S7 hardware-facing smart-home surface built on top of the FERROS runtime. |
| Profile | The FERROS identity record and related contracts owned by S2. |
| ProfileId | The stable identifier that ties grants, manifests, and policy decisions back to a profile. |
| Runtime | The S4 execution layer that hosts agents, routes messages, and enforces policy decisions. |
| Stream | A delivery lane with its own README, BACKLOG, PROGRESS, and CONTRACTS files. |
| Surface | A user-facing interaction context, especially in S5 shell planning and the HTML prototype set in `docs/`. |
| Truth-sync | Updating docs and status surfaces to match what exists in the repo today rather than what is only planned. |