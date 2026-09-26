# Personnel baseline audit, 22 September 2026

No historical headcount was changed. Sao Tome's existing 1,000-person input is
confirmed; Comoros remains unsourced. The repaired simulation defect was a
different reference date for sourced and estimated personnel, not evidence
that either country's coup outcome should be predetermined.

## Checked data

- The official World Bank API gives Sao Tome 1,000 in each year 1989-1992,
  with 1988 null. Comoros is null throughout 1988-1992. These are active armed
  services plus qualifying active paramilitary, not land-army strength alone
  or the population available for military service. The metadata identifies
  IISS as the source and admits estimates. This confirms the existing
  transcription, not an exact payroll census.
  [Sao Tome observations](https://api.worldbank.org/v2/country/STP/indicator/MS.MIL.TOTL.P1?date=1988:1992&format=json),
  [Comoros observations](https://api.worldbank.org/v2/country/COM/indicator/MS.MIL.TOTL.P1?date=1988:1992&format=json),
  [indicator definition](https://databank.worldbank.org/metadataglossary/world-development-indicators/series/MS.MIL.TOTL.P1).
- The CIA's 1990 Factbook scan was downloaded, relevant pages rendered and
  visually checked. Printed page 70 lists Comoros' 97,504 males aged 15-49 and
  58,274 fit for service. Page 273 gives Sao Tome 27,805 and 14,662 respectively.
  Neither page states serving strength. Its expenditure percentages are
  dated 1981 and 1980, outside the requested window.
  [1990 Factbook scan](https://libsysdigi.library.illinois.edu/oca/Books2007-06/worldfactbook/worldfactbook90natiilli/worldfactbook90natiilli.pdf).
- The original COW NMC v7 archive reports one thousand personnel for both
  states in 1988-1992. Its supplemental Comoros rows have an empty source and
  the note `Not in ACDA, 1998`; Sao Tome cites ACDA 1998, page 101. The codebook
  uses thousands and a narrower regular-force definition than WDI's qualifying
  paramilitary scope. Comoros' rounded, individually undocumented value is
  therefore not admitted as a verified substitute for its missing source.
  No ACDA original was independently retrieved.
  [Original dataset and documentation](https://correlatesofwar.org/data-sets/national-material-capabilities/).

The Library of Congress 1995 country study is a useful further lead, but its
original PDF returned HTTP 403 here. The catalogue dates completion of research
to August 1994. Search excerpts mentioning 700-800 regular troops cannot be
treated as a verified January 1990 total, nor combined casually with guards,
police or foreign troops. This audit does not establish Comoros' actual opening
personnel count. [Catalogue](https://www.loc.gov/item/95016570/).

## Reference-time repair

Previously `army_personnel_assessment` returned fixed historical personnel for
sourced countries while multiplying the missing-data median by **current**
population. Doubling current population therefore doubled only an unsourced
force. This changed resources per member solely because source coverage differed.

The fallback now uses cached embedded opening population, matching the sourced
series' reference. Its unchanged median comes from 125 valid sourced pairs:
0.005431034482758621. Comoros still starts with a labelled model estimate of
2,237.5862 members; its historical source API still returns `None`. Sao Tome
still uses 1,000. No median, fiscal coefficient, coup threshold or nation datum
was fitted to a political test result.

The targeted regression now changes live populations for both an unsourced
country and a sourced country, checks unchanged personnel and resources,
preserves exact save/load state, and retains invalid-input and disabled-lens
guards. A future recruitment/demobilization model should alter both baseline
types consistently; this repair does not implement one.

`git diff --check` passed. Native compilation and execution are coordinated by
the parent task and remain unclaimed in this receipt. Compact fetched-response
hashes, observations, definitions and limits are in [source-audit.json](source-audit.json).
