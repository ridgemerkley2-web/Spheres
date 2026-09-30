# Repository cleanup — 30 September 2026

The game now has one GitHub default/integration branch:
`codex/campaign-certification`. README introduces the game and directs work to
one current ROADMAP. CONTRIBUTING, AGENTS and CLAUDE use that same entry point.

## Verified changes

- Remote branches reduced from **145 to 5**: the game integration branch,
  `claude/c01-jp-31`, `claude/c01-ru-28`, `claude/c01-gaps-01-fix`, and `dashboard`.
- All **140 retired branch tips** preserved as exact GitHub archive tags. A
  dry run and atomic deletion used an explicit expected SHA for every branch;
  the final remote branch set and every recovery tag were independently reread.
- Root Markdown files reduced from **51 to 5**. **48 documents** moved into
  design, reference or archive folders; 16 historical snapshots retain the
  original Git blob bytes. README, ROADMAP and work assignments were rewritten
  around current accepted status and next work.
- Packaging still includes its former optional documents at their relocated
  paths and now includes the player guide. Workflow jobs ignore branch-deletion
  events; checks and acceptance criteria on real candidates remain unchanged.

## Verification

- 231 local documentation links checked against the staged repository tree:
  no unresolved paths in the changed current navigation/reference documents.
- All 48 relocation mappings and 16 exact archived snapshots verified.
- Workboard validation: 44 canonical markers and 39 bounded tasks pass.
- Eight workboard tests and 19 packaging tests pass, including real package
  extraction of the relocated player documentation.
- All three workflow YAML files parse; Git whitespace checks pass.
- No simulation, UI, game-data, campaign-evidence or planning-status files changed.
  No local branches, worktrees, user campaigns or uncommitted work were deleted.

Remote full CI for the cleanup is separate from these focused checks. The
existing A1 political-calibration failure remains an open development task;
this cleanup makes no new campaign or release qualification claim.

The inventory records the pre-cleanup branch tips. Later work on retained branches
may advance normally. Preserve the [branch inventory](branches-2026-09-30.json),
[document path map](../document-map.json) and original historical evidence.
