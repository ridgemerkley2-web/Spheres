# S26 preparation-tool validation

**23 synthetic regression tests pass on both Python 3.13.12 and 3.12.14.**
These are two runs of the same 23 tests, not 46 different scenarios or any
human observations. Exact commands, interpreter versions, source-byte pins and
log identities are retained in [validation.json](validation.json).

The tests cover empty plans, AI/non-independent exclusions, duplicate anonymous
participants/session IDs/JSON fields, first-attempt scoring, coached attempts,
in-game help, retries and early stops, every participant's explanations,
country/keyboard/390px/later-save coverage, package/save/evidence integrity,
path escapes, changed builds, chronology, overlapping sessions, persistent
failure records, non-destructive CLI outputs and exact raw artifact hashing.

The initial 21-test run found one CLI portability error: Windows redirected
stdout could not encode the USSR → Russia label using its default legacy code
page. [initial-tests.log](initial-tests.log) preserves the reproduced failure.
The CLI now explicitly uses UTF-8; no assertion or coverage target was weakened.
Two additional tests then covered duplicate JSON fields and raw artifact pins.
The final logs are [Python 3.13](python313-tests.log) and
[Python 3.12](python312-tests.log).

The checked-in empty records/report contain **zero participants and zero
observed sessions**. Passing synthetic bookkeeping tests does not make those
empty coverage cells pass. Every report keeps `s26_complete: false` and
`qualification_awarded: false`, including synthetic all-fields-complete fixtures.

This change contains no runtime, art, save-schema or global roadmap edit. No
native Cargo suite, browser session, human session or campaign advancement ran
as part of this bounded preparation. The S24 prerequisite and human authenticity,
source/save suitability, observation quality and final acceptance remain for
the independently conducted session review.
