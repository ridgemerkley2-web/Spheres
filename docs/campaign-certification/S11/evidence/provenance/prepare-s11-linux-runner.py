import pathlib
base=pathlib.Path(__file__).resolve().parent
source=base/'run-s10-final-linux.py';dest=base/'run-s11-linux.py'
assert not dest.exists()
t=source.read_text(encoding='utf8').replace("out = base / 'evidence/S10-final-linux'", "out = base / 'evidence/S11-final-linux'")
t=t.replace('spheres-s10-final-linux/v1','spheres-s11-linux/v1').replace("base / 'run-s10g-linux.py'","base / 'run-s10-final-linux.py'")
t=t.replace('Full native web and Node suites, native agency/succession integration and party-leadership library tests on Linux.', 'Full native simulation library, web and Node suites, plus eleven ground/shared-force/supply/refit integration targets on Linux.')
old="""        ('agency', ['cargo', 'test', '--locked', '--release', '-p', 'spheres-sim', '--test', 'agency', '--test', 's02_succession', '--no-fail-fast']),
        ('leadership', ['cargo', 'test', '--locked', '--release', '-p', 'spheres-sim', '--lib', 'party_leadership::', '--no-fail-fast']),"""
targets=['ground_equipment_integration','military_operations','campaign_operations','equipment_integration','company_refits','equipment_supply_automation','company_ammunition','ammunition_reserves','aviation_ammunition','campaign_movement_audit','campaign_peace_audit']
new="        ('sim', ['cargo', 'test', '--locked', '--release', '-p', 'spheres-sim', '--lib', '--no-fail-fast']),\n        ('integration', "+repr(['cargo','test','--locked','--release','-p','spheres-sim',*[arg for target in targets for arg in ['--test',target]],'--no-fail-fast'])+"),"
assert old in t;t=t.replace(old,new)
dest.write_text(t,encoding='utf8')
