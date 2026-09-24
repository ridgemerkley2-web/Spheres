# A1 independent read-only audit — iteration 12

Status: A1 remains failed. This is diagnosis, not an alternate acceptance rule or a new simulation run. Parent is measuring subsequent candidates separately.

## Exact statistic and small samples

The unchanged gate requires the median electoral-coup count in 4–14 AND the median of each seed's three-largest-country coup share strictly below 0.5. Annulments and regime coups are separate routes. Zero-event seeds contribute a share of zero.

For positive counts 1–6, the concentration condition is impossible even if every event occurs in a different country. At 7 or 8 events it requires every event in a different country. At 9 or 10, it permits at most one duplicated country (one pair), with all remaining events separate. This creates a count-dependent constraint within the accepted count band; it is not a logical contradiction for the joint median test. Zero-event treatment could bias a different sample downward, but iteration12 contains no zero-event seeds.

The source comment explicitly records the numerical share threshold as a prior session's interpretation of a qualitative objective, not a measured historical number. Its historical examples mix ordinary coups, an annulment and presidential dismissals, while several example countries are outside the current roster. This is a construct-validity issue for future explicit design review; it does not authorize weakening or marking the current assertion passed.

## Existing development sample

Seeds 0–199, 252 months, iteration12 compiled provenance: 1,574 electoral coups; count min5 / median8 / max11. Median per-seed top3 share 0.60. Only6/200 individual shares are below0.5,22 equal0.5;18 seeds have 5–6 coups and128 have7–8.

Country totals: Myanmar306, SaoTome276, Comoros271, Guatemala240, Guyana235, Suriname200, Mozambique36, Cambodia6, Chad4. Pooled top3 is853/1574=54.193%; top6 is1528/1574=97.078%. Country counts exceeding200 imply at least328 repeated events (20.84% of events); exact repetitions require event-level records. The poor result is therefore not merely an estimator artifact. Removing repeats alone could leave six one-time coups, which still gives a share of0.5; adding arbitrary coups elsewhere to satisfy this arithmetic would be unacceptable.

For illustration only, if eight events were independent uniform draws over K equally likely countries, the pass chance equals the chance of eight distinct draws: K9=.00843, K15=.10124, K30=.35969, K60=.61421. The game has unequal risks and dependent trajectories; this is a mathematical explanation, not an empirical null model.

## Current mechanism scan

No additional proven iteration12 funding or transition defect emerged from this bounded read. The formerly incorrect raw funding inverse and once-yearly appropriation timing are already repaired: the AI inverts the actual confidence/war-adjusted target, respects fiscal affordability and useful resources, and reviews at month end when loyalty crosses its operational margin. Electoral coups recheck live disloyalty/discontent, reset pressure on actual takeover, and cannot remove a pending first-ballot interim. Prior candidate11 evidence cannot establish a current12 failure. No current12 detailed fiscal/event trace exists in the reviewed artifact set; aggregate counts alone cannot prove an insolvency or handover defect.

The institution audit is now systematic for the opening137-country roster:71 original Army specifications,57 explicitly sourced overlays,9 explicit excluded/uncertain cases. Do not infer more armies from positive personnel figures or add only countries desired for the outcome. Army material input quality still varies: almost all sourced headcounts have thousand-person granularity; some spending shares are approximations, currencies/vintages differ, and total armed services including qualifying paramilitaries are not Army salaries. The prior cost audit does not identify a defensible empirical numerical replacement for the operating basket.

A remaining initialization caveat merits explicit diagnostics: inherited historical electoral governments start elected=false and unrestricted_mandate=false. Until a simulated ballot, the mandate helper returns0 even if the seeded governing coalition has meaningful public support; confidence begins after six months of a valid party record. Candidate11 Suriname seed1 was deposed before its first modeled ballot, but this is older evidence, not proof for12. Some opening party tables mix dates or lack a proven free prior vote, so blanket initial legitimacy would also fabricate evidence. A sourced prior-election mandate record would be a distinct principled improvement if this channel materially drives current failures.

## Predeclared confidence sensitivity grid

The .65/.90/1.20 grid is defensible as a bounded game-assumption sensitivity experiment with unchanged acceptance rules, controls and held-out seeds. It is not an empirical estimate from the qualitative military-disposition literature. Relative to .65, the alternatives increase the political penalty by38.46% and84.62% for the same state.

Let q=autonomy*sustained_crisis*(1-public_mandate). With no war and fully funded material resources, target loyalty is0.85-weight*q. Funding can attain the AI's0.40 target only for q<=0.45/weight: .6923/.5000/.3750. Even full material provision leaves loyalty below the coup threshold0.35 when q>0.50/weight: .7692/.5556/.4167. With war exhaustion E, subtract0.45E from each numerator. These saturation crossings are material behavioral changes, not mere scaling of event counts.

The grid may broaden genuinely autonomous crisis cases but can also intensify the same six countries or amplify the initial zero-mandate assumption. Preserve the full-resource cap and inspect appropriation, actual/effective loyalty, current versus completed-ballot status, mandate, autonomy, record, fiscal cash, and first/repeat events. A large funded increase with no possible target recovery is not evidence of a successful policy. Keep strong public support, quiet, newly seated and strong civilian-control checks fixed. Select no candidate on A1 alone; other routes and the predeclared holdout remain necessary. If all fail, retain the failed gate and investigate causality rather than extending the grid to seek a passing number.

## Evidence

- iteration-12-a1-statistical-audit.json: exact arithmetic and parsed rows; shares reconstructed uniquely from integer event counts and two-decimal diagnostic output.
- iteration-12-development-n200-provenance.json and iteration-12-source-input-manifest.json: compiled sample provenance.
- army-opening-cost-audit.json/.md: earlier input-quality audit.
- iteration-11-army-lifecycle.log: explicitly older contextual observations only.
- confidence-calibration-plan.json: parent's predeclared experiment, unmodified.

Reviewed source hashes (shared current files, may include later unrelated repairs; simulation results remain pinned to iteration12 manifest):
- spheres-sim\src\government.rs: 9ce2ce8dd9618fdea15bcbb40198edb7a94be401043b18b13f264d7f8540884d
- spheres-sim\tests\bloc_census.rs: 4d0b0c242d12933b1bd23e11cd2185c97656315820e2f9c72a7c671f5eb9bf56
