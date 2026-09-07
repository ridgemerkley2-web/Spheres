"""Generate the initial production scope. Do not rerun over a tracked progress register.

Uses only the standard library. Existing DECK IDs/names come from the current
simulation; proposed assets are explicitly visual-only until mapped by engineering.
"""
from pathlib import Path
import csv
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).with_name('3D_ASSET_BACKLOG.csv')
rows = []

def add(asset_id, name, category, kind, phase, game_id='', baseline='Proposed', brief='', dependency='P0 pipeline', variants='LOD0/1/2; neutral previews; fallback'):
    rows.append(dict(asset_id=asset_id, name=name, category=category, deliverable_type=kind,
        phase=phase, game_id=game_id or 'visual_only', baseline=baseline,
        status='planned', integration_status='not_integrated', dependencies=dependency,
        required_variants=variants, acceptance_brief=brief,
        source_path='', output_path='', provenance='', review_evidence=''))

# Exact current catalogue coverage; do not rename these game identities.
source = (ROOT / 'spheres-sim/src/arsenal.rs').read_text(encoding='utf-8')
deck = source.split('pub const DECK: &[EquipmentDef] = &[', 1)[1].split('\n];', 1)[0]
for game_id, name, category in re.findall(r'kit\("([^"\n]+)",\s*"([^"\n]+)",\s*Class::(\w+)', deck):
    add('catalogue.' + game_id + '.v1', name, 'Existing catalogue / ' + category,
        'base_asset', 'P3', game_id, 'Existing procedural model',
        'Refine existing recognizable silhouette; exact DECK identity; shared renderer; retain glyph fallback.',
        variants='LOD0/1/2; existing game name; representative formation/refit label where applicable')
assert len(rows) == 46, f'Catalogue changed: inspect expected coverage, found {len(rows)}'
for row in rows:
    row['integration_status'] = 'existing_baseline_upgrade_pending'

ground = [('tank_standard','Main battle tank'),('tank_heavy','Heavy tank'),('tank_light','Light tank'),
    ('tank_destroyer','Tank destroyer'),('ground_ifv','Infantry fighting vehicle'),
    ('ground_apc','Armored personnel carrier'),('ground_recon','Reconnaissance vehicle'),
    ('ground_artillery','Self-propelled artillery'),('ground_air_defense','Mobile air defense')]
for game_id, name in ground:
    add('designer.' + game_id + '.v1',name,'Ground designer','modular_platform',
        'P0' if game_id in ['tank_standard','ground_apc'] else 'P1',game_id,
        'Existing configurable model','Improve current geometry; exact slot identity and picking; legal module variants; portable export.',
        variants='Baseline plus all legal current component swaps; olive/sand/winter; LOD0/1/2')
    rows[-1]['integration_status'] = 'existing_baseline_upgrade_pending'

component_kits = {
 'mobility':'Powertrain, exhaust and cooling fixtures', 'transmission':'Transmission access and removable display pack',
 'tracks':'Standard, wide and padded tracks', 'wheels':'Six/eight-wheel running gear and tire variants',
 'suspension':'Suspension and wheel assemblies', 'turret':'Turrets, casemate and specialist weapon stations',
 'armament':'External barrels and specialist weapon modules', 'ammunition':'Stowage and inspection bay representations',
 'protection':'Hull and modular armor panels', 'active_protection':'Countermeasure and protection fittings',
 'sensors':'Day/night/thermal sight fittings', 'fire_control':'Fire-control housings and inspection representation',
 'communications':'Radio antennas and data-link fittings', 'troop_compartment':'Troop bays and rear access',
 'recon_package':'Scout sights, mast and observation package', 'artillery_loader':'Loading and stowage fittings',
 'radar':'Search and tracking arrays'}
for slot, name in component_kits.items():
    add('components.ground.'+slot+'.v1',name,'Ground components','component_kit','P1',slot,
        'Existing slot; geometry refinement required',
        'Enumerate every compatible component ID from actual registry; maintain per-platform legality, part metadata and pivots.',
        variants='Every implemented choice in this slot; LOD0/1/2; no invented stats')

