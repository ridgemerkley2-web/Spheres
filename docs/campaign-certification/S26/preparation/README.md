# S26 human playtest kit — preparation only

This kit is ready to collect observations. **No human sessions have been run or
passed by this delivery. S26, G5 and CP1 are not awarded.** Its canonical
prerequisite is S24; preparing the kit does not bypass that dependency.

The source is [S26 and the approved usability criteria](../../../CERTIFIED_CAMPAIGN_PATHWAY.md#s26),
read at `d2b21c15ad8070ad9bde5363d888a31e1c4dc63c`. That revision is the planning
reference, not a claim that it is a qualified human-test build. The kit changes
no game, art, research, save format or global workboard.

## What needs actual observations

- At least five independent first-time people and at least eight sessions,
  collectively covering France, Japan, India, Brazil, South Africa, Tonga,
  Saudi Arabia and the controlled USSR → Russia case.
- At least four of the five newcomers complete **budget → construction →
  recorded result → save** without facilitator coaching. Built-in game advice
  is allowed and recorded separately from human assistance.
- Every participating independent human identifies who governs and explains an
  actionable blocker. Retain their actual words and the observer's comparison
  with the current campaign, including incorrect answers.
- Include keyboard navigation, the approved 390px narrow layout and later-game
  saves. Record actual dates and save lineage; no new numeric definition of
  “later game” is introduced by this preparation.

Agent-operated checks, made-up participants, planned slots and synthetic fixtures
cannot satisfy human coverage. A real person playing a disclosed controlled
USSR → Russia fixture can provide usability evidence; the fixture's validity and
its separate S24 continuity evidence still need review.

## Start a study

1. A facilitator and integrator select the candidate after reviewing the S24
   prerequisite. Record exact Git source and data/art revisions, package
   SHA-256, OS and browser version. Keep the executable/package available as
   evidence. Use a separate study if the build changes.
2. Run `python -X utf8 tools/playtests/s26_playtest.py init work/s26-study-001`
   from the repository root. It creates empty `records.json` and an unfilled
   `session-template.json`; it does not launch the game.
3. Assign anonymous `P001`-style participant codes, one per actual person.
   Record whether the person is independent and new to Spheres, with a short
   facilitator eligibility note. Do not include names, emails or contact details.
   Use the same code for repeat visits; a repeat session never makes someone
   first-time again. The facilitator must attest that codes refer to distinct
   people; software cannot prove that from anonymous labels.
4. Before anyone starts, declare the five-person opening sample in
   `study.first_time_cohort` and record its UTC lock time. Additional people may
   participate, but do not replace a coached or unsuccessful newcomer after
   looking at results. Score the first actual attempt; retain retries separately.
   This is bookkeeping for the approved four-of-five sample, not a larger target.
5. Fill the planned slots below. Keep `participants` and `sessions` empty until
   there are real eligibility records and observations. Never turn the plan
   into a completed session by copying assumed outcomes.

| Slot | Country case | Proposed session | Planned variation |
| --- | --- | --- | --- |
| PLAN-01 | France | Newcomer opening | Integrated finance/construction/output |
| PLAN-02 | Japan | Newcomer opening | Import/workforce constraint explanation |
| PLAN-03 | India | Newcomer opening | Large-country information/navigation |
| PLAN-04 | Brazil | Newcomer opening | Fiscal adjustment and construction |
| PLAN-05 | South Africa | Later save | Keyboard-only, resource/access explanation |
| PLAN-06 | Tonga | Newcomer opening | Affordable country-appropriate decisions |
| PLAN-07 | Saudi Arabia | Later save | 390px viewport, finance/import choices |
| PLAN-08 | USSR → Russia | Controlled later save | Explain actual government after valid transition |

Slots are unassigned suggestions, not observations. Schedule each newcomer's
opening before their later visit. No small country must build heavy industry or
complete every equipment loop to fill a checklist. Extra sessions are allowed.

## Facilitate without supplying the answer

Before timing begins, explain that this evaluates the game, not the person.
Ask permission before making any recording; anonymous written observations and
relevant screenshots/save artifacts are sufficient. Set the assigned viewport
and input method, open a copy of the selected campaign and record its actual
country, date, seed, and starting save identity. Describe any authored fixture.
Never overwrite someone's personal campaign to run this exercise.

Read this neutral opening goal:

> Make a useful, affordable improvement for your country. Review its budget,
> start a construction project, find a recorded result of that work, and save
> the campaign. You may use the game's own help. Tell me what you think is
> happening and where you would look next.

Do not point at controls, choose a building, set funding or navigate on the
participant's behalf. In-game tutorials/advisors may guide them. If they ask
for help or wish to stop, provide it respectfully and record the time, exact
hint/intervention and affected task in `coaching_events`. It becomes a coached
attempt; do not erase it or silently restart their first-attempt score.

For each of the four steps, record the actual action and visible outcome, UTC
time, assistance, evidence references, and `complete`, `blocked` or
`not_attempted`. “Recorded result” means an observed consequence of the project
in the campaign, not a quoted expected effect in a preview. Record the
country-appropriate result in the notes. A successful save needs the resulting
save artifact, not just an assertion that a button was clicked.

Ask every participant:

> Who governs your country right now? What is currently preventing something
> you want to do, and what concrete action could you take about it?

Preserve the response and current-government/blocker UI evidence. The observer
marks whether it is correct and explains why; the validator does not decide
political truth or correctness from prose. Record hints even when the person
eventually gives the right answer. The report accepts later correct explanations
but retains all earlier observations; independent review can see that learning.

For later saves, ask the person to orient themselves, explain government and a
blocker, make or inspect a useful country-appropriate decision, inspect an actual
result and save. Record relevant tasks honestly; a later session cannot rescue
a failed newcomer opening score. Keyboard and narrow sessions must contain real
interactions in those modes, described in `interaction_notes`, not merely a
resized browser or an unused keyboard.

## Record and review

Use [FIELD_GUIDE.md](FIELD_GUIDE.md) while filling records. Copy the unfilled
session template once per real session, give it a stable `S001` ID, and mark it
`observed` or `aborted` only after observation. Record early stops and confusion.
Every blocked path gets a stable `S26-F001` issue, severity 0–3, affected task,
evidence and concrete notes. The approved defect policy remains unchanged;
later repair work may link that issue to S27. Do not remove a failed country.

Pin notes, screenshots, package and save files using the tool's `pin` command
(see [tool usage](../../../../tools/playtests/README.md)). Keep original evidence
unchanged. Run:

```text
python -X utf8 tools/playtests/s26_playtest.py report work/s26-study-001/records.json --output work/s26-report-001
```

Exit 2 means invalid records/evidence. Exit 1 means coverage is still missing.
Exit 0 means the recorded sample is ready for independent human review, **not
S26 acceptance**. The report always states `s26_complete: false` and
`qualification_awarded: false`. Read errors, missing country cells, coached
first attempts and failures together. Preserve previous reports and sessions.

The integrator must review participant authenticity/independence, actual saved
campaign and build context, scoring, fixture suitability and S24 dependency
before any canonical status change. AI-authored tests cannot sign that review.

The shipped [empty records](records.json) and [empty coverage report](empty-report.md)
show zero human observations. [Validation](validation/README.md) covers the tool
and its synthetic regression fixtures only.
