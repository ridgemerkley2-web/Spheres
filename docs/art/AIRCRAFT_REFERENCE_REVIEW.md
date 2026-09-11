# Aircraft reference review

9 September 2026. Research in response to the user's dissatisfaction with the
current aircraft. This document proposes the visual rebuild; it does not claim
any external model has been imported or the aircraft already reworked.

## Assessment of the current models

The previous pass improved curved-surface sampling and increased the two inspection
models to roughly 198k/210k triangles. That did not resolve their overall appearance.
My assessment from the existing preview and source is that the central bodies still
read too much like tubes with attached wings. Intake installations need stronger
integration into the airframe, wing-root transitions need more deliberate sculpting,
and the opaque blue canopy obscures the opportunity for a recognizable cockpit.
Broad, uniform surface colors and generic skin lighting also limit the result.

The next quality gate should be a convincing untextured silhouette in front, side,
top and three-quarter views, then material/close-up review. The user's 100,000+
triangle inspection requirement remains. Detail should be spent on visible curved
forms, separate control surfaces, cockpit interiors, engine fittings and landing
gear, rather than treating tessellation alone as a quality measure.

## Free reference shortlist

License labels below are those displayed on the original asset listings, checked
on 9 September. Only the Hornet and GreenMotion F-16 previews were visually
inspected in the browser in this review. No source archives were downloaded, so
topology, actual triangulated counts and export quality have not been verified.

| Priority | Original asset / creator | Listed license | Purpose and known limitation |
|---|---|---|---|
| First | [High Poly Hornet — ChrisKuhn](https://blendswap.com/blend/8636) | CC-BY | Best immediate reference for a twin-engine tactical aircraft. Preview shows coherent wing/body blends, intake openings, canopy framing and distinct surface treatment. Creator describes modeled landing gear/engine detail, a blocked-out cockpit and basic textures. Some preview decals were added after rendering; the download will need its own material review. |
| First | [F16 jet fighter — GreenMotion](https://blendswap.com/blend/16639) | CC0 | Clean shape study for the compact fighter family: a smoothly blended upper body, defined canopy and coherent wing roots. Visually reviewed as an untextured base, not a finished material benchmark. Blender 2.7-era file. |
| Secondary | [F-15E Strike Eagle Clean model — weeliano](https://blendswap.com/blend/17665) | CC0 | Alternate twin-engine foundation. The creator calls this a basic model. Investigate silhouette/topology before selecting it as a replacement. |
| Secondary | [PC-9 Pilatus Plane — DaSergant](https://blendswap.com/blend/20993) | CC-BY | Candidate for a distinct trainer/light-aircraft family. Avoid making every light aircraft a scaled fighter. Listed Blender 2.7 file; visual inspection pending. |
| Secondary | [MiG-21 Fighter — DaSergant](https://blendswap.com/blend/20994) | CC0 | Candidate for a separate legacy interceptor silhouette. Visual and topology inspection pending. |
| Secondary | [A-10 Thunderbolt II — AIRMAN Magazine / Alfredo Tirado](https://sketchfab.com/3d-models/a-10-thunderbolt-ii-9521ff6ba5d448958ea45a89e94ab23d) | CC Attribution | Original-page indexed metadata lists 54.8k triangles and download availability. Potential dedicated close-support reference. Full-page fetching was restricted in the parent session; archive access and visual quality are unverified. This is below the finished inspection triangle requirement as listed. |

The [Hornet download page](https://blendswap.com/blend/8636/download),
[F-16 download page](https://blendswap.com/blend/16639/download) and
[F-15E download page](https://blendswap.com/blend/17665/download) each require sign-in.
No paid purchase, account creation, source extraction or third-party model import
was performed. Keep source author/license/changes with any future imported asset;
inspect the archive's license version and included textures before redistribution.
CC0 permits copying/adapting/distributing under its public-domain dedication;
see the [Creative Commons CC0 deed](https://creativecommons.org/publicdomain/zero/1.0/).

Not selected for importing: unknown-license model aggregators, and the polished
bohmerang Sketchfab aircraft listings marked noncommercial/personal use. A free
download by itself is not enough to clear an asset for the game.

## Concrete rebuild order

1. Rework one tactical airframe first using the Hornet as the main visual reference
   and the F-16 as a shape-construction comparison. Produce a clay render before
   adding dense fittings. Keep the aircraft an original Spheres concept unless an
   imported historical model is deliberately identified as such.
2. Shape the fuselage, wing-root shoulders, intake lips/throats and engine housings
   together. Give the nose/canopy/tail a consistent design language. Preserve thin
   trailing edges and independent flaps, rudders and stabilizers.
3. Give the canopy real depth, framing and a visible seat/interior. Add a separate
   glass material rather than applying opaque body shading to its blue vertices.
4. Add per-aircraft texture coordinates and material regions: painted skin,
   rougher seams/access panels, metallic exhausts, rubber tires and canopy glass.
   Use restrained surface wear and clear markings. Preserve source attribution if
   any external geometry or textures are adapted.
5. Spend inspection geometry on landing-gear bays/struts, intake interiors,
   exhaust petals and hardware that the viewer can actually inspect. Measure the
   assembled result against the 100k+ floor; maintain small catalogue/map meshes.
6. Keep component sockets and semantic assemblies so wings, engines, sensors,
   protection and stores still respond visibly to design changes. An attractive
   fixed model must not silently replace the configurable equipment system.
7. Compare the rebuilt aircraft beside the current model under identical lighting
   and camera views. Review recognizability, surface transitions, cockpit quality,
   selected-part behavior, GLB export and render responsiveness before expanding
   the treatment to other aircraft families.

The strongest immediate direction is a shape-and-material rebuild of the tactical
airframe, informed by ChrisKuhn's Hornet. The additional assets form a research
queue, not a claim that a complete usable aircraft library has already been acquired.

## Implemented reference pass — 9 September

The original tactical aircraft has been rebuilt with blended shoulders, lower
open intakes, thinner swept wings, separate trailing controls, underwing fuel
tanks, an actual cockpit opening and a transparent canopy around two seats and
instruments. Skin seams follow the body surface; exhausts retain their separate
petals and actuator detail. The current default inspection build has 211,780
triangles, with 13,752 at catalogue detail and 1,696 at map detail.

The Hornet and F-16 informed visual proportions and assembly relationships.
Their files were not imported or redistributed. This remains a configurable
Spheres concept, not an F/A-18 reproduction. UV textures, liveries and adapting
additional reference families are still outstanding; the present finish uses
vertex colors, aircraft-specific shading and a portable alpha material for glass.
