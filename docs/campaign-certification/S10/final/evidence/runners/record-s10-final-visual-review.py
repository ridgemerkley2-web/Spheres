import datetime,hashlib,json,pathlib
base=pathlib.Path(__file__).resolve().parent
names=[
('S10-browser-final-browser/france-IxwIwf/government-fresh-review.png','France desktop decision preview: readable before/after cards and costs.'),
('S10-browser-final-browser/france-IxwIwf/government-result-narrow-320.png','France320 result: completion notice and following decision remain legible without horizontal clipping.'),
('S10-final-matrix/matrix-AV6gfq/japan-Q4PWYB/leadership-desktop.png','Japan desktop leadership: campaign/reference separation, partial-research indicators and party rows visible.'),
('S10-final-matrix/matrix-AV6gfq/tonga-NhVmvH/overview-desktop.png','Tonga desktop: institutional government, current king, art and summary metrics visible.'),
('S10-final-matrix/matrix-AV6gfq/ussr-I1RzUm/review-narrow-320.png','USSR320 preview: financial/political costs and stacked debt effects fit viewport. Screenshot is scrolled below heading, not a full-page heading review.')]
record={'format':'spheres-s10-final-visual-review/v1','candidate_revision':'bcdf72bcbe8947966359deb95f715cb526a68def','reviewed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'passed':True,'scope':'Root visually inspected these five retained screenshots using view_image. This supplements automated eight-country interactions/layout checks; it does not claim manual inspection of every screenshot or a new long campaign.','images':[]}
for name,note in names:
    p=base/'evidence'/name
    record['images'].append({'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'observation':note})
dest=base/'evidence/S10-final-visual-review.json'
with dest.open('x',encoding='utf-8',newline='\n') as f: json.dump(record,f,indent=2);f.write('\n')
print(dest)
