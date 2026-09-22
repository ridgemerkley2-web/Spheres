"""Build a lagged V-Dem military-removal proxy; no outcome calibration or future data."""
import argparse
import hashlib
import json
import math
import pathlib
import re

ALIASES = {'Congo':'COG', 'Myanmar':'MMR', 'Swaziland':'SWZ', 'Turkey':'TUR', 'USA':'USA', 'Zaire':'COD',
           'USSR':'RUS', 'Czechoslovakia':'CZE', 'Yugoslavia':'SRB'}
UNIT_NOTES = {
 'USSR':'V-Dem country-unit v16 p30 treats the USSR under the continuous RUS unit before 1991; no score is transferred to a later independent successor.',
 'Czechoslovakia':'V-Dem country-unit v16 p29 identifies the CZE unit as Czechoslovakia in 1989; no later Czech score is used.',
 'Yugoslavia':'V-Dem country-unit v16 p28 identifies the SRB unit as the federal Yugoslav polity in 1989; its Slovenia coverage boundary is 1989 (Slovenia is also separately coded). This is executive authority, not a territorial average.',
}
REFUSED = {
 'Germany':'The game starts with unified Germany; the 1989 DEU and DDR source units are separate. No combined executive-removal score is fabricated.',
 'Yemen':'The game starts with unified Yemen; the 1989 YEM and YMD source units are separate. No combined executive-removal score is fabricated.',
}
COMMIT = 'f4dd26922e658442524dfd954bf14f7ebe622d5d'
RDATA_SHA = '39b412d39a061c18f20c98e4ad4d6355b05a0441df31be4ee9aec420dc3d95ea'
EXTRACT_SHA = '362db5c52447501a1403ac31d3abf59fd474768e9301b06042b35cdf1e136894'
CODEBOOK = 'https://www.v-dem.net/documents/70/codebook_v16.pdf'
UNITS = 'https://v-dem.net/documents/72/countryunit_v16.pdf'

def norm(x):
    return re.sub('[^a-z]', '', x.lower())

def finite_unit(x):
    return isinstance(x, (int, float)) and not isinstance(x, bool) and math.isfinite(x) and 0 <= x <= 1

