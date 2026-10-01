"""Offline verification of the bounded C01-31 resumed acceptance receipt."""
import argparse,hashlib,html,json,pathlib,re,subprocess
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--external-originals',action='store_true'); parser.add_argument('--repo',type=pathlib.Path)
args=parser.parse_args(); HERE=pathlib.Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
def read(name):return json.loads((HERE/name).read_text(encoding='utf8'))
manifest=read('manifest.json')
actual={p.relative_to(HERE).as_posix() for p in HERE.rglob('*') if p.is_file() and p.name!='manifest.json' and '__pycache__' not in p.parts}
assert actual=={r['path'] for r in manifest['files']}
for row in manifest['files']:
 b=(HERE/row['path']).read_bytes(); assert len(b)==row['bytes'] and sha(b)==row['sha256'],row['path']
sources=read('source-verification.json'); claims=read('claim-review.json'); holders=read('holder-review.json'); decision=read('decision.json'); scope=read('scope-audit.json')
assert (len(sources),len(claims),len(holders))==(31,64,15)
assert len({s['source_id'] for s in sources})==31 and len({c['claim_id'] for c in claims})==64
assert all(s['retrieval_decision']=='exact_original_reproduced' for s in sources)
assert not any(c['decision'].startswith('held') for c in claims)
assert all(h['decision']=='supported_bounded_holder_observation' and not h['missing_source_ids'] for h in holders)
assert sum(bool(h['prior_missing_source_ids']) for h in holders)==5
assert sum(c['content_review_checkpoint']=='resumed-20261001' for c in claims)==14
assert decision['recommendation']=='accept_bounded_research_with_corrections'
assert not decision['remaining_missing_sources'] and not decision['remaining_held_claims'] and not decision['remaining_held_holder_dependencies'] and not decision['parent_gates_closed']
assert len(scope['authored_reviewed_paths'])==40 and len(scope['preserved_prior_receipt'])==95
if args.external_originals:
 for s in sources:
  b=pathlib.Path(s['body_external']).read_bytes(); assert len(b)==s['expected_bytes'] and sha(b)==s['expected_sha256'],s['source_id']
  assert sha(pathlib.Path(s['readable_external']).read_bytes())==s['readable_sha256']
 for row in read('retrieval.json')['results']:
  b=pathlib.Path(row['headers_external']).read_bytes(); assert len(b)==row['headers_bytes'] and sha(b)==row['headers_sha256']
  b=(HERE/row['headers_derivative']).read_bytes(); assert len(b)==row['headers_derivative_bytes'] and sha(b)==row['headers_derivative_sha256']
 recovered={r['id']:r for r in read('readable-manifest.json')}
 kanzaki=recovered['jp_komeito_kanzaki_fifth_convention_address_20041101']
 raw=pathlib.Path(kanzaki['original_external']).read_bytes().decode(kanzaki['encoding'])
 body=raw.split('<div id="newsbody">',1)[1]
 ps=re.findall(r'<p\b[^>]*>(.*?)</p>',body,re.S|re.I)
 text=lambda t:html.unescape(re.sub('<[^>]+>','',t)).strip()
 assert 'ご来賓' in text(ps[1]) and '再び党代表の重責' in text(ps[2])
 ota=recovered['jp_komeito_ota_sixth_convention_20061001']; ot=pathlib.Path(ota['readable_external']).read_text(encoding='utf8')
 for anchor in ['「新しい公明党」が勇躍スタート','これに先立ち、代表選出が行われ']:assert ot.count(anchor)==1
 saito=recovered['jp_komeito_saito_recommended_20241108']; st=pathlib.Path(saito['readable_external']).read_text(encoding='utf8')
 assert '公明党中央幹事会は7日' in st and 'あす9日に開かれる臨時全国大会' in st and '過半数から信任を得られれば' in st
if args.repo:
 for key,rev in [('authored_reviewed_paths',scope['reviewed_source']),('preserved_prior_receipt',scope['prior_checkpoint'])]:
  for row in scope[key]:
   b=subprocess.check_output(['git','show',rev+':'+row['path']],cwd=args.repo)
   assert len(b)==row['bytes'] and sha(b)==row['sha256'],row['path']
print('Receipt verified: 31 exact originals,64 reviewed claims,15 bounded holder observations. Acceptance recommendation only; no parent gate closed.')
