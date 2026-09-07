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

/// GROUND SCATTER IS SERVED, DRAWN UNDER EVERYTHING, AND GRANTS NOTHING.
///
/// The scatter kit sat finished but unrouted for its whole life: 30 kinds
/// across 9 biomes that the browser had never once loaded, because a module
/// with no route is a 404 and a module with no script tag is never asked for.
/// Both halves are asserted here, in both directions, for the same reason the
/// deck parity test is two-directional.
///
/// The ORDER is the load-bearing claim. Scenery is drawn at the top of
/// drawMapOverlay so the war glow, the resource chips, the logistics arcs, the
/// construction marks and every label land on top of it. A tree that can cover
/// a mark the player is meant to read is a bug, and the cheapest way for that
/// to happen is someone inserting a layer above this call.
#[test]
fn ground_scatter_is_served_and_stays_beneath_the_information_layers() {
    for (name, src) in [("scatter-mesh.js", SCATTER_MESH_JS), ("world-scatter.js", WORLD_SCATTER_JS)] {
        assert!(!src.contains("https://"), "{name} must stay self-contained -- no CDN");
        assert!(INDEX.contains(&format!("<script src=\"/{name}\"></script>")),
            "{name} is routed but never loaded, so the browser never asks for it");
    }
    // Drawn FIRST -- searched INSIDE drawMapOverlay, because the war glow is
    // iterated in two places and a whole-file find reads the wrong one. This
    // test failed exactly that way when written, which is the only reason it is
    // known to be able to fail at all.
    let body = &INDEX[INDEX.find("function drawMapOverlay(ctx, view, globe) {")
        .expect("the map overlay moved")..];
    let scatter = body.find("  drawScatterOverlay(ctx, view, globe);")
        .expect("the scatter layer is not drawn at all");
    let stock = body.find("  drawStockOverlay(ctx, view, globe);")
        .expect("the resource layer moved");
    let war = body.find("  for (const w of S.wars) {").expect("the war glow moved");
    assert!(scatter < war && scatter < stock,
        "scenery must be drawn before the information layers, not over them");

    // The two traps this layer walked into, both measured rather than guessed.
    // ScatterMesh::isFar accepts the NUMBER 1 but not the string "1", so an id
    // built from item.lod directly bakes a near-LOD mesh at map scale.
    assert!(INDEX.contains("const lod = item.lod ? \"far\" : \"near\";"),
        "the lod must be spelled as a word before it enters an id");
    // A kind shared between biomes takes its palette from the biome, so an id
    // without one colours a Congo boulder for a temperate wood.
    assert!(INDEX.contains("${item.biome}"), "the biome is not carried into the bake");

    // Presentation, not fact. The placement is derived from a baked cover
    // image, latitude and elevation; it must never become a claim about what
    // grows anywhere, and iron rule 4 makes inventing that a refusal.
    assert!(WORLD_SCATTER_JS.contains("not a survey") || WORLD_SCATTER_JS.contains("not a land-cover"),
        "the placement module must say what it is not");
    // The first draft of this assertion policed the words "timber" and "forage"
    // and went red on the module's own disclaimer sentence. The property worth
    // holding is not vocabulary but ISOLATION: every sampler arrives as an
    // argument, so the module can neither read the world nor write to it, and
    // that is also what lets the check harness run it under plain node.
    for reach in ["fetch(", "document.", "XMLHttpRequest", "localStorage",
                  "window.WORLD", "window.S", "Command"] {
        assert!(!WORLD_SCATTER_JS.contains(reach),
            "the placement module must stay a pure function of its arguments -- found {reach}");
    }
    for sampler in ["o.height", "o.land", "o.vegetation"] {
        assert!(WORLD_SCATTER_JS.contains(sampler),
            "{sampler} must be injected, not sourced inside the module");
    }
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
fn a_selected_city_shows_representative_buildings_and_admits_it() {
    assert!(INDEX.contains("function townBlockFor(city)"),
        "nothing chooses a block for a city");
    assert!(INDEX.contains("not a street map of"),
        "the city vignette has lost the caption that stops it reading as a survey");
    assert!(INDEX.contains("function registerMeshProviders()"),
        "the mesh providers are no longer registered from one place");
    // Bounded on purpose: a unique block per city would rebuild 75,000
    // triangles on every click and cache without limit.
    assert!(INDEX.contains("% 8"), "the block pool is no longer bounded");
}
