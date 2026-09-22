# S19 design — outcome-aware first-hour guidance

Packet: CLAUDE-S19-01 (Claude). Branch `claude/s19-guidance-01`, base `939e8f9`.
Status: design for implementation; not qualification evidence.

## Principle

Guidance keeps three things apart and never lets one stand in for another:

1. **Reading progress:** which lessons the player read or skipped. This is a browser
   preference shared across campaigns (`spheres.guidance.tutorial.v1`, unchanged).
2. **Campaign results:** what this campaign has actually done, taken from dated native
   records in one identity-checked `/api/guidance` reading. They are recomputed on every
   reading and never stored as truth.
3. **Advice:** the current obstacle for the next unmet step. It explains why that step
   matters and opens an existing review screen. It sends no command and changes no money.

Clicking a lesson, opening a screen, a `/api/command` response, a toast, a preview or
quote, or a stale response can never complete a result.

## The first-hour route (six steps)

Each step has milestones. A milestone is `done` only when a dated native record for the
player's nation exists in the current reading with `day <= as_of_day`. Receipts written by
the daily tick carry `as_of_day - 1` after an advance (settlement runs before the date
moves). Checks are therefore `receipt_day <= as_of_day`; they never use `== as_of_day`.

| Step id | Title | Goal milestone (achieved) | Other milestones | Lesson |
|---|---|---|---|---|
| `finances` | Enact a yearly budget | Money-journal decision of kind `program_budget`, `annual_budget` or `construction_budget` whose day falls in the current calendar year | — | `budget-treasury` |
| `construction` | Fund a useful construction project | Paid work (a player project with `last_spent_bn > 0` and `last_day <= as_of_day`, or a player mine with `spent_bn > 0` and a valid `last_day`, shown undated), or a dated completion headline for the player, including a mine opening | producing (dated output at the exact district and kind of a dated player completion) | `construction-effects`, `industry-operations` |
| `research_design` | Research toward a design | A saved draft the designer's native check accepts (`valid: true`, `updated_day`), a design revision (`created_day`/`certified_day`), development paid (a dated daily payment, or the project's `spent_bn` total, dated by `completed_day` when present), or component research completed (`learned > 0`, dated by `last_completed_day` when present) | component research active (progress/cost) | `research-components` |
| `procurement` | Buy equipment from a company | A delivery or import with `delivered_day <= as_of_day` | manufacturer established; certified product; purchase placed (`purchased_day`); paid (`settled_day`) | `equipment-procurement` |
| `air_force` | Prepare an air force | A completed airbase improvement sponsored by the player (`completed_day`), or a squadron with assigned aircraft and no blocker | airbase project funded (`paid_bn > 0`, `last_paid_day`); squadron formed; mission flown (dated report) | `air-force` |
| `save_resume` | Save and resume | This session began from a native load of a saved campaign | saved: native `/api/save` receipt | `save-review` |

Why these sources:
- A successful `construction_budget` order dispatches `SetAnnualBudget{fiscal_year: w.year}`
  (`programs::set_construction_budget`), so it seats the current year's plan. Its journal
  row is written only on success. Last year's row does not count.
- `equipment::start_research` clears `last_research_completed_day`, but the `learned` set
  only grows. `learned > 0` therefore keeps a finished result after the next research
  starts. A `learned` value that is not a count reads `unknown`.
- The equipment tick sets `last_spent_bn = 0` on every development project each day, so
  the lasting `spent_bn` total stands in when the daily receipt is zero.
- An unmapped completion headline has `district`/`kind` null. It still counts as a
  completion, but it never credits output at a site.
- A draft with `valid: false` is not a saved design; the obstacle becomes "Make a saved
  design valid". A `valid` value that is not a boolean reads `unknown`.
- A paid lot with `status: "blocked"` gets a "Delivery blocked" obstacle, never an arrival
  promise. Only `in_transit` lots name a due date, and only when it is not already past.

Step status is one of:
- `achieved`: the goal milestone is done.
- `progress`: an earlier milestone is done.
- `not_yet`: the data is present and nothing is done.
- `unknown`: data is missing, disabled, malformed or has a mismatched identity.

A missing field is never read as zero or as done.

