import json,sys
d=json.load(open(sys.argv[1]))
st=[(d,1)];md=0;nv=0;nk=0
while st:
    x,dp=st.pop();nv+=1;md=max(md,dp)
    if isinstance(x,dict): nk+=len(x); st.extend((y,dp+1) for y in x.values())
    elif isinstance(x,list): st.extend((y,dp+1) for y in x)
print('max_depth',md,'values',nv,'keys',nk)
