# Read-only advisor model

`spheres-web/ui/advisor-model.js` exports `AdvisorModel` in a browser and through CommonJS in Node. `evaluate(state, production = null, outcomes = null, receipts = null)` returns at most 12 cards. Each card has `id`, `area`, `priority`, `title`, `reason`, `evidence`, `caution`, `action` and `actionLabel`. Priorities sort attention, opportunity, routine; deterministic rule order breaks ties. Evidence contains at most five plain-text strings. The model performs no DOM access, network requests, wall-clock reads, random choices, commands or input mutation.

When `outcomes` is supplied, `evaluate` adds at most one `route-next` card: the first first-hour step that is not achieved and has an obstacle, unless another card already opens the same screen. `recognize(response, receipts)` is a pure function over one `/api/guidance` response (`{state, production, outcomes}`) and the stored save/load receipts. It returns the frozen six-step route and never throws; invalid or mismatched input gives `status: "unknown"`. A receipt kind passed as `null` is unknown, not empty. The route contract is in `campaign-certification/S19/DESIGN.md`.

Cards explain supplied readings and navigate to review screens. They never enact budgets, choose research, construct projects, accept offers or buy equipment. An empty result means there are no supported recommendations in the available reading; it does **not** mean the nation is healthy. The controller should explain unavailable endpoints separately.

## Source contracts

The source of truth is `state_json`, `nation_json`, `research_json`, `production_json`, `construction_suggestions_json` and `manufacturing_summary_json` in `spheres-web/src/main.rs`, `guidance_outcomes::outcomes_json` in `spheres-web/src/guidance_outcomes.rs`, plus the player-filtered `agency::view` and `operations::view` in `spheres-sim/src/`.

- The player must have exactly one matching live nation row. A missing or null treasury is unavailable. Only a finite negative balance supports a treasury review: deficits spend cash down to zero before borrowing, so zero is ordinary play. No invented low-cash threshold or borrowing assumption is used.
- Annual renewal is due when `annual_budget.due === true`, or when `programs.enabled === true && programs.due === true`. `programs.due` is also true while programmes are disabled, so it never counts alone.
- Fiscal recovery distress, `fiscal_recovery.recovery_required === true` with status `adjustment` or `crisis`, gives an attention card that opens Money and recovery (`cash_flow`). The card changes no policy.
- Production advice requires `mode: "province_projects"`, the exact player nation, and `suggestions.as_of_day` matching the Gregorian state date relative to 1990-01-01. The controller must also reject responses from a replaced or changed state object, including same-day changes: dates alone cannot identify a session or intervening order.
- Funding enrollment, a zero daily limit, available cash and explicit queue status determine construction attention. A project ID must be a nonnegative safe integer to route directly; string mine IDs route to Construction. Progress text describes the recorded status and fraction, without promising future progress.
- Only fresh daily-mode construction suggestions are eligible for effects review. Annual renewal, funding issues, queue attention or active zero-capacity assignments suppress expansion recommendations. Suggestions need exact IDs, finite cost, positive integer minimum days, evidence and an owned province in the served production view. Ordinary projects must also match the catalog and province start permissions. Explicit false or malformed eligibility flags are rejected.
- The current endpoint filters every suggestion through a fresh authoritative `preview.can_start`, but does not include an eligibility flag on each serialized suggestion. `starter_industry` has no catalog row: its exact-size server preview, matching province and bounded integer `capacity_micros` (1–1,000,000) are the relevant admission contract. Standard-size permission cannot establish whether a smaller workshop is allowed. The controller must open a fresh effects quote; this is never automatic construction.
- The optional upstream `industry_rebuild.enabled === true` contract is feature-detected. A queued project's finite positive `allocation.requested` with `allocation.assigned === 0` warrants an allocation review even when ample funding exists. No assignment is changed. The new sized `industry_preset` is not automatically treated as a legacy project.
- Government warnings require an explicitly open route with served `armed` or `half_armed` status. Aggregate half-armed flags and disabled calibration routes cannot create alarms. The model calculates no stability or takeover threshold.
- Research is player-owned through its human-readable `nation` field. A current-year option must appear in a domain's options and satisfy `year <= state.year`; future options are not called currently available. Paid calendar waits, zero-effort stalls and monthly acquisition limits are separate cards. No client-side completion outcome is predicted.
- A conflict must explicitly contain the player's nation in `wars[].posture[].id`. Shortfall advice also requires an enabled player operation, a matching live conflict, and valid requested/deployed amounts. Disjoint enemy wars and orphan/foreign deployment rows are ignored. Manufacturing attention is a served line summary, not a newly computed equipment shortage.
- Incoming diplomacy is `agency.offers`, whose native view filters recipients. Offer identity, live foreign sender and consistent nonexpired calendar/deadline fields are required. `stratagems.offers` and domination offers are action menus and cannot become incoming requests. Resource offers similarly require a valid foreign sender, relative deadline and political cost.

## Navigation contract

| Action | Payload and destination |
| --- | --- |
| `home` | Centre the map on the player's nation |
| `budget` | National budget review |
| `cash_flow` | Money and recovery through `openCashFlow()` |
| `construction` | Current construction/funding view |
| `project` | `id`: safe integer project identifier |
| `suggestion` | `project_kind`, `district`, `capacity_micros`: exact integer or null; opens a fresh effects quote |
| `industry` | Industry operating view; with string `district` and `project_kind`, that site (`openIndustry({district, kind})`). The facility kind travels as `project_kind` because `kind` is the route name |
| `government` | Government conditions |
| `research` | Optional `domain` from the served research row |
| `equipment` | Equipment/manufacturing review; an optional `tab` opens that Equipment tab only when it is in `EQUIPMENT_TABS` |
| `air` | Equipment Air command tab; `page: "bases"` then selects the Bases page, where airbase work is reviewed |
| `companies` | Equipment Companies & Procurement tab |
| `world` | Player conflicts and operations |
| `decisions` | Incoming diplomacy inbox through `openAgency()` |
| `resources` | Resource offer review |
| `campaign` | Campaigns: named saves, loads and Continue |

The renderer owns text escaping, the router owns its action whitelist and the controller owns stale-response protection. Unknown actions must never be interpreted as command names.

## Validation

Run `node --test tools/ui/check_advisor_model.cjs tools/ui/check_guidance_outcomes.cjs`; the second file checks `recognize` against native field shapes. Fixtures exercise missing/malformed values, construction eligibility and exact sizes, stale dates, calendar versus funding research waits, disabled government warnings, foreign wars, incoming-offer identity/deadlines, immutable deterministic results and browser UMD loading without browser globals. A real archived `/api/state` snapshot is additionally checked when `../leadership-2035-evidence/source-state-before.json` exists; portable fixtures remain mandatory everywhere.
