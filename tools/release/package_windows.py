#!/usr/bin/env python3
"""Package an already-built Windows executable, provenance and reviewable patch.
No compilation, network access, live saves or browser interaction. Existing
release folders are refused; choose a new output root or --name for a new build.
"""
from pathlib import Path
import argparse,hashlib,json,re,shutil,subprocess,zipfile
ROOT=Path(__file__).resolve().parents[2]
DEFAULT_NAME='SPHERES-0.6-Windows'
def package_name(value):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9._-]{0,63}',value) or '..' in value or value.endswith('.') or re.fullmatch(r'CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9]',value.split('.')[0],re.IGNORECASE):
        raise argparse.ArgumentTypeError('Use a simple folder name of 1-64 ASCII letters, digits, hyphens, underscores or single dots, starting with a letter or digit and without a trailing dot; Windows device names are reserved.')
    return value
def git(*args):
    return subprocess.check_output(['git',*args],cwd=ROOT)
def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--binary',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    parser.add_argument('--name',type=package_name,default=DEFAULT_NAME,help='Release folder and ZIP stem; defaults to %(default)s.')
    parser.add_argument('--base',default='4b3116827c0164b62e6b5247fd690bf851ef979d')
    args=parser.parse_args()
    if git('status','--porcelain','--untracked-files=no').strip():
        raise SystemExit('Commit tracked changes before packaging a release identity.')
    revision=git('rev-parse','HEAD').decode().strip()
    release=args.output/args.name
    release.mkdir(parents=True,exist_ok=False)
    shutil.copy2(args.binary,release/'spheres-web.exe')
    launcher='@echo off\nsetlocal\ncd /d "%~dp0"\ntitle SPHERES 0.6\necho Keep this window open while playing. Saves stay in this folder.\n"%~dp0spheres-web.exe" --port 7777\nif errorlevel 1 pause\n'
    (release/'Play SPHERES.cmd').write_bytes(launcher.replace('\n','\r\n').encode('ascii'))
    (release/'START-HERE.txt').write_text('SPHERES 0.6 — Windows portable release\n\nExtract the full ZIP to a writable folder. Double-click Play SPHERES.cmd.\nNo Rust installation or internet connection is needed. Keep the server window open.\nIf port 7777 is already used by another copy, close that copy first, or run\nspheres-web.exe --port 7824 from a terminal in this folder.\n\nUse About to verify the exact build and save location. Use Save before closing.\nNamed campaigns are under saves; the default is save.json. Backup and monthly\nautosave slots are available from the campaign menu. Do not overwrite your only\nold save when testing an upgrade; choose a new named slot.\n\nAdvisor guides the first development chain. Decisions offers peaceful goals.\nSee README.md, CURRENT_ARCHITECTURE.md and PLAYTEST.md for current play rules.\n',encoding='utf-8')
    for name in ['README.md','CURRENT_ARCHITECTURE.md','DECISIONS.md','PLAYTEST.md','PLAYER_DECISIONS.md','MILITARY_OPERATIONS.md','CAMPAIGN_AIMS.md','SECTOR_PROFILES.md','TECH_REFERENCE_REPAIR.md','HEADLESS_BASELINE_2026-09-04.md','DAILY_CALIBRATION.md','PERFORMANCE.md','INVESTMENT_COMPLETION_AUDIT.md','ART_DIRECTION.md']:
        source=ROOT/name
        if source.exists():shutil.copy2(source,release/name)
    provenance=release/'attribution';provenance.mkdir()
    for src,dst in [('spheres-web/data/nation_figures.json','nation-figures-and-rights.json'),('tools/avatars/historical_flags/sources.json','historical-flag-sources.json'),('tools/avatars/README.md','artwork-source-policy.md'),('tools/avatars/flag-icons-LICENSE','flag-icons-LICENSE'),('spheres-web/ui/display-art/manifest.json','display-derivatives.json'),('spheres-sim/data/sectors_1990.json','broad-sector-observations.json')]:
        shutil.copy2(ROOT/src,provenance/dst)
    # Ship reproducible artwork attribution, not the large source PNGs.
    area_sources=[(ROOT/'spheres-web/ui/area-art/manifest.json',provenance/'areas/manifest.json')]
    area_sources.extend((source,provenance/'areas/prompts'/source.name) for source in sorted((ROOT/'tools/area-art/prompts').glob('*.md')))
    for source,target in area_sources:
        if source.is_file():
            target.parent.mkdir(parents=True,exist_ok=True)
            shutil.copy2(source,target)
    metadata=json.loads(subprocess.check_output(['cargo','metadata','--locked','--offline','--format-version','1'],cwd=ROOT))
    dependencies=[]
    for package in metadata['packages']:
        if not package['source']:continue
        entry={k:package.get(k) for k in ['name','version','license','repository']};dependencies.append(entry)
        source=Path(package['manifest_path']).parent
        licenses=[f for f in source.iterdir() if f.is_file() and (f.name.upper().startswith('LICENSE') or f.name.upper().startswith('COPYING') or f.name.upper().startswith('NOTICE'))]
        for f in licenses:
            target=provenance/'rust'/f"{package['name']}-{package['version']}"/f.name
            target.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(f,target)
    (provenance/'rust-dependencies.json').write_text(json.dumps(dependencies,indent=2),encoding='utf-8')
    (release/'release.json').write_text(json.dumps({'version':'0.6.0','revision':revision,'branch':git('branch','--show-current').decode().strip(),'base':args.base,'executable_bytes':args.binary.stat().st_size,'source_repository':'https://github.com/ridgemerkley2-web/Spheres'},indent=2),encoding='utf-8')
    files=sorted(p for p in release.rglob('*') if p.is_file())
    (release/'SHA256SUMS.txt').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+p.relative_to(release).as_posix()+'\n' for p in files),encoding='utf-8')
    archive=args.output/(args.name+'.zip')
    # Cached dependency licenses can predate ZIP's 1980 epoch; clamp only the
    # archive timestamps, preserving copied license contents and their hashes.
    with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED,compresslevel=9,strict_timestamps=False) as z:
        for p in sorted(release.rglob('*')):
            if p.is_file():z.write(p,p.relative_to(args.output))
    # Preserve the original default patch filename for existing release scripts.
    patch=args.output/('SPHERES-0.6-source.patch' if args.name==DEFAULT_NAME else args.name+'-source.patch')
    patch.write_bytes(git('diff','--binary',args.base,revision))
    print(json.dumps({'release':str(release),'zip':str(archive),'zip_bytes':archive.stat().st_size,'sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'patch':str(patch),'revision':revision},indent=2))
if __name__=='__main__':main()
