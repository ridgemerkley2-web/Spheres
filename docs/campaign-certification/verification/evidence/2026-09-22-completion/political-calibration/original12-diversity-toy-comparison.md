# Prospective comparison of two diversity statistics

No criterion change is approved or implemented. The original A1 code still
asserts median elected coups in 4..14 and median top-three share strictly below
.50. The external copy adds only complete count output for original seeds0..11.
Compilation and execution must wait for the parent to release the native build.

For counts c_i totaling N, top-three share is the three largest counts divided
by N. HHI is sum((c_i/N)^2), and its inverse N^2/sum(c_i^2) is the effective
country count. The latter measures the number of equally represented countries
with the same concentration; it is not the literal number of distinct countries.

| Toy distribution | Top-three share | Original <.50 | Effective countries | Proposed >=4 |
|---|---:|---|---:|---|
| Four unique coups: 1,1,1,1 | 3/4 = .750 | Fail | 4 | Pass |
| Seven across six: 2,1,1,1,1,1 | 4/7 = .571 | Fail | 49/9 = 5.444 | Pass |
| Seven across three: 3,2,2 | 1.000 | Fail | 49/17 = 2.882 | Fail |

These totals all satisfy the 4..14 count band. The proposed criterion plainly
accepts distributions rejected by the current gate; it is a substantive change
of the diversity requirement, not an equivalent formula or a repair of a
failing implementation. Four distinct countries alone also does not guarantee
effective count4: unequal event shares reduce the effective count.

The diagnostic will report every per-seed distribution and both statistics,
then the medians and numbers of seeds satisfying each rule. Any median of the
new statistic is a proposed interpretation for review, not an adopted gate.
The original failed test and its exit code remain part of the evidence.
