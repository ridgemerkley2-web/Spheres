"""Reproduce the small source subset from the official RData (requires pyreadr)."""
import argparse
import hashlib
import pathlib

from build_army_authority import EXTRACT_SHA, RDATA_SHA

def main():
    p = argparse.ArgumentParser()
    p.add_argument('rdata', type=pathlib.Path)
    p.add_argument('output', type=pathlib.Path)
    a = p.parse_args()
    if hashlib.sha256(a.rdata.read_bytes()).hexdigest() != RDATA_SHA:
        raise SystemExit('RData differs from the pinned official v16 source')
    import pyreadr
    data = pyreadr.read_r(str(a.rdata))['vdem']
    wanted = {'country_name', 'country_text_id', 'country_id', 'year', 'v2exhoshog',
              'v2exrmhsol_4', 'v2ex_hogw', 'v2exrmhgnp_4', 'v2ex_hosw', 'v2x_ex_military'}
    columns = [name for name in data if name in wanted]
    if set(columns) != wanted:
        raise SystemExit('Official source columns are incomplete')
    text = data.loc[data['year'] == 1989, columns].to_json(orient='records', indent=2)
    encoded = text.encode('utf-8')
    if hashlib.sha256(encoded).hexdigest() != EXTRACT_SHA:
        raise SystemExit('Extraction differs from the reviewed subset; no output written')
    a.output.write_bytes(encoded)
    print('Verified official RData and reproduced the exact 158-row 1989 source subset')

if __name__ == '__main__':
    main()
