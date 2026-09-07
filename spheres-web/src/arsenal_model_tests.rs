use super::*;

#[test]
fn claude_models_cover_the_live_equipment_deck_in_both_directions() {
    for def in spheres_sim::arsenal::registry() {
        assert!(ARSENAL_MODELS_JS.contains(&format!("\n    {}: {{",def.id)),"Missing model for {}",def.id);
        assert!(ARSENAL_MODELS_JS.contains(&format!("name: \"{}\"",def.name)),"Model name differs for {}",def.id);
    }
    assert_eq!(ARSENAL_MODELS_JS.matches("build(m) {").count(),spheres_sim::arsenal::registry().len());
    for class in ["infantry","armour","air","naval","missile","space"] {assert!(ARSENAL_MODELS_JS.contains(&format!("{class}: \"")));}
}

#[test]
fn catalogue_renderer_is_shared_and_fallback_keeps_existing_controls() {
    assert_eq!(ARSENAL3D_JS.matches("getContext(\"webgl2\"").count(),1);
    assert!(!ARSENAL_MODELS_JS.contains("document.")&&!ARSENAL_MODELS_JS.contains("getContext("));
    for path in ["arsenal-models.js","arsenal3d.js","arsenal3d.css"] {assert!(INDEX.contains(path));}
    assert!(INDEX.contains("Arsenal3D.scan(body)"));
    assert!(INDEX.contains("<b class=\"mark\">${manufacturingClassMark(line.class)}</b>"));
    assert!(ARSENAL3D_CSS.contains(".equipment-choice.has-kit3d::before { display: none; }"));
}

#[test]
fn component_illustrations_are_embedded_and_allowlisted() {
    for key in ["chassis","engines","weapons","armor","optics","communications"] {
        let bytes=page_art_assets::component_asset(&format!("{key}-v1.webp")).unwrap();
        assert!(bytes.starts_with(b"RIFF")&&bytes.get(8..12)==Some(b"WEBP"));
    }
    for key in ["../page-art/healthcare-v1.webp","unknown.webp","chassis-v1.webp/extra"] {assert!(page_art_assets::component_asset(key).is_none());}
}

/// THE SURFACE TREATMENT IS WIRED, AND THE LOSERS ARE STILL THERE.
///
/// Three procedural treatments were written and compared on the same meshes.
/// `weathering` ships because it was the only one that survives a real card:
/// arsenal3d frames a 34px ledger chip and a 380px inspection view at the SAME
/// model-space distance, so a treatment carried by high-frequency noise aliases
/// into speckle at the sizes this game actually draws, while a gravity gradient
/// downsamples cleanly. The other two stay served because `setSurface` makes
/// re-judging a one-line swap, and the next art pass should not have to take
/// that finding on trust.
#[test]
fn the_card_renderer_ships_a_surface_treatment() {
    for (name, src) in [("surface-grain.js", SURFACE_GRAIN_JS),
        ("surface-wear.js", SURFACE_WEAR_JS), ("surface-material.js", SURFACE_MATERIAL_JS)] {
        assert!(!src.contains("https://"), "{name} must stay self-contained -- no CDN");
        assert!(src.contains("vec3 surface(vec3"), "{name} has lost its entry point");
        // No animation, ever: determinism is iron rule 1, and a shimmering
        // surface is also motion that reduced-motion could not switch off.
        assert!(!src.contains("uniform float uTime") && !src.contains("iTime"),
            "{name} has grown a clock");
        assert!(INDEX.contains(&format!("<script src=\"/{name}\"></script>")),
            "{name} is not loaded by the page");
    }
    assert!(ARSENAL3D_JS.contains("function setSurface(glsl)")
        && ARSENAL3D_JS.contains("const DEFAULT_SURFACE ="),
        "the renderer has lost its swappable surface slot");
    // Installed, not merely available: a treatment that ships unreferenced is
    // the same as no treatment at all.
    assert!(INDEX.contains("function installSurfaceTreatment()")
        && INDEX.contains("window.Surfacewear"),
        "nothing installs a treatment, so every card renders flat");
}

/// A CITY GETS A REPRESENTATIVE BLOCK, AND SAYS SO.
///
/// The town mesh is deliberately NOT on the map symbol. At the 24-32 px the
/// city layer draws, a baked block is a mush and city-detail.js's drawn
/// isometric icon beats it outright — the roadmap makes the same point about
/// abstract information staying crisp UI rather than becoming a 3D prop. It
/// lives on the card instead, which is a deliberate click with room to read.
///
/// The caption is the load-bearing part and is asserted here rather than left
/// to a reviewer: a settlement drawn beside a real place name is exactly where
/// someone assumes it is that place's streets. It is not, TownMesh says so in
/// its own description, and the card must keep saying so too.
#[test]
fn a_selected_city_shows_a_birds_eye_and_denies_the_street_plan() {
    // The card used to show one of eight generic town BLOCKS chosen by hashing
    // the name, so Tokyo and a ninety-thousand-person town got the same street
    // corner. Ridge: "the town asset makes no sense to me. I would rather it
    // just be a birds eye of the city." A city cannot be built from those
    // blocks either — a million people is ~40 km2, which is 2,599 of them and
    // 507 million triangles — so the primitive is city massing.
    assert!(INDEX.contains("function cityIndexFor(city)"),
        "nothing resolves the selected city to a record");
    assert!(INDEX.contains("CityMesh.build(CITIES[cityIndex]"),
        "the card no longer builds a city from the record it selected");

    // THE BAR THAT MATTERS MOST, and it matters MORE now than it did with a
    // block: this is a convincing city drawn beside a real place name taken
    // from Natural Earth, and every street in it is invented. The caption must
    // say so. Iron rule 4 is that starting data is transcribed, not invented;
    // a plausible plan captioned only with the name is how that gets broken.
    assert!(INDEX.contains("the size and setting follow the record, the streets do not"),
        "the city vignette has lost the caption that stops it reading as a survey");

    // The size cannot be in the picture. Every card frames its model to fill
    // it, so a 32 km city and a 3 km one fill it identically, and fixing the
    // scale instead would draw the small ones a few pixels wide. So the extent
    // is stated in words, and that sentence is load-bearing.
    assert!(INDEX.contains("across about ") && INDEX.contains("km'"),
        "the caption no longer carries the extent, which the framing cannot show");

    // A birds-eye needs a birds-eye camera. arsenal3d rests at 20 degrees,
    // which on something 32 km wide and 600 m tall is edge-on to the plan.
    assert!(INDEX.contains("{ pitch: 58 }"),
        "the city is drawn at the default oblique, where its plan is a plate");

    assert!(INDEX.contains("function registerMeshProviders()"),
        "the mesh providers are no longer registered from one place");
}
