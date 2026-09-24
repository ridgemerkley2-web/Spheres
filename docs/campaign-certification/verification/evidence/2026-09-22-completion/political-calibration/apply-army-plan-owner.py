from pathlib import Path
from datetime import datetime, timezone
import hashlib,json

here=Path(__file__).resolve().parent
repo=here.parents[1]/'integration'
p=repo/'spheres-sim/src/government.rs'
before=p.read_bytes()
sha=lambda b:hashlib.sha256(b).hexdigest()
old=b'    if n.on_the_books() || crate::programs::enrolled(w, id)\n        || crate::fiscal_recovery::enabled(w)'
new=b'    if n.on_the_books() || n.annual_budget.is_some() || crate::programs::enrolled(w, id)\n        || crate::fiscal_recovery::enabled(w)'
assert before.count(old)==1
assert b'\r' not in before
fixture=(here/'army-plan-owner-regression.rs').read_bytes().replace(b'\r\n',b'\n')
after=before.replace(old,new)
assert after.endswith(b'    }\n}\n')
after=after[:-2]+b'\n'+fixture+b'}\n'
after.decode('utf-8',errors='strict')
wrapper=b'use spheres_sim::world::{GameRules, WorldState, NationId};\nuse spheres_sim::init::world_1990;\nuse spheres_sim::government::{Pillar, GovState, ai_army_funding_floor, tick};\nfn roads_rules(seed:u64)->GameRules { GameRules{seed,ideology_blocs:true,ideology_takeover:true,ai_aggression:0.0,..GameRules::default()} }\nfn state_mut(w:&mut WorldState,id:NationId)->Option<&mut GovState> { w.governments.states.iter_mut().find(|g|g.nation==id) }\n'
fixture_external=fixture.replace(b'crate::',b'spheres_sim::')
(here/'army-plan-owner-original23-witness.rs').write_bytes(wrapper+fixture_external)
rlib=repo.parent/'integration-target/release/deps/libspheres_sim-de80fa3e4f034d8c.rlib'
receipt={'recorded_utc':datetime.now(timezone.utc).isoformat(),'before_sha256':sha(before),
    'after_sha256':sha(after),'fixture_sha256':sha(fixture),'unfixed_rlib_sha256':sha(rlib.read_bytes()),
    'witness_sha256':sha(wrapper+fixture_external),
    'status':'Source applied; fixture and independent unfixed-library witness prepared, neither compiled nor run by this agent',
    'production_change':'Explicit annual plan independently owns fiscal policy even before dollar stocks exist; legacy Army fallback must not clear it.',
    'A1_claim':'No evidence this partial-save boundary explains current closed-book cohort concentration.'}
(here/'army-plan-owner-application.json').write_text(json.dumps(receipt,indent=2)+'\n',encoding='utf-8')
p.write_bytes(after)
print(json.dumps(receipt,indent=2))
