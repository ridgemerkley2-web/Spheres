import json
import re
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
SPECS = {
    '34': ('0eba7867', 'ec8cb88b97f1774f4a9c65f202cc6bc6143d5789', 'Brazil',
           '75 exact original responses plus one separately qualified current registry response; 127 claims and 12 holder observations reviewed. The missing prior registry body, unknown change cause and source limits remain explicit. SOURCE-17 is preserved.'),
    '35': ('e4e8d389', 'ec8cb88b97f1774f4a9c65f202cc6bc6143d5789', 'USSR',
           '16 exact original facsimile responses, 36 claims and eight dated holder observations reviewed. Private/nonprofit host attribution and paper-original authentication limits remain explicit. SOURCE-26 is preserved.'),
    '36': ('364c6f6d', 'c29f17093b9b34bcef90b25ae53d349fe36330a7', 'Tonga',
           '39 exact original responses, 53 claims and ten holder observations reviewed. Seven unsupported explanations for acting service were removed without changing holders or dates. C01-24, C04 preparation and all earlier sources remain intact.'),
    '37': ('4d88fd03', 'ed36670ef6125257bfa9e178c93049a303e0eb37', 'France',
           '31 exact original responses, 48 claims and 16 named source observations reviewed. Twelve inferred effective starts and fourteen inferred effective ends were removed; instrument dates remain. This first ten-person batch runs through March 2014. Effective-term and later-PM research remain open. C01-23 and the importer are preserved.'),
}

def replace(path, old, new):
    p = ROOT / path
    text = p.read_text(encoding='utf-8')
    assert old in text, (path, old)
    p.write_text(text.replace(old, new), encoding='utf-8', newline='\n')

p = ROOT / 'docs/planning/ai-task-queue.json'
data = json.loads(p.read_bytes())
for number, (source, review, country, scope) in SPECS.items():
    row = next(t for t in data['tasks'] if t['id'] == 'CLAUDE-C01-' + number)
    assert row['state'] == 'ready_for_review'
    row['state'] = 'complete'
    row['review_revision'] = review
    row['next'] = f'Accepted bounded research intake at {review[:8]}; first-source import {source}. {scope} Evidence: docs/campaign-certification/C01/integrations/CLAUDE-C01-{number}/README.md. No complete country chronology, runtime mapping, portraits or parent C01/C06/S23 closure.'
p.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')

replace('tools/avatars/certified_gap_ledger.py',
        "    '9d97caa9': 'CLAUDE-C01-33',",
        "    '9d97caa9': 'CLAUDE-C01-33',\n" + '\n'.join(f"    '{v[0]}': 'CLAUDE-C01-{n}'," for n, v in SPECS.items()))
replace('tools/avatars/certified_gap_ledger.py', '; submitted, not accepted', '; bounded observations accepted, no runtime mapping')
replace('tools/avatars/test_certified_boundary_matrix.py',
        "'27', '30', '32', '33'})", "'27', '30', '32', '33', '34', '35', '36', '37'})")
replace('tools/avatars/test_certified_gap_ledger.py',
        "'CLAUDE-C01-30', 'CLAUDE-C01-32', 'CLAUDE-C01-33'})",
        "'CLAUDE-C01-30', 'CLAUDE-C01-32', 'CLAUDE-C01-33',\n                                         'CLAUDE-C01-34', 'CLAUDE-C01-35', 'CLAUDE-C01-36', 'CLAUDE-C01-37'})")
replace('tools/avatars/test_certified_gap_ledger.py', '(28, 29, 34, 35, 36, 37)', '(28, 29)')
# These targets become explicit unresolved research roles after intake acceptance,
# rather than being hidden forever by an in-flight reservation.
replace('tools/avatars/test_certified_gap_ledger.py',
        "        self.assertNotIn('institution:fr_prime_minister', entities)\n        self.assertNotIn('to_cabinet#to_deputy_pm', entities)",
        "        self.assertNotIn('institution:fr_prime_minister', entities)  # The institution now exists.\n        self.assertIn(('France', 'fr_pm'), {(r['nation'], r['id']) for c in self.data['cases'] for r in c['research_roles']})\n        self.assertIn(('Tonga', 'to_deputy_pm'), {(r['nation'], r['id']) for c in self.data['cases'] for r in c['research_roles']})")

p = ROOT / 'docs/AI_WORKSTREAMS.md'
text = p.read_text(encoding='utf-8')
text = text.replace('C01-30, C01-32 and C01-33 are now accepted as bounded intake after independent\nsource and content review; C01-34–37 remain ready for review.',
                    'C01-30 and C01-32–37 are now accepted as bounded intake after independent\nsource and content review and scoped corrections.')
for number, (source, review, country, scope) in SPECS.items():
    pattern = re.compile(r'^(\| C01-' + number + r':[^\n]+? \| `[^`]+` \| ).* \|$', re.M)
    text, count = pattern.subn(lambda m: m.group(1) + f'Bounded intake accepted at `{review[:8]}`; [review](campaign-certification/C01/integrations/CLAUDE-C01-{number}/README.md). {scope} |', text)
    assert count == 1, number
text = text.replace('C01-23/24/25/27/30/32/33 are accepted bounded intake', 'C01-23/24/25/27/30/32–37 are accepted bounded intake')
text = text.replace('Russia C01-28 and C01-34–37 are submitted, not accepted.', 'Russia C01-28 remains submitted for review; no acceptance is implied.')
text = text.replace('and C01-32 South Africa, plus C01-33 India, are accepted as bounded intake. C01-28 and C01-34–37 are submitted;',
                    'and C01-32 South Africa, plus C01-33–37 India/Brazil/USSR/Tonga/France, are accepted as bounded intake. C01-28 remains submitted;')
p.write_text(text, encoding='utf-8', newline='\n')

p = ROOT / 'docs/planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md'
text = p.read_text(encoding='utf-8')
start = text.index('- Brazil `CLAUDE-C01-34` is **ready_for_review**')
end = text.index('\n\nThe exact authored handoffs', start)
text = text[:start] + '\n'.join(f'- {country} `CLAUDE-C01-{number}` is **complete as bounded research intake** at `{review[:8]}`.\n  {scope}\n  [Independent review and root decision](../../campaign-certification/C01/integrations/CLAUDE-C01-{number}/README.md).' for number, (source, review, country, scope) in SPECS.items()) + text[end:]
p.write_text(text, encoding='utf-8', newline='\n')
print('Updated four bounded intake decisions, exact source attribution, expected accepted sets and current workboard prose.')
