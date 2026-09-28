# Industry generation geometry scope

Source inspection by Codex subagent `/root/s20_preflight`, based on `cc66b1a2`
(runtime `d50f7ee1`). No benchmarks, builds or native test runs were performed
by this agent. Performance improvement remains unmeasured.

The completed first qualification pair had 17 passing cells and one failed
late native cell. This proposal changes no measurement limits or fixtures.
Parent-owned subsequent diagnosis `s22-subsystem-2035-pair01-failure-01`
reported industry mean 21.295 ms, p95 30.656 ms; these are attribution evidence,
not an isolated measurement of this proposed change.

Source inspection found that each operating line rereads owned/uncontested
generation layout and national inherited power headroom for fuel/fee quotes,
GDP factory/power accounting, and contractor work. The actual saved 2035
checkpoint's last receipts suggest about 195 direct energy reads across 58
countries (plus successful GDP power reads). This is an offline estimate
from stored statuses/output and energy assignments, not instrumented calls.

The implementation owns a temporary `OperatingEnergy` only inside
`industry::tick_day_impl`. It lazily stores sorted generation geometry and
inherited headroom per nation. It does not survive this call, reach the later
Materials pass, or change any public standalone read. GDP factory/power
helpers receive the same temporary scope only from this operating pass.

Invariant: within this pass, the observed mutations concern raw/manufactured
inventory, operating authority, GDP receipts, contractor work, fees and XP.
No district owner, conflict/control field, installed facility/module,
inherited opening capacity or contractor assignment identity changes.
All contractor modifiers are reread live for every rates/dispatch operation.
Each dispatch vector is fully computed before its first company XP write,
including editable cross-sector assignment cases.

Operating dispatch and GDP dispatch retain separate original arithmetic:

- Operating: `generation_level * 10 + module_micros / 1_000_000 * 10`.
- GDP: `effective_capacity * 10 * current_operator_work_rate`.

All vector order, floating sums, rounding, output, actual stock consumption,
payments, GDP reporting and company work remain on the original path.
The only eliminated GDP modifier reads are pure results for zero-capacity
districts that the original vector immediately discarded.

Tests retain the unscoped d50 implementation via a test-only flag. Focused
coverage checks exact rate/dispatch bits, modules and fresh geometry after
same-date ownership/control/capacity changes, both XP thresholds, malformed
cross-sector assignment behavior, input/storage/funding shortages, GDP
receipts, replay, and complete native worlds. A separate ignored 31-day
actual-checkpoint oracle compares every full world/headline result and requires
nonzero repeated geometry reuse. Parent coordinates compilation, both real
input runs and any subsequent isolated timing/qualification.
