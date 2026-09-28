import concurrent.futures, datetime, hashlib, json, pathlib, urllib.request, urllib.error

BASE = pathlib.Path(__file__).resolve().parent
REPO = BASE.parent.parent / 'c01-source-followups-review'
SOURCE = REPO / 'docs/campaign-certification/C01/research/sources'
BODY = BASE / 'responses'
BODY.mkdir(exist_ok=True)
items = {}
for path in sorted(SOURCE.glob('*-facts.json')):
    doc = json.loads(path.read_text(encoding='utf-8'))
    review = doc.get('source_review', {})
    if review.get('review') not in ('CLAUDE-C01-SOURCE-17', 'CLAUDE-C01-SOURCE-26'):
        continue
    refs = [{'url': doc.get('source_response_url', doc['source_url']), 'bytes': doc.get('source_response_bytes'), 'sha256': doc.get('source_response_sha256'), 'role': 'source_response'}]
    refs += [dict(r, role='facsimile_responses') for r in doc.get('facsimile_responses', [])]
    if 'live_file_response' in doc:
        refs.append(dict(doc['live_file_response'], role='live_file_response'))
    for ref in refs:
        item = items.setdefault(ref['url'], {'url': ref['url'], 'expected_bytes': ref['bytes'], 'expected_sha256': ref['sha256'], 'uses': []})
        item['uses'].append({'source_file': path.name, 'source_id': doc['source_id'], 'review': review['review'], 'role': ref['role']})

def retrieve(pair):
    idx, item = pair
    result = dict(item, attempt=1, started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), request_headers={'Accept-Encoding': 'identity'}, user_agent='Python-urllib default', method='ordinary GET; no cookies, UA override, cache-buster, or access-control workaround')
    request = urllib.request.Request(item['url'], headers=result['request_headers'])
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            data = response.read()
            result.update(status=response.status, final_url=response.url, headers=dict(response.headers))
    except urllib.error.HTTPError as error:
        data = error.read()
        result.update(status=error.code, final_url=error.url, headers=dict(error.headers), error=str(error))
    except Exception as error:
        data = b''
        result.update(status=None, error=repr(error))
    result.update(completed_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
    result['matches_claimed_response'] = result['status'] == 200 and result['bytes'] == item['expected_bytes'] and result['sha256'] == item['expected_sha256']
    ext = '.pdf' if data.startswith(b'%PDF') else '.jpg' if data.startswith(b'\xff\xd8') else '.body'
    name = f'{idx:02d}{ext}'
    (BODY / name).write_bytes(data)
    result['body_path'] = 'responses/' + name
    (BASE / f'retrieval-{idx:02d}.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(f'{idx:02d} status={result["status"]} bytes={len(data)} match={result["matches_claimed_response"]} {item["url"]}', flush=True)
    return result

if __name__ == '__main__':
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        results = list(pool.map(retrieve, enumerate(items.values(), 1)))
    (BASE / 'retrievals.json').write_text(json.dumps({'reviewer': '/root/review_source05', 'results': results}, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print('TOTAL', len(results), 'EXACT', sum(r['matches_claimed_response'] for r in results), flush=True)
