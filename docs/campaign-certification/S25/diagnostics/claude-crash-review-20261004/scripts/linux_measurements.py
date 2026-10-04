"""Summarise the retained Linux probe outputs in this packet (read-only).

Run from the repository root:
  python3 docs/campaign-certification/S25/diagnostics/claude-crash-review-20261004/scripts/linux_measurements.py

Prints: (1) memory high-water by simulated year for the japan7-to1995 probe,
(2) test-thread stack Rss change points from the h4 stack probe, and
(3) a row-by-row comparison of the Linux Japan/7 comparisons with the retained
Windows Japan/7 comparisons from the 30 September attempt.
Written by Claude (automated agent). Diagnostic only.
"""
import ast, collections, csv, json, os

HERE = os.path.dirname(os.path.abspath(__file__))
PKT = os.path.dirname(HERE)
REPO = os.path.abspath(os.path.join(PKT, '..', '..', '..', '..', '..'))
WIN = os.path.join(REPO, 'docs/campaign-certification/S25/preparation/local-matrix-20260930/'
                   'interruption-20260930/evidence/cells/japan-7/native/result.json')


def mib(kb):
    return kb / 1024.0


def memory_by_year(name):
    rows = list(csv.DictReader(open(os.path.join(PKT, 'linux-probes', name, 'memory.csv'))))
    res = json.load(open(os.path.join(PKT, 'linux-probes', name, 'native', 'result.json')))
    canon = collections.OrderedDict()
    for c in res['comparisons']:
        y = c['date'][:4]
        canon[y] = max(canon.get(y, 0), c['canonical_bytes'])
    by = collections.OrderedDict()
    for r in rows:
        lp = r['last_progress']
        if not lp:
            continue
        y = lp[:4]
        d = by.setdefault(y, dict(n=0, rss_min=None, rss_max=0, hwm=0, first=lp, last=lp))
        rss, hwm = int(r['rss_kb']), int(r['hwm_kb'])
        d['n'] += 1
        d['rss_min'] = rss if d['rss_min'] is None else min(d['rss_min'], rss)
        d['rss_max'] = max(d['rss_max'], rss)
        d['hwm'] = max(d['hwm'], hwm)
        d['last'] = lp
    print('== %s: %d samples, threads=%s' % (name, len(rows), sorted({r['threads'] for r in rows})))
    print('%-5s %5s %-23s %18s %22s %22s' % ('year', 'n', 'checkpoints', 'RSS min-max MiB',
                                             'cumulative VmHWM', 'max canonical bytes'))
    for y, d in by.items():
        print('%-5s %5d %-23s %8.0f-%-9.0f %8.0f MiB (%d kB) %22s' % (
            y, d['n'], d['first'] + '..' + d['last'], mib(d['rss_min']), mib(d['rss_max']),
            mib(d['hwm']), d['hwm'], format(canon.get(y, 0), ',')))
    print('final VmHWM %d kB; last end_native_date %s; passed=%s' % (
        max(int(r['hwm_kb']) for r in rows), res['end_native_date'], res['passed']))
    return res


def stack_changes():
    print('== h4-stack-hwm-review: test-thread stack mapping Rss (KiB) change points')
    prev = None
    for line in open(os.path.join(PKT, 'linux-probes', 'stack', 'h4-stack-hwm-review', 'stack.csv')):
        t, last, rest = line.rstrip('\n').split(',', 2)
        maps = ast.literal_eval(rest)
        thread = [m for m in maps if m[0] != '[stack]']
        val = thread[0][2] if thread else None
        size = thread[0][1] if thread else None
        if val != prev:
            print('  t=%s last_progress=%-10s mapping=%s KiB rss=%s KiB' % (t, last or '-', size, val))
            prev = val


def compare(linux):
    win = json.load(open(WIN))
    fields = ['kind', 'date', 'canonical_bytes', 'canonical_fnv64', 'history_rows', 'log_rows']
    lc, wc = linux['comparisons'], win['comparisons']
    same = sum(1 for a, b in zip(lc, wc) if all(a.get(f) == b.get(f) for f in fields))
    print('== Linux japan7-to1995 vs Windows Japan/7 (30 Sep) comparisons')
    print('linux rows %d, windows rows %d; index-aligned rows identical on %s: %d of %d' % (
        len(lc), len(wc), '/'.join(fields), same, len(lc)))
    print('linux last row %s %s %d %s' % (lc[-1]['date'], lc[-1]['kind'], lc[-1]['canonical_bytes'],
                                          lc[-1]['canonical_fnv64']))
    w = collections.OrderedDict()
    for c in wc:
        w[c['date'][:4]] = max(w.get(c['date'][:4], 0), c['canonical_bytes'])
    print('windows Japan/7 max canonical bytes per year (MB):')
    print('  ' + ', '.join('%s %.1f' % (y, v / 1e6) for y, v in w.items()))
    mx = max(wc, key=lambda c: c['canonical_bytes'])
    print('windows peak %s %s %d; last %s %s %d %s' % (mx['date'], mx['kind'], mx['canonical_bytes'],
          wc[-1]['date'], wc[-1]['kind'], wc[-1]['canonical_bytes'], wc[-1]['canonical_fnv64']))


if __name__ == '__main__':
    memory_by_year('japan7-to1991')
    print()
    res = memory_by_year('japan7-to1995')
    print()
    stack_changes()
    print()
    compare(res)
