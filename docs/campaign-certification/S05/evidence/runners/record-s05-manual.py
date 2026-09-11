import json, pathlib, hashlib, datetime
root=pathlib.Path(__file__).resolve().parent
def save(folder, value):
    path=root/'evidence'/folder/'visual-observations.json'
    assert not path.exists()
    path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
save('S05-live-map-final', {
 'candidate':'261897909e12564275b802f99e3301adbfad61b2',
 'binary_sha256':'6a3276b53f83b485b770a5632bd18134f0109574d72e3b3e06593a053151e44f',
 'method':'Actual in-app browser, visible controls, native campaign loading/saving, screenshot inspection and rendered district cards; screenshots emitted inline in the task, no local screenshot artifact claimed.',
 'viewport_observed':[1280,720],
 'france_save_date':'1990-03-22',
 'checks':[
  {'site':'Paris','zoom':1500,'result':'Visible narrow Seine corridor clear of generated buildings; ground pick identifies Ile-de-France / FRA_le-de-france.'},
  {'site':'Singapore','zoom':1500,'result':'Generated city stops at visible dark ocean coast; dry buildings retained; pick opens Singapore national dossier.'},
  {'site':'Kathmandu','zoom':[128,1500],'result':'Peaks and valley visible, camera above terrain; ground pick identifies Bagmati / NP-BA.'},
  {'site':'Chicago','zoom':[247.8,1500],'result':'Lake Michigan edge visible, city on land side; ground pick identifies Illinois / US-IL.'},
  {'site':'World','zoom':1,'result':'World overview remains available.'},
  {'site':'Ljubljana','date':'2000-02-01','zoom':[128,1500],'result':'Untouched original legacy successor checkpoint loaded through confirmed saved-campaign UI; Slovenia alive with 12 districts; city and ground visible; ground pick identifies Osrednjeslovenska / SVN_osrednjeslovenska / Slovenia. Saved native slot successor-reviewed.'}
 ],
 'failed_check': 'Raised floating Guidance launcher clears dock but overlaps natural Close city target. This candidate is not the final layout. Campaign launcher removed in 52560db; existing Advisors/Tutorial/F1 retained.',
 'limits':['Procedural city blocks, not surveyed buildings. DEM sampling resolution unchanged (about 1855 metres).','Geneva has no authored Lake Geneva coverage: retained gap owned by later map/content work, not recorded as a lake pass.','Namibia and East Timor lack authored activation sites; no invented activation claim. Existing modeled Slovenia successor used for reload and picking.','Portrait/history accuracy in this legacy campaign is not certified by this map check.']
})
save('S05-live-map-93a49f5', {
 'candidate':'93a49f58e999e65c2df1649c62d9f21ba8ea1d6a',
 'binary_sha256':'183558cfc758128da47c04d08fc0eb4918dce681f0fc95c804364992c30f3997',
 'url':'http://127.0.0.1:59461/',
 'method':'Actual in-app browser. Confirmed native load of previously saved successor-reviewed slot; no capability adoption, no time advance. Visible Find, Zoom in and Close city controls; rendered district card read after ground click.',
 'date':'2000-02-01','player':'United States','zoom':1500,
 'viewport_observed':[2555,1272],
 'viewport_note':'Requested 1280x720 via browser viewport capability but rendered DOM remained 2555x1272; recorded actual dimensions. Temporary override reset afterward.',
 'checks':[
  'Ljubljana Find opens city card; natural Close city click closes it and does not open Guidance. Campaign uses existing Advisors/Tutorial entrypoints; no redundant floating launcher.',
  'At 1500x terrain camera remains above ground and generated Ljubljana blocks visible.',
  'Ground click at viewport (1190,705) opens Osrednjeslovenska / Slovenia / SVN_osrednjeslovenska, population 530k, area 2587 square kilometres, terrain mountain, GDP $5.324bn.',
  'Province Close succeeds; clock remains paused at 1 Feb 2000; browser warning/error log read is empty.'
 ],
 'scope':'Confirms modeled successor map, picking and city controls after a native roundtrip. Assets unchanged from 2618979. This is a manual pre-optimization build, not a claim of manual inspection on final db9d17c; final CI is separately pinned.',
 'environment_note':'Earlier IAB navigation/asset startup stalled for minutes while direct local HTTP responses succeeded; completed after session reset. Cause not established, not claimed as an application or security-policy diagnosis.',
 'recorded_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()
})
src=(root/'run-s05-external-final.py').read_text(encoding='utf-8')
src=src.replace('S05-external-final','S05-external-runtime-final').replace('261897909e12564275b802f99e3301adbfad61b2','db9d17c8d726aa102aa143ceb3599009c558ffee')
(root/'run-s05-external-runtime-final.py').write_text(src,encoding='utf-8')
print('Manual observations and final external runner recorded.')
