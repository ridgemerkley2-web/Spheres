# Independent read-only policy and authority review

Reviewed selected source21 (government.rs before debt correction: SHA256 3254020ea00ec763eb95dabe0c03b79e33b861d4144d6de7307627c03de4e51c). No policy or authority change was applied by this review. No new cohort was run.

## Franchise demand is not the executive's strategic preference

`government::franchise_demand` deliberately measures civilian exclusion, not executive consent. When the actual leader has a pillar tie and a table contains at least two civilian parties, every such party is excluded, so the measure equals authoritarianism. That is coherent as a demand estimate, with its existing warning that dormant shares are model proxies.

`ai_lever` currently uses that demand directly as an alternative to weak armed support. Consequently a newly seated Army executive with loyal forces and no competing party of its own can select a paid opening merely because the public is excluded. The player action is legal; selecting it as the executive's generic strategy is the questionable inference. Affordability, the existing draw and prior lever priorities mean this is not literally an automatic immediate handover.

A bounded correction can restrict only the franchise-only AI route to an actual live party-tied executive whose party appears in the opening table. The existing Albania positive fixture has the actual `al_ppsh` tie, so peaceful party-led negotiation with loyal services remains possible. Do not infer the party from a dormant cabinet, generic pillar affinity, a bloc match or the 1990 leader after an actual coup. Do not change public demand, command refusal, its bill, the player option, gradual opening rules, or the existing weak-armed AI route.

Required controls: retain Albania and same-bloc civilian opposition; stage an actual Army seizure and verify the strong junta does not select the franchise-only opening; preserve its ability to perform the same paid player command; let genuine weakness still reach the original legal AI route; preserve no-state/off/unknown identity behavior and exact read-only/RNG semantics. This is a conservative game strategy rule, not a claim that military rulers never negotiate or that A1 will pass. If a military executive is to compete through a civilian organization, that relationship should be explicit rather than invented by this heuristic.

## Seizure and non-force removal authority are different observations

The primary [V-Dem v16 codebook](https://www.v-dem.net/documents/70/codebook_v16.pdf), sections 3.4.2.4 and 3.4.3.4, measures the named group's ability to remove the executive without force. The imported weighted military assessment is already explicitly a lagged model proxy, not Army command structure, physical strength or a coup probability. The source audit and extraction are preserved separately in civil-military-authority-research.

`army_authority::army_seizure` sets only live campaign leverage to 1.0 after an actual Army-led seizure and leaves the immutable source untouched. Treating a military seizure as control over the office it installs is an understandable gameplay assumption. It is not a sourced inference that every such seizure establishes complete and continuing corporate military power to remove the new executive without force. The event does not distinguish a military council from a faction or personal ruler.

This is an explicit abstraction limit, not evidence for a particular replacement number, automatic post-ballot erosion or an accounting bug. Separating temporary control over the installed office from continuing military leverage would require a further defined lifecycle/state concept. Do not silently substitute either the historical assessment or zero after a counterfactual transition. No coefficient, success rule, threshold or expected A1 improvement is recommended by this review.
