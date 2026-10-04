import gzip, json, sys
path=sys.argv[1]
raw=gzip.open(path,'rb').read()
d=json.loads(raw)
print('compact bytes',len(raw))
def size(v): return len(json.dumps(v,separators=(',',':'),ensure_ascii=False).encode())
def stats(v,depth=1):
    # returns (max_depth, n_values, n_objects, n_arrays, n_keys, n_strings)
    st=[(v,1)]; md=0; nv=no=na=nk=ns=0
    while st:
        x,dp=st.pop(); nv+=1; md=max(md,dp)
        if isinstance(x,dict):
            no+=1; nk+=len(x); st.extend((y,dp+1) for y in x.values())
        elif isinstance(x,list):
            na+=1; st.extend((y,dp+1) for y in x)
        elif isinstance(x,str): ns+=1
    return md,nv,no,na,nk,ns
print('top-level keys', list(d.keys()))
for k in d: print(' ',k,size(d[k]))
w=d['world']
print('world keys', list(w.keys())[:20])
inner=w.get('world',w)
md,nv,no,na,nk,ns=stats(d)
print('whole archive: max_depth',md,'values',nv,'objects',no,'arrays',na,'keys',nk,'strings',ns)
print('pretty bytes (indent=2) of world', len(json.dumps(w,indent=2,ensure_ascii=False).encode()))
rows=sorted(((size(v),k) for k,v in inner.items()),reverse=True)[:25]
for s,k in rows: print('  world.%s'%k, s, 'depth',stats(inner[k])[0])
# deepest path
def deepest(v,path=()):
    best=(0,path); st=[(v,path)]
    while st:
        x,p=st.pop()
        if len(p)>best[0]: best=(len(p),p)
        if isinstance(x,dict): st.extend((y,p+(k,)) for k,y in x.items())
        elif isinstance(x,list): st.extend((y,p+(i,)) for i,y in enumerate(x))
    return best
dd=deepest(d); print('deepest path len',dd[0], dd[1])
