"""Mirror completed S12-S15 markers in the readable roadmap after qualification."""
import json,pathlib
base=pathlib.Path(__file__).resolve().parent;repo=base/'integration'
road=json.loads((repo/'docs/planning/campaign-pathway.json').read_text())
for key in ['S12','S13','S14','S15']:
    assert next(s for s in road['sessions'] if s['id']==key)['status']=='complete'
assert next(s for s in road['sessions'] if s['id']=='S16')['status']=='planned'
p=repo/'docs/CERTIFIED_CAMPAIGN_PATHWAY.md';before=p.read_text(encoding='utf8');text=before
assert 'S01–S11 complete; S12–S30 planned.' in text
text=text.replace('S01–S11 complete; S12–S30 planned.','S01–S15 complete; S16–S30 planned.')
text=text.replace('S01–S11 are complete; S12–S30 remain planned.','S01–S15 are complete; S16–S30 remain planned.')
text=text.replace('C01 and worldwide character work remain incomplete; CP1 is not earned.',
 'C01 and worldwide character work remain incomplete; CP1 is not earned.\nS12–S15 complete the qualified squadron, basing, support and tactical-mission loop.\n[Combined qualification and review build](campaign-certification/S15/README.md).\nExecution stops after S15; S16 remains planned, and G3 is not yet earned.')
text=text.replace('records the qualified build and complete evidence. S12 remains planned.',
 'records the qualified ground build and evidence. Subsequent flight work is qualified in the [S12–S15 record](campaign-certification/S15/README.md).')
for n in range(12,16):
    start=text.index(f'<a id="s{n}"></a>');end=text.index(f'<a id="s{n+1}"></a>',start)
    section=text[start:end];assert '**Status:** Planned' in section and section.count('- [ ]')==3
    section=section.replace('**Status:** Planned','**Status:** Complete',1).replace('- [ ]','- [x]')
    section=section.replace('**Completion marker:**',f'[S{n} completion record](campaign-certification/S{n}/README.md) · [shared evidence](campaign-certification/S15/manifest.json).\n\n**Completion marker:**',1)
    text=text[:start]+section+text[end:]
assert text!=before
p.write_text(text,encoding='utf8',newline='\n')
print('Updated readable S12-S15 markers; S16 and later sessions remain planned.')
