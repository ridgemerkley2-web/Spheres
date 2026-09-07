//! Versioned, embedded page paintings. Request paths never access the filesystem.

pub const CSS: &str = include_str!("../ui/page-art.css");

pub fn asset(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "nation-selection-v1.webp" => include_bytes!("../ui/page-art/nation-selection-v1.webp"),
        "saved-campaigns-v1.webp" => include_bytes!("../ui/page-art/saved-campaigns-v1.webp"),
        "construction-v1.webp" => include_bytes!("../ui/page-art/construction-v1.webp"),
        "trade-v1.webp" => include_bytes!("../ui/page-art/trade-v1.webp"),
        "world-markets-v1.webp" => include_bytes!("../ui/page-art/world-markets-v1.webp"),
        "influence-v1.webp" => include_bytes!("../ui/page-art/influence-v1.webp"),
        "military-research-v1.webp" => include_bytes!("../ui/page-art/military-research-v1.webp"),
        "tank-designer-v1.webp" => include_bytes!("../ui/page-art/tank-designer-v1.webp"),
        "equipment-library-v1.webp" => include_bytes!("../ui/page-art/equipment-library-v1.webp"),
        "proving-ground-v1.webp" => include_bytes!("../ui/page-art/proving-ground-v1.webp"),
        "tank-factory-v1.webp" => include_bytes!("../ui/page-art/tank-factory-v1.webp"),
        "army-service-v1.webp" => include_bytes!("../ui/page-art/army-service-v1.webp"),
        "global-command-v1.webp" => include_bytes!("../ui/page-art/global-command-v1.webp"),
        "decisions-v1.webp" => include_bytes!("../ui/page-art/decisions-v1.webp"),
        "league-v1.webp" => include_bytes!("../ui/page-art/league-v1.webp"),
        "intelligence-v1.webp" => include_bytes!("../ui/page-art/intelligence-v1.webp"),
        "policy-v1.webp" => include_bytes!("../ui/page-art/policy-v1.webp"),
        "science-computing-v1.webp" => include_bytes!("../ui/page-art/science-computing-v1.webp"),
        "science-communications-v1.webp" => include_bytes!("../ui/page-art/science-communications-v1.webp"),
        "science-energy-v1.webp" => include_bytes!("../ui/page-art/science-energy-v1.webp"),
        "science-materials-v1.webp" => include_bytes!("../ui/page-art/science-materials-v1.webp"),
        "science-aerospace-v1.webp" => include_bytes!("../ui/page-art/science-aerospace-v1.webp"),
        "science-biotech-v1.webp" => include_bytes!("../ui/page-art/science-biotech-v1.webp"),
        "science-transport-v1.webp" => include_bytes!("../ui/page-art/science-transport-v1.webp"),
        "science-agriculture-v1.webp" => include_bytes!("../ui/page-art/science-agriculture-v1.webp"),
        "healthcare-v1.webp" => include_bytes!("../ui/page-art/healthcare-v1.webp"),
        "community-services-v1.webp" => include_bytes!("../ui/page-art/community-services-v1.webp"),
        "city-life-v1.webp" => include_bytes!("../ui/page-art/city-life-v1.webp"),
        "resource-oil-gas-v1.webp" => include_bytes!("../ui/page-art/resource-oil-gas-v1.webp"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_page_painting_is_embedded_and_requests_are_allowlisted() {
        for key in ["nation-selection", "saved-campaigns", "construction", "trade", "world-markets", "influence", "military-research", "tank-designer", "equipment-library", "proving-ground", "tank-factory", "army-service", "global-command", "decisions", "league", "intelligence", "policy", "science-computing", "science-communications", "science-energy", "science-materials", "science-aerospace", "science-biotech", "science-transport", "science-agriculture", "healthcare", "community-services", "city-life", "resource-oil-gas"] {
            let bytes = asset(&format!("{key}-v1.webp")).expect("page painting");
            assert!(bytes.len() > 1024, "empty page painting: {key}");
            assert_eq!(&bytes[..4], b"RIFF");
            assert_eq!(&bytes[8..12], b"WEBP");
            assert!(CSS.contains(&format!("/art/pages/{key}-v1.webp")), "unused page painting: {key}");
        }
        for key in ["", "../tank-designer-v1.webp", "tank-designer.webp", "tank-designer-v1.webp?x", "unknown-v1.webp", "%2e%2e/tank-designer-v1.webp"] {
            assert!(asset(key).is_none());
        }
    }
}

/// Research illustrations use a fixed allowlist, never a filesystem path.
pub fn component_asset(name: &str) -> Option<&'static [u8]> {
    Some(match name {
        "chassis-v1.webp" => include_bytes!("../ui/component-art/chassis-v1.webp"),
        "engines-v1.webp" => include_bytes!("../ui/component-art/engines-v1.webp"),
        "weapons-v1.webp" => include_bytes!("../ui/component-art/weapons-v1.webp"),
        "armor-v1.webp" => include_bytes!("../ui/component-art/armor-v1.webp"),
        "optics-v1.webp" => include_bytes!("../ui/component-art/optics-v1.webp"),
        "communications-v1.webp" => include_bytes!("../ui/component-art/communications-v1.webp"),
        _ => return None,
    })
}
