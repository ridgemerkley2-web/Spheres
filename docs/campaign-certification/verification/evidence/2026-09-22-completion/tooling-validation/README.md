# Python, leadership provenance and geometry validation

These are local working-tree checks from 22 September 2026, not a claim that a
final commit or hosted CI passed. Native tests, political calibration and browser
journeys have separate receipts. The independent tooling batch preceded the final
snap-election grace fix; the affected leadership checks were repeated afterward.

## Current metadata: checkpoint26

The latest provenance receipt is `checkpoint26/leadership-provenance-result.json`,
after the fresh-only Army enrollment and fresh-browser initialization repair.
All seven commands pass, including 8 production self-tests and 7 census tests.
Government source is
`0e2dbdefe5ed1909804b81d43b736c94315be647d615bda85e62e1fe78a2b73f`;
the frozen server source is
`f954638dbdc4db92d838c8aeaa0bd471a029b9ebe662256f327d685ac6876709`.
All protected source hashes and modification times remained unchanged. The wrapper
refuses noncanonical source bytes instead of rewriting a native input.

Exactly four provenance fields changed: the government hash in production and
census, the census byte count, and its downstream production hash. The five other
generated reports remain byte-identical. Production is
`f98cbae966b91bbcccc3ce6a76f0d15f52a59fd11ce53d0063b8effd55acded8`;
census is `3bd35544f2cc363e9a45ef317169b461d3d98a9ef3841a7e8fb5f3f127e87244`.
Targeted Node leadership/research checks pass **89/89 with zero skips**, preserving
source and metadata bytes and timestamps. Historical coverage, artwork and C01
completion status are unchanged. These passes establish metadata coherence;
the new native build, archived-save regression and browser behavior require their
own receipts. **A1 remains unresolved.**

The separate `checkpoint24-python/` archive retains the full **252-test Python
pass with zero skips** on clean `b51b7333`: 162 avatar, 25 terrain, 3 UI-export,
47 self-tests, 7 military-authority and 8 resource-coverage tests. Its eleven raw
logs and exact commands are preserved, with all protected source/generated hashes
and modification times unchanged. That result belongs to checkpoint24; it is not
renamed as a checkpoint26 run. The latest seven-command pass above rechecks the
changed government parsing/provenance inputs. Repeated tests are not added to a
unique-test total.

## Earlier provenance history

Earlier source and generated records were restored to **iteration21** after candidate
22 was rejected: its joint outcome criteria failed, including A1 concentration and
A2. Its measurements and prospective review remain preserved. That historical restoration
receipt is `iteration22-rejected-restored21/`: the parent reran all seven commands
successfully, and an independent read verified all seven generated outputs exactly
match their iteration21 bytes. Government source is
`3254020ea00ec763eb95dabe0c03b79e33b861d4144d6de7307627c03de4e51c`.
The wrapper did not write that source. **A1 remains outstanding**; neither this
restoration nor the earlier successful tooling checks certify the campaign.
The rejection receipt's pending-provenance wording records its earlier timestamp;
the accompanying successful result and exact-byte comparison establish completion.
No native build was run by this reviewer, and the previously built iteration22
binaries must not be mistaken for a validation of the restored source.

The rejected candidate22 provenance pass is in `iteration22/`, after the single predeclared
confidence-response change and three mechanical regressions. All seven commands
passed again (eight production and seven census tests), with only four provenance
fields changed and five other outputs unchanged. Government SHA-256 is
`ae02aa1e59abdf3409c299fbfca3d2558049142eb97e1fe79fa173c2287a7591`;
its exact source bytes and modification time stayed unchanged during this pass.
The independent fixture review covers the actual pressure route, source-zero and
counterfactual campaign separation, spending-review eligibility, and unchanged
threshold/clamp boundaries. It does not claim native execution or calibration
acceptance; those results belong in the separate political measurement record.

The preceding provenance pass is in `iteration21/`, after the successor-election
calendar fix and explicit-authority guard. All seven build/check commands passed
again (eight production and seven census tests); only four provenance fields
changed, with five other generated outputs byte-identical. The government source
SHA-256 is `3254020ea00ec763eb95dabe0c03b79e33b861d4144d6de7307627c03de4e51c`.
The no-op writer guard retained both its bytes and its exact modification time.
The final independent review verifies the named court/Army authority exclusion;
the initial review and its missed case are preserved alongside the superseding
review. Native execution and political-gate outcomes are not claimed here.

The preceding provenance pass is in `iteration20-encoding-corrected/`. It supersedes
`iteration20/` after a separately preserved UTF-8 decoding repair in government
source. Both receipts remain available; the earlier pass does not describe the
corrected source bytes. The corrected source SHA-256 is
`6dc3b30e5fa4c967d5122d8d0b07e49650e29f5179e09a8ec4f80c3bf9a8ac98`.
All seven build/check commands passed again, including eight production and seven
census tests. Exactly the same four provenance fields changed; five other generated
outputs stayed byte-identical. A read-only scan of 42 changed or new text source and
catalogue files found no analogous encoding signatures in the corrected snapshot.

