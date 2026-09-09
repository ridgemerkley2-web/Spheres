# Canada party leaders research, 1990–2026

Staged on 7 September 2026: **24 people, 26 leadership windows, four exact baseline parties, and four visually inspected opening-leader references**. This is partial research; it does not claim every Canadian party or completed cartoon assets.

| Baseline row | 1 January 1990 leader | Person ID |
| --- | --- | --- |
| ca_pc | Brian Mulroney | brian_mulroney |
| ca_lib | John Turner | john_turner |
| ca_ndp | Audrey McLaughlin | audrey_mclaughlin |
| ca_reform | Preston Manning | preston_manning |

## Integration requirements

Keep party leadership separate from parliamentary substitutes and executive appointment. The [House appendix](https://www.ourcommons.ca/procedure-and-practice-4/app08-e.html) lists leaders **in the House**; its footnotes distinguish them from actual party leaders. Turner remained Liberal leader after surrendering the Opposition office in February 1990. Blaikie, Caron and Hill must not replace Layton, Singh or Lynch-Staunton merely because they performed House functions. The [Governor General's explanation](https://www.gg.ca/en/governor-general/role-and-responsibilities/constitutional-duties/swearing-process) requires a separate appointment and House-confidence assessment.

Preserve existing IDs, native names, birth facts and office links. Brian Mulroney's existing native/birth facts are preserved. His [verified death on 29 February 2024](https://www.pm.gc.ca/en/news/news-releases/2024/03/05/prime-minister-announces-state-funeral-right-honourable-brian) remains a staged research fact and was not imported: changing the saved identity requires a separately reviewed migration, as recorded in `canada-identity-ingestion.json`. Missing life dates remain unknown, not proof of survival or candidacy.

Intervals are half-open. Initial 1990 bounds are scope clips, not original election dates. Month precision expresses uncertain transition days. Open terms are verified only through the research cutoff. Historical terms do not authorize a future real-person prediction; subsequent game successors must be explicitly fictional.

## Historical scope and gaps

- **Progressive Conservatives:** original party ends at the December 2003 merger. Campbell's December 1993 resignation and Charest's April 1998 transition need final day-level reconciliation. The [Quebec National Assembly biography](https://www.assnat.qc.ca/fr/deputes/charest-jean-525/biographie.html) distinguishes Charest's interim and elected mandates and dates his resignation one day differently from the federal House footnote. Wayne's national interim leadership ends with Clark's election in November 1998; her later House role is separate.
- **Liberals:** the regular/interim sequence is staged through Carney's [9 March 2025 victory speech](https://www.youtube.com/watch?v=tJZ0Ib3f6jw). Ignatieff's May 2011 departure remains a coarse transition. Announcing an intention to leave is not automatically the effective day of resignation.
- **NDP:** McLaughlin's continuous leadership runs through 14 October 1995. Her April 1994 announcement changed her status to interim without a vacancy, according to the [National Library's 1997 exhibit](https://epe.lac-bac.gc.ca/100/200/301/nlc-bnc/celebrating_women-ef/women97/eaudrey.htm?nodisclaimer=1). The exact status-transition day remains uncertain in the term note; it does not split the known identity window. Layton's active office window stops at [Turmel's interim appointment on 28 July 2011](https://www.ndp.ca/news/statement-new-democrat-interim-leader-nycole-turmel); this is not a death or party-departure date. The sequence includes Don Davies and [Avi Lewis's election on 29 March 2026](https://www.ndp.ca/news/avi-lewis-elected-leader-ndp).
- **Reform:** [Elections Canada](https://elections.ca/content.aspx?dir=pre&document=decision&lang=e&section=med) describes a registered-name change effective 27 March 2000. The Canadian Alliance sequence is recorded in research issues, pending a reviewed identity display and lifecycle decision. It is not silently assigned to a permanently named Reform row.

The new Conservative Party, Bloc Québécois, Greens, People's Party and other omitted parties need reviewed IDs. The Bloc did not exist on 1 January 1990. No new opening vote share or fictional historical party is manufactured to fill the omission.

## Portrait audit

All local references are in `C:\Users\ridge\AppData\Local\Temp\spheres-canada-reference-audit\`. The JSON records full media URLs and appearance cues.

| Person | Inspected file | Date limitations |
| --- | --- | --- |
| Mulroney | mulroney-1989.jpg | 10 February 1989; Mulroney is the person on the left |
| Turner | turner-1987.jpg | 5 April 1987 |
| McLaughlin | audrey-lac-1997.png | Parliamentary-era Canapress photograph in a 1997 National Library exhibit; exposure date unspecified |
| Manning | manning-glenbow.png | Page 1 of a Glenbow museum biography bearing 2005 copyright; exposure unspecified and visibly later than the opening campaign |

Mulroney and Turner references are US government photographs marked public domain on Commons. The McLaughlin and Manning photographs remain copyrighted reference material. None is shipped in the game. The missing dated Manning reference should be resolved before claiming exact early-1990 eyewear or hairstyle. Original cartoon interpretations must carry their own provenance and may not claim these later or undated photographs were taken in 1990.

The separate Yukon Archives McLaughlin brochure is catalogued to the 1993 election (printed page 63/PDF page 71), but its PDF was not visually inspected because the download failed. That catalog reference is kept distinct from the inspected Canapress image.
