# Italy party leaders research, 1990–2026

Staged on 7 September 2026: **19 people, 22 leadership windows, seven exact baseline parties and seven visually inspected starter references**. This is a partial source audit, with no runtime import or save migration.

| Baseline row | 1 January 1990 leader | Person ID |
| --- | --- | --- |
| it_dc | Arnaldo Forlani | arnaldo_forlani |
| it_pci | Achille Occhetto | achille_occhetto |
| it_psi | Bettino Craxi | bettino_craxi |
| it_msi | Gianfranco Fini | gianfranco_fini |
| it_pri | Giorgio La Malfa | giorgio_la_malfa |
| it_psdi | Antonio Cariglia | antonio_cariglia |
| it_pli | Renato Altissimo | renato_altissimo |

## Ingestion prerequisites

The companion JSON uses the v3 Person/Party/Term schema and preserves original game IDs. Party secretary, president, coordinator and national executive are separate roles. Existing Giulio Andreotti must remain the prime minister, not the DC secretary. Italy's [Article 92](https://www.senato.it/istituzione/la-costituzione/parte-ii/titolo-iii/sezione-i) specifies presidential appointment, while [Article 94](https://www.senato.it/istituzione/la-costituzione/parte-ii/titolo-iii/sezione-i/articolo-94) requires both chambers' confidence. Review executive eligibility separately before import.

Intervals are half-open. Coarse bounds indicate uncertainty, not known vacancies. Several observed-office clips intentionally stop where evidence ends; they are not claims of resignation. Birth/death dates omitted from records remain unverified. Fictional successors after 7 September 2026 need explicit fictional metadata and must respect separately reviewed organizational lifecycles.

## Scope and outstanding gaps

- **DC:** Forlani and Martinazzoli precede a January 1994 transition. The municipal biography conflicts with the original PPI announcement on the exact date. PPI/CCD and subsequent branches need distinct IDs; legal DC-continuity claims remain unreviewed.
- **PCI:** the [national archival profile](https://siusa-archivi.cultura.gov.it/cgi-bin/siusa/pagina.pl?Chiave=139&TipoPag=profist) dates dissolution to 3 February 1991 and describes separate PDS/Rifondazione paths. Do not merge their leaders under the old ID.
- **PSI:** Craxi, Benvenuto and Del Turco are staged. The [12 February 1993 assembly recording](https://www.radioradicale.it/scheda/51740/assemblea-nazionale-del-psi-lelezione-del-segretario) supplies Benvenuto's election. Spini's subsequent coordination and precise 1994 transition mandates remain gaps. The [archival record](https://siusa-archivi.cultura.gov.it/cgi-bin/siusa/pagina.pl?Chiave=91573&RicLin=en&TipoPag=prodente) supplies the original party's 13 November 1994 endpoint.
- **MSI:** Fini is the correct starter, followed by Rauti at the [14 January 1990 congress](https://www.radioradicale.it/scheda/34480/xvi-congresso-del-movimento-sociale-italiano-destra-nazionale). Fini returns in July 1991. AN, Fiamma Tricolore and later parties require distinct lifecycle records.
- **PRI:** retain La Malfa's withdrawal and Bogi's regency. Nucara's conservative verified-office clip ends after a dated 10 May 2013 secretary reference; election as president in March 2014 does not establish his secretary resignation day. Collura and Saponaro's coordinator phases precede the restored secretary role. The [18 April 2026 resolution](https://www.prinazionale.it/new/18%20Aprile%202026/DOCUMENTO%20POLITICO%20CN%20PRI%20-%2018%20APRILE%202026%20-%20ROMA.pdf) confirms Saponaro and discusses prospective MRE rapprochement, not a completed merger. Morelli/Piro were deputy coordinators, not invented equal secretaries.
- **PSDI:** Cariglia, Vizzini, Ferri and Schietroma are staged, with gaps around resignations and caretaker leadership. The [29 January 1995 congress](https://www.radioradicale.it/scheda/69953/congresso-nazionale-straordinario-del-psdi) identifies Schietroma as newly elected. The 1998 majority's SDI entry does not prove every remnant ceased to exist. Modern contested/revived leadership needs legal and organizational review.
- **PLI:** Altissimo and Costa end at the original dissolution. The [National Archives proceedings](https://dgagaeta.cultura.gov.it/public/uploads/documents/Saggi/Saggi_39.pdf) date that vote to 6 February 1994. Subsequent liberal parties cannot silently inherit the original ID.

Contemporary l'Unità reports are primary event reporting for Ferri's election and the SDI merger, not modern historical commentary. The SIUSA Assisi PSDI profile explicitly borrows its national history from Wikipedia; it was not used as independent primary proof of national legal continuity.

## Portrait references

All seven source photographs were visually inspected in:
`C:\Users\ridge\AppData\Local\Temp\spheres-italy-reference-audit\`

| Person | Filename | Catalogued era |
| --- | --- | --- |
| Forlani | forlani-1992.jpg | Camera XI, 1992 |
| Occhetto | occhetto-1992.jpg | Camera XI, 1992–1994; 2021 metadata is upload date |
| Craxi | craxi-circa1989.jpg | European Parliament, circa 1989 |
| Fini | fini-1992.jpg | Camera XI, 1992 |
| La Malfa | lamalfa-1994.jpg | European Parliament, 1994 |
| Cariglia | cariglia-1992.jpg | Camera XI, 1992–1994; exposure unknown |
| Altissimo | altissimo-1992.jpg | Camera XI, 1992; exact exposure day unverified |

JSON records exact source/media URLs, provenance and observed likeness cues. The four Camera portraits distributed through Commons are labelled CC BY-SA 4.0 there. EP portraits carry attribution-based reuse conditions, not a public-domain claim. Cariglia's direct Camera image has no independently verified shipping license.

These images inform original cartoon interpretation and were not added to shipped assets. Later photographs must not be described as having been taken in 1990. Generated-art records belong in a separate asset provenance registry.
