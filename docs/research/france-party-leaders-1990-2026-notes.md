# France party-leader research, 1990–2026

Checked through **2026-09-07**. This is a partial research staging artifact, not an imported runtime roster, a complete inventory of French parties, or a finished avatar batch.

The companion JSON contains **40 people, 54 leadership windows, five exact simulation party IDs, 11 UDF component identities, five top-level 1990 starters, six UDF constituent starters, and five visually audited period portrait references**. The exact baseline IDs were checked against `spheres-sim/src/government.rs`: `fr_ps`, `fr_rpr`, `fr_udf`, `fr_pcf`, and `fr_fn`.

## Integration prerequisite

**Keep this staged until role-aware integration and save migration are reviewed.** The runtime's generic party-to-executive mapping cannot assume that a party chair or a small constituent's executive should replace a national president, prime minister or parliamentary leader. The 11 UDF component slots also need a deliberate migration decision. This work changes no core registry, executive links, game IDs, saves, artwork, simulation logic or runtime mapping.

The `people`, `parties`, `components`, `terms` and `gaps` use the current Rust field shapes, including `DateBound`. A later import should merge identities by exact ID, preserve existing names and `office_links`, and retain François Mitterrand's executive identity. Existing `michel_rocard` is reused. Mitterrand is not substituted for the PS first secretary.

Term and association windows are **half-open [from, until)**. Month/year precision deliberately exposes uncertainty; the resolver's conservative bounds must not be rewritten as proof of an actual vacancy. Some UDF records are explicitly clipped observation windows rather than complete terms. Open ends establish office only through the historical cutoff. Missing birth or death fields mean unknown.

## 1990 starters

| Exact party ID | Real party leader | Office |
|---|---|---|
| fr_ps | Pierre Mauroy | First secretary |
| fr_rpr | Jacques Chirac | President |
| fr_udf | Valéry Giscard d'Estaing | Federal president |
| fr_pcf | Georges Marchais | General secretary |
| fr_fn | Jean-Marie Le Pen | President |

