"""CLAUDE-C01-37: French prime ministers, 1990-2014 (the first ten people to hold the office in the period), keep the
Prime Minister's resignation letter, the decree ending the Government's functions, the appointment decree, in-office
signatures and a later decree's reference to an appointment apart, state a start or an end only where a decree does, and
reach france.json only through the importer's supplement merge, beside CLAUDE-C01-23's presidency."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
import import_cnccfp_census as importer


# Original response identity recorded in each extract: (bytes, sha256), the body as received.
RESPONSES = {
    'fr_jorf_rocard_signs_decree_90_89_19900124': (125155, 'a1908cee8352f8bce16ea1a46be6c9a344e0af5e329c7367b26e87af4ed8dad5'),
    'fr_jorf_cessation_rocard_19910515': (96151, '641c04b8c856483c5ebcfe38ef193f6bea867f3a127e849f6a018e566310b2a4'),
    'fr_jorf_nomination_pm_19910515': (86950, '26ba717b5e2d7302abb15e1b71c579c805117395a33aae52150f7c9bb54c52f4'),
    'fr_jorf_composition_cresson_19910516': (98372, '2f72ff08be03ab20b5165c83ce0e3574662ece21db02407832696976460cd90a'),
    'fr_jorf_nomination_beregovoy_19920402': (90663, '46a3e8c1106490530df0e6a1fb74279380ec8ef09a31cf9e6b8d8b896dd85889'),
    'fr_jorf_cessation_beregovoy_19930329': (95411, '682e71ea3654f2807aaeb7a6f604840e88b03bb14a485753ba4a13538626d463'),
    'fr_jorf_composition_balladur_19930330': (99254, '14185647806643feaf46d8bfc268a8499f15415bf26aa32ed0d2639a3ed72d5a'),
    'fr_jorf_cessation_balladur_19950511': (95793, 'a0f08cb31288e7b5a5cecd169438dea4af94a9eab5d674ea0152538425fffa6b'),
    'fr_jorf_composition_juppe_19950518': (101106, 'e449ce207f90d626e822b1ad9c5d9a352f718a99ab8033da868f3126becfcafe'),
    'fr_jorf_nomination_juppe_19951107': (95598, 'dcfa2ee83c47edaac5b3bfe6d2f2b2b649b40b09b3ad81f16c5150b651c6f773'),
    'fr_jorf_cessation_juppe_19970602': (95888, '8dc74e4f25d994971c91bcc6ac8693203ff0f0e217c2a75f98c9d6f94e44127b'),
    'fr_jorf_nomination_jospin_19970602': (13700, '73daa5d467b701ce66a9064709013310f9fa64970a94fcc97fe595984ec2be01'),
    'fr_jorf_cessation_jospin_20020506': (95813, '65a12638e940b7f74cb2a497a5a21f10fd7bb367cda793489c6abe90f25e7072'),
    'fr_jorf_nomination_raffarin_20020506': (90159, '628999a2a21b5b3698dc8150a53a1479bd0cb7c4ddf2564009dbdbb18e2ae594'),
    'fr_jorf_cessation_raffarin_20020617': (95977, '71f6a7afe936baccce05cbc100fe66b5e9ed35d8ee99e2e91aa820d00ead473c'),
    'fr_jorf_nomination_raffarin_20020617': (14117, '6fa110753a8578d21c93fbf044d4fa3edd2912e346ca0a40124e10e88b3f7023'),
    'fr_jorf_cessation_raffarin_20040330': (14219, '1afd2ac3da974c8b058cd6e1033f28115609097e25dd015fa3e1a91e71c710c5'),
    'fr_jorf_nomination_raffarin_20040330': (84406, 'd298c137e04651d9b1a84135499896e285aef38679a60785758685e49848d54f'),
    'fr_jorf_cessation_raffarin_20050531': (96315, 'deb19cc59a340becedb59312a1a0dd01871ad503e4cb7cb24fa1ac272cebb9b7'),
    'fr_jorf_nomination_villepin_20050531': (14329, 'ae15c310d8216fe6065630ea04a0822e85d78506e8a4cdfe1d43d4215a67af19'),
    'fr_jorf_cessation_villepin_20070515': (96367, 'b8caaed39abdbe2f798da49acbc3820e782f4d01c861169c07cb0ba7ec041006'),
    'fr_jorf_nomination_fillon_20070517': (86376, '5f5145d374235a955e245a6d01fd7dbc7adb7d19d0b6d4335bfaf6f8df0ffaa1'),
    'fr_jorf_cessation_fillon_20070618': (96182, '2104cfbb73ca35e6f3a02f8a638e5c784815db91e8cb4231c0afa7feb4fdd4cc'),
    'fr_jorf_nomination_fillon_20070618': (12853, '8afa43792d59eefb13bf20ff887dbd26446339995e868b1c172d1286c3ded11a'),
    'fr_jorf_cessation_fillon_20101113': (96330, 'dc045200ebe6d940f97d97ffe376a328c5a271bcd7c675009340ba67ebb6413c'),
    'fr_jorf_nomination_fillon_20101114': (13934, '4c7e31a2fddd3a08157b531fc243f320913ede2323704aca89ce59de751e931f'),
    'fr_jorf_cessation_fillon_20120510': (14237, 'ee373dbfdc5ba19d6d2c03a1e1cf90a6d5e46c69ec12e977bc6514b21ff267ef'),
    'fr_jorf_nomination_ayrault_20120515': (9254, '21123c4f083fb056d2d23f0691770b31a8985e119748bc885aa9c79bfb2a8c4f'),
    'fr_jorf_cessation_ayrault_20120618': (96311, '617c0575880c6e69423c1e8770af660bb742ed8e79494902517138cdde750d5c'),
    'fr_jorf_nomination_ayrault_20120618': (11111, '1e0b3e6eb62a3fa56542421426f920f8e754fa3dbb74a899b716381f44894a91'),
    'fr_jorf_cessation_ayrault_20140331': (96306, '77133a1371cbac62e854844db44ab7bcb4673f1805ebf879ecea09c2b9736d0d'),
}
NEW_SOURCES = list(RESPONSES)
# Raw Internet Archive captures (id_ form, all made before the 7 September 2026 cutoff): id -> timestamp.
ARCHIVED = {
    'fr_jorf_rocard_signs_decree_90_89_19900124': '20240811131158',
    'fr_jorf_cessation_rocard_19910515': '20240906160302',
    'fr_jorf_nomination_pm_19910515': '20230405155856',
    'fr_jorf_composition_cresson_19910516': '20240503222542',
    'fr_jorf_nomination_beregovoy_19920402': '20211030085525',
    'fr_jorf_cessation_beregovoy_19930329': '20240906145330',
    'fr_jorf_composition_balladur_19930330': '20240906145339',
    'fr_jorf_cessation_balladur_19950511': '20240906200847',
    'fr_jorf_composition_juppe_19950518': '20240906155743',
    'fr_jorf_nomination_juppe_19951107': '20240623064712',
    'fr_jorf_cessation_juppe_19970602': '20240814014613',
    'fr_jorf_nomination_jospin_19970602': '20231210052251',
    'fr_jorf_cessation_jospin_20020506': '20240906163240',
    'fr_jorf_nomination_raffarin_20020506': '20211202095758',
    'fr_jorf_cessation_raffarin_20020617': '20240906160054',
    'fr_jorf_nomination_raffarin_20020617': '20160316141105',
    'fr_jorf_cessation_raffarin_20040330': '20160317044028',
    'fr_jorf_nomination_raffarin_20040330': '20201030005445',
    'fr_jorf_cessation_raffarin_20050531': '20240906175445',
    'fr_jorf_nomination_villepin_20050531': '20190714133804',
    'fr_jorf_cessation_villepin_20070515': '20240906193501',
    'fr_jorf_nomination_fillon_20070517': '20220702183622',
    'fr_jorf_cessation_fillon_20070618': '20240906152409',
    'fr_jorf_nomination_fillon_20070618': '20220702212658',
    'fr_jorf_cessation_fillon_20101113': '20240906150128',
    'fr_jorf_nomination_fillon_20101114': '20160411134055',
    'fr_jorf_cessation_fillon_20120510': '20190422124400',
    'fr_jorf_nomination_ayrault_20120515': '20120525031934',
    'fr_jorf_cessation_ayrault_20120618': '20240906202615',
    'fr_jorf_nomination_ayrault_20120618': '20130614074937',
    'fr_jorf_cessation_ayrault_20140331': '20240906145012',
}
# Captures the Internet Archive serves gzip-compressed even to Accept-Encoding: identity; identity = compressed body as received.
GZIP = {'fr_jorf_nomination_jospin_19970602', 'fr_jorf_nomination_fillon_20070618'}
# Every new claim's (attested_on, event_kind, review observation), exactly.
EVENTS = {
    'fr_jorf_rocard_signs_decree_as_pm_19900124': ('1990-01-24', 'in_office_signature_as_pm', 'FR-PM-01'),
    'fr_jorf_rocard_government_resignation_letter_19910515': ('1991-05-15', 'government_resignation_presented', 'FR-PM-01'),
    'fr_jorf_rocard_functions_ended_19910515': ('1991-05-15', 'cessation_of_functions_decree', 'FR-PM-01'),
    'fr_jorf_pm_appointment_decree_title_19910515': ('1991-05-15', 'appointment_decree_text_not_rendered', 'FR-PM-02'),
    'fr_jorf_cresson_countersigns_composition_19910516': ('1991-05-16', 'in_office_countersignature', 'FR-PM-02'),
    'fr_jorf_composition_cites_pm_appointment_19910515': ('1991-05-15', 'appointment_decree_reference', 'FR-PM-02'),
    'fr_jorf_beregovoy_appointed_pm_19920402': ('1992-04-02', 'appointment_decree', 'FR-PM-03'),
    'fr_jorf_beregovoy_government_resignation_letter_19930329': ('1993-03-29', 'government_resignation_presented', 'FR-PM-03'),
    'fr_jorf_beregovoy_functions_ended_19930329': ('1993-03-29', 'cessation_of_functions_decree', 'FR-PM-03'),
    'fr_jorf_balladur_countersigns_composition_19930330': ('1993-03-30', 'in_office_countersignature', 'FR-PM-04'),
    'fr_jorf_composition_cites_pm_appointment_19930329': ('1993-03-29', 'appointment_decree_reference', 'FR-PM-04'),
    'fr_jorf_balladur_government_resignation_letter_19950510': ('1995-05-10', 'government_resignation_presented', 'FR-PM-04'),
    'fr_jorf_balladur_functions_ended_19950511': ('1995-05-11', 'cessation_of_functions_decree', 'FR-PM-04'),
    'fr_jorf_juppe_countersigns_composition_19950518': ('1995-05-18', 'in_office_countersignature', 'FR-PM-05'),
    'fr_jorf_composition_cites_pm_appointment_19950517': ('1995-05-17', 'appointment_decree_reference', 'FR-PM-05'),
    'fr_jorf_juppe_appointed_pm_19951107': ('1995-11-07', 'appointment_decree', 'FR-PM-05'),
    'fr_jorf_juppe_government_resignation_letter_19970602': ('1997-06-02', 'government_resignation_presented', 'FR-PM-05'),
    'fr_jorf_juppe_functions_ended_19970602': ('1997-06-02', 'cessation_of_functions_decree', 'FR-PM-05'),
    'fr_jorf_jospin_appointed_pm_19970602': ('1997-06-02', 'appointment_decree', 'FR-PM-06'),
    'fr_jorf_jospin_government_resignation_letter_20020506': ('2002-05-06', 'government_resignation_presented', 'FR-PM-06'),
    'fr_jorf_jospin_functions_ended_20020506': ('2002-05-06', 'cessation_of_functions_decree', 'FR-PM-06'),
    'fr_jorf_raffarin_appointed_pm_20020506': ('2002-05-06', 'appointment_decree', 'FR-PM-07'),
    'fr_jorf_raffarin_government_resignation_letter_20020617': ('2002-06-17', 'government_resignation_presented', 'FR-PM-07'),
    'fr_jorf_raffarin_functions_ended_20020617': ('2002-06-17', 'cessation_of_functions_decree', 'FR-PM-07'),
    'fr_jorf_raffarin_appointed_pm_20020617': ('2002-06-17', 'appointment_decree', 'FR-PM-07'),
    'fr_jorf_raffarin_government_resignation_letter_20040330': ('2004-03-30', 'government_resignation_presented', 'FR-PM-07'),
    'fr_jorf_raffarin_functions_ended_20040330': ('2004-03-30', 'cessation_of_functions_decree', 'FR-PM-07'),
    'fr_jorf_raffarin_appointed_pm_20040330': ('2004-03-30', 'appointment_decree', 'FR-PM-07'),
    'fr_jorf_raffarin_government_resignation_letter_20050531': ('2005-05-31', 'government_resignation_presented', 'FR-PM-07'),
    'fr_jorf_raffarin_functions_ended_20050531': ('2005-05-31', 'cessation_of_functions_decree', 'FR-PM-07'),
    'fr_jorf_villepin_appointed_pm_20050531': ('2005-05-31', 'appointment_decree', 'FR-PM-08'),
    'fr_jorf_villepin_government_resignation_letter_20070515': ('2007-05-15', 'government_resignation_presented', 'FR-PM-08'),
    'fr_jorf_villepin_functions_ended_20070515': ('2007-05-15', 'cessation_of_functions_decree', 'FR-PM-08'),
    'fr_jorf_fillon_appointed_pm_20070517': ('2007-05-17', 'appointment_decree', 'FR-PM-09'),
    'fr_jorf_fillon_government_resignation_letter_20070618': ('2007-06-18', 'government_resignation_presented', 'FR-PM-09'),
    'fr_jorf_fillon_functions_ended_20070618': ('2007-06-18', 'cessation_of_functions_decree', 'FR-PM-09'),
    'fr_jorf_fillon_appointed_pm_20070618': ('2007-06-18', 'appointment_decree', 'FR-PM-09'),
    'fr_jorf_fillon_government_resignation_letter_20101113': ('2010-11-13', 'government_resignation_presented', 'FR-PM-09'),
    'fr_jorf_fillon_functions_ended_20101113': ('2010-11-13', 'cessation_of_functions_decree', 'FR-PM-09'),
    'fr_jorf_fillon_appointed_pm_20101114': ('2010-11-14', 'appointment_decree', 'FR-PM-09'),
    'fr_jorf_fillon_government_resignation_letter_20120510': ('2012-05-10', 'government_resignation_presented', 'FR-PM-09'),
    'fr_jorf_fillon_functions_ended_20120510': ('2012-05-10', 'cessation_of_functions_decree', 'FR-PM-09'),
    'fr_jorf_ayrault_appointed_pm_20120515': ('2012-05-15', 'appointment_decree', 'FR-PM-10'),
    'fr_jorf_ayrault_government_resignation_letter_20120618': ('2012-06-18', 'government_resignation_presented', 'FR-PM-10'),
    'fr_jorf_ayrault_functions_ended_20120618': ('2012-06-18', 'cessation_of_functions_decree', 'FR-PM-10'),
    'fr_jorf_ayrault_appointed_pm_20120618': ('2012-06-18', 'appointment_decree', 'FR-PM-10'),
    'fr_jorf_ayrault_government_resignation_letter_20140331': ('2014-03-31', 'government_resignation_presented', 'FR-PM-10'),
    'fr_jorf_ayrault_functions_ended_20140331': ('2014-03-31', 'cessation_of_functions_decree', 'FR-PM-10'),
}
# Exact holder observations, in chronological order: (name, attested_on, from, until).
HOLDERS = [
    ('Michel Rocard', '1990-01-24', None, '1991-05-15'),
    ('Édith Cresson', '1991-05-16', None, None),
    ('Pierre Bérégovoy', None, '1992-04-02', '1993-03-29'),
    ('Édouard Balladur', '1993-03-30', None, '1995-05-11'),
    ('Alain Juppé', '1995-05-18', None, None),
    ('Alain Juppé', None, '1995-11-07', '1997-06-02'),
    ('Lionel Jospin', None, '1997-06-02', '2002-05-06'),
    ('Jean-Pierre Raffarin', None, '2002-05-06', '2002-06-17'),
    ('Jean-Pierre Raffarin', None, '2002-06-17', '2004-03-30'),
    ('Jean-Pierre Raffarin', None, '2004-03-30', '2005-05-31'),
    ('Dominique de Villepin', None, '2005-05-31', '2007-05-15'),
    ('François Fillon', None, '2007-05-17', '2007-06-18'),
    ('François Fillon', None, '2007-06-18', '2010-11-13'),
    ('François Fillon', None, '2010-11-14', '2012-05-10'),
    ('Jean-Marc Ayrault', None, '2012-05-15', '2012-06-18'),
    ('Jean-Marc Ayrault', None, '2012-06-18', '2014-03-31'),
]
HOLDER_CLAIMS = [
    ['fr_jorf_rocard_signs_decree_as_pm_19900124', 'fr_jorf_rocard_functions_ended_19910515'],
    ['fr_jorf_cresson_countersigns_composition_19910516'],
    ['fr_jorf_beregovoy_appointed_pm_19920402', 'fr_jorf_beregovoy_functions_ended_19930329'],
    ['fr_jorf_balladur_countersigns_composition_19930330', 'fr_jorf_balladur_functions_ended_19950511'],
    ['fr_jorf_juppe_countersigns_composition_19950518'],
    ['fr_jorf_juppe_appointed_pm_19951107', 'fr_jorf_juppe_functions_ended_19970602'],
    ['fr_jorf_jospin_appointed_pm_19970602', 'fr_jorf_jospin_functions_ended_20020506'],
    ['fr_jorf_raffarin_appointed_pm_20020506', 'fr_jorf_raffarin_functions_ended_20020617'],
    ['fr_jorf_raffarin_appointed_pm_20020617', 'fr_jorf_raffarin_functions_ended_20040330'],
    ['fr_jorf_raffarin_appointed_pm_20040330', 'fr_jorf_raffarin_functions_ended_20050531'],
    ['fr_jorf_villepin_appointed_pm_20050531', 'fr_jorf_villepin_functions_ended_20070515'],
    ['fr_jorf_fillon_appointed_pm_20070517', 'fr_jorf_fillon_functions_ended_20070618'],
    ['fr_jorf_fillon_appointed_pm_20070618', 'fr_jorf_fillon_functions_ended_20101113'],
    ['fr_jorf_fillon_appointed_pm_20101114', 'fr_jorf_fillon_functions_ended_20120510'],
    ['fr_jorf_ayrault_appointed_pm_20120515', 'fr_jorf_ayrault_functions_ended_20120618'],
    ['fr_jorf_ayrault_appointed_pm_20120618', 'fr_jorf_ayrault_functions_ended_20140331'],
]
INSTITUTION, ROLE = 'fr_prime_minister', 'fr_pm'
PRESIDENCY, PRESIDENT = 'fr_presidency', 'fr_president'
C01_23_SOURCES = 49
ACCESSED = '2026-09-29'
# The claim each `from` rests on (an appointment decree whose text names the appointee), by holder index.
FROM_BASIS = {2: 'fr_jorf_beregovoy_appointed_pm_19920402', 5: 'fr_jorf_juppe_appointed_pm_19951107',
              6: 'fr_jorf_jospin_appointed_pm_19970602', 7: 'fr_jorf_raffarin_appointed_pm_20020506',
              8: 'fr_jorf_raffarin_appointed_pm_20020617', 9: 'fr_jorf_raffarin_appointed_pm_20040330',
              10: 'fr_jorf_villepin_appointed_pm_20050531', 11: 'fr_jorf_fillon_appointed_pm_20070517',
              12: 'fr_jorf_fillon_appointed_pm_20070618', 13: 'fr_jorf_fillon_appointed_pm_20101114',
              14: 'fr_jorf_ayrault_appointed_pm_20120515', 15: 'fr_jorf_ayrault_appointed_pm_20120618'}
# The claim each `until` rests on (the decree ending the Government's functions), by holder index.
UNTIL_BASIS = {0: 'fr_jorf_rocard_functions_ended_19910515', 2: 'fr_jorf_beregovoy_functions_ended_19930329',
               3: 'fr_jorf_balladur_functions_ended_19950511', 5: 'fr_jorf_juppe_functions_ended_19970602',
               6: 'fr_jorf_jospin_functions_ended_20020506', 7: 'fr_jorf_raffarin_functions_ended_20020617',
               8: 'fr_jorf_raffarin_functions_ended_20040330', 9: 'fr_jorf_raffarin_functions_ended_20050531',
               10: 'fr_jorf_villepin_functions_ended_20070515', 11: 'fr_jorf_fillon_functions_ended_20070618',
               12: 'fr_jorf_fillon_functions_ended_20101113', 13: 'fr_jorf_fillon_functions_ended_20120510',
               14: 'fr_jorf_ayrault_functions_ended_20120618', 15: 'fr_jorf_ayrault_functions_ended_20140331'}
# The claim each observation date rests on (an in-office signature or countersignature), by holder index.
ATTESTED_BASIS = {0: 'fr_jorf_rocard_signs_decree_as_pm_19900124', 1: 'fr_jorf_cresson_countersigns_composition_19910516',
                  3: 'fr_jorf_balladur_countersigns_composition_19930330', 4: 'fr_jorf_juppe_countersigns_composition_19950518'}
SURNAMES = {'Michel Rocard': 'rocard', 'Édith Cresson': 'cresson', 'Pierre Bérégovoy': 'bérégovoy',
            'Édouard Balladur': 'balladur', 'Alain Juppé': 'juppé', 'Lionel Jospin': 'jospin',
            'Jean-Pierre Raffarin': 'raffarin', 'Dominique de Villepin': 'villepin', 'François Fillon': 'fillon',
            'Jean-Marc Ayrault': 'ayrault'}
HOLDER_KINDS = {'appointment_decree', 'cessation_of_functions_decree', 'in_office_signature_as_pm', 'in_office_countersignature'}
# Kinds that never feed a holder: the resignation letter, a reference to an appointment that names nobody and a page
# that renders no decree text.
NEVER_KINDS = {'government_resignation_presented', 'appointment_decree_reference', 'appointment_decree_text_not_rendered'}
NEVER_HOLDER = tuple(cid for cid, (_, kind, _) in EVENTS.items() if kind in NEVER_KINDS)
# Per holder: days that must never be its start or end (publication days, the resignation letter where it precedes the
# decree, a reference to an appointment that names nobody, and the successor's start where no end is stated or the
# successor was appointed later).
NEVER_BOUNDARY = {
    0: {'1991-05-16'}, 1: {'1991-05-15', '1991-05-17', '1992-04-02', '1992-04-03'}, 2: {'1992-04-03', '1993-03-30'},
    3: {'1993-03-29', '1993-03-31', '1995-05-10', '1995-05-12', '1995-05-17'}, 4: {'1995-05-17', '1995-05-19', '1995-11-07'},
    5: {'1995-11-08', '1997-06-03'}, 6: {'1997-06-03', '2002-05-07'}, 7: {'2002-05-07', '2002-06-18'},
    8: {'2002-06-18', '2004-03-31'}, 9: {'2004-03-31', '2005-06-01'}, 10: {'2005-06-01', '2007-05-16', '2007-05-17'},
    11: {'2007-05-19', '2007-06-19'}, 12: {'2007-06-19', '2010-11-14'}, 13: {'2010-11-16', '2012-05-11', '2012-05-15'},
    14: {'2012-05-16', '2012-06-19'}, 15: {'2012-06-19', '2014-04-01'}}
STARTS = [(HOLDERS[i][0], HOLDERS[i][2]) for i in sorted(FROM_BASIS)]
ENDS = [(HOLDERS[i][0], HOLDERS[i][3]) for i in sorted(UNTIL_BASIS)]
PEOPLE = ['Michel Rocard', 'Édith Cresson', 'Pierre Bérégovoy', 'Édouard Balladur', 'Alain Juppé', 'Lionel Jospin',
          'Jean-Pierre Raffarin', 'Dominique de Villepin', 'François Fillon', 'Jean-Marc Ayrault']
# Leads (encyclopaedias, retrospective lists, listing pages, the next batch's decree, identifiers without a usable capture
# and unfetched downloads): never an identity URL.
LEAD_URL_MARKERS = ('wikipedia', 'attribmin', 'persee', 'info.gouv.fr', 'gouvernement.fr', 'assemblee-nationale.fr',
                    'france-politique', '/jorf/jo/', 'WAspad', '/eli/', 'Freemium', 'download', 'gallica',
                    'JORFTEXT000028811098', 'JORFTEXT000000189843', 'JORFTEXT000000726609', 'JORFTEXT000000718399',
                    'JORFTEXT000000539723')
# URL fragments that mark a response generated per request, a cache-busting query, a signed or session URL, an unresolved or
# non-raw capture, or a growing search or listing page; never allowed in a recorded identity. The earlier Légifrance
# address carries only the fixed cidTexte parameter.
VOLATILE_URL = re.compile(r'([?&](cb|_|_cb|nocache|token|sig|exp|s|q|query|page|dateTexte|categorieLien|oldAction)=|jsessionid|'
                          r'PHPSESSID|/recherche|/search|/cdx/|/api/|/web/\d{4}id_/|/web/\d{14}/|Freemium)', re.I)
REPORT = research.RESEARCH / 'france-prime-ministers-1990-2026-37.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-37.md'


def pm_invariants(packet):
    """Packet-level rules this test owns; raises AssertionError, KeyError or IndexError on any violation."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    owner = {c['id']: s['id'] for s in packet['sources'] for c in s['claims']}
    assert [e['id'] for e in packet['institutions']] == [PRESIDENCY, INSTITUTION], 'presidency, then the prime ministership'
    pm = packet['institutions'][1]
    assert pm['kind'] == 'executive_institution' and pm['name'] == 'Premier ministre'
    assert pm['represented_party_ids'] == [] and pm['reconciled_organization_id'] is None
    assert pm['lifecycle']['status'] == 'unknown'
    assert pm['lifecycle']['from'] is None and pm['lifecycle']['until'] is None
    assert [(r['id'], r['kind'], r['title']) for r in pm['roles']] == [(ROLE, 'head_of_government', 'Premier ministre')]
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles'] if r['kind'] == 'head_of_government']
    assert heads == [ROLE], 'no other head-of-government role'
    role = pm['roles'][0]
    holders = role['holder_claims']
    assert all(isinstance(h, dict) for h in holders)
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in holders] == HOLDERS
    assert [h['claim_ids'] for h in holders] == HOLDER_CLAIMS
    assert sorted({h['name'] for h in holders}, key=PEOPLE.index) == PEOPLE and len(PEOPLE) == 10, 'at most ten people'
    new_claims, new_sources = set(EVENTS), set(RESPONSES)
    for index, h in enumerate(holders):
        assert not set(h['claim_ids']) & set(NEVER_HOLDER), h['name']
        assert not {h['attested_on'], h['from'], h['until']} & NEVER_BOUNDARY[index], (index, h['name'])
        assert (h['attested_on'] is None) != (h['from'] is None), 'a holder is dated by a start or by an observation'
        assert set(h['claim_ids']) <= new_claims and set(h['sources']) <= new_sources, 'no cross-role claim feeds a holder'
        for cid in h['claim_ids']:
            assert cid in role['claim_ids'] and cid in pm['claim_ids'], cid
            assert owner[cid] in h['sources'], cid
        # A start is exactly the day of its appointment decree, an end exactly the day of its decree ending the functions,
        # an observation exactly the day of its signature; nothing else dates a holder.
        if h['from'] is not None:
            basis = FROM_BASIS[index]
            assert h['claim_ids'][0] == basis and claims[basis]['attested_on'] == h['from'], index
            assert 'est nommé Premier ministre' in claims[basis]['text'], index
        else:
            assert index not in FROM_BASIS, index
        if h['attested_on'] is not None:
            basis = ATTESTED_BASIS[index]
            assert h['claim_ids'][0] == basis and claims[basis]['attested_on'] == h['attested_on'], index
        else:
            assert index not in ATTESTED_BASIS, index
        if h['until'] is not None:
            basis = UNTIL_BASIS[index]
            assert h['claim_ids'][-1] == basis and claims[basis]['attested_on'] == h['until'], index
            assert 'Il est mis fin' in claims[basis]['text'], index
            assert (h['from'] or h['attested_on']) <= h['until'], index
        else:
            assert index not in UNTIL_BASIS, index
            assert 'No end' in h['uncertainty'], index
    assert [(h['name'], h['from']) for h in holders if h['from']] == STARTS
    assert [(h['name'], h['until']) for h in holders if h['until']] == ENDS
    # No end is inferred from a successor's start: where the successor was appointed later (2007, 2010, 2012) the end
    # stays on the earlier day of its own decree.
    for index in range(len(holders) - 1):
        nxt = holders[index + 1]
        if holders[index]['until'] is not None and nxt['from'] and nxt['from'] != holders[index]['until']:
            assert holders[index]['until'] < nxt['from'], index
    for cid, (day, _, _) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid
    # Separation: no organization cites this institution's claims or sources, and the presidency cites none of them.
    for entry in packet['organizations']:
        assert not set(entry['claim_ids']) & new_claims and not set(entry['sources']) & new_sources, entry['id']
        assert entry['roles'] == [] and entry['represented_party_ids'] == [], entry['id']
    presidency = packet['institutions'][0]
    assert presidency['id'] == PRESIDENCY and not set(presidency['claim_ids']) & new_claims
    assert not set(presidency['sources']) & new_sources
    for r in presidency['roles']:
        assert not set(r['claim_ids']) & new_claims and not set(r['sources']) & new_sources
        for h in r['holder_claims']:
            assert not set(h['claim_ids']) & new_claims
    assert role['claim_ids'] == pm['claim_ids'] and set(role['claim_ids']) == new_claims
    assert role['sources'] == pm['sources'] and set(role['sources']) == new_sources


class FrancePrimeMinistersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.csv = (importer.ROOT / importer.RAW).read_bytes()
        cls.raw = (research.ROOT / research.RESEARCH / 'france.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.supplement_bytes = (importer.ROOT / importer.SUPPLEMENT).read_bytes()
        cls.supplement = json.loads(cls.supplement_bytes)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.pm = cls.packet['institutions'][1]
        cls.role = cls.pm['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = {row['claim_id']: row for sid in NEW_SOURCES for row in cls.extracts[sid]['rows']}
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'France'}, {'France': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_packet_is_the_importer_output_and_only_appends_to_the_supplement(self):
        built = importer.build(self.csv)
        self.assertEqual(self.raw, json.dumps(built, ensure_ascii=False, indent=2) + '\n')
        self.assertNotIn(b'\r', (research.ROOT / research.RESEARCH / 'france.json').read_bytes())
        self.assertNotIn(b'\r', self.supplement_bytes)
        self.assertEqual(self.supplement_bytes.decode('utf-8'), json.dumps(self.supplement, indent=2, ensure_ascii=False) + '\n')
        self.assertEqual(list(self.supplement), ['sources', 'institutions', 'coverage_unresolved'])
        # CLAUDE-C01-23's 49 sources and its institution come first; this packet only appends.
        ids = [s['id'] for s in self.supplement['sources']]
        self.assertEqual(len(ids), C01_23_SOURCES + len(NEW_SOURCES))
        self.assertEqual(ids[C01_23_SOURCES:], NEW_SOURCES)
        self.assertFalse(set(ids[:C01_23_SOURCES]) & set(NEW_SOURCES))
        self.assertEqual([i['id'] for i in self.supplement['institutions']], [PRESIDENCY, INSTITUTION])
        self.assertEqual(self.packet['institutions'], self.supplement['institutions'])
        notes = self.supplement['coverage_unresolved']
        self.assertEqual(len(notes), 2)
        self.assertTrue(notes[0].startswith('CLAUDE-C01-23 adds') and notes[1].startswith('CLAUDE-C01-37 adds'))
        self.assertEqual(self.packet['coverage']['unresolved'][-2:], notes)
        base = importer.build(self.csv, supplement=b'{"sources": [], "institutions": [], "coverage_unresolved": []}')
        self.assertEqual(self.packet['organizations'], base['organizations'])
        self.assertEqual(len(base['organizations']), 635)

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (31, 48))
        self.assertEqual([s['id'] for s in self.packet['sources']][-31:], NEW_SOURCES)
        self.assertEqual(len(ids['entries']), 637)
        self.assertEqual(ids['roles'], {PRESIDENT, ROLE})
        self.assertEqual(set(self.new_claims), set(EVENTS))
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual({self.rows[cid]['event_kind'] for cid in holder_claims}, HOLDER_KINDS)
        self.assertEqual({kind for _, kind, _ in EVENTS.values()}, NEVER_KINDS | HOLDER_KINDS)
        self.assertEqual(self.pm['claim_ids'], self.new_claims)
        self.assertEqual(self.role['claim_ids'], self.new_claims)
        self.assertEqual(self.pm['sources'], NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES)
        observations = re.findall(r'^### (FR-PM-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'FR-PM-{n:02d}' for n in range(1, 13)])
        # FR-PM-11 (current affairs, acting service) records an absence and FR-PM-12 the next batch; neither owns a row.
        self.assertEqual({row['review_observation'] for row in self.rows.values()}, {f'FR-PM-{n:02d}' for n in range(1, 11)})
        # Rows name a holder only where the source text does; references and the title-only page name nobody.
        for cid, (_, kind, _) in EVENTS.items():
            if kind in ('appointment_decree_reference', 'appointment_decree_text_not_rendered'):
                self.assertIsNone(self.rows[cid]['holder_name'], cid)
            else:
                self.assertTrue(self.rows[cid]['holder_name'], cid)

    def test_holders_are_exactly_as_intended(self):
        pm_invariants(self.packet)
        for holder in self.role['holder_claims']:
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected:
                    expected.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected, holder['name'])
            surname = SURNAMES[holder['name']]
            for cid in holder['claim_ids']:
                row = self.rows[cid]
                self.assertEqual((row['role_id'], row['observation_id']), (ROLE, INSTITUTION), cid)
                self.assertIn(row['event_kind'], HOLDER_KINDS, cid)
                self.assertIn(surname, row['holder_name'].casefold(), cid)
            if holder['from']:
                self.assertTrue(holder['note'].startswith('From '), holder['name'])
            else:
                self.assertIn('No start', holder['uncertainty'], holder['name'])
                self.assertTrue(holder['note'].startswith('Observed on '), holder['name'])
        # The resignation letter, references and the title-only page say why they never carry a boundary.
        for cid in NEVER_HOLDER:
            self.assertRegex(self.claims[cid]['uncertainty'], r"never (an end|a holder date|a holder's start)", cid)
        scope = self.role['scope_note']
        for text in ('never ends a term', "a successor's appointment never ends a term", 'is never a start',
                     'Publication in the Journal officiel is never a boundary', 'procedure only, never a date',
                     'claim only, never a holder', 'first ten people'):
            self.assertIn(text, scope, text)

    def test_distinct_events_keep_distinct_claims(self):
        claims = self.claims
        # The resignation letter and the decree ending the functions are two claims even on the same day; in 1995 the
        # letter (10 May) precedes the decree (11 May), and the end is the decree's day.
        self.assertEqual(claims['fr_jorf_balladur_government_resignation_letter_19950510']['attested_on'], '1995-05-10')
        self.assertEqual(claims['fr_jorf_balladur_functions_ended_19950511']['attested_on'], '1995-05-11')
        self.assertEqual(self.role['holder_claims'][3]['until'], '1995-05-11')
        letters = [cid for cid, (_, kind, _) in EVENTS.items() if kind == 'government_resignation_presented']
        ended = [cid for cid, (_, kind, _) in EVENTS.items() if kind == 'cessation_of_functions_decree']
        self.assertEqual(len(letters), len(ended))
        for letter, end in zip(letters, ended):
            self.assertEqual(self.claim_source[letter], self.claim_source[end])
            self.assertLessEqual(claims[letter]['attested_on'], claims[end]['attested_on'])
        # Appointment and cessation are separate decrees, even on the same day, and a gap stays a gap.
        for cess, nom in (('fr_jorf_raffarin_functions_ended_20020617', 'fr_jorf_raffarin_appointed_pm_20020617'),
                          ('fr_jorf_fillon_functions_ended_20101113', 'fr_jorf_fillon_appointed_pm_20101114'),
                          ('fr_jorf_villepin_functions_ended_20070515', 'fr_jorf_fillon_appointed_pm_20070517'),
                          ('fr_jorf_fillon_functions_ended_20120510', 'fr_jorf_ayrault_appointed_pm_20120515')):
            self.assertNotEqual(self.claim_source[cess], self.claim_source[nom])
            self.assertLessEqual(claims[cess]['attested_on'], claims[nom]['attested_on'])
        # References are dated by the decree they cite and are never a start.
        for cid, day in (('fr_jorf_composition_cites_pm_appointment_19910515', '1991-05-15'),
                         ('fr_jorf_composition_cites_pm_appointment_19930329', '1993-03-29'),
                         ('fr_jorf_composition_cites_pm_appointment_19950517', '1995-05-17')):
            self.assertEqual(claims[cid]['attested_on'], day)
            self.assertIn('dated by the decree it cites', claims[cid]['uncertainty'])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            self.assertEqual(extract['scope_note'], source['scope_note'])
            self.assertEqual(extract['access_method'], source['access_method'])
            self.assertEqual(extract['published_date'], source['published_date'])
            self.assertEqual((extract['accessed_date'], source['accessed_date']), (ACCESSED, ACCESSED))
            self.assertLessEqual(source['published_date'], research.CUTOFF)
            self.assertFalse(extract['source_response_checked_in'])
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            for text in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                         'Raw Internet Archive capture'):
                self.assertIn(text, extract['provenance_note'], text)
            if sid in GZIP:
                self.assertIn('Content-Encoding: gzip', extract['provenance_note'])
                self.assertIn('the gzip stream', extract['provenance_note'])
                self.assertIn('gzip-compressed', source['scope_note'])
            else:
                self.assertIn('served without Content-Encoding', extract['provenance_note'])
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['rights_note'], source['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], [])
            self.assertEqual(source['source_type'], 'primary_official_journal_text_archived')
            self.assertIn('which was not bypassed', source['scope_note'])
            snapshot = source['snapshot']
            self.assertRegex(snapshot['path'], r'^docs/campaign-certification/C01/research/sources/france-jorf-[a-z0-9-]+-\d{8}-facts\.json$')
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim['attested_on']))
                self.assertEqual((row['observation_id'], row['role_id'], row['role_title']), (INSTITUTION, ROLE, 'Premier ministre'))
                self.assertNotIn('name', row)
                self.assertTrue(row['holder_name'] is None or row['holder_name'].strip())
                self.assertEqual((row['attested_on'], row['event_kind'], row['review_observation']), EVENTS[row['claim_id']])
                self.assertTrue(claim['uncertainty'])
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(urlsplit(source['original_url']).hostname, 'www.legifrance.gouv.fr')
            self.assertTrue(source['url'].replace(':80/', '/').endswith(source['original_url'].split('://', 1)[1]), sid)
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'), stamp)
        self.assertEqual(set(ARCHIVED), set(NEW_SOURCES))

    def test_leads_and_volatile_responses_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            source = self.sources[sid]
            for url in (source['url'], source['original_url']):
                self.assertIsNone(VOLATILE_URL.search(url), url)
                for marker in LEAD_URL_MARKERS:
                    self.assertNotIn(marker, url, (sid, marker))
            query = urlsplit(source['original_url']).query
            self.assertIn(query, ('', f"cidTexte={source['original_url'].rsplit('=', 1)[-1]}"), sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'wikipédia', 'britannica', 'larousse', 'attribmin', 'persee', 'france-politique'):
            self.assertNotIn(marker, lowered, marker)
        leads, added, attempted = self.section('Leads not imported'), self.section('Sources added'), self.section('Sources attempted')
        for marker in ('attribmin', 'info.gouv.fr', 'assemblee-nationale.fr', '/jorf/jo/', 'JORFTEXT000028811098'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)
        for marker in ('legifrance.gouv.fr', 'HTTP 403', 'not bypassed', 'JORFTEXT000000539723', 'JORFTEXT000000726609',
                       'JORFTEXT000000718399', 'JORFTEXT000000189843', 'gallica'):
            self.assertIn(marker, attempted, marker)
            self.assertNotIn(marker, added, marker)

    def test_mutations_are_rejected(self):
        def mutated(change):
            packet = copy.deepcopy(self.packet)
            change(packet)
            return packet

        def source(packet, sid):
            return next(s for s in packet['sources'] if s['id'] == sid)

        def claim(packet, cid):
            return next(c for s in packet['sources'] for c in s['claims'] if c['id'] == cid)

        def role(packet):
            return packet['institutions'][1]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        validator_cases = [
            (lambda p: source(p, 'fr_jorf_cessation_rocard_19910515')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'fr_jorf_nomination_jospin_19970602')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'fr_jorf_cessation_ayrault_20140331')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 15).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'fr_jorf_ayrault_functions_ended_20140331').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 2).update(until='1992-04-01'), 'Reversed historical interval'),
            (lambda p: holder(p, 6)['claim_ids'].append('fr_jorf_juppe_functions_ended_19970602'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('fr_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))
        acting = {'name': 'Acting Prime Minister', 'attested_on': '1991-05-15', 'from': None, 'until': None,
                  'sources': ['fr_jorf_cessation_rocard_19910515'], 'claim_ids': ['fr_jorf_rocard_government_resignation_letter_19910515'],
                  'note': 'Continued handling of current affairs.', 'uncertainty': 'No end.'}
        invariant_cases = [
            ("successor's start used as an end (Cresson)", lambda p: holder(p, 1).update(until='1992-04-02')),
            ("successor's start used as an end (Juppé 1995)", lambda p: holder(p, 4).update(until='1995-11-07')),
            ("successor's start used as an end (Villepin)", lambda p: holder(p, 10).update(until='2007-05-17')),
            ('reappointment used as an end (Fillon 2007-2010)', lambda p: holder(p, 12).update(until='2010-11-14')),
            ("successor's start used as an end (Fillon 2012)", lambda p: holder(p, 13).update(until='2012-05-15')),
            ('resignation letter used as an end (Balladur)', lambda p: holder(p, 3).update(until='1995-05-10')),
            ('publication used as an end (Jospin)', lambda p: holder(p, 6).update(until='2002-05-07')),
            ('publication used as a start (Bérégovoy)', lambda p: holder(p, 2).update({'from': '1992-04-03'})),
            ('nameless reference used as a start (Balladur)', lambda p: holder(p, 3).update({'from': '1993-03-29', 'attested_on': None})),
            ('title-only page used as a start (Cresson)', lambda p: holder(p, 1).update({'from': '1991-05-15', 'attested_on': None})),
            ('countersignature used as a start without an appointment decree (Juppé 1995)',
             lambda p: holder(p, 4).update({'from': '1995-05-18', 'attested_on': None})),
            ('pre-period appointment used as a start (Rocard)', lambda p: holder(p, 0).update({'from': '1988-05-10', 'attested_on': None})),
            ('acting holder added', lambda p: role(p)['holder_claims'].insert(1, acting)),
            ('resignation claim cited by a holder', lambda p: holder(p, 3)['claim_ids'].append(
                'fr_jorf_balladur_government_resignation_letter_19950510')),
            ('reference claim cited by a holder', lambda p: holder(p, 4)['claim_ids'].append(
                'fr_jorf_composition_cites_pm_appointment_19950517')),
            ('presidency claim feeding a prime-minister holder', lambda p: holder(p, 14)['claim_ids'].append(
                'fr_jorf_hollande_signs_pm_appointment_decree_20120515')),
            ('presidency claim added to the role', lambda p: role(p)['claim_ids'].append(
                'fr_jorf_hollande_signs_pm_appointment_decree_20120515')),
            ('prime-minister claim added to the presidency', lambda p: p['institutions'][0]['roles'][0]['claim_ids'].append(
                'fr_jorf_ayrault_appointed_pm_20120515')),
            ('holders reordered', lambda p: role(p)['holder_claims'].reverse()),
            ('holder removed', lambda p: role(p)['holder_claims'].pop(5)),
            ('eleventh person added', lambda p: role(p)['holder_claims'].append(
                dict(copy.deepcopy(holder(p, 15)), name='Next Batch Holder'))),
            ('second head-of-government role', lambda p: p['organizations'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='fr_other'))),
            ('institutions reordered', lambda p: p['institutions'].reverse()),
            ('institution mapped to a party', lambda p: p['institutions'][1]['represented_party_ids'].append('France/fr_ps')),
            ('institution given a lifecycle end', lambda p: p['institutions'][1]['lifecycle'].update(until='2026-09-07')),
            ('organization citing a prime-minister claim', lambda p: p['organizations'][0]['claim_ids'].append(
                'fr_jorf_jospin_appointed_pm_19970602')),
            ('claim re-dated to its publication', lambda p: claim(p, 'fr_jorf_jospin_appointed_pm_19970602').update(
                attested_on='1997-06-03')),
            ('stated end dropped', lambda p: holder(p, 6).update(until=None)),
        ]
        pm_invariants(self.packet)
        for label, change in invariant_cases:
            with self.subTest(label=label), self.assertRaises((AssertionError, KeyError, IndexError)):
                pm_invariants(mutated(change))

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted', '02': 'Accepted in part', '03': 'Accepted', '04': 'Accepted in part',
                     '05': 'Accepted in part', '06': 'Accepted', '07': 'Accepted', '08': 'Accepted', '09': 'Accepted',
                     '10': 'Accepted', '11': 'Accepted', '12': 'Deferred'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| FR-PM-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        self.assertIn('\n### Date ledger', self.report)
        for heading in ('Response identities and stability checks', 'Sources attempted', 'Leads not imported',
                        'Next work', 'Integration notes', 'Checks'):
            self.assertIn(f'\n## {heading}', self.report)
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('1a3fc352', '44098c5a', 'research-index.json', 'test_campaign_research.py', 'test_france_presidents_c01_23.py',
                     'supplements/france.json', '262d5f61'):
            self.assertIn(text, notes)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'france-prime-ministers-1990-2026-37.md', 'claude/c01-fr-37', '1a3fc352', '44098c5a',
                     'test_france_prime_ministers_c01_37.py', 'supplements/france.json'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'France')
        self.assertFalse(country['country_census_complete'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (2, 2))
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
