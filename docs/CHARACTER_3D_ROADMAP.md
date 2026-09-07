# Physical historical characters

The requested direction is **actual rotatable 3D cartoon characters**. This supersedes the earlier generated headshots and full-body raster studies. Those images remain as production history; none is registered as an accepted person avatar. National-selector artwork remains a separate national-symbol system.

## Working pilot

Four authored models cover Margaret Thatcher, Neil Kinnock, Paddy Ashdown and George H. W. Bush, using an explicitly limited early-1990s appearance window. They are labelled **likeness studies**, not finished portrait sculptures or approved likenesses. These are physical triangle meshes with sculpted facial features, hair, jackets, hands, shoes and individual accessories. They have no skeleton, facial animation or automatic ageing yet.

Government displays compact models with left/right controls. The larger viewer supports pointer/touch rotation, arrow keys, zoom buttons, +/−, Home/reset and a downloadable GLB. The viewer does not send campaign commands. Missing people and out-of-era appearances retain a clearly labelled pending avatar instead of being assigned somebody else's face.

The existing Arsenal3D renderer supplies one shared WebGL2 context, a bounded geometry cache and studio lighting. Character materials bypass vehicle weathering. Each card uses a 2D canvas copy of a real 3D render, while manual rotation and the larger viewer redraw the underlying geometry. There is no idle spinning or WebGL context per party. Camera listeners, observers and scheduled frames are disposed on Government redraw/close.

| File | Responsibility |
| --- | --- |
| `spheres-sim/data/party_leaders.json` | Historical people, explicit party terms, source gaps and executive identity links |
| `spheres-web/data/person_models.json` | Exact person/model identity, half-open appearance era, artistic profile, credit and study status |
| `spheres-web/ui/person-models.js` | Original authored geometry, shared by browser and export |
| `spheres-web/ui/person-3d.js` | Read-only camera and modal lifecycle |
| `tools/ui/build_person_models.cjs` | Deterministic physical GLB exports; `--check` proves exact agreement |
| `spheres-web/ui/person-models/*.glb` | Downloadable geometry with source/era/person metadata and named mesh-part ranges |

Build with `node tools/ui/build_person_models.cjs`. Verify with `node tools/ui/build_person_models.cjs --check` and `node --test tools/ui/check_person_models.cjs`. The exporter preserves equipment's existing default metadata; character exports use Character Studio metadata.

## Production sequence

1. **Approve the physical style.** Inspect the pilot at card size and in the viewer, front/profile/back. Refine head/body proportions, expressions, clothes and hair as a group. Likeness studies are not automatically promoted to finished work by passing geometry tests.
2. **Finish the first party set.** Complete reviewed 1990 SNP and Plaid leaders, then UK succession candidates and the current government's most visible opposition. Add an explicit researched appearance profile and era for each identity. Do not use a generic random person as a historical face.
3. **Commission production sculpts.** Keep stable person IDs and the existing model/era contract. Author editable source scenes, clean topology, eye/mouth geometry, separate clothing/accessories and material assignments. Imported GLBs will need a validated loader/provider before replacing procedural meshes; that loader is not implemented in this pilot.
4. **Add a shared rig.** Specify humanoid joints, neutral pose, hand proportions, facial blend shapes and restrained greeting/idle animations. Respect reduced motion. Political party and government role are campaign records, not features embedded in the mesh.
5. **Add appearance eras.** Research hairstyles, facial hair and ageing references for each person. Use reviewed era variants, not 37 independent guessed annual portraits. Display a variant only inside its documented visual era, even when gameplay keeps a person in office beyond their historical term.
6. **Scale the catalogue.** Use head/wardrobe/accessory templates with individual face sculpts, source records, stable IDs and a review queue. Prioritize every represented party, including small coalition components. The 624 game-party inventory is not a complete inventory of real-world political organizations.
7. **Ship performance tiers.** Produce near/card/map LODs, atlas materials, measured triangle/texture budgets and lazy downloads for a large roster. The four current models use roughly 24–31k triangles each and remain inside the existing shared geometry cache; a worldwide roster must not embed full-detail geometry for hundreds of people in the initial page payload.

## Historical foundation and remaining coverage

The person registry contains 284 identities and 132 explicit original-executive links. Most were transcribed from the game's existing sourced office records; this import establishes identities, not verified party leadership. The researched UK pilot contains 43 people and 51 terms across four game rows, with SNP/Plaid components kept separate. All 624 party histories still contain gaps; none claims complete 1990–2026 verification. The research cutoff is 7 September 2026.

An existing campaign can display its intact original executive through the exact authored office link without enabling historical succession or changing its save. New campaigns opt into saved historical candidates. Both paths remove an executive model when the corresponding person identity no longer holds that office. Historical browsing is read-only.

The full national catalogue, later appearance variants, minor-party inventory expansion, rigs and production likeness review remain future work. This pilot makes that work visible and playable without claiming hundreds of finished characters.
