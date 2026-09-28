# CLAUDE-C03-REVIEW-01 — cartoon review workbench

Owner: Claude. State: **claimed** (27 September 2026; in progress, not complete). Parent: C03, **preparation only**.
Branch: `claude/c03-review-01`. Base: `e41aa18d` (current `codex/campaign-certification`). Claim commit: this record's first commit on
the branch. Touched paths: only the owned paths below. Next checkpoint: the standalone reviewer, asset check/export and a visibly reviewed six-to-eight-portrait sample, submitted `ready_for_review`.
Follow the [expanded working contract](CLAUDE-EXPANDED-NEXT.md).

## Build

Build a standalone reviewer for existing cartoon files, with country/person/era
filters, side-by-side small-card and full-size views, provenance, identity/appearance
bindings and visible missing/unknown/fictional labels. Use the approved country-selector
cartoon style and existing Tupou cartoon review as references. The user wants cartoons;
do not restart 3D character production or replace cartoons with photographs.

Add a read-only asset check/export listing file existence, dimensions, content hashes,
duplicate images bound to different people, appearance intervals, source/rights gaps
and review decisions. A duplicate is a review finding, not automatic proof of wrong
identity. Review six to eight existing portraits visibly at both sizes and record
specific fixes needed. This packet delivers working tooling and a reviewed sample,
not newly completed art jobs or historical acceptance.

## Owned paths

New `tools/ui/cartoon-review/`, `tools/ui/check_cartoon_review.cjs`,
`tools/avatars/cartoon_review.py`, `tools/avatars/test_cartoon_review.py`,
`docs/campaign-certification/C03/preparation/cartoon-review/`; this handoff.
Read existing manifests, art and `tools/ui/character-studio.html`; no shared UI,
manifest or image edits. Load repository assets locally without third-party runtime services.

## Acceptance

Test missing files, invalid intervals, duplicate bindings and unknown identities.
Demonstrate filtering, keyboard navigation and a narrow layout in the real browser;
retain screenshots and input hashes. Clearly distinguish automated findings from
actual visual approval. Export without changing production assets or a saved campaign.
