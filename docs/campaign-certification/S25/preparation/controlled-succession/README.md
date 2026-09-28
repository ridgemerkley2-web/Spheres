# Controlled USSR-to-Russia continuity

The controlled engineering proof passes. This is separate from the ordinary
24-cell campaign matrix and does not certify organic history, S25, G5 or CP1.
CODEX-S25-SUCCESSION-01 is complete following the verified worldwide startup checkpoint.

The native test executable was compiled from clean commit
`ae8084e853a8eb01ef4d834017af3e94c8e2cc82`, SHA-256
`e90b8f0cb34603f5290c8a19b106a0d6dc31e3ea7ad293eb0135199fe1fd59e2`.
Both source harnesses, the request, invocation, native output, twenty complete
compressed campaign archives and their fingerprints are retained in
[passing-02](evidence/passing-02/receipt.json). Only each archive envelope's
terminal wall-clock `saved_unix` is excluded from byte comparison.

## What actually ran

Two fresh USSR campaigns use seed 1990 and ordinary play rules. The fixture
sets only USSR stability to zero and separatism to one, as explicitly listed
in the native report. Both legally choose the Prosperity aim. The uninterrupted
leg is never loaded; the other leg has ten scheduled save/reloads:

- Immediately before the authored collapse.
- At the real paused, ceased-government state.
- After choosing the served Russia continuation.
- After each of seven ordinary successor days, ending 9 January 1990.

All twenty before/after comparisons match, with forty native invariant checks
and sixteen daily invariant checks. Player, property, government, economy,
forces, ammunition, retained history and journey are included in complete
archive equality. Russia must actually exist and receive territory; observing
cannot satisfy this controlled case. Paused advancement and an unrelated
France successor are refused. A repeated continuation request cannot create a
second transition. The old USSR aim is archived as government-ended, without a
reward or reassignment to Russia.

The [separate retained-evidence verification](evidence/independent-verification.json)
decompresses and rehashes all twenty archives and checks their actual inner
world dates, identities, ownership, government and journey. It executes no new
simulation and awards no qualification.

## Repairs and retained failures

The first native run passed, but its Python verifier incorrectly required a
serialized `day` field on 1 January. Native `World` deliberately omits the
first day and defaults it to one on load. Commit `320bb9d1` makes that same
default explicit; missing later days and explicit invalid days still fail.
The second attempt reran the actual native case with that frozen corrected
verifier and passed. The first attempt remains failed in its
[original receipt](evidence/failed-01-summary/receipt.json); its complete twenty
archives remain in the local bundle identified by [manifest.json](manifest.json).
The failed attempt has not been relabeled or replaced.

Three focused native regressions and seventeen synthetic verifier tests pass.
An earlier sparse-worktree Cargo invocation failed before compilation because
the workspace's CLI manifest was absent; its log is retained separately from
the successful build in the full integration checkout.

To verify the portable passing bundle from the repository root:

```text
python -B -X utf8 tools/campaign/controlled_succession.py --verify docs/campaign-certification/S25/preparation/controlled-succession/evidence/passing-02
```

The existing A1 political concentration failure and the canonical S23/S24/S25
prerequisites are unchanged.
