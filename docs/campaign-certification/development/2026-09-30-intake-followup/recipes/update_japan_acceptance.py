import json
import re
from pathlib import Path

ROOT = Path(r'C:\Users\ridge\Documents\Codex\2026-09-05\pick-up-the-spheres-game-on\work\campaign-certification\integration')
def replace(path, old, new):
    p = ROOT / path
    text = p.read_text(encoding='utf-8')
    assert old in text, (path, old)
    p.write_text(text.replace(old, new), encoding='utf-8', newline='\n')

p = ROOT / 'docs/planning/ai-task-queue.json'
data = json.loads(p.read_bytes())
row = next(t for t in data['tasks'] if t['id'] == 'CLAUDE-C01-29')
assert row['state'] == 'ready_for_review'
row['state'] = 'complete'
row['review_revision'] = 'dc65b5100cbce2dac447e08987235566376fcd14'
row['next'] = 'Bounded intake accepted at dc65b510 after all 19 held original responses were recovered; all 54 originals, 111 claims and 16 holder observations reviewed. First-source import b2098862 and retained correction 5879bc23 preserve earlier Japan data. The unchanged old held checkpoint and fresh resumed review remain under docs/campaign-certification/C01/reviews/CLAUDE-C01-29. Root decision: docs/campaign-certification/C01/integrations/CLAUDE-C01-29/README.md. No complete history, runtime mapping, portraits or parent closure.'
row = next(t for t in data['tasks'] if t['id'] == 'CLAUDE-C01-28')
row['review_revision'] = '0719ba7bdab5c75749e4577aa1d67a090adaf2d0'
row['next'] = 'Independent review held at 0719ba7b: 56 of 68 original responses reproduced; 12 remain inaccessible after one missing-only retry. All 173 focused test executions pass and prior Russia/SOURCE-05 isolation is proven. The original receipt records 43 reviewed claims, 24 inaccessible claims and 70 reproduced claims still awaiting content review; that available-content review is continuing in an additive record. Research remains unimported and unaccepted. See docs/campaign-certification/C01/integrations/CLAUDE-C01-28/README.md. No historical or parent closure.'
p.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n', encoding='utf-8', newline='\n')

replace('tools/avatars/certified_gap_ledger.py', "    '9d97caa9': 'CLAUDE-C01-33',",
        "    'b2098862': 'CLAUDE-C01-29',\n    '9d97caa9': 'CLAUDE-C01-33',")
replace('tools/avatars/certified_gap_ledger.py', 'Japan Socialist Party / Social Democratic Party chairs; submitted with source review held, no accepted mapping',
        'Japan Socialist Party / Social Democratic Party chair observations; bounded intake accepted, no runtime mapping')
replace('tools/avatars/test_certified_boundary_matrix.py', "'25', '27', '30', '32'", "'25', '27', '29', '30', '32'")
replace('tools/avatars/test_certified_gap_ledger.py', "'CLAUDE-C01-27', 'CLAUDE-C01-30'", "'CLAUDE-C01-27', 'CLAUDE-C01-29', 'CLAUDE-C01-30'")
replace('tools/avatars/test_certified_gap_ledger.py', '(28, 29)', '(28,)')
replace('tools/avatars/test_certified_gap_ledger.py', "'CLAUDE-C01-30': ('1b2c1ae2', 39)", "'CLAUDE-C01-29': ('b2098862', 54), 'CLAUDE-C01-30': ('1b2c1ae2', 39)")

p = ROOT / 'docs/AI_WORKSTREAMS.md'
text = p.read_text(encoding='utf-8')
text = text.replace('submitted at `03141c43`, and Japan C01-29 remains held.',
                    'held at review `0719ba7b` with 12 unavailable originals. Japan C01-29 is now accepted\nafter the resumed review recovered all 19 previously unavailable responses.')
marker = '| C01-30: ACDP, Freedom Front and IFP |'
assert marker in text
row = '| C01-29: Japanese Socialist / Social Democratic Party chairs | `3b30478562ec78c2392d0472061e773e23cb25dd` | Bounded intake accepted at `dc65b510`; [review](campaign-certification/C01/integrations/CLAUDE-C01-29/README.md). All 54 original responses and 111 claims reviewed; old held checkpoint preserved. |\n'
text = text.replace(marker, row + marker)
text, count = re.subn(r'^\| Claude research review \|.*$',
    '| Claude research review | C01-23/24/25/27/29/30/32–37 are accepted bounded intake after independent review; parent historical coverage remains open. Russia C01-28 has 56/68 reproduced originals and 12 inaccessible sources, so its new research remains unimported and unaccepted. | [Japan acceptance](campaign-certification/C01/integrations/CLAUDE-C01-29/README.md), [Russia held receipt](campaign-certification/C01/integrations/CLAUDE-C01-28/README.md), and the exact inventory above. |', text, flags=re.M)
assert count == 1
text = text.replace('C01-29 Japan awaits source-review completion.', 'C01-29 Japan is accepted after the complete resumed source review.')
p.write_text(text, encoding='utf-8', newline='\n')

p = ROOT / 'docs/planning/ai-handoffs/CODEX-NEXT-ENGINEERING.md'
text = p.read_text(encoding='utf-8')
start = text.index('- Japan `CLAUDE-C01-29` remains held')
end = text.index('- South Africa `CLAUDE-C01-32`', start)
text = text[:start] + '''- Japan `CLAUDE-C01-29` is **complete as bounded research intake** at `dc65b510`.
  The resumed review recovered all 19 previously unavailable originals: 54/54
  now match, with all 111 claims and sixteen holder observations reviewed.
  [Acceptance](../../campaign-certification/C01/integrations/CLAUDE-C01-29/README.md)
  retains the original held checkpoint and its earlier failures unchanged.
''' + text[end:]
text = text.replace('  chains and submitted test/source claims require independent review.',
                    '  chains remain unaccepted: [the held review](../../campaign-certification/C01/integrations/CLAUDE-C01-28/README.md)\n  reproduced 56/68 originals, with 12 inaccessible. Available-content review is\n  being completed separately; no new Russia research has been imported.')
p.write_text(text, encoding='utf-8', newline='\n')
print('Recorded bounded Japan acceptance and the still-held Russia review; preserved original task scope.')