Reachability, stated honestly in the UI and evidence:
- A fresh campaign can reach `finances`, `construction` (paid progress), `research_design`
  (saved design or research), an `air_force` airbase foundation (12 funded days) and
  `save_resume` in the first hour.
- `procurement` needs a certified product. That takes at least 180 days of development
  for ground designs and 240 for air, or foreign stock. It will usually read `not_yet`
  with the obstacle named.
- Squadron readiness and mission results need delivered aircraft and an eligible conflict.

## Native read model (read-only)

`GET /api/guidance` gains one key, `outcomes`, which is `null` when no nation is chosen.
It is built by `guidance_outcomes::outcomes_json(&Game)` in the new module
`spheres-web/src/guidance_outcomes.rs`. It runs under the same lock as `state` and
`production`. It only reads: no commands, no mutation, no clock movement. The main.rs
change is `mod guidance_outcomes;`, one line in `guidance_json`, and tests.

All days are absolute days since 1 Jan 1990 (`clock::date_day`). Money is a number in
$bn. All lists are filtered to the player's nation and sorted.

```json
{
  "nation": "FRA", "date": "2 Jan 1990", "as_of_day": 1,
  "money": {
    "journal_available": true,
    "decisions": [{"id": 1, "day": 0, "kind": "program_budget"}],
    "program": {"enabled": true, "fiscal_year": 1990, "settled_day": 0}
  },
  "construction": {
    "projects": [{"id": 7, "kind": "starter_industry", "district": "FR-IDF",
                  "spent_bn": 0.12, "contract_cost_bn": 1.0,
                  "last_day": 3, "last_spent_bn": 0.01, "progress_days": 3.0, "total_days": 90}],
    "completions": [{"date": "28 Dec 1990", "day": 361, "text": "France completes ...",
                     "district": "FR-IDF", "kind": "processing_plant"}],
    "operating": [{"district": "FR-IDF", "kind": "starter_industry", "day": 400, "output_daily": 0.002}],
    "mines": [{"district": "FR-IDF", "commodity": "coal", "spent_bn": 0.02, "last_day": 6,
               "progress_days": 5.0, "total_days": 60}]
  },
  "research": {
    "active": {"component": "...", "progress": 4.0, "cost": 24.0},
    "learned": 12, "last_completed_day": null,
    "drafts": [{"name": "Test", "updated_day": 1, "valid": true}],
    "revisions": [{"id": "FRA-design-1", "platform": "air_fighter", "created_day": 5,
                   "certified_day": null, "imported": false}],
    "projects": [{"id": 3, "revision": "FRA-design-1", "started_day": 5, "completed_day": null,
                  "last_day": 9, "last_spent_bn": 0.002, "spent_bn": 0.01}]
  },
  "procurement": {
    "companies": 0, "certified_products": 0, "developing_products": 0,
    "deliveries": [{"id": 4, "company": "...", "revision": "...", "quantity": 12,
                    "purchased_day": 900, "settled_day": 900, "due_day": 907,
                    "delivered_day": null, "status": "in_transit"}],
    "imports": [{"id": 2, "seller": "USA", "revision": "import-...", "ammunition": false,
                 "quantity": 4, "purchased_day": 0, "settled_day": null, "due_day": null,
                 "delivered_day": null, "cancelled_day": null, "status": "awaiting_settlement"}]
  },
  "aviation": {
    "bases": [{"id": "FR-IDF", "name": "...",
               "project": {"track": "capacity", "target_level": 1, "started_day": 2,
                           "last_paid_day": 6, "paid_bn": 0.01, "total_cost_bn": 0.024,
                           "completed_day": null, "cancelled_day": null},
               "history": [{"track": "capacity", "target_level": 1, "started_day": 2,
                            "completed_day": 14}]}],
    "squadrons": [{"id": 1, "assigned": 0, "ready": false, "blocker": "..."}],
    "missions": [{"id": 1, "status": "flown", "report_day": 700, "aircraft": 4}]
  }
}
```

Field notes:
- `completions[]`: the newest 20 construction headlines tagged with the player, oldest
  first. `kind` is the `ProjectKind` key (the same key space as `projects[].kind` and
  `operating[].kind`), or `"mine"` for a mine or field opening. `district` and `kind` are
  null when the headline does not map to exactly one.
