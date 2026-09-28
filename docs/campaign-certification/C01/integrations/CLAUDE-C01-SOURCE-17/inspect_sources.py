import json, pathlib, hashlib
import pypdfium2 as pdfium
from PIL import Image, ImageDraw
BASE=pathlib.Path(__file__).resolve().parent
OUT=BASE/'renders'; OUT.mkdir(exist_ok=True)
def render(file, pages, label):
    pdf=pdfium.PdfDocument(str(BASE/'responses'/file))
    notes={'file':file,'pages':len(pdf),'metadata':pdf.get_metadata_dict(),'renders':[]}
    for number in pages:
        page=pdf[number-1]
        image=page.render(scale=1.55).to_pil()
        target=OUT/f'{label}-p{number}.jpg'; image.save(target,quality=86)
        notes['renders'].append({'page_one_based':number,'path':str(target.relative_to(BASE)), 'sha256':hashlib.sha256(target.read_bytes()).hexdigest()})
    (BASE/f'pdf-{label}.json').write_text(json.dumps(notes,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
render('25.pdf',[1,165],'snd4')
render('34.pdf',[43],'bulletin2')
render('35.pdf',[1,5,8],'ved35')
render('37.pdf',[1,5,39],'ved36')
render('39.pdf',[1,4,15,18],'ved37')
render('40.pdf',[1,4,20],'ved41')
for stem in ['29','30','31','32','03','04','06']:
    image=Image.open(BASE/'responses'/f'{stem}.jpg')
    image.resize((image.width*3,image.height*3)).save(OUT/f'facsimile-{stem}.jpg',quality=92)
if (BASE/'responses/01.pdf').exists():
    render('01.pdf',[1,2,3,4,5,6,7,8,18,19,20,23,26],'dcn')
print('rendered relevant source-review correction and identity pages')
