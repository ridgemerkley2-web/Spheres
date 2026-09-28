import json,pathlib,re,sys
P=pathlib.Path(__file__).parent
D=json.loads((P/'all-inputs.json').read_text(encoding='utf-8'))
def normal(s):
    return re.sub(r'\s+',' ',s).replace('’',"'").replace('‘',"'").replace('“','"').replace('”','"').strip()
for source in D[int(sys.argv[1]):int(sys.argv[2])]:
    original=normal((P/'text'/f'{source["id"]}.txt').read_text(encoding='utf-8'))
    print('\nSOURCE',D.index(source),source['id'])
    for c in source['claims']:
        q=re.findall(r"'([^']{15,})'",c['text'])
        hits=[]
        for quote in sorted(q,key=len,reverse=True):
            at=original.lower().find(normal(quote).lower())
            if at>=0 and not any(abs(at-other)<500 for other in hits): hits.append(at)
        print('CLAIM',c['id'],'\nDECLARED',c['text'])
        if hits:
            for at in hits[:2]: print('ORIGINAL',original[max(0,at-110):at+650])
        else: print('NO-EXACT-QUOTE-MATCH; needs direct original text')
