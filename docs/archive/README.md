# Archive and recovery

Current priorities live in [ROADMAP](../../ROADMAP.md). This directory retains
superseded plans, large historical journals and the branch-cleanup inventory.

## Branches

The [30 September inventory](branches-2026-09-30.json) records all 145 original
remote branches, exact commit IDs, ancestor comparisons and dispositions.
The verified retained set is one game integration branch, three unfinished
research/follow-up branches, and the separate dashboard publishing branch.
All 140 retired tips were verified on GitHub before their old branch names were
removed. See the [cleanup record](REPOSITORY_CLEANUP_2026-09-30.md).

Retired branch tips are preserved as `archive/2026-09-30/<original-branch>` tags.
Archiving does not mean every commit was merged: unique experiments and earlier
implementations remain recoverable without being silently added to the game.
Local worktrees and local branches are not removed by this cleanup.

To inspect or deliberately recover one recorded tip:

```sh
git fetch origin --tags
git show <commit-from-inventory>
git switch -c codex/recovery-name <commit-from-inventory>
```

Confirm its scope and integration status before publishing a recovered branch.

## Historical documents

The `2026-09-30/` snapshots retain the original text. Open their source view below
when following old relative links; those links belong to the original layout.
The [path map](../document-map.json) locates moved current system documents.

- [AI_INDUSTRIAL_SUPPLY_RESULTS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/AI_INDUSTRIAL_SUPPLY_RESULTS.md)
- [AI_WORKSTREAMS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/docs/AI_WORKSTREAMS.md)
- [AUDIT_FIXES_2026-09-03.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/AUDIT_FIXES_2026-09-03.md)
- [AUDIT_IMPLEMENTATION.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/AUDIT_IMPLEMENTATION.md)
- [BUGS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/BUGS.md)
- [CLAUDE.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/CLAUDE.md)
- [ECONOMIC_COMPETITION_RESULTS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/ECONOMIC_COMPETITION_RESULTS.md)
- [HEADLESS_BASELINE_2026-09-04.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/HEADLESS_BASELINE_2026-09-04.md)
- [HISTORICAL_INDUSTRY_1990_AUDIT.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/HISTORICAL_INDUSTRY_1990_AUDIT.md)
- [INVESTMENT_COMPLETION_AUDIT.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/INVESTMENT_COMPLETION_AUDIT.md)
- [MATERIALS_AI_INTEGRATION_RESULTS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/MATERIALS_AI_INTEGRATION_RESULTS.md)
- [MATERIALS_OPERATIONS_RESULTS.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/MATERIALS_OPERATIONS_RESULTS.md)
- [PLAN.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/PLAN.md)
- [README.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/README.md)
- [ROADMAP.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/ROADMAP.md)
- [TECH_REFERENCE_REPAIR.md — original source view](https://github.com/ridgemerkley2-web/Spheres/blob/ff01ee613808fae8ac97d4082b6d2df6d797a222/TECH_REFERENCE_REPAIR.md)

Campaign evidence, research originals, artwork and source history are retained.
Historical manifests and handoffs keep their exact original bytes and source
pins; use the pinned commit when a recorded path reflects the old layout.