The [Socialist archive inventory](https://archives-socialistes.fr/les-collections/les-inventaires/premiers-secretaires), [Élysée Chirac biography](https://www.elysee.fr/jacques-chirac), [Giscard institutional biography](https://www.charles-de-gaulle.org/lhomme/biographies/valery-giscard-destaing/), [Marchais commemorative biography](https://www.georgesmarchais.fr/biographie/) and [Assembly Le Pen record](https://www2.assemblee-nationale.fr/sycomore/fiche/7607) provide the principal starter references. Party offices are distinguished from executive offices.

Six UDF streams are retained: François Léotard for the PR, Pierre Méhaignerie for the CDS, Yves Galland for the Radical Party, André Santini as documented PSD general secretary, Hervé de Charette for the club federation, and Paul Girod for direct members. A [February 14, 1990 contemporary report](https://www.lemonde.fr/archives/article/1990/02/14/apres-les-assises-du-rpr-mm-pasqua-et-seguin-contestent-les-resultats-du-vote-sur-les-motions_3963807_1819218.html) corroborates the constituent leadership context. Santini's [1986 primary statement](https://www.vie-publique.fr/discours/246314-declaration-de-m-andre-santini-secretaire-general-du-psd-sur-le-psd-e) gives the general-secretary title; Max Lejeune's formal/honorary presidency needs its own audit.

## Chronology and limits

- **PS:** Mauroy through Faure, including Rocard's provisional leadership, Hollande's delegated phase, Désir's acting periods and Temal's collective-direction coordination. Several day-level transitions and the June–September 2017 legal mandate remain unresolved. The [PS Paris history](https://www.parti-socialiste.paris/histoire) explicitly dates Cambadélis's election to April 15, 2014.
- **RPR:** Chirac, Juppé, Séguin, Sarkozy as acting president, and Alliot-Marie through dissolution. The final September 21, 2002 congress is included using September 22 as the half-open end. UMP/LR require separate identities and research. November 1994, October 1995, June/July 1997 and April 1999 transitions retain uncertainty.
- **PCF:** Marchais, Hue, Buffet, Laurent and Roussel. Hue's presidency alongside Buffet's national-secretary office is preserved as dual leadership. Exact October 2001 and Hue's 2003 end remain unresolved; a 2002 report announcing an intended resignation is not treated as proof of its exact execution.
- **UDF:** Federal leadership and the six original constituent streams, plus distinct FD, DL, PPDF and PRIL components. PR/direct-member changes inside 1990, the Radical 1993/1994 conflict, PSD office distinctions and several component lifecycles need more evidence. The [National Archives Bayrou biography](https://rdf.archives-nationales.culture.gouv.fr/garance/entities/agent/010083/) distinguishes the FD presidency ending September 17, 1998 from the later organizational merger, and records his UDF presidency through November 30, 2007. Election day versus assumption day is explicit. MoDem, Nouveau Centre and other successor branches are not collapsed into one chain.
- **FN/RN:** Jean-Marie Le Pen, Marine Le Pen, the Jalkh and Briois interim appointments, and Bardella's acting and elected presidencies. The 2018 rename does not fabricate a separate party or person. April 2017 and September 2021 precise delegation dates still need original party instruments.

The official [Faure profile](https://parti-socialiste.fr/le-premier-secretaire/), [PCF executive announcement](https://www.pcf.fr/le_conseil_national_du_pcf_a_adopte_son_nouvel_executif_national) and [Bardella profile](https://rassemblementnational.fr/membre/jordan-bardella) support current leadership observations. Current office does not imply an uninterrupted, day-verified chronology. André Santini's [June 2026 obituary](https://lcp.fr/actualites/andre-santini-maire-historique-d-issy-les-moulineaux-est-mort-437101) is reflected using conservative life-date precision; omission of other deaths is not evidence those people remain alive.

France-politique.fr is a **specialist secondary chronology**, not an official government or party source. Contemporary press, quoted announcements and retrospective institutional biographies support parts of the timeline where original minutes have not been located. Every party remains `partial`; source conflicts and coarse boundaries remain visible in JSON.

## Starter portrait audit

Full URLs, observed features, subject positions and provenance are in `portrait_reference_audit`. All five references were visually inspected. No photograph is shipped in the repository.

| Person | Observed reference period and provenance | Cartoon appearance cues |
|---|---|---|
| Pierre Mauroy | [February 7, 1990 Bundesarchiv image](https://commons.wikimedia.org/wiki/File:Pierre_Mauroy_1990.jpg), Peer Grimm, CC BY-SA 3.0 Germany | Rounded face/full neck, thinning silver-gray hair brushed back, large thin metal rounded-square glasses, clean-shaven, checked dark suit. |
| Jacques Chirac | [1990 INRA portrait crop](https://commons.wikimedia.org/wiki/File:Jacques_Chirac_1990_(crop).jpg), Jean Weber, CC BY 2.0 | High receding forehead, dark swept-back hair and gray temples, long oval face, prominent nose and ears, no glasses, gray plaid jacket. |
| Valéry Giscard d'Estaing | [March 9, 1990 Bundesarchiv rally photograph](https://commons.wikimedia.org/wiki/File:Bundesarchiv_Bild_183-1990-0309-027,_Dresden,_Volkskammerwahl,_BFD-Wahlkundgebung.jpg), Ulrich Häßler, CC BY-SA 3.0 Germany | Far-left speaker: bald domed crown, fine gray fringe, long face and nose, slender neck, no glasses, light trench coat. |
| Georges Marchais | [Commemorative gallery](https://www.georgesmarchais.fr/ressources/photos/), book-signing filename attributes October 1990 and L'Humanité/AD93; reference only, no free license established | Seated center-left: receding bare forehead, dark side hair, thick eyebrows, curved nose, broad animated smile, no glasses, striped open collar. |
| Jean-Marie Le Pen | [TF1 debate retrospective](https://www.tf1info.fr/politique/videos-quand-bernard-tapie-faisait-le-show-a-la-tele-face-a-jean-marie-le-pen-1538703.html); December 8, 1989 event corroborated by a [separate AFP/Getty photograph](https://www.gettyimages.com.au/detail/news-photo/jean-marie-le-pen-frances-far-right-leader-and-founder-of-news-photo/51350223); reference only | Far left: broad face/full cheeks, light blond-gray wavy receding hair, thin light-rim glasses, dark suit, burgundy-red tie and pocket square. No eyepatch in this era. |

Audit cautions:

- Mauroy's archived image caption contains an anachronistic Socialist International office. It is an appearance source, not proof of that office in 1990.
- Chirac's year is reliable for this reference; the metadata day and event caption are not reconciled, so no exact day is asserted.
- Marchais's October date comes from the filename. It is not independently dated to an exact day and is not the book's publication date.
- Le Pen is the far-left subject, not Tapie at right. AFP photographer Jean-Pierre Muller is credited for the separate Getty corroborating photograph, not asserted as the creator of the TF1 frame.
- Photo licenses and attribution must be checked before distributing photographic assets or photographic derivatives. The current task uses observed features to inform original cartoons; neither the TF1 frame nor the Marchais photograph has a claimed free license.

## Remaining work

Resolve marked chronology gaps, research the unrepresented parties and successor branches, complete non-starter life-date and portrait audits, and design the role-aware import. Fictional successors belong only after **2026-09-07**, must be labelled fictional, and should use institutional/career research without predicting real people's future offices. This artifact creates no future candidates.
