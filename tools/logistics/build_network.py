#!/usr/bin/env python3
"""Bake a schematic strategic freight graph from the game's OWN map.

No historical port count, rail kilometre or traffic capacity is inferred here.
District locations and coast contacts derive from the committed Natural Earth
map; sea nodes are a navigable coarse sampling, not surveyed shipping lanes.
Narrow straits/canals are schematic geographic connectors. Their identities and
connectivity are documented by EIA's World Oil Transit Chokepoints atlas; the
coordinates below are map placement, NOT precise navigation or 1990 throughput.
"""
import hashlib
import json
import math
from pathlib import Path
import re
from PIL import Image

ROOT = Path(__file__).resolve().parents[2]
RX = [1,.9986,.9954,.99,.9822,.973,.96,.9427,.9216,.8962,.8679,.835,.7986,.7597,.7186,.6732,.6213,.5722,.5322]
RY = [0,.062,.124,.186,.248,.31,.372,.434,.4958,.5571,.6176,.6769,.7346,.7903,.8435,.8936,.9394,.9761,1]
W = 2400
R = W / (2 * .8487 * math.pi)
def interp(table, lat):
    t = min(abs(lat) / 5, 18)
    i = int(t)
    return table[i] if i == 18 else table[i] + (t-i)*(table[i+1]-table[i])
def ry(lat):
    return 1.3523 * R * interp(RY, lat) * (-1 if lat < 0 else 1)
def project(lon, lat):
    return W/2 + .8487*R*interp(RX,lat)*math.radians(lon), ry(83)-ry(lat)
def unproject(x,y):
    lo,hi=-58,83
    for _ in range(32):
        m=(lo+hi)/2
        if project(0,m)[1]>y: lo=m
        else: hi=m
    lat=(lo+hi)/2
    return max(-180,min(180,math.degrees((x-W/2)/(.8487*R*interp(RX,lat))))),lat
def distance(a,b):
    la,lb=math.radians(a[1]),math.radians(b[1])
    dl=math.radians((b[0]-a[0]+180)%360-180)
    h=math.sin((lb-la)/2)**2+math.cos(la)*math.cos(lb)*math.sin(dl/2)**2
    return max(1,round(12742*math.asin(min(1,math.sqrt(h)))))

coast=Image.open(ROOT/'spheres-web/ui/coast.png').convert('L')
pix=coast.load()
def water(lon,lat):
    x,y=project(lon,lat)
    x,y=round(x),round(y)
    return 0<=x<coast.width and 0<=y<coast.height and pix[x,y]<127
def sea_line(a,b, shore=False):
    length=distance(a,b)
    steps=max(2,math.ceil(length/18))
    dl=(b[0]-a[0]+180)%360-180
    # The gateway's first 25 km are a shoreline connector, not a sea lane.
    for i in range(1,steps):
        if shore and i/steps*length < 25: continue
        t=i/steps
        if not water((a[0]+dl*t+180)%360-180,a[1]+(b[1]-a[1])*t): return False
    return True

district_file=json.loads((ROOT/'spheres-sim/data/districts.json').read_text(encoding='utf-8'))
records={d['id']:d for nation in district_file['nations'].values() for d in nation}
text=(ROOT/'spheres-web/ui/districts.js').read_text(encoding='utf-8')
pattern=re.compile(r'\{id:("(?:[^"\\]|\\.)*"),name:("(?:[^"\\]|\\.)*"),path:("(?:[^"\\]|\\.)*"),cx:([\d.-]+),cy:([\d.-]+)\}')
nodes={}
shapes={}
for match in pattern.finditer(text):
    did,name,path=map(json.loads,match.group(1,2,3))
    if did not in records: continue
    lon,lat=unproject(float(match[4]),float(match[5]))
    nodes[did]={'id':did,'name':name,'kind':'district','lon':round(lon,5),'lat':round(lat,5),'district':did}
    shapes[did]=path
assert len(nodes)==len(records), (len(nodes),len(records))
edges={}
def coord(key): return nodes[key]['lon'],nodes[key]['lat']
def edge(a,b,kind,choke=None):
    if a==b: return
    a,b=sorted((a,b))
    row={'a':a,'b':b,'km':distance(coord(a),coord(b)),'kind':kind}
    if choke: row['chokepoint']=choke
    edges[a,b]=row
for did,row in sorted(records.items()):
    for adj in row.get('adj',[]):
        edge(did,adj,'land')

points=set()
def grid(x0,x1,y0,y1,step):
    for yi in range(round((y1-y0)/step)+1):
        for xi in range(round((x1-x0)/step)+1):
            x,y=round(x0+xi*step,4),round(y0+yi*step,4)
            if water(x,y): points.add((x,y))
grid(-177.5,177.5,-55,75,5)
grid(-8,42,30,46,2)
grid(32,45,12,30,1)
grid(48,58,23,31,1)
grid(7,32,54,67,1)
grid(90,130,-10,10,2)
grid(-84,-76,5,13,1)
for i,(lon,lat) in enumerate(sorted(points)):
    key=f'sea:{i:04}'
    nodes[key]={'id':key,'name':'Open sea','kind':'sea','lon':lon,'lat':lat}
sea_keys=[k for k in nodes if k.startswith('sea:')]

# Every sea edge touching these small boxes is labelled, so a route policy
# cannot evade a named strait by choosing the parallel edge beside its marker.
boxes=[('Strait of Hormuz',55,58,24,27.5),('Bab el-Mandeb',42,45,11.5,14),
       ('Strait of Malacca',99,104.5,0,6.8),('Strait of Gibraltar',-6.5,-4.5,35,37),
       ('Danish Straits',9,14,54,58)]
