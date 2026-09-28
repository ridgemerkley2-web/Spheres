import json,pathlib,subprocess
P=pathlib.Path(__file__).parent
D=json.loads((P/'all-inputs.json').read_text(encoding='utf-8'))
exe='C:/Users/ridge/.cache/codex-runtimes/codex-primary-runtime/dependencies/native/poppler/Library/bin/pdftoppm.exe'
(P/'renders').mkdir(exist_ok=True)
for i,pages in {0:[202,203,246,268],2:[1],3:[28,31,85],7:[1,2],8:[1],9:[1,2],10:[310,438],51:[1],63:[6,7,8,9,12],65:[8],68:[2,6,36],69:[1],70:[1],72:[6,7,36]}.items():
    for n in pages:
        out=P/'renders'/f'{i:02d}-p{n}'
        subprocess.run([exe,'-f',str(n),'-l',str(n),'-scale-to','1450','-singlefile','-png',D[i]['body'],str(out)],check=True,capture_output=True)
subprocess.run([exe,'-f','1','-l','1','-scale-to','1450','-singlefile','-png',str(P.parent/'in27-source-preparation/bodies/advani-date-anchor.pdf'),str(P/'renders/anchor-p1')],check=True,capture_output=True)
print('rendered',len(list((P/'renders').glob('*.png'))),'cited pages')