The earlier military-authority review remains in `iteration20/`.
The corrected pass kept source bytes unchanged but its external wrapper rewrote
identical LF government bytes, advancing mtime and causing an unnecessary Cargo
rebuild. That wrapper now writes only when bytes differ; three isolated guards
pass. The original executed wrapper is preserved outside the repository, and no
metadata or runtime input was regenerated while fixing the wrapper.
The additional authority importer check and all seven importer tests passed. An
independent read-only review reproduced the exact 158-row source fixture from
the pinned official RData, checked the dated mappings and data-license notice,
and found no remaining issue after two ambiguous-input guards were added.
The authority field is a gameplay proxy for executive-removal leverage, not an
empirical coup probability. This receipt makes no political-gate outcome claim.

The independent batch passed all 22 commands: 237 Python unit/self-tests, 11
industry assertions, 63 resource-1990 assertions, current avatar/model/art records,
191 distinct component pairs, and the 44-marker workboard. The strict art gate
graded 245 configurations with zero budget overruns, missed required detail floors
or export overruns. Its source/input manifest did not change during the batch.
Repeated targeted checks are not added to the unique-test total.

After the runtime freeze, leadership production build/check/eight self-tests,
campaign census build/check/seven unit tests, and research-index validation passed.
All 162 avatar tests, 89 JavaScript leadership tests and the workboard check also
passed on the refreshed files. Exactly four provenance fields changed across
`leadership_production_2035.json` and `C01/census.json`: the government source hash
in each, its byte length in the census, and the production-file hash. The other
five census/research outputs are byte-identical. No historical term, artwork,
coverage count or completion claim changed; C01 remains incomplete.

Government and generated-record bytes equal their Git-normalized blobs. A broader
six-file normalization assertion additionally found one trailing CRLF in each new
opening-mandate source/data file; that assertion failure is preserved in the
receipt. They are runtime inputs, not leadership-catalogue inputs. Their raw and
Git-normalized identities are both recorded; this must not be presented as an
exact-byte match. No reviewer rewrote those inputs during a native compile.
The parent subsequently normalized those two files before iteration 20; the
original failed normalization assertion remains evidence of the earlier state.

## Raw-source audit limits

These older scripts are documented post-generation audits, not commands in the
current GitHub workflow. Their coverage is useful but distinct from runtime tests.
The initial missing-cache failures remain archived alongside subsequent results.

| Audit | Current outcome and remaining scope |
| --- | --- |
| Resource ground truth, `check.py --fast` | 95 assertions pass after exact pinned MRDS/minfac downloads. Nine existing data-limit warnings remain; byte-identical regeneration is skipped. |
| Resource rules, `check_resources.py --fast` | 168 assertions pass against a temporarily staged **current official** WEP archive; historical ZIP-pin identity is unresolved, and regeneration is skipped. |
| Broad terrain, `tools/terrain/check.py` | Zero failures using the existing pinned lake source. Three warnings include the existing Gobi classification caveat and two skipped coarse-raster regeneration checks (relief/occlusion and cover). Coast/lake regeneration and shipped decoding/geometry assertions run. |
| Detailed lake surfaces | Strict read-only regeneration passes for all six records using the pinned HydroLAKES and Natural Earth inputs. |
| Population, `--fast` | Passes with two explicit skips: rasterized area and byte-identical regeneration. The full command stops on absent decoded GHS-POP input; it does not establish a runtime population failure. |
| Full offline resource/population reconstruction | Not completed. Additional resource sources and the population source/cache remain absent. The population README specifies a 443 MB source download and 3.7 GB decoded cache. |

The [official WEP download](https://www.sciencebase.gov/catalog/file/get/60ad2fa1d34e4043c850ed98)
builds a new ZIP wrapper: two downloads have different archive hashes but all eight
member payloads are identical. Both ZIPs are the same length as the historical
pin. Because that older receipt stored no member hashes, this establishes current
payload stability, not historical payload equality. The shipped pin is unchanged.
Exact MRDS/minfac hashes and official download URLs are in
`small-source-retrieval.json`; no alternate dataset was substituted.

During temporary source staging, all 1,626 tracked shipped assets/data files were
unchanged. Temporary lake and WEP paths were removed. The two exact-pinned USGS
archives remain only as ignored local caches. No full generators or native builds
were run by this verification task.

The JSON receipts record exact commands, durations, return codes and log hashes.
`artifact-hashes.json` covers the installed evidence bytes. Raw log line endings
are preserved; generated JSON and this README use LF.

## Selected fiscal repair source: iteration23 final provenance

The seven commands in `iteration23-final/leadership-provenance-result.json` pass
against government source `4453a90a9949090337e3bafe4b211ccedc0755efdf50022888acb2f748fb7cff`,
including the independent annual-plan Army funding guard and measured default
fingerprint updates. Production self-tests pass 8/8 and census tests pass 7/7.
Every protected input retained both its bytes and modification time. The wrapper
refuses non-LF runtime input instead of rewriting it during a native build.

Exactly four provenance fields changed: the government hash in production and
census, its census byte count, and the downstream production hash. The five other
generated census/research outputs are byte-identical. The production record is
`74f24fc9e9a612e3e6ef7fcc64372486d70d558b5b5838f2d5053e8cf8a24438`; census is
`e54f7e230608cda0d93464b0cf5bd571d7707ddc8cdcf2b327b8a2debaf01d97`.

Historical coverage remains 624 simulated party rows, 590 known people, 393 terms,
and 58 validated cartoon assets. No country has completed all-party history;
research indexing covers nine country packets with zero exhaustive censuses.
C01 remains incomplete. No historical claim, artwork or runtime input changed
in this regeneration. These tooling results do not establish political A1
acceptance or a final native/hosted-CI pass. Earlier receipts remain intact.
