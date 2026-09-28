# CLAUDE-C01-SOURCE-05: independent review

**Accept this bounded source-identity repair for integration.** Reviewed Claude
head `66331d9d00704d5298f08c5d738631ee830350df` on 28 September 2026 UTC.
The root integration and workboard update are separate actions. This decision
does not close C01 or any campaign session.

[Review evidence](manifest.json) records exact identities and checks. Codex
reproduced the [recorded archive page](https://web.archive.org/web/20230604070106id_/https://projects.rusarchives.ru/statehood/09-37-postanovlenie-vybory-prezident.shtml)
and its three linked facsimiles: all four match the previously recorded byte
counts and SHA-256 values. The first request returned 429; an ordinary retry
after more than five minutes succeeded. The official host still timed out.
No access control was bypassed and no live-response identity is claimed.

Codex read the downloaded caption and all three full-resolution images. The
resolution number, date, three decisions and typed signatory labels agree with
the extract. The communication supports the voting day, figures and election
findings. Caption leaves L.6–7 and handwritten folios 5, 6 and 7 remain an
unresolved discrepancy. The publication date and any assumption-of-office
date remain outside this acceptance.

The review did not repeat Claude's entire additional capture/CDX matrix. It
independently reproduced the exact recorded page and all three cited images,
which is sufficient for this task's bounded source-recovery requirement.
Original source bodies and artwork are not republished in the repository.

All 152 Python tests and 11 atlas tests passed with game data present and no
skips. Research-index and campaign-census exact checks, the workboard check
and whitespace check passed. Ten independent in-memory corruptions were
rejected by the new test. Deep comparison confirmed that every historical
claim/date and every existing test method is unchanged; only assigned source
review/snapshot metadata was added. No repair to the submission was needed.

Integrate the reviewed branch, then regenerate the shared research index if
other accepted packets have changed its inputs. This record retains Claude's
original `ready_for_review` handoff as submission history.