def choke_for(a,b):
    for name,x0,x1,y0,y1 in boxes:
        for t in (0,.25,.5,.75,1):
            x=a[0]+((b[0]-a[0]+180)%360-180)*t
            y=a[1]+(b[1]-a[1])*t
            if x0<=x<=x1 and y0<=y<=y1: return name
    return None
for a in sea_keys:
    ca=coord(a)
    nearby=sorted(((distance(ca,coord(b)),b) for b in sea_keys if b!=a),key=lambda r:(r[0],r[1]))
    for km,b in nearby[:14]:
        if km<=850 and sea_line(ca,coord(b)):
            edge(a,b,'sea',choke_for(ca,coord(b)))

# Canals/very narrow straits are below the coarse water raster. These are
# explicit schematic links, never accidental overland ocean edges.
connectors=[
 ('Suez Canal',(32.3,31.3),(32.55,29.8)),
 ('Panama Canal',(-79.95,9.4),(-79.55,8.75)),
 ('Turkish Straits',(28.6,40.5),(29.2,41.4)),
 ('Strait of Hormuz',(55.5,26.5),(57,25.5)),
 ('Bab el-Mandeb',(42.7,13.2),(43.5,12.3)),
 ('Strait of Gibraltar',(-6.3,35.8),(-4.4,36)),
 ('Danish Straits',(10.5,57.5),(12.5,54.8)),
 ('Strait of Malacca',(99,5),(104,1.2)),
]
for i,(name,a,b) in enumerate(connectors):
    pair=[]
    for j,p in enumerate((a,b)):
        key=f'choke:{i}:{j}'
        pair.append(key)
        nodes[key]={'id':key,'name':name,'kind':'chokepoint','lon':p[0],'lat':p[1]}
        connected=0
        for km,s in sorted((distance(p,coord(s)),s) for s in sea_keys):
            if km>850: break
            if sea_line(p,coord(s),shore=True):
                edge(key,s,'sea',name)
                connected+=1
                if connected>=4: break
    edge(pair[0],pair[1],'sea',name)
sea_keys += [k for k in nodes if k.startswith('choke:')]

# Only the main ocean component is eligible for gateways. Caspian and other
# inland water do not become imaginary access to world shipping.
adj={k:[] for k in sea_keys}
for row in edges.values():
    if row['kind']=='sea':
        adj[row['a']].append(row['b']);adj[row['b']].append(row['a'])
components=[]
unseen=set(sea_keys)
while unseen:
    stack=[min(unseen)];unseen.remove(stack[0]);component=[]
    while stack:
        a=stack.pop();component.append(a)
        for b in sorted(adj[a]):
            if b in unseen:unseen.remove(b);stack.append(b)
    components.append(component)
ocean=set(max(components,key=len))
for key in sea_keys:
    if key not in ocean: del nodes[key]
edges={k:v for k,v in edges.items() if v['a'] in nodes and v['b'] in nodes}
sea_keys=sorted(ocean)

gateways=0
for did,path in sorted(shapes.items()):
    candidates=[]
    for xs,ys in re.findall(r'[ML]([\d.-]+) ([\d.-]+)',path):
        x,y=float(xs),float(ys)
        ix,iy=round(x),round(y)
        if not(0<=ix<coast.width and 0<=iy<coast.height): continue
        # Positive field less than 2 px from the coast, or a simplified vertex
        # just outside the fill. Flood-connected ocean still must be reachable.
        if pix[ix,iy]>198: continue
        p=unproject(x,y)
        candidates.append((distance(coord(did),p),p))
    chosen=None
    for _,p in sorted(candidates)[:24]:
        links=[]
        for km,key in sorted((distance(p,coord(s)),s) for s in sea_keys):
            if km>850:break
            if sea_line(p,coord(key),shore=True):
                links.append(key)
                if len(links)==2:break
        if links:
            chosen=p,links;break
    if chosen:
        p,links=chosen;key=f'gateway:{did}'
        nodes[key]={'id':key,'name':nodes[did]['name']+' coastal gateway','kind':'gateway','lon':round(p[0],5),'lat':round(p[1],5),'district':did}
        edge(did,key,'terminal')
        for s in links:edge(key,s,'sea',choke_for(p,coord(s)))
        gateways+=1

sources=['spheres-web/ui/districts.js','spheres-web/ui/coast.png','spheres-sim/data/districts.json']
output={'meta':{'generator':'tools/logistics/build_network.py',
    'sources':{p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources},
    'geography':'Natural Earth geometry already committed in the game; mapgen Robinson inverse; centroid/gateway anchors are modeled, not historical ports.',
    'chokepoint_source':'https://www.eia.gov/international/analysis/special-topics/World_Oil_Transit_Chokepoints',
    'limitations':'Coarse schematic sea graph; not ship navigation. No historical capacities, roads, rail lengths, ports or shipping services claimed. Six nations without district geometry remain unmapped.',
    'districts':len(records),'gateways':gateways,'sea_nodes':len(sea_keys)},
    'nodes':[nodes[k] for k in sorted(nodes)],'edges':[edges[k] for k in sorted(edges)]}
out=ROOT/'spheres-sim/data/logistics_network.json'
out.write_text(json.dumps(output,ensure_ascii=False,separators=(',',':'))+'\n',encoding='utf-8')
print(f'Baked {len(nodes)} nodes, {len(edges)} edges, {gateways} coastal gateways, {len(sea_keys)} connected ocean nodes; {out.stat().st_size:,} bytes')