projects = {
 'infrastructure':'Road, bridge and utility corridor', 'civilian_industry':'Civilian factory compound',
 'power_grid':'Power substation and transmission yard', 'research_center':'Research campus',
 'arms_plant':'Arms assembly plant', 'machinery_works':'Machinery works', 'generation':'Power generation compound',
 'processing_plant':'Processing plant', 'freight_terminal':'Freight interchange terminal', 'warehouse':'Warehouse compound',
 'automation':'Automation retrofit kit', 'efficiency':'Efficiency retrofit kit', 'starter_industry':'Starter workshop compound'}
for game_id, name in projects.items():
    retrofit = game_id in ['automation','efficiency']
    add('construction.'+game_id+'.v1',name,'Construction','retrofit_kit' if retrofit else 'facility_assembly',
        'P0' if game_id=='arms_plant' else 'P2',game_id,
        'Game project exists; new 3D asset planned',
        'Bind appearance to actual work and level; no fictional capacity; retrofit host facility rather than duplicate it.' if retrofit else
        'Five work stages; reusable yard; meaningful level expansion; project pin connects to actual province record.',
        variants='Installation stages and completed upgrade' if retrofit else 'Site/foundation/frame/enclosed/complete; paused overlay; level expansion')

def batch(category, phase, prefix, items, kind='base_asset', brief='', variants='LOD0/1/2; neutral previews; fallback'):
    for key, name in items:
        add(prefix+'.'+key+'.v1',name,category,kind,phase,brief=brief,variants=variants)

batch('Future ground support','P6','support',[
 ('logistics','Logistics truck'),('fuel','Fuel truck'),('recovery','Armored recovery vehicle'),
 ('engineer','Combat engineer vehicle'),('bridge','Bridge layer'),('command','Command vehicle'),
 ('ambulance','Field ambulance'),('missile','Missile carrier'),('radar','Radar carrier'),
 ('rocket','Multiple-launch rocket vehicle'),('towed_artillery','Towed artillery'),('unmanned','Unmanned ground scout')],
 brief='Original distinct exterior silhouette; reusable chassis/sockets; visual-only until gameplay support exists.')

batch('Future air designer','P4','air',[
 ('light_fighter','Light fighter'),('multirole','Twin-engine multirole aircraft'),('stealth_fighter','Stealth fighter'),
 ('interceptor','Interceptor'),('close_support','Close-air-support aircraft'),('bomber','Strategic bomber'),
 ('flying_wing','Flying-wing bomber'),('transport','Military transport aircraft'),('tanker','Aerial tanker'),
 ('early_warning','Airborne early-warning aircraft'),('electronic','Electronic warfare aircraft'),
 ('maritime','Maritime patrol aircraft'),('trainer','Trainer aircraft'),('scout_drone','Scout drone'),
 ('armed_drone','Armed drone'),('stealth_drone','Stealth drone'),('attack_helicopter','Attack helicopter'),
 ('utility_helicopter','Utility helicopter'),('heavy_helicopter','Heavy-lift helicopter')],kind='modular_platform',
 brief='Distinct fuselage; future compatible module sockets; gear contact points; retain static-only status until designer implementation.')
batch('Air components','P4','components.air',[
 ('wings','Wings and control surfaces'),('tails','Tail assemblies'),('engines','Engines, intakes and nacelles'),
 ('gear','Landing gear'),('cockpit','Cockpit and canopy'),('sensors','Nose sensors and avionics representation'),
 ('pylons','Pylons and external store adapters'),('stores','External stores and tanks'),
 ('countermeasures','Countermeasure fittings'),('rotors','Rotor, hub and tail assemblies')],kind='component_kit',
 brief='Document compatible families and pivot/socket metadata; no universal mix-and-match promise.')

batch('Future naval designer','P5','naval',[
 ('patrol','Patrol boat'),('missile_boat','Missile boat'),('corvette','Corvette'),('frigate','Frigate'),
 ('destroyer','Destroyer'),('cruiser','Cruiser'),('carrier','Aircraft carrier'),('amphibious','Helicopter/amphibious ship'),
 ('landing','Landing craft'),('conventional_sub','Conventional submarine'),('attack_sub','Nuclear attack submarine'),
 ('strategic_sub','Strategic submarine'),('replenishment','Fleet replenishment ship'),('mine_countermeasure','Mine-countermeasure vessel')],
 kind='modular_platform',brief='Recognizable class silhouette; waterline root; authored deck sockets; future rules explicit.')
