# Recording fields

Use `records.json` for one candidate and one declared newcomer cohort. Use
`session-template.json` as an unfilled form; nulls are intentionally not results.
All timestamps need an explicit time-zone offset, preferably `Z` for UTC.

| Field | Enter from actual evidence |
| --- | --- |
| `study.id` | Local study label, e.g. S26-STUDY-001. |
| `study.first_time_cohort` | Exactly five P001-style codes chosen before sessions; at least four must succeed on their first attempt. Extra people are listed separately. |
| `study.cohort_locked_at_utc` | When the cohort was fixed, before its observations. |
| `study.distinct_humans_attestation` | Facilitator's statement that codes identify different independent people; no names/contact data. |
| `build.source_revision` | Exact 40-character Git source revision of the tested game. |
| `build.data_assets_revision` | Exact 40-character data/art revision, normally the same checkout. |
| `build.package_sha256`, `evidence_refs` | Raw tested package hash and E001-style reference to the actual build artifact. |
| `planned_sessions` | Scheduling suggestions only. These never count as observations. |
| `participants` | One object per person: `code`, `operator` (`human` or `agent`), boolean `independent`, boolean `first_time`, `eligibility_evidence` list. Record newness before their first session. |
| `evidence` | Objects printed by `pin`: unique ID, local relative path, kind, raw byte count and SHA-256. |
| `sessions` | Actual observations copied from the session template. `planned` forms do not count. |

Example participant shape, **unfilled and not a real participant**:

```json
{"code": null, "operator": null, "independent": null,
 "first_time": null, "eligibility_evidence": []}
```

| Session field | Meaning |
| --- | --- |
| `id`, `participant`, `state` | Unique S001-style ID, existing P001-style code, `planned`/`observed`/`aborted`. Keep aborted attempts and give an `abort_reason`. |
| `country_case`, `phase` | Exact case label from the eight-country plan; `opening` or `later`. |
| `started_utc`, `ended_utc` | Actual session window; one person cannot have overlapping sessions. |
| `build_source_revision` | Must equal the study build. A changed build needs separate records. |
| `environment` | OS, browser/version, positive actual CSS viewport width/height and `keyboard_only` boolean. Narrow coverage uses the approved 390px viewport. |
| `campaign.nation`, `date`, `seed` | Actual nation/date/seed. USSR → Russia case uses nation `USSR` or `Russia`, with its transition context explained. |
| `campaign.origin` | `fresh`, `ordinary_save` or `authored_fixture`. Later sessions require a save. |
| `campaign.start_save`, `provenance` | E001-style save reference or null for fresh starts; explain actual lineage and fixture creation. No fictional outcomes. |
| `evidence_refs`, `interaction_notes` | Session observation artifacts and real navigation/layout/input behavior. |
| `tasks` | Four ordered observations: budget, construction, recorded_result, save. Each needs outcome, assistance, at_utc, notes and evidence_refs. |
| Task `outcome` | `complete`, `blocked` or `not_attempted`; blocked tasks need a failure record. |
| Task `assistance` | `none`, `in_game` or `facilitator`. Built-in advice does not count as facilitator coaching. |
| `government` | Outcome `correct`/`incorrect`/`not_answered`, participant's answer, observer's factual basis, assistance and evidence references. |
| `actionable_blocker` | Same answer fields, plus `next_action` for a correct concrete remedy. An ordinary game constraint need not be a defect. |
| `coaching_events` | List every facilitator hint/intervention: at_utc, task and notes. Navigation hints affect the opening score even if individual tasks say “none.” |
| `failures` | Stable S26-F001-style ID, affected task, severity 0–3, notes and evidence_refs. Keep observed confusion and bugs; never invent a failure or a successful recovery. |

The validator requires evidence and consistent bookkeeping. It cannot verify a
person's independence, interpret a native save or judge a participant's answer
from a hash. Those are explicit independent-review responsibilities, not
automatically passed checks. Preserve all records even when subsequent attempts
or a later build perform better.