def build(raw, nations, source_sha):
    byiso, byname = {}, {}
    for row in raw:
        if row['year'] != 1989:
            raise ValueError('Only lagged 1989 observations are allowed')
        if row['country_text_id'] in byiso:
            raise ValueError('Duplicate source country')
        if norm(row['country_name']) in byname:
            raise ValueError('Ambiguous normalized source country name')
        byiso[row['country_text_id']] = row
        byname[norm(row['country_name'])] = row
    rows, units, unknown = [], [], []
    if len({n['id'] for n in nations}) != len(nations):
        raise ValueError('Duplicate game roster identity')
    for n in sorted(nations, key=lambda n: n['id']):
        nation = n['id']
        if nation in REFUSED:
            unknown.append({'nation':nation, 'reason':REFUSED[nation]})
            continue
        r = byiso.get(ALIASES.get(nation)) or byname.get(norm(n['name'])) or byname.get(norm(nation))
        if r is None:
            unknown.append({'nation':nation, 'reason':'No 1989 observation in the official v16 country-year extract. Unknown remains distinct from observed zero.'})
            continue
        wh, wg = r['v2ex_hosw'], r['v2ex_hogw']
        if not finite_unit(wh) or not finite_unit(wg) or abs(wh + wg - 1) > 1e-9:
            raise ValueError(f'{nation}: invalid executive weights; no renormalization')
        if r['v2exhoshog'] == 1 and (wh != 1 or wg != 0):
            raise ValueError(f'{nation}: combined executive must be counted exactly once')
        h, g = r['v2exrmhsol_4'], r['v2exrmhgnp_4']
        for weight, value in [(wh, h), (wg, g)]:
            if (value is not None and not finite_unit(value)) or (weight > 0 and value is None):
                raise ValueError(f'{nation}: required source component is missing or invalid')
        value = wh * (h or 0) + wg * (g or 0)
        key = f'vdem-v16-1989-{r["country_text_id"]}'
        rows.append({'nation':nation, 'source_key':key, 'source_year':1989, 'hos_removal':h, 'hog_removal':g,
                     'hos_weight':wh, 'hog_weight':wg, 'assessment':value})
        units.append({'source_key':key, 'country_id':int(r['country_id']), 'country_name':r['country_name'],
                      'country_text_id':r['country_text_id'], 'hos_is_hog':r['v2exhoshog'],
                      'mapping_note':UNIT_NOTES.get(nation, 'Direct 1989 source-unit match; name aliases are crosswalks only, not aggregation.')})
    return {'version':1, 'as_of':'1990-01-01',
      'coverage':f'{len(rows)} of {len(nations)} opening roster countries have a lagged 1989 annual proxy; {len(unknown)} explicitly unknown. The campaign start date is not a source observation date. No future rows, later successors, generated probabilities or country-specific coup triggers.',
      'provenance':{'dataset':'V-Dem Country-Year Dataset v16', 'authors':'Coppedge et al. (2026), Varieties of Democracy (V-Dem) Project',
        'doi':'https://doi.org/10.23696/vdemds26', 'reference_year':1989,
        'source_url':f'https://raw.githubusercontent.com/vdeminstitute/vdemdata/{COMMIT}/data/vdem.RData',
        'source_commit':COMMIT, 'rdata_sha256':RDATA_SHA, 'extract_sha256':source_sha,
        'codebook_url':CODEBOOK, 'country_units_url':UNITS,
        'license':'CC BY-SA 4.0', 'license_url':'https://creativecommons.org/licenses/by-sa/4.0/',
        'license_statement':'https://www.v-dem.net/about/faq/',
        'adaptation':'1989 subset, roster crosswalk, and cabinet-power-weighted HOS/HOG military removal means. This derived data is shared under CC BY-SA 4.0; code has its own repository license.',
        'meaning':'The _4 inputs are mean binary expert coding of practical military power to remove the relevant executive short of military force. Cabinet appointment/dismissal weights identify the relevant executive. Their combination is an explicit gameplay proxy for leverage, not observed troop obedience, a coup probability, or a physical counterforce measurement.',
        'limitations':'Annual 1989 observations can predate late-year changes. Zero is observed absence of this informal removal power, not immunity to underfunding, war or armed coups. Missing inputs stay unknown. The assessment initializes new campaigns only; actual gameplay changes are recorded separately and never refreshed from later historical observations.'},
      'rows':rows, 'units':units, 'not_imported':unknown}

def main():
    p = argparse.ArgumentParser()
    p.add_argument('--source', type=pathlib.Path, required=True)
    p.add_argument('--nations', type=pathlib.Path, required=True)
    p.add_argument('--output', type=pathlib.Path, required=True)
    p.add_argument('--check', action='store_true')
    a = p.parse_args()
    b = a.source.read_bytes()
    if hashlib.sha256(b).hexdigest() != EXTRACT_SHA:
        raise SystemExit('Source extract differs from the pinned official v16 subset')
    nations = [json.loads(f.read_text(encoding='utf-8-sig')) for f in a.nations.glob('*.json')]
    result = build(json.loads(b), nations, hashlib.sha256(b).hexdigest())
    text = json.dumps(result, ensure_ascii=False, indent=2) + '\n'
    if a.check:
        if a.output.read_text(encoding='utf-8') != text:
            raise SystemExit('Army authority catalog differs from its source-derived output')
    else:
        a.output.write_text(text, encoding='utf-8', newline='\n')
    print(f"Army authority: {len(result['rows'])} sourced lagged proxies; {len(result['not_imported'])} explicitly unknown")

if __name__ == '__main__':
    main()