batch('Naval components','P5','components.naval',[
 ('bridge','Bridge and superstructure'),('mast','Masts and sensor arrays'),('funnel','Funnels and propulsion fittings'),
 ('deck_mount','Deck weapon and defense mounts'),('launch','Closed launch-module visuals'),('hangar','Hangars and flight decks'),
 ('boats','Ship boats and davits'),('deck_props','Carrier aircraft and deck service props')],kind='component_kit',
 brief='Reusable fittings with class-specific placement and no clipping; preserve waterline scale.')

batch('Infantry and abstract systems','P6','personnel',[
 ('human','Strategy-scale human base rig'),('gear','Uniform, helmet, pack and radio kit'),
 ('field_team','Infantry representative diorama'),('mechanized','Mechanized formation diorama'),
 ('command','Field command and network terminals'),('maintenance','Civilian and military maintenance crew')],
 kind='assembly_kit',brief='Readable external gear; shared low-detail rig; standing/walking/carrying/idle; representative scale, no literal formation count.')
batch('Missile support','P6','ordnance',[
 ('canisters','Storage and transport canisters'),('trailers','Transport and launcher trailers'),
 ('battery','Air-defense battery scene kit'),('display','Munition inspection supports')],kind='assembly_kit',
 brief='External visual context for existing catalogue assets; no changes to simulation capability.')
batch('Space','P6','space',[
 ('observation','Observation satellite'),('communications','Communications satellite'),('navigation','Navigation satellite'),
 ('radar','Radar satellite'),('small_bus','Small-satellite bus'),('ground_station','Ground antenna station'),
 ('launch_vehicle','Launch vehicle'),('launch_pad','Launch-pad assembly')],
 brief='Reuse existing satellite meshes where appropriate; named panel/antenna pivots; future entries remain visual-only.')

batch('Resources and industrial facilities','P6','industry',[
 ('open_pit','Open-pit mine'),('underground','Underground mine entrance'),('quarry','Stone quarry'),
 ('oil_field','Oil-field equipment'),('offshore','Offshore platform'),('gas','Gas processing facility'),
 ('refinery','Oil refinery'),('ore','Ore processing facility'),('steel','Steel works'),('cement','Cement plant'),
 ('farm','Farm compound'),('grain','Grain silos'),('greenhouse','Greenhouse compound'),('timber','Timber yard'),
 ('fishing','Fishing harbor'),('water','Water treatment works'),('recycling','Recycling facility'),
 ('wind','Wind generation variant'),('solar','Solar generation variant'),('hydro','Hydroelectric generation variant'),
 ('thermal','Thermal generation variant'),('nuclear','Nuclear generation exterior variant')],kind='facility_assembly',
 brief='Match actual facility/technology if integrated; otherwise representative scenery; shared construction/yard parts; no invented resource yields.')
batch('Civilian vehicles','P2','transport',[
 ('car','Passenger car'),('bus','Bus'),('van','Delivery van'),('truck','Articulated freight truck'),
 ('tanker','Road tanker'),('excavator','Excavator'),('bulldozer','Bulldozer'),('mobile_crane','Mobile crane'),
 ('locomotive','Locomotive'),('coach','Passenger rail coach'),('container_wagon','Container wagon'),
 ('grain_wagon','Grain wagon'),('tank_wagon','Tank wagon'),('barge','Inland cargo barge'),
 ('container_ship','Container ship'),('bulk_carrier','Bulk carrier'),('oil_tanker','Oil tanker'),
 ('ferry','Ferry'),('cargo_plane','Cargo aircraft')],
 brief='Map LOD first; bounded traffic; share chassis where appropriate; movement is not cargo accounting.')
batch('Transport networks','P2','network',[
 ('roads','Road straights, bends and junctions'),('highway','Divided-highway kit'),('rail','Rail, curves and switches'),
 ('bridges','Bridge span, deck and pier kit'),('tunnels','Tunnel portal kit'),('station','Rail station'),
 ('airport','Runway, taxiway, gate and hangar kit'),('quay','Port quay and breakwater kit'),
 ('cranes','Port container and bulk cranes')],kind='assembly_kit',
 brief='Compatible endpoints and terrain anchors; spline-ready where appropriate; placement follows available geography.')

