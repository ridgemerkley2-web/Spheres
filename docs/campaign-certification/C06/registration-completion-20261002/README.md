# Six cast batches completed — 2 October 2026

All **31 requested portraits are reviewed, registered and verified in the compiled game**. The six bounded batch-1 tasks can close: Brazil **7**, South Africa **4**, Japan **7**, India **8**, Saudi Arabia **4**, USSR/Russia **1**. This follows the user's authorization for Codex to fill the remaining gaps; no Claude review or human approval is invented.

Doi's apparent age and Gorbachev's birthmark received new versioned image_gen illustrations and independent source/likeness reviews. The PNGs are unchanged tool outputs. Earlier versions, held attempts, original references, prompts, source limits and render-return receipts are preserved.

The game’s stale embedded avatar list was rebuilt. The compiled game serves all **38** newly enabled PNGs byte-for-byte: France's seven previously registered portraits plus these 31. The new native regression checks each registered historical image at both date boundaries and immediately outside them; an intentionally wrong inclusive-end implementation fails, while the restored correct implementation passes.

## Verification

| Check | Observed result |
|---|---|
| Ordinary native workspace suite | 2,006 passed, 0 failed; 115 existing ignored tests |
| Resource timing, run alone | Passed enforced 0.15 ms/month bar; measured total 0.0578 ms/month (aspirational 0.05 target not met) |
| Interface suite | 1,831 passed, 0 failed; 1 existing skip |
| Avatar and planning tooling | 828 avatar tests and 69 planning tests passed |
| Runtime serving and references | 38 exact PNG hashes; 15 historical reference cases; 4 invalid/out-of-range dates rejected |
| Save safety | GET-only isolated server; before/after state identical; fresh save directory remained empty; owned process stopped |
| Provenance | 208 independent integration pins verified; all 33 earlier returned outputs preserved; exact raw request/evidence bytes protected across checkouts |

The release build and command logs are in [verification.json](verification.json). Runtime code was introduced at `a982f752`; later verification checkpoints refresh derived tracking metadata and preserve evidence newline bytes. The runtime receipt records the exact compiled revision and binary hash. Original failed checkout/export attempts and the expected mutation failure remain alongside their passing reruns. No test tolerance was widened or regression removed.

## Deliverables

- [31 additive registrations, source rights, appearance windows and preserved-record checks](registration.json)
- [Every registered output path and SHA-256](SHA256SUMS.txt)
- [Two corrected renders: exact prompts, ordered inputs, original output paths and SHA-256](correction-renders.json)
- [Independent combined integration review](independent-integration-review.json)
- [Fresh-checkout byte integrity](checkout-byte-integrity.json)
- [Original six-batch render return, unchanged](../render-return-20261002/README.md)

Brazil's Temer and Lupi source references intentionally use the same licensed group photograph with two different identified subjects. Person-specific prompts and distinct generated portraits were reviewed. The automatic duplicate-reference warnings remain visible.

The historical registry and all 107 earlier portraits are unchanged. There are now **138 historical portraits for 115 people**, with **55 historical art jobs resolved** and **631 global jobs still pending**. Appearance windows do not establish political tenure or fill uncertain historical dates.

These six batch completions do not close full country casts, C06, S23 or CP1. Tonga retains 15 open requirements, including a fresh production browser journey; its earlier browser receipt stays historical. Unprepared reserves, additional historical windows and the separately claimed France/South Africa batch2 remain outside this deliverable.
