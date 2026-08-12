//! The canonical 1990 content, compiled in.
//!
//! An explicit ordered list rather than a directory scan, and that is a
//! determinism requirement rather than a stylistic one: roster order is
//! serialization order is the golden hash, and `read_dir` returns whatever
//! order the filesystem feels like — which differs between Windows and Linux,
//! precisely the split PLAN 1.3 is worried about.
//!
//! Adding a nation is two lines here and one file beside it. When ids become
//! runtime values it becomes one line here.

use super::Source;

pub const EMBEDDED_NATIONS: &[Source<'static>] = &[
    Source { file: "data/nations/usa.json", json: include_str!("../../data/nations/usa.json") },
    Source { file: "data/nations/ussr.json", json: include_str!("../../data/nations/ussr.json") },
    Source { file: "data/nations/china.json", json: include_str!("../../data/nations/china.json") },
    Source { file: "data/nations/japan.json", json: include_str!("../../data/nations/japan.json") },
    Source { file: "data/nations/germany.json", json: include_str!("../../data/nations/germany.json") },
    Source { file: "data/nations/uk.json", json: include_str!("../../data/nations/uk.json") },
    Source { file: "data/nations/france.json", json: include_str!("../../data/nations/france.json") },
    Source { file: "data/nations/italy.json", json: include_str!("../../data/nations/italy.json") },
    Source { file: "data/nations/india.json", json: include_str!("../../data/nations/india.json") },
    Source { file: "data/nations/pakistan.json", json: include_str!("../../data/nations/pakistan.json") },
    Source { file: "data/nations/iraq.json", json: include_str!("../../data/nations/iraq.json") },
    Source { file: "data/nations/kuwait.json", json: include_str!("../../data/nations/kuwait.json") },
    Source { file: "data/nations/saudiarabia.json", json: include_str!("../../data/nations/saudiarabia.json") },
    Source { file: "data/nations/iran.json", json: include_str!("../../data/nations/iran.json") },
    Source { file: "data/nations/southkorea.json", json: include_str!("../../data/nations/southkorea.json") },
    Source { file: "data/nations/poland.json", json: include_str!("../../data/nations/poland.json") },
    Source { file: "data/nations/brazil.json", json: include_str!("../../data/nations/brazil.json") },
    Source { file: "data/nations/indonesia.json", json: include_str!("../../data/nations/indonesia.json") },
    Source { file: "data/nations/egypt.json", json: include_str!("../../data/nations/egypt.json") },
    Source { file: "data/nations/israel.json", json: include_str!("../../data/nations/israel.json") },
    Source { file: "data/nations/turkey.json", json: include_str!("../../data/nations/turkey.json") },
    Source { file: "data/nations/nigeria.json", json: include_str!("../../data/nations/nigeria.json") },
    Source { file: "data/nations/vietnam.json", json: include_str!("../../data/nations/vietnam.json") },
    Source { file: "data/nations/yugoslavia.json", json: include_str!("../../data/nations/yugoslavia.json") },
    Source { file: "data/nations/spain.json", json: include_str!("../../data/nations/spain.json") },
    Source { file: "data/nations/netherlands.json", json: include_str!("../../data/nations/netherlands.json") },
    Source { file: "data/nations/belgium.json", json: include_str!("../../data/nations/belgium.json") },
    Source { file: "data/nations/sweden.json", json: include_str!("../../data/nations/sweden.json") },
    Source { file: "data/nations/switzerland.json", json: include_str!("../../data/nations/switzerland.json") },
    Source { file: "data/nations/austria.json", json: include_str!("../../data/nations/austria.json") },
    Source { file: "data/nations/portugal.json", json: include_str!("../../data/nations/portugal.json") },
    Source { file: "data/nations/greece.json", json: include_str!("../../data/nations/greece.json") },
    Source { file: "data/nations/denmark.json", json: include_str!("../../data/nations/denmark.json") },
    Source { file: "data/nations/norway.json", json: include_str!("../../data/nations/norway.json") },
    Source { file: "data/nations/finland.json", json: include_str!("../../data/nations/finland.json") },
    Source { file: "data/nations/ireland.json", json: include_str!("../../data/nations/ireland.json") },
    Source { file: "data/nations/czechoslovakia.json", json: include_str!("../../data/nations/czechoslovakia.json") },
    Source { file: "data/nations/hungary.json", json: include_str!("../../data/nations/hungary.json") },
    Source { file: "data/nations/romania.json", json: include_str!("../../data/nations/romania.json") },
    Source { file: "data/nations/bulgaria.json", json: include_str!("../../data/nations/bulgaria.json") },
    Source { file: "data/nations/albania.json", json: include_str!("../../data/nations/albania.json") },
    Source { file: "data/nations/eastgermany.json", json: include_str!("../../data/nations/eastgermany.json") },
    Source { file: "data/nations/argentina.json", json: include_str!("../../data/nations/argentina.json") },
    Source { file: "data/nations/mexico.json", json: include_str!("../../data/nations/mexico.json") },
    Source { file: "data/nations/chile.json", json: include_str!("../../data/nations/chile.json") },
    Source { file: "data/nations/colombia.json", json: include_str!("../../data/nations/colombia.json") },
    Source { file: "data/nations/venezuela.json", json: include_str!("../../data/nations/venezuela.json") },
    Source { file: "data/nations/peru.json", json: include_str!("../../data/nations/peru.json") },
    Source { file: "data/nations/cuba.json", json: include_str!("../../data/nations/cuba.json") },
    Source { file: "data/nations/bolivia.json", json: include_str!("../../data/nations/bolivia.json") },
    Source { file: "data/nations/ecuador.json", json: include_str!("../../data/nations/ecuador.json") },
    Source { file: "data/nations/uruguay.json", json: include_str!("../../data/nations/uruguay.json") },
];

pub const EMBEDDED_RELATIONS: Source<'static> = Source {
    file: "data/relations_1990.json",
    json: include_str!("../../data/relations_1990.json"),
};
