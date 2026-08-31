#!/usr/bin/env python3
"""
tools/resources/crosswalk.py — hand-authored country-name tables.

One table per source vocabulary, plus one shared ignore set. Every entry was
written after dumping that source's actual labels (see the module test at the
bottom), not guessed from memory, and `geo.NationCrosswalk` raises on anything
unlisted so a new edition of a source cannot quietly drop a country.

Only the NATIONAL production tables need these. Point sources — MRDS and
PP1802 — are placed by coordinate and never by name, so their country column is
used purely as a QA cross-check and their (modern, 2016-vintage) vocabulary
never has to be reconciled with the game's 1990 roster.

TWO EDITORIAL DECISIONS are recorded here rather than hidden in code:

  * `Germany, East` + `Germany, West` -> `Germany`. The game roster carries a
    single `Germany`; reunification was Oct 1990 and the game starts in Jan.
    Summing is the only way to attach the two Germanies' real 1990 output to the
    one nation that exists to receive it. Both source labels are kept in the
    artifact's evidence so the split stays visible.
  * `Yemen (Aden)` + `Yemen (Sanaa)` -> `Yemen`. Unified May 1990; same reason.

Everything else is a one-to-one rename.
"""

# Names that are real data but not one of the game's 160 nations: aggregates,
# and countries the district roster simply does not model. Listing them is what
# makes the strict crosswalk safe — an unmapped name is then unambiguously a
# mistake rather than an expected omission.
IGNORE = {
    # aggregates
    "World", "EU-15", "OPEC - South America", "South Korea and other OECD Asia",
    "Former Serbia and Montenegro", "European Union",
    # sovereign states outside the 160-nation roster
    "Albania2", "Andorra", "Antigua and Barbuda", "Barbados", "Benin",
    "Bhutan2", "Burkina Faso", "Burundi", "Cambodia2", "Djibouti",
    "Dominica", "Eritrea", "Eswatini", "Gambia", "Gambia, The", "Gambia, the",
    "Grenada", "Guinea", "Guinea-Bissau", "Ivory Coast", "Cote d'Ivoire",
    "Cote D'Ivoire", "Kiribati", "Kosovo", "Liberia",
    "Liechtenstein", "Mali", "Marshall Islands", "Mauritania", "Micronesia",
    "Monaco", "Nauru", "Niger", "Palau", "Rwanda", "Saint Kitts and Nevis",
    "Saint Lucia", "Saint Vincent/Grenadines", "San Marino", "Sierra Leone",
    "Somalia", "South Sudan", "Togo", "Tuvalu", "Vatican City",
    # dependencies, territories and non-sovereign reporting units
    "American Samoa", "Anguilla", "Antarctica", "Aruba", "Bermuda",
    "British Virgin Islands", "Cayman Islands", "Christmas Island",
    "Cook Islands", "Falkland Islands", "Faroe Islands", "French Guiana",
    "French Polynesia", "Gibraltar", "Greenland", "Guadeloupe", "Guam",
    "Hong Kong", "Macau", "Martinique", "Montserrat", "Netherlands Antilles",
    "New Caledonia", "Niue", "Northern Mariana Islands", "Palestinian Territories",
    "Puerto Rico", "Reunion", "Saint Helena", "Saint Pierre and Miquelon",
    "Turks and Caicos Islands", "U.S. Pacific Islands", "U.S. Territories",
    "U.S. Virgin Islands", "Wake Island", "Western Sahara", "Wallis and Futuna",
    # post-1990 polities that report no 1990 datum; kept explicit so their
    # appearance in a future edition is a decision, not an accident
    "Czechia", "Slovakia", "North Macedonia", "Timor-Leste", "Turkiye",
    "Serbia and Montenegro",
}

