# Arcade UI checks

Run the UI suites from the repository root:

```sh
node --test tools/ui/check_arcade.cjs tools/ui/check_cabinet.cjs tools/ui/check_operations_arcade.cjs tools/ui/check_discovery_arcade.cjs tools/ui/check_chronicle.cjs tools/ui/check_history_charts.cjs tools/ui/check_programs.cjs tools/ui/check_province_economy.cjs tools/ui/check_session.cjs
cargo test -p spheres-web --release --bin spheres-web
```

The JavaScript suites use Node's built-in test runner and VM to execute the
actual functions extracted from `spheres-web/ui/index.html`. They need no
browser, npm install, running server or campaign. Run a single file with
`node --test tools/ui/check_operations_arcade.cjs`, for example.

| Suite | Main coverage |
| --- | --- |
| `check_arcade.cjs` | Room precedence, explicit globe-view exceptions, Tab containment, hidden/inert controls, accessible action rows, nation pages and roving tabindex. |
| `check_cabinet.cjs` | Mixed budget drafts, close/reopen and discard, ministry curves, range-key behavior, one-day enactment, in-flight protection, refusal messages, treasury/interest units and local art. |
| `check_operations_arcade.cjs` | Production, manufacturing and freight renderers; real daily quantities; eligibility; materials/route disclosure; room/globe navigation; native control keys; focus restoration and existing command payloads. |
| `check_discovery_arcade.cjs` | Resource and research presentation, focus/inspector behavior, progressive disclosure and existing action wiring. |
| `check_chronicle.cjs` | Read-only history windows, born/ended nations, missing values, deltas, chart geometry, date cursor retention, first-day states and infographic asset wiring. |
| `check_history_charts.cjs` | Comparison-chart calendar spacing, daily labels, hover dates and retained-history event markers. |
| `check_programs.cjs` | Exact department-share conservation, immutable drafts, strict inputs, enacted-plan gates, safe labels, preview freshness and physical industry rendering. Run with `node --test tools/ui/check_programs.cjs`. |
| `check_province_economy.cjs` | Province and national GDP decomposition, real project contributions, honest modeled-data labels, and deferred rendering safety. |
| `check_session.cjs` | Campaign continuation/load, pending-turn preservation and retries, visible network failures, and initial globe positioning. |
| `check_competition.cjs` | Exchange view rendering, authoritative supply forecasts and saved AI review snapshots, escaping, all-size filters, served purchase quantities, repeat-safe receipts, campaign binding and stale-request invalidation. |

Run Exchange helper checks with `node --test tools/ui/check_competition.cjs`.
`check_competition_browser.cjs URL --disposable [screenshots]` exercises all four
views at desktop/tablet/mobile sizes, responsive World decision cards and live
Materials/Machinery supply cards, plus 137-country filtering, pure supplier
queries, no blind time advance and a real committed-but-lost order followed by
reload/retry. Like `check_session_browser.cjs`, it must only target a disposable
local server. Exclude both `*_browser.cjs` files from the serverless Node suite.

`check_small_modules_browser.cjs URL --disposable [screenshots]` uses a real Tonga
campaign to check server-priced workshop sizes, province selection, responsive
purchase cards, and exactly-once retry after a committed order loses its response.
It supplies no free construction or operating inputs. This also requires a
disposable local server; exclude **all** `*_browser.cjs` files from serverless runs.

The Rust web suite checks the served simulation contract as well as the UI's
entry points. Do not replace it with source checks or loosen its expectations
to accommodate a presentation change. The UI still owns no simulation rules:
amounts, dates, recipes, eligibility and allowed actions come from the server.
Annual budget values remain annual; daily settlements remain daily.

## Preview QA without touching user campaigns

Use a fresh preview process on an unused local port with its own disposable
campaign. Never reset, load over, advance or issue test commands in an existing
user campaign. Do not assume another browser tab is isolated: tabs using the
same server share that server's world. Do not stop unrelated preview processes.

The HTML, CSS and SVG are embedded at compile time. Rebuild after editing, then
start the rebuilt binary with `--port <unused-port> --no-open` and reload its
preview tab. If an existing executable is locked by a running test or server,
wait for your test or use a separate preview target directory; do not terminate
the user's process to unlock it.

Check both desktop and narrow layouts:

- Setup and Global Command: labels match the rendered elements, character art
  stays intact, actions and supporting text remain readable.
- Every room: Escape closes the expected layer, Tab stays within the active
  full-screen room, close returns focus, and native buttons, summaries and
  sliders keep their own keyboard behavior.
- Cabinet: select ministries, change a draft, revert, and enact once. Test
  network/refusal states only in the disposable campaign.
- Operations: project catalogue to province, line catalogue to plant, blocked
  and empty states, expanded details after a daily refresh, and Show on globe
  followed by Back to room and close/reopen.
- Resources and research: filters, inspectors, locked/unavailable actions,
  and visible daily units; verify room transitions do not leave hidden focus.
- Nation pages, province details, world dispatches and history: readable cards,
  no horizontal overflow hiding controls, and no console errors.
- Chronicle: first-day single points; shared date slider and keyboard controls;
  nation and time-window selection; all six metrics and real starting-point
  deltas; comparison view and return; desktop and narrow chart labels. History
  retains up to 3,000 snapshots and restarts when a save is loaded: do not
  fabricate earlier values or describe a retained window as the full campaign.

Restore temporary viewport changes afterward. Report which states were actually
checked; fixture-based renderer checks are not a substitute for visual QA.

For the save/load and network-recovery browser regression, start a separate
server **from an empty disposable directory** (its `save.json` will be replaced),
then run:

```sh
node tools/ui/check_session_browser.cjs http://127.0.0.1:7791 --disposable
```

This requires Playwright on `NODE_PATH`; set `PLAYWRIGHT_CHANNEL=msedge` when
using an installed Edge browser. It checks lost-response retry across reload,
queued edits, malformed batches, save/load, and desktop/phone room navigation.
Never point it at an existing user's campaign server.

The inherited-industry/Welfare browser check also requires a disposable server:

```sh
node tools/ui/check_starting_industry_browser.cjs http://127.0.0.1:7796 --disposable /path/to/screenshots
```

It starts a fresh France campaign, checks the five estimated industrial groups
at desktop/tablet/phone widths, verifies USA/Japan/Tonga/Bahrain account totals,
and opens Welfare using its backward-compatible `pensions` control key. It must
not be pointed at a campaign the user has already started reviewing.

The Materials operating pilot has its own disposable-server regression:

```sh
node tools/ui/check_materials_browser.cjs http://127.0.0.1:7797 --disposable /path/to/screenshots
```

It uses real command and quote endpoints with an unchanged program budget;
checks finite order placement, cancellation, no free inventory, once-only retry
after a lost response, and existing construction/trade navigation. It captures
desktop/tablet/mobile cards and true 390×844 order/action viewports, checking
control bounds against both the viewport and fixed-header scroll region. This
is intentionally a blocked-but-eligible fresh order, not a fixture granting
power, resources or cash. Rust integration tests verify actual paid operation.
