# Government room

The Government room rebuild presents the existing political simulation as a readable briefing and a reviewable set of decisions. It introduces no simulation rule, political activation, fiscal account, command type, or save migration. Prices, eligibility, immediate effects, elections, political standing and institutional loyalty still come from the existing simulation.

## Files and routes

| Responsibility | Location |
| --- | --- |
| Existing government board and action payloads | `spheres-web/src/main.rs`, `government_json` |
| Briefing fields and immediate decision review | `spheres-web/src/government_view.rs` |
| Presentation renderer | `spheres-web/ui/government-ui.js` |
| Styles, scoped to `#govScreen` | `spheres-web/ui/government-ui.css` |
| Room navigation, refresh, review lifecycle and command submission | Government section of `spheres-web/ui/index.html` |
| New decorative council art and generation record | `spheres-web/ui/government-art/` |

`GET /api/government?nation=Poland` returns the existing board, enriched with a `briefing` object and stable action keys/categories. The board can show a foreign government, but only the player government's actions may be reviewed or executed. Missing nation selection uses the current player where supported by the existing route.

`POST /api/government/preview` accepts an optional `nation` and an exact `command` from that government's current action list. For example:

```json
{"nation":"Iraq","command":{"kind":"secure_pillar","pillar":"army"}}
```

The response carries `valid`, `reason`, `title`, `kind`, `command`, `price_pc`, `money_cost_bn`, `description`, `effects`, `changes`, `warnings`, and the active `session_id`. Each change has a key, label, before value, after value and explanation. Money is returned in billions of 1990 dollars; display formatting converts it where appropriate.

## What the player sees

- **Overview:** country and leadership context, the current cabinet or ruling institution, live standing and stability, majority or institutional concerns, and links to budgets, public finances, construction and diplomacy.
- **Governing decisions:** searchable decisions grouped by their purpose. Available and all-decision filters retain the server's action indices. Refused choices show their actual reason. Reviewing a decision opens immediate before/after effects and a separate confirmation control.
- **Parliament and parties / Regime and institutions:** cabinet seats, public support, coalition strain and upkeep, election dates, or the regime's institutions and loyalty. Support and seat share are distinct percentages. An inactive party table is identified as a dormant record in a regime.
- **Takeover watch:** the existing ideological route gates, reasons and individual conditions. Gauge progress describes a threshold; it is neither a probability nor a forecast.

The original nation picker, open/close entry points and government shortcuts remain available. The host handles keyboard tab navigation, focus restoration, decision-search focus and open disclosures across a refresh. Foreign government views retain information while withholding order controls and domestic department links.

## Decision authority and immediate effects

The review endpoint accepts only the player's living government with a current government record. The complete command object must equal an action supplied by `government_json`; arbitrary commands, extra fields and a different target nation are refused. The endpoint parses that existing command and applies it to a disposable clone of the world. It does not advance a day or retain any change to the live world.

The review reports the actual immediate deduction and result, including simulation clamps. It compares political capital, public money, stability, authoritarianism, separatism, government accountability, recorded officeholder, cabinet composition, seats, coalition strain and upkeep, the standing modifier, election timing, regime pressure, party support/status, institutional loyalty and affected diplomatic relations. Officeholder changes use the existing recorded person or institutional description; they do not invent successors. When the ideological lens is enabled, it also compares ruling colour, domestic bloc shares, discontent and aggregate foreign backing. Unchanged formatted rows are omitted.

| Existing command | What the review helps assess |
| --- | --- |
| `invite_to_government` | Added cabinet partner and seats, resulting strain, political upkeep and standing consequences |
| `expel_from_government` | Remaining cabinet and majority, public support/stability effects and actual PC debit |
| `call_election` | The result of holding an election **now**, using current support, including the resulting cabinet and next election |
| `secure_pillar` | Immediate loyalty/pressure changes, political capital and the one-time public payment |
| `stratagem` with `security_crackdown` or `liberalisation` | Immediate political conditions and diplomatic effects; applicable existing backing effects |
| `suspend_constitution` | Change in accountability, retained dormant cabinet, movements and immediate stability/diplomatic effects |
| `declare_programme` | Change of ruling bloc, institutional response, movements and affected relations |
| `convene_round_table` | Transition to electoral accountability, party participation and the first election date |
| `ban_party` / `legalize_party` | Legal participation and resulting seats, kept support, authoritarianism and relations |

All prices and refusals remain authoritative in the simulation. In particular, a low-capital expulsion may remain legal: its review reports the capital actually held and spent, rather than disabling it because its nominal price is higher. Coalition upkeep and the government's contribution to the standing target are ongoing consequences, not immediate PC awards or deductions.

Securing an institution currently makes the existing one-time payment of 0.008 times GDP. The review measures the actual change in treasury less debt for a country with open books, or in debt relative to GDP for a legacy country without those accounts. The operation does not create a new ministry budget, recurring payment, or financial order system. Loyalty continues to move toward its existing policy-dependent target afterward.

Calling an election does not preview a future scheduled election. Likewise, if liberalisation crosses the threshold into electoral government, the review distinguishes the immediate threshold change from the election scheduling performed by a later government tick.

## Confirmation, refresh and receipts

The browser associates a review with the exact board object, nation, command and current campaign state. It discards stale replies after a different country, refreshed board, changed world, closed room or replaced campaign. A valid review's returned command must match the requested command, and its session must match the active campaign.

