import json, glob, os, datetime as dt
B='/home/user/Spheres/docs/campaign-certification/S25/preparation/local-matrix-20260930'
def P(s): return dt.datetime.fromisoformat(s.replace('Z','+00:00'))
cells={}
for f in glob.glob(B+'/cells/*/native/result.json')+glob.glob(B+'/interruption-20260930/evidence/cells/*/native/result.json'):
    d=json.load(open(f)); e=json.load(open(f.replace('native/result.json','execution.json')))
    rows=sorted([c for c in d['comparisons']],key=lambda c:c['absolute_day'])
    cells[d['id']]=dict(rows=rows,start=P(e['started_utc']),elapsed=e['elapsed_seconds'],days=d['days_each_leg'])
for c,v in cells.items():
    r=v['rows']; mx=max(r,key=lambda x:x['canonical_bytes'])
    last=r[-1]
    print('%-11s peak %s %d B | last %s %d B hist %d log %d | ratio last/peak %.2f'%(c,mx['date'],mx['canonical_bytes'],last['date'],last['canonical_bytes'],last['history_rows'],last['log_rows'],last['canonical_bytes']/mx['canonical_bytes']))
# estimate canonical size of each worker at a wall time assuming uniform pace (days/elapsed) - crude
def size_at(c,t):
    v=cells[c]; frac=(t-v['start']).total_seconds()/v['elapsed']
    if frac<0 or frac>1: return None
    day=frac*v['days']; best=None
    for row in v['rows']:
        if row['absolute_day']<=day: best=row
    return best
for label,t in [('wave1 ~2010',P('2026-09-30T12:30:00+00:00')),('wave1 13:30',P('2026-09-30T13:30:00+00:00')),('crash',P('2026-09-30T23:28:41+00:00'))]:
    tot=0; parts=[]
    for c in cells:
        b=size_at(c,t)
        if b: tot+=b['canonical_bytes']; parts.append((c,b['date'],b['canonical_bytes']))
    print(label,t.time(),'sum canonical %.1f MB'%(tot/1e6),parts)
# max over wave1 grid of the sum
best=(0,None)
t=P('2026-09-30T09:10:00+00:00')
while t<P('2026-09-30T16:30:00+00:00'):
    tot=sum((size_at(c,t) or {'canonical_bytes':0})['canonical_bytes'] for c in ['france-1990','france-7','france-42','japan-1990'])
    if tot>best[0]: best=(tot,t)
    t+=dt.timedelta(minutes=5)
print('wave1 max est sum canonical %.1f MB at %s'%(best[0]/1e6,best[1].time()))