batch('Urban core','P2','urban',[
 ('house','Detached house'),('row_house','Row houses'),('low_apartment','Low-rise apartments'),
 ('mid_apartment','Mid-rise apartments'),('high_apartment','High-rise apartments'),('office','Office blocks'),
 ('shop','Storefront and market'),('warehouse','Urban warehouse'),('civic','Civic building'),('school','School'),
 ('hospital','Hospital'),('university','University'),('stadium','Stadium'),('park','Park and public space'),
 ('utility','Neighborhood utilities')],kind='building_kit',brief='Shared footprints; representative density and regional materials; readable map LOD.')
add('urban.temperate_block.v1','Representative temperate town block','Urban core','scene_assembly','P0',
    brief='Small reusable mixed-use block; prove terrain placement, label clearance, map LOD and close-up readability.')
batch('Regional architecture','P6','region',[
 ('temperate','Temperate masonry'),('north_american','North American grid and suburb'),
 ('east_asian','East Asian dense-city kit'),('mediterranean','Mediterranean urban kit'),
 ('arid','Arid courtyard kit'),('tropical','Tropical mixed-density kit'),
 ('continental','Continental apartment-estate kit'),('high_latitude','High-latitude settlement kit')],
 kind='style_kit',brief='Variants of urban core; era/climate tags; mixed usage rather than one national stereotype.',
 variants='Roof/wall/material/footprint variants; population-scale clusters; shared LODs')
batch('Terrain detail','P2','terrain',[
 ('deciduous','Deciduous tree kit'),('conifer','Conifer tree kit'),('tropical','Tropical tree kit'),
 ('scrub','Scrub and shrubs'),('grass','Grass patches'),('rock','Rock clusters'),('cliff','Cliff dressing'),
 ('snow','Snow-edge dressing'),('shore','Shoreline dressing'),('riverbank','River-bank dressing'),
 ('fields','Agricultural field patches')],kind='scatter_kit',
 brief='Preserve measured elevation/coastlines; deterministic visual placement; cull distant instances; slope/water/label exclusions.')
batch('Shared construction props','P2','site',[
 ('foundation','Foundation and excavation kit'),('structure','Structural frame kit'),('scaffold','Scaffolding and tower crane kit'),
 ('yard','Containers, barriers and service yard'),('retrofit','Factory retrofit machinery kit')],kind='assembly_kit',
 brief='Reusable project stages driven by actual funded progress; paused sites stop activity.')

scenes = [('menu','Main menu'),('nation','Nation selection'),('economy','Economy'),('budget','Treasury and budget'),
 ('construction','Construction manager'),('industry','Industry'),('resources','Resources'),('research','Research campus'),
 ('chassis','Chassis research'),('engines','Engine research'),('weapons','Weapon research'),('armor','Armor research'),
 ('optics','Optics research'),('communications','Communications research'),('library','Equipment library'),
 ('ground','Ground designer'),('air','Aircraft hangar'),('naval','Naval dockyard'),('production','Equipment production'),
 ('service','Service and refit'),('logistics','Logistics'),('diplomacy','Diplomacy'),('intelligence','Intelligence'),
 ('government','Domestic government'),('history','World and history'),('help','Help and onboarding')]
batch('Page backgrounds','P7','scene',scenes,kind='scene_assembly',
 brief='Reuse accepted assets; text-safe composition; neutral interactive-object lighting; no input obstruction.',
 variants='Desktop/narrow composition; still fallback; reduced-motion option; optional bounded animation')

ids = [r['asset_id'] for r in rows]
assert len(ids) == len(set(ids)), 'Duplicate asset identity'
if OUT.exists():
    raise SystemExit('Backlog already exists: preserve manual progress; use a new output for comparison.')
with OUT.open('w',newline='',encoding='utf-8-sig') as f:
    writer = csv.DictWriter(f,fieldnames=list(rows[0]))
    writer.writeheader()
    writer.writerows(rows)
from collections import Counter
print(f'{len(rows)} production work packages written to {OUT}')
print('Phases:',dict(sorted(Counter(r['phase'] for r in rows).items())))
print('Types:',dict(sorted(Counter(r['deliverable_type'] for r in rows).items())))
