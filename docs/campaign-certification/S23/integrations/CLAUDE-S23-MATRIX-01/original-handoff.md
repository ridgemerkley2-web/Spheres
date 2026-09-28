# CLAUDE-S23-MATRIX-01 — historical and cartoon boundary audit

Owner: Claude. State: **ready_for_review** (27 September 2026; submitted, not accepted or merged). Parent: S23, **preparation only**.
Branch: `claude/s23-matrix-01`. Base: `e41aa18d`, claim `b4c021ab`; merged `codex/campaign-certification`
`10a03a09` at `8934d033` and then `846df479` (a workstream-query doc update only) at `64c3caab`, both without
conflicts, before submission. Touched paths: only the owned paths below.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md). See [Result](#result).

## Build

Build a standalone audit generating date/role/appearance cases for the eight CP1
country cases from current checked-in research and production bindings. Cover yearly
samples 1990–2035, day-before/day-of/day-after known handovers and deaths/dissolutions,
the frozen 2026-09-07 cutoff, future eligibility, 2030 and 2035-12-31. Unknown interval
boundaries must remain unknown, not extrapolated from an isolated attestation.

Report historical identity, role, source acceptance, actual image binding and asset
availability separately. Compare a historical lookup and a saved gameplay incumbent
without assuming history overwrites a divergent campaign. Keep fixture assertions
separate from actual production observations. Reuse existing readable sources;
this tool must not depend on CLAUDE-C01-GAPS-01 being merged first.

## Owned paths

New `tools/avatars/certified_boundary_matrix.py`, `test_certified_boundary_matrix.py`,
`docs/campaign-certification/S23/preparation/boundary-matrix/`; this handoff.
Read existing research, data and tests; no shared schema, runtime or acceptance edits.

## Acceptance

Deliver deterministic case generation, JSON/Markdown findings and `--check` for
stale output. Test off-by-one handovers, co-leaders, missing versus inapplicable roles,
pending evidence, death/dissolution and missing/wrong image bindings. Run against
current integration and keep actual coverage gaps visible. Passing checker tests
does not mean its coverage report passes; C06/S20 and final S23 review remain required.

## Result

**Submitted `ready_for_review`. Preparation only:** this delivers an audit tool and its
report. It completes no part of S23, C06 or C01, accepts no research, certifies no
country and changes no runtime, research, art, save, index, acceptance or CI record.
**Passing the checker's tests does not mean the coverage report passes** — the report
shows large real gaps. C06 and the final S23 review remain required. S20 is recorded
complete in the merged pathway (`10a03a09`); this audit does not evaluate it.

### Touched paths

- New [`tools/avatars/certified_boundary_matrix.py`](../../../tools/avatars/certified_boundary_matrix.py): the audit, `--check`, and `--campaign SAVE`.
- New [`tools/avatars/test_certified_boundary_matrix.py`](../../../tools/avatars/test_certified_boundary_matrix.py): 19 fixture tests and 4 real-output tests.
- New [`docs/campaign-certification/S23/preparation/boundary-matrix/`](../../campaign-certification/S23/preparation/boundary-matrix/README.md):
  generated `README.md` findings, `summary.json` and eight `cases-*.json` (2.4 MB in total; largest file 398 KB).
- This handoff. The merge commit brings in integration unchanged. Read-only sparse-checkout
  additions in the worktree: `tools/planning`, `docs/campaign-certification/verification`.

### What the matrix covers

6,837 cases over 115 roles in the eight cases (nine identities): each identity's national
executive, every production party row/component, and every C01 research role, including
roles without holders. Each role is sampled on 1 January 1990–2035 (5,290 role-samples),
1989-12-31, the 2026-09-06/07/08 cutoff triple, 2030-01-01, 2035-12-31 and 2036-01-01. It
also gets day-before/of/after cases for each stated boundary day inside the reference period:
200 handovers, 4 deaths, 6 dissolutions, 2 foundings and 1 institution creation. Each of 143
research attestation days is paired with the following day, showing that an isolated
attestation does not extend.

Each case reports these separately:

1. historical identity and evidence strength: boundary day, established, attested, period-attested, bracketed, uncertain, unknown, unresearched or inapplicable;
2. role;
3. acceptance: accepted C01 packet, integrated-but-pending packet, S10 intake, production registry or fictional catalogue;
4. the actual image binding, a mirror of `person_portraits::portrait`;
5. asset availability: file exists, manifest SHA-256, served allowlist, and shared or misnamed wrong-person flags.

Registry lookups mirror `party_leadership::{within, life_contains, historical_on,
possibly_historical_on, eligible, choose}`. Portraits use half-open windows, and the
fictional window is 2026-09-08 to 2036-01-01. Packet classes come from each report's
"Sources added" table, numbered integration acceptance records and the 27 September
integration note. Accepted packets are 01/02/03/04/07/08. Pending packets are
05/06/09–22/26. Sources claimed by no packet report are S10 intake. Mixed evidence is
classified conservatively. SOURCE-05/06 source repairs and the GAPS-01 record do not
change any packet's class.

The fresh-campaign start is observed from `leaders_1990.json`, `office_links` and a
`choose()` mirror. `--campaign` compares a supplied JSON or gzip save against the
historical lookup for that date, without writing anything. Divergence is
`campaign_diverged_expected`, never an error, and history never overwrites the
campaign. Divergence assertions exist only in fixtures. The committed matrix contains
only actual production observations. The census, research, integration and production
inputs are recorded with bytes and SHA-256: 82 files, with text hashed after CRLF→LF
normalisation because this checkout uses `core.autocrlf=true`. The tool does not read
or depend on the GAPS-01 ledger.

### Commands and results (after merging `10a03a09`; rechecked after `846df479`)

```text
python -X utf8 tools/avatars/certified_boundary_matrix.py            # 6,837 cases written
python -X utf8 tools/avatars/certified_boundary_matrix.py --check    # exit 0 (fails on stale, missing or unexpected files)
python -X utf8 -m unittest discover -s tools/avatars -p "test_certified_boundary_matrix.py"   # 23 OK
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"                  # 79 OK
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"                   # 16 OK
python -X utf8 -m unittest discover -s tools/avatars -p "test_*c01*.py"                       # 208 OK
python -X utf8 tools/avatars/campaign_census.py --check                                       # pass
python -X utf8 tools/avatars/campaign_research.py --check                                     # pass
python tools/planning/workboard.py --check                                                    # PASS: 44 markers, 17 tasks
git diff --check (owned paths)                                                                # clean
```

The tests cover:

- a half-open production handover, a handover on a death day and a death ending an open term;
- research boundary days that list both sides;
- isolated attestations and unstated ends;
- co-leaders, including their campaign-start seating, and an acting overlap seated by rank;
- missing, unknown, unresearched and inapplicable roles, dissolution and institution creation;
- accepted, pending, S10, mixed and unclassified evidence, and a source repair that must not promote its packet;
- missing, rejected, ambiguous, unlisted, missing-file, hash-mismatch, shared and misnamed images, fictional windows and identity;
- campaign verdicts and a gzip save;
- byte-identical regeneration, stale/missing/unexpected detection and CRLF-independent hashes.

An uncommitted in-process mutation check injected eight faults: inclusive term ends, an
extended attestation, promoted mixed evidence, ignored deaths, divergence treated as an
error, one-sided extrapolation, ignored review flags and ignored hash mismatches. Each
turned at least one test red, and the restored suite passed.

After the merge, the pre-merge cases were byte-identical. Only input hashes, the
non-packet record list and the README wording changed. This agrees with the
SOURCE-05/06 reviews: metadata only, no historical claim changed.

### Headline gaps (actual observations; details in the generated README)

**Research acceptance**

- Only Tonga has accepted C01 evidence: 19 accepted and 10 S10 observations.
- Every other certified research chain is pending (C01-05/06/09–22/26) or S10 intake.
- France has no C01 office or leadership research at all. Its executive (the President) is unresearched, and every French historical identity comes from the unreviewed registry.

**Yearly research identification** (1990–2026 role-samples that identify a holder)

- Japan: 9/185. India: 6/111. Brazil: 20/111. South Africa: 4/259. Tonga: 23/555.
- Saudi Arabia: 0/370 (22 bracketed). USSR: 0/222. Russia: 3/333.
- Most research holders have a stated start and no stated end. They therefore remain `uncertain`, which is correct under the no-extrapolation rule. Many offices are attested only on isolated days.

**Registry holders and portraits**

- Registry holders are established at 197/555 France, 151/296 Japan, 109/148 India, 99/222 Brazil and 151/259 South Africa yearly samples.
- USSR and Russia rows are unknown-kind and unresearched: 0/296 samples.
- Tonga and Saudi Arabia have no party rows, so neither a production party chain nor a fictional pool exists.
- Holder portraits are served at only 23/198 France, 15/151 Japan, 10/109 India, 7/100 Brazil and 13/151 South Africa samples. All 912 unbound established-holder images, across every case date, fall inside a pending art job.

**Campaign start and the future pool**

- Campaign-start executive portraits are unbound for Mitterrand, V. P. Singh, Sarney, Fahd and Gorbachev.
- Tupou IV's portrait covers 1990 only. No art job covers him if a campaign retains him.
- Campaign-start executive comparisons: 8 are `historical_identity_not_established`, because no research observation covers 1990-01-01, and Russia is absent at the start.
- Campaign-start party seating: 25 rows match the registry, 1 is a rank subset (the PMDB acting overlap) and 17 are vacant on both sides.
- Future: of 192 fictional candidates, only the four Japan LDP candidates have portraits.
- France, India, Brazil and South Africa have 0 executive-authorized fictional candidates, because there are no reviewed future office grants.

**Data-quality flags**

- Portrait windows run past recorded deaths for Rajiv Gandhi, Oliver Tambo and Zephania Mothopeng.
- V. P. Singh's Janata Dal term has an unknown end, so he stays "possible" through the cutoff.
- The DP joint leaders are never established on any date: their end is year-precision 1990.
- USSR→Russia has no structured dissolution, creation or retitling day, so the transition has no day-level boundary case.
- All 26 checked portrait assets are available; none is shared or misnamed.

### Limitations

- Research roles are not reconciled to production rows or people, because `represented_party_ids` is empty. The executive pairing is an audit aid based on the 1990 office title, not a mapping.
- Only structured holder fields are used. Deaths or dates that appear only in notes or claim text create no cases.
- The production-side campaign observation is the fresh 1990 start. S10.b saved campaigns are outside this sparse checkout; they are fresh starts equivalent to the 1990 rows.
- Portrait checks mirror the selector and file hashes. They are not a visual likeness review.

### Outside scope (observed, not changed)

The merged CLAUDE-C01-GAPS-01 ledger (`certified_gap_ledger.py --check`) is already stale at
integration `10a03a09`. `docs/planning/ai-task-queue.json` changed in `f2d34895` and
`10a03a09` after the ledger was pinned at `ad498418`. This branch does not touch that
queue, and the ledger is not an owned path.