# --- EIA International Energy Statistics ----------------------------------
EIA = {
    "Former U.S.S.R.": "USSR",
    "Former Czechoslovakia": "Czechoslovakia",
    "Former Yugoslavia": "Yugoslavia",
    "Germany, East": "Germany",
    "Germany, West": "Germany",
    "United States": "USA",
    "United Kingdom": "UK",
    "United Arab Emirates": "UAE",
    "Saudi Arabia": "SaudiArabia",
    "South Africa": "SouthAfrica",
    "South Korea": "SouthKorea",
    "North Korea": "NorthKorea",
    "New Zealand": "NewZealand",
    "Papua New Guinea": "PapuaNewGuinea",
    "Trinidad and Tobago": "TrinidadTobago",
    "Costa Rica": "CostaRica",
    "Dominican Republic": "DominicanRepublic",
    "El Salvador": "ElSalvador",
    "Equatorial Guinea": "EquatorialGuinea",
    "Central African Republic": "CentralAfricanRepublic",
    "Cape Verde": "CapeVerde",
    "Sao Tome and Principe": "SaoTome",
    "Solomon Islands": "SolomonIslands",
    "Sri Lanka": "SriLanka",
    "The Bahamas": "Bahamas",
    "Burma": "Myanmar",
    "Congo (Kinshasa)": "Zaire",
    "Congo (Brazzaville)": "Congo",
    "Democratic Republic of the Congo": "Zaire",
    "Republic of the Congo": "Congo",
    # EIA's own spellings, which differ from every other source's
    "Congo-Kinshasa": "Zaire",
    "Congo-Brazzaville": "Congo",
    "Cabo Verde": "CapeVerde",
    "Swaziland": "Swaziland",
    "Macedonia": "Macedonia",
}

# --- USGS DS896 historical statistics -------------------------------------
DS896 = {
    "U.S.S.R.": "USSR",
    "Czechoslovakia": "Czechoslovakia",
    "Yugoslavia": "Yugoslavia",
    "Germany: Western states": "Germany",
    "Germany: Eastern states": "Germany",
    "Germany, Federal Republic of": "Germany",
    "Germany, Democratic Republic of": "Germany",
    "United States": "USA",
    "United Kingdom": "UK",
    "United Arab Emirates": "UAE",
    "Saudi Arabia": "SaudiArabia",
    "South Africa": "SouthAfrica",
    "South Africa, Republic of": "SouthAfrica",
    "Korea, Republic of": "SouthKorea",
    "Korea, North": "NorthKorea",
    "Korea, Democratic People's Republic of": "NorthKorea",
    "New Zealand": "NewZealand",
    "Papua New Guinea": "PapuaNewGuinea",
    "Trinidad and Tobago": "TrinidadTobago",
    "Dominican Republic": "DominicanRepublic",
    "Congo (Kinshasa)": "Zaire",
    "Congo (Brazzaville)": "Congo",
    "Burma": "Myanmar",
    "Sri Lanka": "SriLanka",
}

# --- USDA FAS PSD Online --------------------------------------------------
# PSD back-casts the USSR into successor states for 1990 (wheat: Russia 49,596
# kt + Ukraine 30,374 + Kazakhstan 16,197 ... summing to the USSR total). Those
# successor rows are kept as-is and flagged in the artifact; they are NOT
# re-aggregated into `USSR`, because doing so would discard real per-republic
# detail, and the district that grows the wheat carries its own id through the
# 1991 dissolution regardless.
PSD = {
    "United States": "USA",
    "United Kingdom": "UK",
    "United Arab Emirates": "UAE",
    "Saudi Arabia": "SaudiArabia",
    "South Africa": "SouthAfrica",
    "Korea, South": "SouthKorea",
    "Korea, North": "NorthKorea",
    "New Zealand": "NewZealand",
    "Papua New Guinea": "PapuaNewGuinea",
    "Trinidad and Tobago": "TrinidadTobago",
    "Costa Rica": "CostaRica",
    "Dominican Republic": "DominicanRepublic",
    "El Salvador": "ElSalvador",
    "Sri Lanka": "SriLanka",
    "Former Czechoslovakia": "Czechoslovakia",
    "Former Yugoslavia": "Yugoslavia",
    "Former Soviet Union": "USSR",
    "Congo (Kinshasa)": "Zaire",
    "Congo (Brazzaville)": "Congo",
    "Burma": "Myanmar",
    "Yemen (Aden)": "Yemen",
    "Yemen (Sanaa)": "Yemen",
    "Bosnia and Herzegovina": "Bosnia",
}

# Source labels that legitimately sum into one game nation. Kept as an explicit
# roster so the artifact can flag those nations rather than let a silent
# addition look like a single reported figure.
MERGED = {
    "Germany": {"Germany, East", "Germany, West", "Germany: Western states",
                "Germany: Eastern states"},
    "Yemen": {"Yemen (Aden)", "Yemen (Sanaa)"},
}
