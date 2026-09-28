import concurrent.futures, json
from retrieve import BASE, SOURCE, retrieve
old = json.loads((BASE/'retrievals.json').read_text(encoding='utf-8'))['results']
seen = {r['url'] for r in old}
items = []
for path in sorted(SOURCE.glob('ussr-*-facts.json')):
    doc=json.loads(path.read_text(encoding='utf-8'))
    for r in doc.get('source_review',{}).get('downloads',[]):
        if r['url'] in seen: continue
        seen.add(r['url'])
        items.append({'url':r['url'],'expected_bytes':r['bytes'],'expected_sha256':r['sha256'],'uses':[{'source_file':path.name,'source_id':doc['source_id'],'review':'CLAUDE-C01-SOURCE-26','role':r['response']}]})
items.append({'url':'https://web.archive.org/web/20230109012106id_/https://legis.senado.leg.br/diarios/BuscaPaginasDiario?codDiario=111711&paginaInicial=&paginaFinal=','expected_bytes':24950218,'expected_sha256':'d6c9c275da00f4d8b73a3127fa98c53a7723f7779445d6a33e93131d7ec854ee','uses':[{'review':'CLAUDE-C01-SOURCE-17','role':'independent_capture'}]})
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool:
    results=list(pool.map(retrieve,enumerate(items,len(old)+1)))
(BASE/'retrievals-extra.json').write_text(json.dumps({'reviewer':'/root/review_source05','results':results},ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print('TOTAL',len(results),'EXACT',sum(r['matches_claimed_response'] for r in results))
