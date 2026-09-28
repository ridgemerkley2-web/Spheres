# S19 complete — outcome-aware guidance

Closed 28 September 2026 UTC by Codex. Qualified runtime:
`c8a59bfd48cb49b7596332ab76f42aa8b2f03712`.

S19's original acceptance criteria now have implementation, regression checks and
ordinary native/browser campaign evidence. S20 and CP1 remain separate gates.

| Acceptance | Evidence |
|---|---|
| Actual budget, construction/output, paid company purchases and mission results | [First-hour integration](README.md), [retained construction payments](PAYMENT_RETENTION.md), [actual tank procurement](PROCUREMENT_DELIVERY.md), and the flight proof below. |
| Advice explains the cause, opens the real control and does not place orders | Retained first-hour navigation/purity and stale-response browser checks; [supplier-input recovery](INPUT_RECOVERY.md); actual Cabinet funding recovery during the flight proof. |
| Reading/skipping, stale replies, save/load and later campaigns do not create false completion | Retained full first-hour route plus actual later construction, procurement and flight outcomes across named save/load, Continue and new-campaign isolation. |

## Final ordinary flight route

The route resumes the previously recorded delivered-tank France campaign. Through
visible controls it saves a light-attack design, commissions and funds company
development, waits for real shared-plant tooling/manufacturing, buys one finished
aircraft, and receives it in December 1998. It forms a squadron and bases it at the
campaign's already funded and completed Paris airbase.

The first operations run correctly cannot buy mission stores: Defense maintenance
authority is exhausted after protecting current fleet upkeep. It is retained as a
failure. Recovery uses that run's unmodified **1 May 1999** monthly autosave,
opens the real Cabinet programme, and reviews/enacts **Maintenance & supply from
20% to 40% of the unchanged Defense envelope**. This changes allocation, not
treasury or granted equipment. Native daily authority becomes about **$54.45m**;
actual upkeep is paid, and the authorized automatic policy buys 24 finite company
stores. They enter national inventory only after paid transit.

The player then declares a nearby eligible Belgium conflict through the national
dossier, reviews a 25% force allocation and confirms a native Strike target order.
On **10 May 1999**, one aircraft launches, settles approximately **0.0959 sortie
equivalents**, uses **0.1918 compatible unguided stores**, and suffers zero aircraft
loss. The target has **no opposing formation contact**, so applied power is zero.
This qualifies a supported flown result; it does not claim combat damage.

Two funded post-flight service days restore physical readiness. At the final
**13 May 1999** checkpoint, all five aviation milestones are done. Native guidance
and the rendered accepted advisor model agree. Those results, plus the existing
construction and paid procurement milestones, survive named save/load and page
reload/Continue. A freshly started France campaign has no inherited outcomes;
loading the qualified save restores the original campaign results.

The desktop flight report, fresh-campaign reset and 390px guidance screenshots
were inspected. The narrow panel has no horizontal overflow. **81 guidance tests
pass**, and the successful browser route records **zero page errors**. This is
bounded S19 qualification; no new full-workspace test pass is claimed.

## Reproduction and provenance

[Manifest, captured executed helpers, raw results/logs, screenshots and real saves](flight-closeout-evidence/manifest.json).
Large evidence JSON and saves use lossless gzip with both compressed and original
hashes. The chain is the previously archived February 1997 tank-delivered save →
actual aircraft-development save → actual December 1998 aircraft delivery →
actual May 1999 maintenance-shortfall autosave → qualified flight save. Every
resumed input matches its prior recorded output hash. All four runs use the same
native binary and record their served asset identities.

The first development attempt's incorrect test-driver Back selector is retained
as a failed attempt; its already-created native development save is unchanged.
The corrected driver resumes it. The failed supply-budget attempt is likewise
retained separately from successful funded recovery. No inventory, work, funds,
receipt or campaign outcome is injected.

Use `tools/ui/ci-supplier-input-recovery.cjs`, a binary matching the stated runtime,
`SPHERES_EXPECTED_REVISION` set to its full revision, and
`SPHERES_GUIDANCE_FLIGHT=1`. For each archived input set
`SPHERES_SUPPLIER_CHECKPOINT` and its **uncompressed**
`SPHERES_SUPPLIER_CHECKPOINT_SHA256`. `SPHERES_FLIGHT_RESUME=production` resumes
development; `SPHERES_FLIGHT_RESUME=operations` resumes actual delivered aircraft.
`SPHERES_SUPPLY_OUTPUT` selects an isolated evidence directory. Windows validation
uses Playwright with `SPHERES_BROWSER_CHANNEL=msedge`. The executed copies in each
run define its exact historical behavior; later helper changes are not claimed to
have run retroactively.

S19 is complete. The next session is S20's combined map, province and accessible
room journey. Broader military balancing, worldwide content and CP1 retain their
own acceptance requirements.