- `mines[]`: player-started mine projects funded from the construction budget, sorted by
  (district, commodity). `spent_bn`, `last_day`, `progress_days` and `total_days` come from
  the mine's funding row and are null when there is none (never estimated). The funding
  row stamps `last_day` on every settle attempt, so it bounds the payment date.
- `drafts[].valid`: `equipment::design_preview(w, me, &spec).valid`, the same native check
  the designer uses, against what the nation knows at the time of the reading.
- `missions[]`: the newest 10 orders, plus the earliest `flown` order when it falls outside
  them (at most 11 rows, ascending id), so a dated first flight is never cut off.
- Only projects and history entries whose `sponsor` is the player are served for bases.
Each section is `null` when its system is not enabled for the campaign, which the browser
reads as `unknown`. The native tests check that:
- `outcomes.nation == state.player`, and `as_of_day` equals the date served in `state`;
- `outcomes` is `null` with no player;
- the save is byte-identical before and after the read;
- player filtering excludes another nation's records.

## Save and resume receipts (host hook)

The server keeps no record of a save or a load after the response. The host captures the
native responses in one place: `api()` in `index.html`, after a successful `POST /api/save`
or `POST /api/load`. It calls `window.GUIDANCE_RECEIPT({path, body, data, state})`.
Guidance validates the response, then stores it under a new key
`spheres.guidance.receipts.v1` (at most 20 saves and 20 loads, bounded parsing):

- **save:** kept only if `data.ok === true`, `data.slot` equals `body.slot`, `data.date`
  equals `state.date`, and `body.session_id` equals `state.session_id`. Record:
  `{session_id, player, slot, date, dispatches}`.
- **load:** kept only if `data.session_id` is a new non-empty string, `data.storage_notice`
  is exactly `Campaign and its history restored.`, and `data.player` and `data.date` are
  present. Record: `{session_id: data.session_id, player, slot: body.slot,
  backup: body.backup === true, date, dispatch_count}`.

Every tab of the browser shares the store:
- Stored rows are merged into the controller's copy before every write and at the start
  of every refresh, so a write never drops a row another tab stored.
- `mount()` listens for the window `storage` event on this key (or a cleared store). It
  merges and re-evaluates the current reading only while that reading is ready and current.
- A missing key means nothing is recorded in this browser. A record that exists but
  cannot be read (corrupt, oversize, missing arrays or another `version`) vouches for
  nothing: it is never overwritten, and a notice says so. A failed read is treated the
  same way. While the store is not readable, each kind is passed to the model as `null`
  (unknown) unless this visit captured a receipt of that kind for the current session.

`save_resume` counts as `achieved` only when the current reading's `state.session_id`
equals a stored load receipt's `session_id` and the player and date agree. The `saved`
milestone is done for a save receipt made in the current session, or for the save whose
slot, date and player match the load. A page reload followed by Continue keeps the same
session, so the stored receipt still applies. A new game or a different campaign has a
different session and does not inherit it.

## Browser model

- `advisor-model.js` gains `recognize(response, receipts)`. It is pure and has no storage
  or clock. It returns a `route` object (contract below) and uses the existing `dayIndex`
  helpers. `evaluate(state, production, outcomes, receipts)` also adds at most one
  `route-next` card: the first step that is not achieved, if it has an obstacle and no
  other card already opens the same screen.
- Advisor accuracy fixes:
  - Annual renewal is due only when `annual_budget.due === true`, or when programmes are
    enabled and due. Before this fix, disabled programmes read as due.
  - The treasury card fires only for a negative balance, because deficits spend cash down
    to zero before borrowing.
  - Fiscal recovery distress (`recovery_required` with status `adjustment` or `crisis`)
    opens Money and recovery.