Confirmation sends the reviewed command through the existing `/api/command` receipt channel. It does not apply the preview's result to the live world. The ordinary simulation still validates the order when executing it. Busy, pending-turn and pending-command protections use the shared campaign controls; there is no second Government receipt ledger. The screen adopts the returned campaign state and refreshes its figures after the result.

The original `govAct` entry point remains compatible with existing callers and retains its confirmation path. The rebuilt decision controls use the explicit preview flow.

## Political rules and information boundaries

The board's `on` field means `rules.ideology_blocs`. It is not an on/off switch for the ordinary government simulation. When it is false, ordinary cabinets, parliamentary seats, elections and regime institutions still exist and remain visible. Ideological movement/backing readings and takeover-specific information are gated separately. This rebuild must not enable that rule to obtain a richer page.

The separate `ideology_takeover` rule controls ideological takeover routes. A closed ideological watch does not disable ordinary scheduled elections, cabinet failures or regime coups. Route explanations must preserve this distinction; displayed conditions do not activate a route.

Domestic support and foreign backing remain separate quantities. The server names a covert sponsor only once the existing exposure record permits it; an unexposed entry carries no sponsor name. Review comparisons expose aggregate backing changes only. The renderer escapes server text and retains the undisclosed-sponsor label, rather than deriving or guessing an identity.

## Art and accessibility

The new decorative image is served at `/art/government/council-v1.png` with an immutable cache policy. It is embedded in the web binary, so the Government room needs no external image host. The original is a 1536 by 1024 PNG, 2,491,538 bytes, copied unchanged from the built-in image generator output.

The illustration depicts a fictional council chamber: dark wood and paper, warm brass/amber architecture on the right, and quiet navy/teal shadow beneath live text on the left. People are distant silhouettes. It contains no identifiable real nation, leader, flag or readable text. The image is decorative (`alt=""`, `aria-hidden="true"`); headings, values and controls remain real HTML.

The exact prompt, built-in generation method, original output path, visual review and SHA-256 are recorded in [the artwork README](../spheres-web/ui/government-art/README.md). This is atmosphere, not a historical source or a political data visualization. Future revisions should use a new versioned filename and preserve provenance instead of replacing a cache-immutable asset in place.

## Acceptance — 7 September 2026

The final non-browser UI run passes **1,031 tests across 52 files**, including 38 renderer checks and seven additional host interaction checks. The full release web suite passes **263 tests, with three existing ignored tests**. A final focused release run passes all ten government checks after the host polish. Simulation source and save formats did not change.

Browser acceptance covered Iraq and Poland in a separate campaign, plus the preserved active United States campaign. A reviewed Iraqi institutional payment showed **14 PC / $480 million**, then changed standing **39.276 → 25.276**, debt/GDP **110.0% → 110.8%**, and Republican Guard loyalty **65% → 85%** once. A Polish coalition review showed **24.28 PC**, cabinet seats **60% → 82%**, and the resulting strain and monthly upkeep without applying an order.

All eleven Polish decisions were disabled while inspecting Poland as a foreign government. Phone-width inspection measured **390px page / 390px viewport**, with scroll contained inside the government room and its tab strip. The budget link opened the actual budget tab and closed Government. Keyboard Home selected Overview. Refreshing invalidated the open review, retained expanded watch details and restored focus to Refresh. Party and decision searches retained independent queries. Browser error and warning logs were empty.

The active playset was saved, backed up and reloaded at **United States, 11 February 1990**, with **42 history points and 16 dispatches**. Deep comparison confirmed unchanged world, history, log, history epoch, player and saved date. Final executable SHA-256: `6343C26AAD9786D8C8BC1A2A468B1FD06B62286FA5E5C3F7261D15534D60146A`.

## Verification scope

The focused backend tests in `government_view.rs` exercise save-byte purity, exact own-government action allowlisting, malformed or foreign requests, real cash/debt and PC effects, ordinary rules when the ideological lens is off, exact immediate elections and officeholder succession, and low-capital expulsion.

`node --test tools/ui/check_government.cjs` passes 38 tests. These exercise renderer purity, live country/leadership/art, stable action indices through filtering and search, foreign restrictions, ideology-off parliament and regime continuity, attention treatments, sponsor privacy, authoritative gauges and reviewed costs, loading/error/busy states, escaping and focus hooks. The normal `tools/ui/run-unit.cjs` runner discovers this file automatically.

The rebuilt release server passed 10 read-only route smoke checks on 2026-09-07: HTML/module/style delivery, the exact embedded PNG hash and immutable cache header, a domestic Iraqi regime board, a foreign Polish electoral board, a valid institutional-payment review, refused foreign and extra-field reviews, and an invalid-nation response. The real Iraqi review returned 14 PC and $0.48 billion with its actual changed fields. These checks used a separate QA campaign and issued no command, load, new-game or save request.

Browser acceptance complements those isolated checks: inspect an electoral country and a regime, a changed campaign or refreshed board during review, real confirmation and retry behavior, keyboard focus, department navigation, and narrow-screen overflow. Node markup checks alone do not establish visual layout or browser event behavior.

No save or political rule should change merely because the room, its overview, watch or decision review is opened. A successful confirm must go through the existing command receipts and change the live campaign once.
