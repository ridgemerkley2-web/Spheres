import json, sys
d=json.load(open(sys.argv[1]))
def size(v): return len(json.dumps(v,separators=(',',':'),ensure_ascii=False).encode())
w=d['world']['world']
out={'_archive_total':sum(size(d[k]) for k in d),'history':size(d['history']),'log':size(d['log']),'log_rows':len(d['log']),'history_rows':len(d['history'])}
for k,v in w.items():
    s=size(v)
    if s<300000: continue
    out[k]=s
    if isinstance(v,dict):
        for k2,v2 in v.items():
            s2=size(v2)
            if s2>300000:
                out[k+'.'+k2]=s2
                if isinstance(v2,list): out[k+'.'+k2+'#len']=len(v2)
                if isinstance(v2,dict): out[k+'.'+k2+'#keys']=len(v2)
    if isinstance(v,list): out[k+'#len']=len(v)
json.dump(out,open(sys.argv[2],'w'),indent=1)
for k,v in out.items(): print(k,v)
# extra counts (added later): population courses/children entries and company/import records
try:
    provs=w['population_system']['provinces']
    out2={'pop_courses_total':sum(len(p.get('courses',[])) for p in provs.values()),
          'pop_courses_max_per_province':max(len(p.get('courses',[])) for p in provs.values()),
          'pop_children_cohorts_total':sum(len(p.get('children',{})) for p in provs.values()),
          'pop_children_max_per_province':max(len(p.get('children',{})) for p in provs.values())}
    comp=w.get('companies',{}) or {}
    out2['companies_deliveries']=len(comp.get('deliveries',[]) or [])
    out2['companies_import_contracts']=len(((comp.get('imports') or {}).get('contracts')) or [])
    out2['flags']=len(w.get('flags',[]))
    out.update(out2); json.dump(out,open(sys.argv[2],'w'),indent=1)
    for k,v in out2.items(): print(k,v)
except Exception as e:
    print('extra counts failed',e)
