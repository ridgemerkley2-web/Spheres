import json, glob, os, datetime as dt, statistics as st
B='/home/user/Spheres/docs/campaign-certification/S25/preparation/local-matrix-20260930'
def P(s): return dt.datetime.fromisoformat(s.replace('Z','+00:00'))
def ft(v): return dt.datetime(1601,1,1,tzinfo=dt.timezone.utc)+dt.timedelta(microseconds=v/10)
cells={}
for f in glob.glob(B+'/cells/*/execution.json')+glob.glob(B+'/interruption-20260930/evidence/cells/*/execution.json'):
    cid=f.split('/')[-2]; e=json.load(open(f))
    r=json.load(open(os.path.join(os.path.dirname(f),'native/result.json')))
    cells[cid]=dict(start=P(e['started_utc']),finish=P(e['finished_utc']),elapsed=e['elapsed_seconds'],exit=e['exit_code'],
        days=r['days_each_leg'],date=r['end_native_date'],passed=r['passed'],failure=r['failure'],
        result_bytes=os.path.getsize(os.path.join(os.path.dirname(f),'native/result.json')))
journal=[json.loads(l) for l in open(B+'/journal.jsonl')]
jf={j['id']:P(j['utc']) for j in journal if j['event']=='cell_finished'}
js={j['id']:P(j['utc']) for j in journal if j['event']=='cell_started'}
print('%-11s %-26s %-26s %9s %10s %-10s %6s %5s'%('cell','exec_start','exec_finish','elapsed','exit','lastdate','days','pct'))
for c,v in sorted(cells.items(), key=lambda kv: kv[1]['start']):
    print('%-11s %-26s %-26s %9.1f %10s %-10s %6d %5.1f journal_start=%s journal_finish=%s'%(c,v['start'].time(),v['finish'].time(),v['elapsed'],v['exit'],v['date'],v['days'],100*v['days']/16801,
        js.get(c).time() if c in js else None, jf.get(c).time() if c in jf else None))
done=[v['elapsed'] for v in cells.values() if v['passed']]
print('completed durations',sorted(done),'mean',st.mean(done),'median',st.median(done),'min',min(done),'max',max(done))
for c,v in cells.items():
    rate=v['days']/v['elapsed']
    print(c,'rate days/s %.4f'%rate,'s/day %.3f'%(1/rate),'elapsed/mean %.3f'%(v['elapsed']/st.mean(done)),'elapsed/min %.3f elapsed/max %.3f'%(v['elapsed']/min(done),v['elapsed']/max(done)),
          'days frac %.3f'%(v['days']/16801))
# projected total if pace constant
for c in ['japan-7','india-7','japan-42','india-1990']:
    v=cells[c]; proj=v['elapsed']*16801/v['days']; print(c,'projected full-horizon wall s (linear) %.0f'%proj)
# crash events
ev={'japan-7':P('2026-09-30T23:28:41.1287122Z'),'india-7':P('2026-09-30T23:28:57.8845869Z')}
print('WER app start japan-7',ft(0x1DD50F91D2D997F),'india-7',ft(0x1DD510327816AE1))
print('WER EventTime japan-7',ft(134352845211429339),'india-7',ft(134352845382353865))
for c,t in ev.items():
    v=cells[c]; print(c,'fault-start %.1f s'%(t-v['start']).total_seconds(),'exit observed - fault %.3f s'%(v['finish']-t).total_seconds())
# concurrency at each crash
for label,t in list(ev.items())+[('kill',P('2026-09-30T23:29:49.124402+00:00')),('runner_exit',P('2026-09-30T23:30:51.085314+00:00'))]:
    act=[c for c,v in cells.items() if v['start']<=t<=v['finish']]
    print(label,t.time(),'active:',act,{c:round((t-cells[c]['start']).total_seconds()) for c in act})
# overlap matrix
print('second wave starts', [(c,cells[c]['start'].time()) for c in ['japan-7','japan-42','india-1990','india-7']])
t0=max(cells[c]['start'] for c in ['japan-7','japan-42','india-1990','india-7'])
print('all four second-wave concurrent from',t0.time(),'to',min(cells[c]['finish'] for c in ['japan-7','india-7']).time(), (ev['japan-7']-t0).total_seconds(),'s before first fault')
# first wave concurrency
fw=['france-1990','france-7','france-42','japan-1990']
print('first wave all 4 concurrent until',min(cells[c]['finish'] for c in fw).time())
# 4 native processes at all times?
import itertools
times=sorted(set([v['start'] for v in cells.values()]+[v['finish'] for v in cells.values()]))
for a,b in zip(times,times[1:]):
    mid=a+(b-a)/2; n=sum(1 for v in cells.values() if v['start']<=mid<=v['finish'])
    if (b-a).total_seconds()>1: print(' %s -> %s  %6.0fs  native processes=%d'%(a.time(),b.time(),(b-a).total_seconds(),n))
print('launcher', P('2026-09-30T09:03:11.605697+00:00')+dt.timedelta(seconds=52058.57260869999))
