import gzip,hashlib,json,pathlib,re,xml.etree.ElementTree as ET
from retrieve import textbody
root=pathlib.Path(__file__).parent
sourcefile=root/'source-verification-final.json'
if not sourcefile.exists():sourcefile=root/'source-verification.json'
rows=json.loads(sourcefile.read_text(encoding='utf-8'))['sources']
chunks=[];decodings=[]
for row in rows:
    if not row['exact']:continue
    text=pathlib.Path(row['text_path']).read_text(encoding='utf-8')
    if 'members' in row:
        parts=[]
        for member in row['members']:
            elem=ET.fromstring(pathlib.Path(member['path']).read_bytes())
            tags=['ID','NOR','TITRE','DATE_TEXTE','DATE_PUBLI','DATES_EFFET','ORIGINE_PUBLI','NUM_SEQUENCE','VISAS','SIGNATAIRES','BLOC_TEXTUEL']
            values=[]
            for tag in tags:
                for found in elem.iter(tag):
                    value=' '.join(' '.join(found.itertext()).split())
                    values.append(tag+': '+value)
            parts.append(pathlib.Path(member['path']).name+'\n'+'\n'.join(values))
        excerpt='\n'.join(parts)
    else:
        response=next(a for a in row['attempts'] if a.get('exact'))
        raw=pathlib.Path(response['body_path']).read_bytes()
        if raw[:2]==b'\x1f\x8b':
            decoded=gzip.decompress(raw);text=textbody(decoded)
            target=root/'source-text-decoded'/(row['source_id']+'.html')
            target.parent.mkdir(exist_ok=True);target.write_bytes(decoded)
            decodings.append({'source_id':row['source_id'],'gzip_decoded_bytes':len(decoded),'gzip_decoded_sha256':hashlib.sha256(decoded).hexdigest(),'path':str(target)})
        pos=text.find('Version initiale')
        if pos<0:
            print('Full text fallback:',row['source_id'])
            chunks.append('SOURCE '+row['source_id']+'\nURL '+row['url']+'\nCLAIMS '+', '.join(row['claims'])+'\n'+text)
            continue
        start=text.rfind('\nDécret',0,pos)
        end=text.find('Extrait du Journal',pos)
        if end<0:end=text.find('Retourner en haut',pos)
        if end<0:end=len(text)
        excerpt=text[max(0,start):end].strip()
    chunks.append('SOURCE '+row['source_id']+'\nURL '+row['url']+'\nCLAIMS '+', '.join(row['claims'])+'\n'+excerpt)
(root/'content-read.txt').write_text('\n\n'.join(chunks),encoding='utf-8')
(root/'content-decoding.json').write_text(json.dumps(decodings,indent=2)+'\n',encoding='utf-8')
print('Prepared '+str(len(chunks))+' exact-source text views; missing originals remain excluded.')
