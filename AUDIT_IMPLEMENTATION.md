# September system review implementation

User authorization: implement all recommendations in the SPHERES system review. Baseline: `4b3116827c0164b62e6b5247fd690bf851ef979d` (includes the economic competition and clock changes made after the reviewed `ca767fe`). Development branch: `codex/audit-improvements`. Existing campaigns must remain readable; simulation migrations are explicit and deterministic.

| Workstream | Required result | Verification / owner |
|---|---|---|
| War integrity | Zero deployed force causes zero combat loss; nationally conserved fronts; control-aware industry and freight | Military agent, targeted regression tests |
| Military gameplay | Explicit front allocations, reserves, useful equipment roles and inventory losses; alternate freight routes | Military agent; adapter and UI integration |
| Player agency | Persistent diplomacy inbox, refusal/expiry/policy, sanctions persist, bank/peg controls | Diplomacy agent, save/reload and command tests |
| Campaign purpose | Optional prosperity/science/stability/supply/diplomacy aims, domination, continue sandbox | Diplomacy agent, observable model-backed objectives |
| Save reliability | Atomic save + backup, named slots, rotating autosaves, world/history/log envelope and legacy migration | Storage agent, crash/round-trip tests |
| Transport/history | Exactly-once mutations, actionable errors, durable history, lazy selected series | Storage agent, request and history tests |
| Budget honesty | First-budget interest and ordered policy preview from the simulation, explicit current/proposed basis | Root, regression against actual command dispatch |
| First playable chain | Advisor steps through funding → project → inputs → completion; explain blockers and causal outcomes | Root, browser smoke and fixture tests |
| Navigation | Home-country camera, shared label priorities, keyboard nation/province search, readable research list | Root, geometry/fixture and browser checks |
| Economic model | Funded science, staged AI, meaningful spheres, small-country production, sourced sector distinctions | Upstream features retained; fill remaining source/model gaps |
| Balance | Daily multi-decade scenarios, uncertainty/variance, honest model limits, resolve known calibration failures | Root/integration; do not weaken calibration tests |
| Performance | Year 0/10/30 turn/read-model/payload measurements, indexed history, smaller embedded image variants | Root + storage agent |
| Maintainability | Separate reusable UI/server modules, routine CI plus real browser smoke and scheduled long runs | Root + storage agent |
| Distribution | Version/build identity, save location, Windows ready-to-run package, current README and decision log | Root, fresh executable smoke |
| Human validation | Runnable playtest protocol and feedback capture | Implement tools; actual outside player results require future participants |

This table is the implementation checklist, not a claim that unfinished rows are complete. Final validation and any limits are recorded in the implementation report.
