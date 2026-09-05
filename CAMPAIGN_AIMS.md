# Optional campaign aims — approved audit implementation (2026-09-04)

The user approved all report changes, including peaceful campaign aims. Selection
is optional, free, and available to a living player in daily simulation. A single
active aim freezes its baseline and target when selected. These numbers are named
game rules, not historical claims or calibrated promises of completion time.

| Aim | Fixed target and conditions | Consecutive days |
| --- | --- | --- |
| Shared prosperity | Real GDP per person +20%; stability at least 45; inflation at most 10%; debt at most 20 GDP percentage points above its selection ratio; at peace | 90 |
| Scientific advancement | Eight technologies not known at selection, or all remaining technologies when fewer than eight remain; at peace | 1 |
| Stable government | Stability at least selection +10, clamped to 75–90; inflation at most 8%; at peace | 365 |
| Reliable strategic supply | Positive scheduled strategic raw-input demand with the next bundle covered by the existing action-stall calculation; resource market and gates enabled; at peace | 90 |
| Trusted diplomatic leadership | Two additional distinct trusted partners, capped by surviving countries at selection; relations at least 55 and a pact, integrated trade treaty or voluntary compact; no mutual sanctions; reputation at least 70; at peace | 90 |
| World domination | Existing discrete sovereignty victory condition; its presentation score alone cannot win | 1 |

All peaceful aims require peace during their qualifying streak. A missed condition
resets the streak. Repeated observations on the same date cannot count again;
skipped dates restart the streak. The observer runs after the daily programs and
province accounts settle. It consumes no RNG and changes no GDP, inventory, money,
research, relations or sovereignty. Completion records the result and produces a
player headline. The world remains playable. Continue in sandbox archives an
achievement (or explicitly sets aside incomplete work); a new choice then starts
from a new baseline. The latest 24 closed aims remain in the save.

Scientific progress counts stable technology IDs so registry insertion cannot
reinterpret the baseline. Existing research and funded prototype requirements are
unchanged. Supply measures the existing strategic raw-input bundle, not household
needs or every manufactured pack. It is not an assertion of an unimplemented
whole-economy supply model. Zero demand cannot win it. Diplomatic leadership never
converts a partner into a subject. Existing voluntary compacts and conquest keep
their own costs, consent and sovereignty rules.

The Decisions screen supplies all six choices, precise conditions, current versus
fixed target, streak, blockers and recorded results. Domination's own agenda screen
also links to peaceful aims. Selection and sandbox commands use the existing
session-bound command transport. Read-only state views do not enroll a campaign.
The empty optional ledger is absent from legacy saves; monthly replay does not
run this observer. Existing default tests and pins are not changed.

Targeted invariant tests cover per-person GDP versus population growth, frozen
targets, broken streaks, actual calendar dates, exactly-once completion, no reward,
new scientific knowledge, distinct diplomatic partners, sanctions, zero demand,
save/resume, the daily tick hook and browser command parsing. These are controlled
fixtures verifying rules, not statistical balance claims. Long-run difficulty
measurement remains part of the separate daily calibration work.
