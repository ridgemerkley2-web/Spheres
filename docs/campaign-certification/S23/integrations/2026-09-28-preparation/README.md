# Combined S23 preparation integration

**Six bounded tasks closed; S23/C06 remain open.** Reviewed 28 September 2026.
The [S23 progress page](../../README.md) lists completed deliveries and remaining
content work. No production runtime, save schema or artwork changed in this work.
The canonical roadmap still has S01-S22 complete and S23 planned pending C06.

## Combined validation

The final [validation record](validation-final/validation.json) passes all ten
commands, including **469 avatar/research tests, zero failures and zero skips**.
It pins the interpreter, exact commands, durations, raw input identities and
logs. All checked inputs remained unchanged during execution. The native
simulation and game web trees match S22 closeout `75626a13` exactly.

The other nine commands check the campaign census, shared research index,
research gap ledger, cartoon export, boundary matrix, fictional proposal
validator, workboard and both source-review evidence packets. The workboard
validates 44 canonical markers and 19 bounded tasks.

The C03 [browser review](../../../C03/integrations/CLAUDE-C03-REVIEW-01/README.md)
separately passed 15 Node/browser entries, with six retained screenshots and
keyboard/narrow-screen checks. Those checks were performed on the final C03
tool bytes, which did not change afterward. They were not rerun as part of the
Python sweep. No new artwork is approved.

## Retained failures and environment scope

The [initial combined run](validation/validation.json) passed nine checks but
failed one of 469 tests: the CNCCFP PDF corroboration test. The bundled pypdf
6.10.0 layout extractor interleaved wrapped party names into the decision
column. The same pinned PDF and unchanged test pass with the already installed
**Python 3.13.12 / pypdf 6.17.0**. The final complete sweep uses that environment.
No source document, party identifier, importer, regex or test assertion was
changed to make it pass. This records a qualified environment, not a claim
that every pypdf version supports the PDF layout audit.
The independent [extractor diagnosis](environment-diagnostics/README.md) pins
the exact source PDF, unchanged test and passing 60-row comparison.

Use Python with pypdf 6.17.0 for the full audit; do not silently omit the optional
PDF check. Run the same sweep into a new directory:

```text
python -X utf8 docs/campaign-certification/S23/integrations/2026-09-28-preparation/run-checks.py --output NEW_EMPTY_DIRECTORY
```

The runner refuses to overwrite an existing run. Its input pins distinguish
the captured working tree from its pre-check Git HEAD, because generated
indexes and queue updates were present when the checks ran.

[Earlier targeted logs](prior-checks/) retain the 25-test claim-reservation pass,
the 26-test grouped-repair pass, and the 73-test combined successor pass.
The first grouped-repair run had one stale generated-ledger failure; regeneration
resolved it without weakening assertions. The separately indexed review packets
retain the original submissions and negative-control failures that motivated
their repairs.

## Integration decisions

| Task | Root integration / repair | Accepted scope |
| --- | --- | --- |
| CLAUDE-C03-REVIEW-01 | `595a6a85`, `9a89e259`, `ae3610b4` | Read-only cartoon review tool, portability and provenance fixes. |
| CLAUDE-C04-PREP-01 | `0bd94e9d`, `99f8a3bc` | Eight fictional proposals with reviewed constraints; no appointments or art. |
| CLAUDE-S23-MATRIX-01 | `f50a614c`, `13c0e0d5`, `36740f8b` | Corrected conservative source-derived audit, including saved-incumbent handling. |
| CLAUDE-C01-SOURCE-17 | `6926a007`, `bdb35a07`, `1c5f129f` | Three reviewed extracts; independently reproduced current/archive source and corrected reproducibility scope. |
| CLAUDE-C01-SOURCE-26 | `567610eb`, `1c5f129f` | Thirty reviewed extracts, exact response identities, checked corrections and evidence withdrawal. |
| CODEX-C01-ACCEPTANCE-01 | `1c5f129f` plus current queue | Four assigned source follow-up reviews complete; parent histories remain pending. |

Root also preserves the active C01-28/29 Russia/Japan claims (`01c68fe1`) and
requires every grouped source snapshot to occur in its intact accepted review
evidence (`e94e07c5`). No broader source-packet acceptance follows from either.
The [source-hash scope sidecar](source-hash-scopes.json) clarifies raw versus LF
and Git blob identities without rewriting prior evidence.

Independent read-only reviews found no further issues in C03's byte handling,
the grouped source completion rules, the three preparation packets' declared
hash scopes, or the final workboard's status separation. Link validation
included the new S23 progress page; this README completes its pending link.

The final generated matrix has 7,093 cases, two fewer than the reviewed
7,095-case preparation snapshot because the unsupported Soviet appointment
was withdrawn. The gap ledger still reports 50 party chains and 49 research
roles with unresolved days. Twelve proposed research batches contain 83
items after excluding active claims. None of these are country certificates.

The [manifest](manifest.json) pins this combined record and links the bounded
reviews. Future generated-output refreshes must not rewrite these retained runs.
