# Applicability of opening Army removal assessments after a first civilian ballot

Read-only source audit, 2026-09-22. No execution or runtime changes.

The six requested source values contain no zero or low weighted result:

| Nation | HOS military-removal mean | HOG mean | HOS/HOG weights | Opening value | Local opening executive tie |
|---|---:|---:|---|---:|---|
| Haiti | .500 | missing | 1/0 | .500 | Prosper Avril / Army |
| Thailand | .143 | .833 | 0/1 | .833 | Chatichai / th_chartthai |
| Pakistan | .800 | .800 | 0/1 | .800 | Bhutto / pk_ppp |
| Bangladesh | .833 | missing | 1/0 | .833 | Ershad / bd_jp |
| Myanmar | .667 | missing | 1/0 | .667 | Saw Maung / Army |
| Nigeria | .500 | missing | 1/0 | .500 | Babangida / Army |

The ties/office labels above describe the checked-in leaders_1990.json, not a
new independent historical classification. Thailand's low HOS score correctly
gets zero weight; it is not suppressing the politically relevant HOG component.
Missing duplicate HOG components do not become observed zero.

V-Dem's v16 questions concern the contemporaneous executive and practical
removal without military force. The military items are means of binary expert
responses. Cabinet-power weights distinguish the relevant executive. They do
not estimate military independence, armed overthrow capability, or authority
over a hypothetical future civilian government. A military body's leverage
over its own incumbent need not match its leverage after a different
constitutional settlement. This is a source-applicability limitation with no
implied direction or magnitude of change. [Codebook, sections 3.4.2.4,
3.4.3.4 and 3.4.3.18](https://www.v-dem.net/documents/70/codebook_v16.pdf).

The exact inputs come from the pinned official
[V-Dem package](https://github.com/vdeminstitute/vdemdata/tree/f4dd26922e658442524dfd954bf14f7ebe622d5d),
reference year 1989, already archived and checked under this directory.

## Current lifecycle

- army_authority::prepare_opening initializes a separate saved campaign value
  from the immutable lagged source only during fresh world construction.
- schedule_first_elections (government.rs:7007) retains Army/Security under the
  takeover rules and does not change saved authority. It schedules the real
  ballot and clears accumulated coup pressure.
- hold_election (7087) replaces an actual military executive on the first
  completed free ballot, even when its provisional party table has the same
  winner. The first handover is not charged as an elapsed elected term.
- Consolidation (7163-7169) therefore does not silently erode authority at this
  first handover; only a later genuine eligible party transfer carries the
  bounded existing consolidation. Actual Army seizure can separately update
  saved current authority to 1, leaving the historical source unchanged.

No mistaken source refresh, erroneous HOS weighting, first-ballot erosion or
zero-reading defect was found. Retaining the saved number after an opening is
an explicit carry-forward model prior, not a claim the 1989 assessment was
re-observed after a counterfactual transition. Removing that prior would need
an independently justified new transition model. The source does not authorize
setting authority to 1, increasing it, or substituting an authoritarianism
fallback merely because a military-led regime yielded to civilians. Bangladesh
also illustrates why a party tie and a politician's military biography are not
interchangeable state classifications.

Recommendation: no runtime correction on this evidence. If later work models
constitutional bargaining or institutional command changes, retain the source
snapshot and record those observed campaign events separately, with same-party,
royal, annulled-ballot, missing-source, save/load and flag-off controls. Do not
promise an A1 outcome from this conceptual extension.
