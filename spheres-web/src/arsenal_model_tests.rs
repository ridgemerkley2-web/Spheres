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
