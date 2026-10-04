import os,sys,time,re
pid=int(sys.argv[1]); out=open(sys.argv[2],'w'); prog=sys.argv[3]
def stack_maps():
    try: txt=open(f'/proc/{pid}/smaps').read()
    except Exception: return None
    blocks=re.split(r'\n(?=[0-9a-f]+-[0-9a-f]+ )',txt)
    res=[];prev=None
    for b in blocks:
        hdr=b.split('\n',1)[0].split()
        a,e=[int(x,16) for x in hdr[0].split('-')]; perms=hdr[1]; path=hdr[5] if len(hdr)>5 else ''
        size=(e-a)//1024
        rss=int(re.search(r'\nRss:\s+(\d+)',b).group(1))
        if perms=='rw-p' and path=='' and 2000<=size<=2200 and prev and prev[0]=='---p' and prev[1]==a:
            res.append((hex(a),size,rss))
        if path=='[stack]': res.append(('[stack]',size,rss))
        prev=(perms,e)
    return res
while True:
    try: os.kill(pid,0)
    except OSError: break
    m=stack_maps()
    last=''
    try:
        t=open(prog).read()[-300:]; mm=re.findall(r'"end_native_date":"([0-9-]+)"',t); last=mm[-1] if mm else ''
    except Exception: pass
    if m is not None: out.write(f"{time.time():.1f},{last},{m}\n"); out.flush()
    time.sleep(0.5)
