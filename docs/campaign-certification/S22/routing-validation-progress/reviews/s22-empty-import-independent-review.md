# S22 empty equipment-import precheck: independent source review

Reviewed implementation: `8f096011f1f1f03192e08993221e13e8245d7f71`.
Independent reviewer: Codex subagent `/root/s20_preflight`.
Author: Codex subagent `/root/review_source05`.

Scope: read-only review of the exact commit's production changes in `spheres-sim/src/military_ai.rs`, the included `military_empty_import_tests.rs` source, and the existing `companies_imports.rs` offer/source implementation. This was performed independently of the author. No findings were identified within that scope.

The existing public import list enumerates only foreign firms. Each non-ammunition offer's `ready_stock` comes directly from its matching equipment product's integer `stock`. The new private procurement guard scans all foreign equipment products, including products later rejected by native validation. Therefore invalid, cancelled or uncertified positive stock conservatively retains the complete old listing path. When the guard finds no positive foreign equipment stock, the unchanged immediate `!ammunition && ready_stock > 0` filter could not retain any native row.

The public listing, native quote/purchase checks and separate ammunition procurement are unchanged. The existing observed import-list stage remains present even when its result is empty. No route, price, stock, order or world mutation is introduced by the guard.

The test source retains the original complete-list path through a test-only selector. It compares filtered row order, serialized quotes and floating-point bits, public-list persistence, complete procurement outcomes, allowance bits, world bytes and headlines. Cases cover stocked, depleted, ammunition-only, own-firm, missing/invalid revision, cancelled and sanctioned offers. The optional actual-checkpoint 31-day oracle requires nonzero skips, compares every complete day, and preserves its input. Its loader distinguishes `spheres-campaign` from the integrated save format before unwrapping `world`.

This is source review only. The reviewer did not compile, run tests, launch a browser or benchmark this change. Coordinated native validation and any performance claims belong to root's retained execution evidence.