- `guidance-ui.js`:
  - Keeps the whole response.
  - Checks `outcomes.nation`/`as_of_day` against `state`, as well as the existing
    session/player/t/date gate.
  - Refreshes when the tutorial opens on a live campaign. This is a read only; the clock
    is already paused.
  - Adds a "Your first hour" route panel after the lessons, so the current lesson and its
    complete/skip controls stay first at every width. One plain line beside lesson
    progress summarises it ("Campaign results: 2 of 6 achieved · see Your first hour
    below", or "unknown"). Each step shows **Read / Skipped / Not read** separately from
    **Achieved in this campaign / In progress / Not yet / Unknown**, with dates, evidence
    and the obstacle with its review button.
  - Stale state: the last verified route stays visible, labelled "Last checked <date> —
    refresh", for the same session and player. It is cleared on a different session or
    player.
- `tutorial-model.js` adds a tenth lesson, `air-force`, with action `{kind: "air"}`.
  Progress schema v1 is unchanged: old progress remains valid and the new lesson starts
  unread.

## Route (JS) contract

```js
AdvisorModel.recognize(response, receipts) => {
  status: 'ready' | 'unknown',
  reason: string | null,                // why unknown
  as_of: {session_id, player, date, day} | null,
  steps: [{
    id, title, lessons: [lessonId...],
    status: 'achieved' | 'progress' | 'not_yet' | 'unknown',
    summary: string,                    // one sentence
    milestones: [{id, label, status: 'done'|'pending'|'unknown', day: int|null, date: 'YYYY-MM-DD'|null, detail: string|null}],
    obstacle: null | {title, reason, action: {kind, ...}, actionLabel}
  }]   // always six steps in route order
}
```

`receipts` is `{saves: [...], loads: [...]}` as stored above. Invalid input yields
`status: 'unknown'` with all six steps `unknown`. The function never throws.

## Host navigation patch (index.html, coordinated)

The following route kinds are added to `GUIDANCE_ROUTES`:

| Route kind | Opens |
|---|---|
| `cash_flow` | `openCashFlow()` |
| `air` with optional `page: "bases"` | `openEquipment({tab: "flight"})`, then, for `page: "bases"`, `equipmentFlightSelectPage("bases")` when that helper exists |
| `companies` | `openEquipment({tab: "companies"})` |
| `equipment` with optional `tab` | `openEquipment({tab})`; without a tab it keeps the existing drawer, and the tab is validated against the known tabs |
| `industry` with optional `district`/`project_kind` | `openIndustry({district, kind: project_kind})` when both are strings, otherwise `openIndustry()`. The facility kind travels as `project_kind` because `kind` is the route name |

The three airbase obstacles emit `{kind: "air", page: "bases"}`; the `air-force` lesson
keeps a plain `{kind: "air"}`. The page is selected after `openEquipment`'s synchronous
campaign reset, which returns the flight page to Command for a new session.
`equipmentFlightSelectPage` validates the page against `EQUIPMENT_FLIGHT_PAGES` and
refuses while work is pending. The patch also adds the receipt hook in `api()` and
`window.GUIDANCE_RECEIPT`. No other host behaviour changes.

## Tests

- Node:
  - `recognize` covers:
    - real outcomes versus reading (reading progress never changes `route`);
    - each milestone against native fields, including `day > as_of_day` rejection;
    - identity mismatch (session, player, date, outcomes nation or day);
    - a loaded save (new session re-derives from native data and inherits no stale flag);
    - skipped lessons keep an achieved result;
    - unknown and disabled sections;
    - receipt validation and bounds.
  - Controller:
    - a stale response is dropped;
    - `changed()` keeps a labelled last-checked route only for the same campaign;
    - receipts are ignored for a foreign session;
    - another tab's receipts survive a write, count on refresh and on a `storage` event;
    - an unreadable store passes `null` kinds, shows a notice and is never overwritten.
  - Render: escaping, the separate Read/Achieved indicators, the lesson before the route
    panel, and the one-line results summary.
  - Host: route set and targets, including an airbase route landing on the real Bases page
    after the campaign reset.
  - Review page: `tools/ui/guidance-review.js` keeps its annual-budget card and a readable
    route.
- Native: `guidance_outcomes` read-only tests as listed above.
- Focused command, from the repository root:
  `node --test tools/ui/check_tutorial_model.cjs tools/ui/check_advisor_model.cjs tools/ui/check_guidance_outcomes.cjs tools/ui/check_guidance_ui.cjs tools/ui/check_guidance_host.cjs`,
  then `cargo test -p spheres-web --release --bin spheres-web guidance` with this
  worktree's own `CARGO_TARGET_DIR`.
- Browser: a fresh disposable campaign driven through visible controls at 1440x1000 and
  390x844. It runs budget → construction → design/research → airbase → named save → load
  → Continue, with guidance readings at each stage and exact build provenance.
