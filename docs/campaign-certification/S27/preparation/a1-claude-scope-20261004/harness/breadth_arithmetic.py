"""Explanatory arithmetic for A1's strict median per-seed top-three rule.

Claude scratch script (automated agent), copied into this packet. It was run
as an inline heredoc on 4 Oct 2026 and its stdout became
diagnostics/a1-breadth-arithmetic.txt. Only the output redirection was removed.
The per-seed country counts are the seeds 0-11 result at 39369f0
(diagnostics/a1-diag-seeds0-11.log). The scenarios are hypothetical
re-arrangements of those counts. They explain why the statistic fails; they
are NOT a tuning target, a predicted outcome or an alternative criterion.
"""
from fractions import Fraction as F
from statistics import median
seeds = {0:[1]*7, 1:[2,1,1,1,1], 2:[1]*5, 3:[2,2,1,1,1,1], 4:[2]+[1]*6, 5:[2]+[1]*6, 6:[2]+[1]*5,
         7:[2,2,1,1,1,1], 8:[2]+[1]*6, 9:[2]+[1]*5, 10:[2,2,1,1,1,1], 11:[2]+[1]*5}
def share(c):
    c=sorted(c,reverse=True); return F(sum(c[:3]),sum(c))
def report(name, f):
    sh=[]; tot=[]
    for s,c in seeds.items():
        c2=f(list(c)); sh.append(share(c2)); tot.append(sum(c2))
    m=median(sh); mc=median(tot)
    print(f"{name:58s} median coups {float(mc):5.1f} median top3 {float(m):.4f} ({m}) seeds<0.5: {sum(1 for x in sh if x<F(1,2))}/12 -> A1 {'PASS' if 4<=mc<=14 and m<F(1,2) else 'FAIL'}")
report("current (diagnostic counts, seeds 0-11)", lambda c:c)
report("repeats removed (each country at most 1/seed)", lambda c:[1]*len(c))
for k in range(1,6):
    report(f"+{k} new distinct single-coup countries per seed", lambda c,k=k:c+[1]*k)
report("+2 new distinct countries AND +1 repeat in top country", lambda c:[c[0]+1]+c[1:]+[1,1])
report("+1 new country with 2 coups per seed", lambda c:c+[2])
report("+2 new countries with 2 coups each per seed", lambda c:c+[2,2])
