"""CLAUDE-C01-12: Japan's prime ministers, 1990-2006, keep each House's designation, the joint committee, the Imperial
appointment ceremony, Kantei statements, a cabinet's formation, its resignation en masse and acting service apart, and
state a start or an end only where a source that names the holder does."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research
# CLAUDE-C01-13 (stacked on this packet) extends the same institution and role from 26 September 2006; its exact
# sources, claims and holders are pinned in its own test and appended to this packet's pins below.
import test_japan_prime_ministers_c01_13 as later


# Original response identity recorded in each extract: (bytes, sha256), of the identity-encoded body. Every new source is
# reproducible: a raw Internet Archive capture, an official Diet minutes API response or a House's static PDF.
RESPONSES = {
    'jp_hr_written_answer_19900112':
        (78598, 'dbe70268afc7a2fef97471c86275efb104063b157b10bfec48b2cc4a7f54410d'),
    'jp_hc_written_answer_19900119':
        (23985, '66623a6ed289d4fcfb1e8a0f1c46f722c2cc47665ce047380f53ea04f8399f24'),
    'jp_hr_written_answer_19900123':
        (89159, 'e3980687d4802935623ec9bb28acbcd6427dc60ad87cbd7759b4bd40eb92a7e6'),
    'jp_kantei_rekidai_076':
        (16232, '2f9bad3a615acab43c02f87c5025658abeb43cc3d147b7b6297446742037d069'),
    'jp_hc_rules_committee_19900227':
        (26192, '02d7740f47d2e9fed583317322c3592d54c83e01d2e09f05ae719bf8367aadb4'),
    'jp_hr_plenary_19900227':
        (81054, 'e69f7ed0eeca6609faecd4d5b58f994f559e8a4477535078c7656b568b470de0'),
    'jp_hc_plenary_19900227':
        (35670, 'ee46738e4f2043784da037fd504efd23b8fdd58645cb57c29a7ae855e17ef399'),
    'jp_hc_precedents_pm_designations':
        (1071345, '9b828b599e4c3842fb1e0a0e82fde66f3a63ffd1a73a2aa775b701bda80874fb'),
    'jp_kantei_rekidai_077':
        (25031, 'f007a6ae4edded840a295b1bee603a883debb2ce1972b20f2863301fc7cdaac2'),
    'jp_hr_plenary_19900302':
        (104570, '22eb38e54c6411cd4d4ee78f5180acf090274e6b6b485b7b1bcc79c3a476bfda'),
    'jp_hc_plenary_19900302':
        (103067, 'ea174d7abab9f3cd85052d2017595b4748593f8d8cb6fd6b41f92310a22d5662'),
    'jp_hr_budget_committee_19900409':
        (555162, '28a60fff8bab6a8ec7963fbd33f9f3db2ffd4e279b0ecb0a020895c76e141a64'),
    'jp_hr_rules_committee_19911105':
        (37436, 'f708bd84a699c0ef5b1daa02c630f291fccaa37096e6a353f6b528e048f4c34f'),
    'jp_hc_rules_committee_19911105':
        (21160, 'a4b37f784f253fca6a504e0262cd175106a3092c94e5299f0812d01a77c45991'),
    'jp_hr_plenary_19911105':
        (41252, 'b6b656d97dcee55119af9ed60015c2af1969ab98d83d30bac83e3f8f23e31960'),
    'jp_hc_plenary_19911105':
        (14936, '323355a254ab19f7768b226aab12007a13b8422d6096e7071066538d43962706'),
    'jp_hr_plenary_19911108':
        (29831, '8c7fb41e3bae23a9ade44af67eee4d1b869dd31392f1da7022bd3b1cf4225405'),
    'jp_hc_plenary_19911108':
        (23813, 'c3f2d2b224106cb7e6accbf314b43c766d7e2807e4a103697334a541ebb15122'),
    'jp_hc_cabinet_committee_19911217':
        (273120, '6edbbbdf914e2546ecf33aa3c6c10b3b5bdc27266685c765a1e7a86abfa1bea9'),
    'jp_kantei_rekidai_078':
        (26936, 'c3959417817a8b0e427e3b70367a5bc8bf8415b9ff9d7cfbbe12703db427687c'),
    'jp_hc_rules_committee_19930806':
        (10406, '7b073e9b3ad3d8084b0a09d9abfc7c31895dd9280330bb0c168830f3f6b59608'),
    'jp_hr_plenary_19930806':
        (39537, '913974c160a6fa35dddca71e16dee5ad8c9de8cb909ac52d952f365603d9334e'),
    'jp_hc_plenary_19930806':
        (6164, '057e07b4ccafed27d1805988375cd4f874de47edd04a4355cf0be6b7daf3b560'),
    'jp_hr_plenary_19930823':
        (27736, '89a4f77378751cc3fffe83a349af94be82ca371c0784bc7ab337dc1df052061e'),
    'jp_hc_plenary_19930823':
        (32382, '3edd3e3dc3dc05e0bada3f1f7404d8aa7151e0050167b7f1952a34c063dd3cdf'),
    'jp_hr_disaster_committee_19930824':
        (443166, 'b8233337d06272c8215ba79c2d39fd98e5a856bf09f4e9ea8ccaa2b656b4c980'),
    'jp_kantei_rekidai_079':
        (15624, '8ce1e7bd93c756da34ab68ef4a450498feda34fe6717fa60eeb80b4ca5f0354d'),
    'jp_hr_plenary_19940425':
        (6323, 'e588f50213e31dbfc30ac9a9d922db7b51e546a6f230f6f63a028857860c30ca'),
    'jp_hc_plenary_19940425':
        (6923, '07d2650e4f57cb809bd3c1dc2a2720660861db417156be62d10f366dfa1d8e46'),
    'jp_kantei_rekidai_hata_cabinet':
        (3274, 'dc6ba2267919c039f19533492270d1bf11b8267015a1cbf62131a0c86d7f7264'),
    'jp_hc_plenary_19940510_hata_speech':
        (24236, 'a19d3c4ee6c602715e979792f4374871b77204601a29d5c4b482a94931a4ab2b'),
    'jp_hr_plenary_19940510_hata_speech':
        (24524, '2a00799263199d0f4e11d0ddd5fe62069f1705b98be51b238b6ceb316c5e88c5'),
    'jp_hr_audit_subcommittee_19940527':
        (10551, '79444bfde2aa4f24afbde43851d3259114001a4720052d10cf3568fb0b211b94'),
    'jp_hc_rules_committee_19940629':
        (1008, '0194e0da014dae3204722a21f19a4ccd255425a7c7f685ecc2c33d97a5bef044'),
    'jp_hr_rules_committee_19940629':
        (29428, '37e090775d64e75d39eaf1bb0104b84de2eea8c3e942fd1d6aea1573fc059f39'),
    'jp_hr_plenary_19940629':
        (29240, 'a847d19e1f8fe9c6b8b3786f0ad777e4e87759710ee20b65da94ad1e579155b9'),
    'jp_hc_plenary_19940629':
        (11694, '19f910c7227a010548703cb3a36293bb710aa77f18267b1fa2351e6f5295b7b3'),
    'jp_kantei_rekidai_murayama_cabinet':
        (3379, 'b75ee146e85826870727a8204ad9b263dd41f6716cef9038f0aafb86664e71e0'),
    'jp_hc_agriculture_committee_19940706':
        (5184, 'c9c9262d2aa081ec4452da0b479e2418805d7406fd9756233e712d2271703e70'),
    'jp_hr_agriculture_committee_19940706_s081':
        (3099, 'ea81a73a58369c8e358be1fb0852133cddf8cc66bc4586a7c34ec2b154e7d7cb'),
    'jp_hr_agriculture_committee_19940706_s116':
        (3396, '2bb389f3ce378fa51d8e54eb379aa5d22152ff5ed29e2170ac6f96cf301fc922'),
    'jp_hr_plenary_19940718_murayama_speech':
        (24932, 'd5816bdaa334ace0fe738c608e6a234c27a2a9fe3b341a79781ca3344210752f'),
    'jp_hr_tax_committee_19941109_murayama':
        (2518, 'd04a33b2397a5d3658b1877031c7024fa4d025993d0080fc61eab08df9cdffe4'),
    'jp_hr_plenary_19960111':
        (14221, '6fe9ef7adaad157afb2c34e5485e6bbc5e65f72409521bc363cce70ed35e7e80'),
    'jp_hc_plenary_19960111':
        (17116, '064bc1475871f22df11737c35a4df7ce44f84229bec789f19c9303b03fb48a07'),
    'jp_kantei_hashimoto_statement_19960111':
        (3939, 'b2f8fb68b7fa2e5487ddcfd5f95b69484410c14d125d06c32f9d77950de09b87'),
    'jp_kantei_rekidai_hashimoto_cabinet_1':
        (2807, 'f71b842cd4d0be1391068a3ce2f6d46190e552c39febe94ce3cbd70440755cdd'),
    'jp_hc_rules_committee_19961107':
        (28209, '35640ad7959f7c541624a0092a6f663fa0467e5e7da05c29cccb8a415c797a0a'),
    'jp_kantei_cabinet_agenda_19961107':
        (1749, '3e5d423f9f56d8e21fa9b78894d0b89ae5de99a11710858c689510b4e0129e02'),
    'jp_hr_plenary_19961107':
        (27726, '53ce81381b4961a6aed8df7b033d34053b2eba3fed4fe7ec3f4361a15c786f6e'),
    'jp_hc_plenary_19961107':
        (14333, 'e560ca9e7db3d8d74a63c2a063137450901f1542d6d5949fbbc755326ec960c0'),
    'jp_kantei_pm_statement_19961107':
        (1953, '1f226807c65758ebc97d4dcc6198b3432f1b408fad98081da0be1a8f99129f44'),
    'jp_kantei_press_conference_19961108':
        (43397, 'ba669228bb0fad69a40772db8bf6b39d5d96a9ffb039a0a3a9dc4bca0eaaef27'),
    'jp_kantei_rekidai_hashimoto_cabinet_2':
        (2741, '9c289bbf1ff56b61632f2f795595262ef61c3853cb0b31e542ed9932b52f5393'),
    'jp_hr_plenary_19980730':
        (20854, 'e947da3919d5ee9fd27881718445d2565b191b4a1383b0973680aa286032a4b7'),
    'jp_hc_plenary_19980730':
        (30579, 'ea863bf0530d4fc3681381e28280b7eb5916c7f044b4a49182135a87ba47254e'),
    'jp_joint_committee_19980730':
        (22992, 'd605c74d9b02e901fe27c2b33bfb8d7b1000e0a3cc2163e80fbe38cda9d2e163'),
    'jp_kantei_cabinet_agenda_19980730':
        (1880, '58691d472c36b8db318ecfcf451e169f5dbca55dc9ef65eaf3c6972e74e17ca6'),
    'jp_kantei_hashimoto_resignation_statement_19980730':
        (2197, 'd29d4d97cd6b840ffe5cbb6682e9e99eb6457b1811d6745d7b17b3e9853b65c5'),
    'jp_kantei_diary_19980730':
        (407, '544b92f11d6b14b2442b05d78e5819081bd5e845c7c210d9b4628f66cc62c893'),
    'jp_kantei_cabinet_agenda_19980731':
        (2051, 'a447be612fae15fb2638193f77b0520a961e864a9291eb88355480d0a2c924fe'),
    'jp_kantei_obuchi_statement_19980731':
        (2019, 'af0c6a3832def80c279bce22b2d8ce5d9e41d4af7d8ce4291e2007693b7f659e'),
    'jp_kantei_obuchi_press_conference_19980731':
        (33398, '519c9042ef49e864b7116c62dd52fafb17a0f9bb8c5d175cbe6fb44139783c8f'),
    'jp_kantei_rekidai_obuchi_cabinet':
        (3446, 'fd40dab55937a95b3e25b9a727b3a28f1afdce72dd68511f808bc8387fe2b055'),
    'jp_shugiin_honkaigi_20000410_aoki':
        (9073, '1eec231c38f4e8827225ad84eed08fbad440b9265cc958db3692e0876cbc74e6'),
    'jp_sangiin_gyoseikanshi_20000403_s35':
        (1028, '54f47c0d3849d2a12bc31c1f2966af155790f731a55fcebe2ddd2f1f0ad74006'),
    'jp_sangiin_gyoseikanshi_20000403_s36':
        (1149, '8236caf46b05e6f8ca927cf0e2a4e5d0690606e1f2bfa3975b841f91150bd6f1'),
    'jp_shugiin_naikaku_20000426_aoki':
        (948, '17d0295267719e4e3c7f05433514d9d53bb8b2e56ea80a28b395ebcb56f331ff'),
    'jp_sangiin_homu_20000523_matsutani':
        (3727, '6cb96b67f06388bcdb0f38c00b290991ebdb005ec2a8aa71b2b9848493a00342'),
    'jp_sangiin_giun_20000405':
        (1039, '8ec976741cfdb3fd505fdf9a61fed220e57c7abd2dfa3cfb4b51606fbd1957c6'),
    'jp_shugiin_giun_20000405':
        (1326, 'ea433f37b67e7b5baa098b449adc57517c7694d921d72d46cd86fa291bd789ce'),
    'jp_shugiin_honkaigi_20000405':
        (22329, 'e6b38e37bf07916d413443a6589b820cf27f42080385deeb21e864b8bcdc657d'),
    'jp_sangiin_honkaigi_20000405':
        (5985, 'ca76c989b80482b3d558313b8f4bb7f39a4daf334ba2e469191e0d5e5094a8ac'),
    'jp_kantei_mori_danwa_20000405':
        (1622, '254b5373618603652414b129d62a7eb4389efbe0b12f9f9f42a65bf44f6beee6'),
    'jp_kantei_mori_hossoku_200004':
        (3221, 'fd019a93fafea4838e12d2a58d35205c7f7dcf33a14a3c5d44b7baf69d00611c'),
    'jp_kunaicho_schedule_h12_q2':
        (18043, 'de92452f2eb2dbcd7b79fd6b9b3748bea274ef9db552260bde58b996ffd64bc8'),
    'jp_kantei_mori_profile_200004':
        (1588, 'b225d2d0dc57a3b93d240c2a77dc8f9d64e008e010ada19b3cb7488da0ff155b'),
    'jp_sangiin_giun_20000704':
        (1023, '81686506d8911b5620a5accb60b4cdbe27e7e53ce1c8ac648c26fe2e041b1afa'),
    'jp_shugiin_honkaigi_20000704':
        (74622, 'e01df453cfcae4b4e993d2146d2e3005dc867b99c3baa75d5317f45f89424ce0'),
    'jp_sangiin_honkaigi_20000704':
        (23235, '8d52148a789cab1b49e236136655e628aa18dcf937838f0a25712969e6902106'),
    'jp_kantei_mori_danwa_20000704':
        (2459, '12a00e19615b76621c4fa4ca908b8836d423dcea2eec84a302ad6fb1d37a6b87'),
    'jp_kantei_mori_hossoku_200007':
        (1669, 'cb24a6763d0d46827287c03f58c27b6f422ab21a5a9d91677bb9481836bd0ebb'),
    'jp_kunaicho_schedule_h12_q3':
        (15296, '88ce38259793d7fe9827ff5cf6f15876e5bd3eb8fd37660bdc5b5bf816009659'),
    'jp_kantei_rekidai_085':
        (15218, '6a8a7a23ea9936f2967c0b8ce96f6134dba2c2e64dc1a1b680f3abd5cb82f462'),
    'jp_kantei_rekidai_086':
        (34418, '324077a348a3eeee67b3111437100234056bf8338ff4b357ea937ce718200baf'),
    'jp_shugiin_giun_20010426':
        (1296, 'd3a7e58f21cd400b33ee0c98a99254cd7ffd77548a7fbae5c60e71be9201b2dc'),
    'jp_sangiin_giun_20010426':
        (1030, '0c6a08900c54783d6bc2b209809135b81534ab416068c8098dc3f6f990557958'),
    'jp_shugiin_honkaigi_20010426':
        (22344, '512a469c0699bb9c3043698e7f2291b60068cef44fca01157862ae20812ca193'),
    'jp_sangiin_honkaigi_20010426':
        (7439, '9f917b903f926c93cfe21cc0e6ba5e576f325720e238c9bd5be34a9af64d987b'),
    'jp_kantei_mori_jisyoku_danwa_20010426':
        (2033, 'b477e37b125f4fac783084d7b668f2f27cb914bcbe3f120dd7b8d4d07c736435'),
    'jp_kantei_koizumi_danwa_20010426':
        (4011, 'c0f2f802afc6cd9813236a8e39052799c885fa722e1a15b0773bc036bd35f0f5'),
    'jp_kantei_koizumi_hossoku_20010426':
        (3391, 'ffb7aa82d8cd4e75450396608d637d4ed9181205f17c6d14bd26305a579934f5'),
    'jp_kunaicho_schedule_h13_q2':
        (20777, '5a313b9a45679483775107c2cc5c99873f310f65f4c07f29383c9bf0ceffc82e'),
    'jp_kantei_rekidai_087':
        (34119, 'dc8766f8213eafe963a18f45a86e0aa42cf06b0d07575a4428509c053a0bf788'),
    'jp_sangiin_giun_20031119':
        (1029, 'b554e96ce026a9debd96dfdbbe4176ead773eee936bb3f235304332ec1a8a6b6'),
    'jp_shugiin_honkaigi_20031119':
        (77918, '89677562588741f4514a7b23a2c73c29ef8ffb54c4b80cb61f65c6eeffb6d41b'),
    'jp_sangiin_honkaigi_20031119':
        (17648, '6b6f97e8585451c1056f4e078063458059b97aff4bac1aa677b2857cc99118fd'),
    'jp_kantei_koizumi_danwa_20031119':
        (2871, 'fcbdfea90e219a334bb35bc9cd47a9c150a9c70aa63c5cf461f38913998b0b77'),
    'jp_kantei_koizumi_sokaku_20031119':
        (3402, '953f02c21c97f22d2a6398bb900b0ad5089dfc35a94b69b379fa8a05fb38a242'),
    'jp_kunaicho_schedule_h15_q4':
        (36347, '0023768004df573f583b5145bfdaed77acd12c90699deda3a65e8866ee9e40a5'),
    'jp_kantei_rekidai_088':
        (25446, 'ad807f920c917995212f0c3208cba244f571e425c8c1d6e355c723193059b327'),
    'jp_sangiin_giun_20050921':
        (1029, 'a167c14524b5d16fd9b802d6f48989f472bdb68a5533d5b5b817dd95cd20cb13'),
    'jp_shugiin_honkaigi_20050921':
        (69001, 'b1a742f442d25175dadf7c3db7c47dc4516c97c13ba390a1d37404dad1001bd4'),
    'jp_sangiin_honkaigi_20050921':
        (22529, '4aadcfe35db74bd67095b2b8a3961bdff47ffaa20a30814058ae62aafdcd2732'),
    'jp_kantei_koizumi_danwa_20050921':
        (4115, '9b31f5db4ef412b5bbd0fd5fe81a2f1b275d40e2686283cf9cb9073bc78505dc'),
    'jp_kantei_koizumi_sokaku_20050921':
        (4098, '1dcd06ccc171cf3cea88ebdb9a6148f62786c73439bfdcf82d79ed17acaba390'),
    'jp_kunaicho_schedule_2005_h2':
        (46090, 'cfe20d3e06fdb2155c654eaa544af77fa64a38f5be2d6bab769a44775048129e'),
    'jp_kunaicho_photo_20050921':
        (1557, '8b6b000453f93f295d7f7c8d277f41d45f8505cd22bfad844d6e1105470604f9'),
    'jp_kantei_koizumi_interview_20060925':
        (4954, '8424abce518cec7c729d4aa5fb9b4e310eea8093fc70e0718c031428d1fae96c'),
    'jp_shugiin_giun_20060926':
        (1507, 'cd1afdccf12a70e6691ce4dedf8435ad9e18adcc64351e96e37f31bf88db43b1'),
    'jp_sangiin_giun_20060926':
        (1029, '7462608f59ed0240e151748e1ed2a9862d55bdce1c368459bf5e0d7f518f24a6'),
    'jp_shugiin_honkaigi_20060926':
        (26138, 'adf156a277b0accab92cc78620430d57bc5a8566b9a2fbdca0b381a370ea2373'),
    'jp_sangiin_honkaigi_20060926':
        (8956, 'be1aa433233b79c401299d59352bfd0e62422e76ebf2948c5c33ad6d58fce34b'),
    'jp_kantei_koizumi_sojishoku_danwa_20060926':
        (3182, '68367c2504098fb70d3eb502b1f1d1ad7db53efeb9aff03dcf28aac33b00cb02'),
    'jp_kantei_abe_danwa_20060926':
        (4910, '8ae3f73ecc88af4cd9695dcb1a0d0f0c8420f90732a1dc5d201d9791d944d886'),
    'jp_kantei_abe_hossoku_20060926':
        (5538, '370ae14d206d380de78296b2a214df8756375e1fed3f9311393dbeb8197f8bbf'),
    'jp_kunaicho_schedule_2006_h2':
        (49890, 'c490d66f9c4322a436c9e71fec9b4c2ea9e6543c8f834144f49ffeb8326e7554'),
    'jp_kunaicho_photo_20060926':
        (1399, '416637aeddc3e58db190599f518f2ba2abde8e8827b95653e9fd097fc8e64136'),
    'jp_kantei_rekidai_089':
        (25746, '7e877396999b0a8e3e3f7b8d1307c5cafecb506dd5c7955cb390cdacf0ff2942'),
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'jp_kantei_rekidai_076': '20260128203848',
    'jp_hc_precedents_pm_designations': '20240419112946',
    'jp_kantei_rekidai_077': '20251221223338',
    'jp_kantei_rekidai_078': '20260613053004',
    'jp_kantei_rekidai_079': '20251207042636',
    'jp_kantei_rekidai_hata_cabinet': '20251118201735',
    'jp_kantei_rekidai_murayama_cabinet': '20240817060317',
    'jp_kantei_hashimoto_statement_19960111': '19970106194609',
    'jp_kantei_rekidai_hashimoto_cabinet_1': '20251209203315',
    'jp_kantei_cabinet_agenda_19961107': '19970106200150',
    'jp_kantei_pm_statement_19961107': '19970106192443',
    'jp_kantei_press_conference_19961108': '19970106192436',
    'jp_kantei_rekidai_hashimoto_cabinet_2': '20250211023026',
    'jp_kantei_cabinet_agenda_19980730': '20000422104013',
    'jp_kantei_hashimoto_resignation_statement_19980730': '19990127232924',
    'jp_kantei_diary_19980730': '20030424005116',
    'jp_kantei_cabinet_agenda_19980731': '20000422133707',
    'jp_kantei_obuchi_statement_19980731': '19990127211108',
    'jp_kantei_obuchi_press_conference_19980731': '19990128003851',
    'jp_kantei_rekidai_obuchi_cabinet': '20251120222328',
    'jp_kantei_mori_danwa_20000405': '20020211200425',
    'jp_kantei_mori_hossoku_200004': '20020621074703',
    'jp_kunaicho_schedule_h12_q2': '20010306203428',
    'jp_kantei_mori_profile_200004': '20000510033035',
    'jp_kantei_mori_danwa_20000704': '20020613022610',
    'jp_kantei_mori_hossoku_200007': '20020818092405',
    'jp_kunaicho_schedule_h12_q3': '20010306203501',
    'jp_kantei_rekidai_085': '20260129204930',
    'jp_kantei_rekidai_086': '20260213192224',
    'jp_kantei_mori_jisyoku_danwa_20010426': '20011227172822',
    'jp_kantei_koizumi_danwa_20010426': '20010501040255',
    'jp_kantei_koizumi_hossoku_20010426': '20010501041643',
    'jp_kunaicho_schedule_h13_q2': '20010609230044',
    'jp_kantei_rekidai_087': '20260605051827',
    'jp_kantei_koizumi_danwa_20031119': '20031121143708',
    'jp_kantei_koizumi_sokaku_20031119': '20031209114816',
    'jp_kunaicho_schedule_h15_q4': '20040216210938',
    'jp_kantei_rekidai_088': '20260419145831',
    'jp_kantei_koizumi_danwa_20050921': '20051028025657',
    'jp_kantei_koizumi_sokaku_20050921': '20051127142022',
    'jp_kunaicho_schedule_2005_h2': '20051122081528',
    'jp_kunaicho_photo_20050921': '20051219210804',
    'jp_kantei_koizumi_interview_20060925': '20061004144637',
    'jp_kantei_koizumi_sojishoku_danwa_20060926': '20061004123042',
    'jp_kantei_abe_danwa_20060926': '20061004122720',
    'jp_kantei_abe_hossoku_20060926': '20061004144947',
    'jp_kunaicho_schedule_2006_h2': '20061010234037',
    'jp_kunaicho_photo_20060926': '20061010232654',
    'jp_kantei_rekidai_089': '20260513220058',
}
# Written answers stored as static PDFs on the Houses' own hosts, with the page rendered and read.
PDF_PAGES = {
    'jp_hr_written_answer_19900112': [1],
    'jp_hc_written_answer_19900119': [1],
    'jp_hr_written_answer_19900123': [1],
    'jp_hc_precedents_pm_designations': [15, 16],
}
OFFICIAL_HOSTS = {'kokkai.ndl.go.jp', 'www.shugiin.go.jp', 'www.sangiin.go.jp'}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation. Retrospective lists, spans and tables carry no structured date.
EVENTS = {
    'jp_hashimoto_acting_pm_signs_hr_answer_19900112': ('1990-01-12', 'acting_prime_minister_signature', 'JP-PM-01'),
    'jp_kaifu_signs_hc_written_answer_as_pm_19900119': ('1990-01-19', 'in_office_attestation', 'JP-PM-01'),
    'jp_kaifu_signs_hr_written_answer_as_pm_19900123': ('1990-01-23', 'in_office_attestation', 'JP-PM-01'),
    'jp_kantei_first_kaifu_span_19890810_19900228': (None, 'retrospective_term_span', 'JP-PM-01'),
    'jp_kaifu_notifies_cabinet_resignation_hc_19900227': ('1990-02-27', 'cabinet_resignation_notice', 'JP-PM-01'),
    'jp_hr_designates_kaifu_19900227': ('1990-02-27', 'designation_vote_house_of_representatives', 'JP-PM-01'),
    'jp_hc_designates_kaifu_runoff_19900227': ('1990-02-27', 'designation_vote_house_of_councillors', 'JP-PM-01'),
    'jp_hc_precedents_hc_designates_kaifu_19900227': (None, 'retrospective_designation_record_house_of_councillors', 'JP-PM-01'),
    'jp_hc_precedents_hr_designates_kaifu_19900227': (None, 'retrospective_designation_record_house_of_representatives', 'JP-PM-01'),
    'jp_hc_precedents_kaifu_cabinet_resignation_19900227': (None, 'retrospective_cabinet_resignation_record', 'JP-PM-01'),
    'jp_hc_precedents_second_kaifu_cabinet_formed_19900228': (None, 'retrospective_cabinet_formation_record', 'JP-PM-01'),
    'jp_hc_precedents_hc_designates_miyazawa_19911105': (None, 'retrospective_designation_record_house_of_councillors', 'JP-PM-02'),
    'jp_hc_precedents_hr_designates_miyazawa_19911105': (None, 'retrospective_designation_record_house_of_representatives', 'JP-PM-02'),
    'jp_hc_precedents_kaifu_cabinet_resignation_19911105': (None, 'retrospective_cabinet_resignation_record', 'JP-PM-02'),
    'jp_hc_precedents_miyazawa_cabinet_formed_19911105': (None, 'retrospective_cabinet_formation_record', 'JP-PM-02'),
    'jp_hc_precedents_hc_designates_hosokawa_19930806': (None, 'retrospective_designation_record_house_of_councillors', 'JP-PM-03'),
    'jp_hc_precedents_hr_designates_hosokawa_19930806': (None, 'retrospective_designation_record_house_of_representatives', 'JP-PM-03'),
    'jp_hc_precedents_miyazawa_cabinet_resignation_19930805': (None, 'retrospective_cabinet_resignation_record', 'JP-PM-03'),
    'jp_hc_precedents_hosokawa_cabinet_formed_19930809': (None, 'retrospective_cabinet_formation_record', 'JP-PM-03'),
    'jp_kantei_second_kaifu_cabinet_formed_19900228': (None, 'retrospective_cabinet_list', 'JP-PM-01'),
    'jp_kantei_second_kaifu_span_19900228_19911105': (None, 'retrospective_term_span', 'JP-PM-02'),
    'jp_kaifu_policy_speech_states_reappointment_19900302': ('1990-03-02', 'in_office_attestation', 'JP-PM-01'),
    'jp_kaifu_hc_policy_speech_states_reappointment_19900302': ('1990-03-02', 'in_office_attestation', 'JP-PM-01'),
    'jp_member_states_second_kaifu_cabinet_formed_19900228': ('1990-02-28', 'cabinet_formation_recalled', 'JP-PM-01'),
    'jp_kaifu_notifies_cabinet_resignation_hr_19911105': ('1991-11-05', 'cabinet_resignation_notice', 'JP-PM-02'),
    'jp_hc_receives_kaifu_resignation_notice_19911105': ('1991-11-05', 'cabinet_resignation_notice', 'JP-PM-02'),
    'jp_hr_speaker_reports_kaifu_resignation_notice_19911105': ('1991-11-05', 'cabinet_resignation_notice', 'JP-PM-02'),
    'jp_hr_designates_miyazawa_19911105': ('1991-11-05', 'designation_vote_house_of_representatives', 'JP-PM-02'),
    'jp_hc_president_reports_kaifu_resignation_notice_19911105': ('1991-11-05', 'cabinet_resignation_notice', 'JP-PM-02'),
    'jp_hc_designates_miyazawa_runoff_19911105': ('1991-11-05', 'designation_vote_house_of_councillors', 'JP-PM-02'),
    'jp_miyazawa_policy_speech_states_appointment_19911108': ('1991-11-08', 'in_office_attestation', 'JP-PM-02'),
    'jp_miyazawa_hc_policy_speech_states_appointment_19911108': ('1991-11-08', 'in_office_attestation', 'JP-PM-02'),
    'jp_minister_states_miyazawa_cabinet_formed_19911105': ('1991-11-05', 'cabinet_formation_recalled', 'JP-PM-02'),
    'jp_kantei_miyazawa_cabinet_formed_19911105': (None, 'retrospective_cabinet_list', 'JP-PM-02'),
    'jp_kantei_miyazawa_span_19911105_19930809': (None, 'retrospective_term_span', 'JP-PM-03'),
    'jp_miyazawa_notifies_cabinet_resignation_19930805': ('1993-08-05', 'cabinet_resignation_notice', 'JP-PM-03'),
    'jp_hr_designates_hosokawa_19930806': ('1993-08-06', 'designation_vote_house_of_representatives', 'JP-PM-03'),
    'jp_hc_designates_hosokawa_19930806': ('1993-08-06', 'designation_vote_house_of_councillors', 'JP-PM-03'),
    'jp_hosokawa_policy_speech_states_appointment_19930823': ('1993-08-23', 'in_office_attestation', 'JP-PM-03'),
    'jp_hosokawa_hc_policy_speech_states_appointment_19930823': ('1993-08-23', 'in_office_attestation', 'JP-PM-03'),
    'jp_minister_states_hosokawa_cabinet_formed_19930809': ('1993-08-09', 'cabinet_formation_recalled', 'JP-PM-03'),
    'jp_minister_recalls_pm_instruction_19930809': ('1993-08-09', 'in_office_act_recalled', 'JP-PM-03'),
    'jp_member_recalls_pm_disaster_visit_19930813': ('1993-08-13', 'in_office_act_recalled', 'JP-PM-03'),
    'jp_kantei_hosokawa_cabinet_formed_19930809': (None, 'retrospective_cabinet_list', 'JP-PM-03'),
    'jp_kantei_hosokawa_span_19930809_19940428': (None, 'retrospective_term_span', 'JP-PM-03'),
    'jp_hr_receives_hosokawa_cabinet_resignation_notice_19940425': ('1994-04-25', 'cabinet_resignation_notice', 'JP-PM-04'),
    'jp_hr_designates_hata_19940425': ('1994-04-25', 'designation_vote_house_of_representatives', 'JP-PM-04'),
    'jp_hc_hosokawa_cabinet_resolves_resignation_19940425': ('1994-04-25', 'cabinet_resignation_notice', 'JP-PM-04'),
    'jp_hc_designates_hata_19940425': ('1994-04-25', 'designation_vote_house_of_councillors', 'JP-PM-04'),
    'jp_kantei_list_hata_cabinet_formed_19940428': (None, 'retrospective_cabinet_list', 'JP-PM-04'),
    'jp_hata_states_appointed_pm_19940510': ('1994-05-10', 'in_office_attestation', 'JP-PM-04'),
    'jp_hata_hr_policy_speech_states_appointment_19940510': ('1994-05-10', 'in_office_attestation', 'JP-PM-04'),
    'jp_member_recalls_hata_cabinet_launch_19940428': ('1994-04-28', 'cabinet_formation_recalled', 'JP-PM-04'),
    'jp_hc_receives_hata_resignation_notice_19940625': ('1994-06-25', 'cabinet_resignation_notice', 'JP-PM-05'),
    'jp_hr_receives_hata_cabinet_resignation_notice_19940625': ('1994-06-25', 'cabinet_resignation_notice', 'JP-PM-05'),
    'jp_hr_first_ballot_no_majority_19940629': ('1994-06-29', 'designation_ballot_no_majority_house_of_representatives', 'JP-PM-05'),
    'jp_hr_designates_murayama_runoff_19940629': ('1994-06-29', 'designation_vote_house_of_representatives', 'JP-PM-05'),
    'jp_hc_hata_cabinet_resolves_resignation_19940625': ('1994-06-25', 'cabinet_resignation_notice', 'JP-PM-05'),
    'jp_hc_designates_murayama_19940629': ('1994-06-29', 'designation_vote_house_of_councillors', 'JP-PM-05'),
    'jp_kantei_list_murayama_cabinet_formed_19940630': (None, 'retrospective_cabinet_list', 'JP-PM-05'),
    'jp_hc_member_refers_to_murayama_cabinet_19940706': ('1994-07-06', 'in_office_reference_by_member', 'JP-PM-05'),
    'jp_hr_member_refers_to_murayama_cabinet_19940706': ('1994-07-06', 'in_office_reference_by_member', 'JP-PM-05'),
    'jp_hr_member_refers_to_prime_minister_murayama_19940706': ('1994-07-06', 'in_office_reference_by_member', 'JP-PM-05'),
    'jp_murayama_speaks_as_pm_19940718': ('1994-07-18', 'in_office_attestation', 'JP-PM-05'),
    'jp_murayama_recalls_cabinet_formed_19940630': ('1994-06-30', 'cabinet_formation_recalled_by_holder', 'JP-PM-05'),
    'jp_hr_receives_murayama_cabinet_resignation_notice_19960111': ('1996-01-11', 'cabinet_resignation_notice', 'JP-PM-06'),
    'jp_hr_designates_hashimoto_19960111': ('1996-01-11', 'designation_vote_house_of_representatives', 'JP-PM-06'),
    'jp_hc_murayama_cabinet_resolves_resignation_19960111': ('1996-01-11', 'cabinet_resignation_notice', 'JP-PM-06'),
    'jp_hc_designates_hashimoto_19960111': ('1996-01-11', 'designation_vote_house_of_councillors', 'JP-PM-06'),
    'jp_kantei_hashimoto_assumes_office_19960111': ('1996-01-11', 'assumption_statement', 'JP-PM-06'),
    'jp_kantei_first_cabinet_meeting_19960111': ('1996-01-11', 'first_cabinet_meeting', 'JP-PM-06'),
    'jp_kantei_list_hashimoto_cabinet_1_formed_19960111': (None, 'retrospective_cabinet_list', 'JP-PM-06'),
    'jp_hc_receives_hashimoto_cabinet_resignation_notice_19961107': ('1996-11-07', 'cabinet_resignation_notice', 'JP-PM-06'),
    'jp_kantei_extraordinary_cabinet_resignation_item_19961107': ('1996-11-07', 'cabinet_resignation_agenda_item', 'JP-PM-06'),
    'jp_kantei_cabinet_statement_item_19961107': ('1996-11-07', 'cabinet_agenda_item', 'JP-PM-06'),
    'jp_hr_designates_hashimoto_19961107': ('1996-11-07', 'designation_vote_house_of_representatives', 'JP-PM-06'),
    'jp_hc_designates_hashimoto_19961107': ('1996-11-07', 'designation_vote_house_of_councillors', 'JP-PM-06'),
    'jp_kantei_pm_assumes_office_again_19961107': ('1996-11-07', 'assumption_statement', 'JP-PM-06'),
    'jp_kantei_hashimoto_designated_again_continues_19961107': ('1996-11-07', 'designation_recalled_by_holder', 'JP-PM-06'),
    'jp_kantei_hashimoto_press_conference_in_office_19961108': ('1996-11-08', 'in_office_attestation', 'JP-PM-06'),
    'jp_kantei_hashimoto_recalls_appointment_19960111': ('1996-01-11', 'appointment_recalled_by_holder', 'JP-PM-06'),
    'jp_kantei_list_hashimoto_cabinet_2_formed_19961107': (None, 'retrospective_cabinet_list', 'JP-PM-06'),
    'jp_hr_receives_hashimoto_cabinet_resignation_notice_19980730': ('1998-07-30', 'cabinet_resignation_notice', 'JP-PM-07'),
    'jp_hr_designates_obuchi_19980730': ('1998-07-30', 'designation_vote_house_of_representatives', 'JP-PM-07'),
    'jp_hr_joint_committee_requested_19980730': ('1998-07-30', 'joint_committee_requested', 'JP-PM-07'),
    'jp_hr_resolution_prevails_obuchi_19980730': ('1998-07-30', 'house_of_representatives_resolution_prevails', 'JP-PM-07'),
    'jp_hc_hashimoto_cabinet_resolves_resignation_19980730': ('1998-07-30', 'cabinet_resignation_notice', 'JP-PM-07'),
    'jp_hc_first_ballot_no_majority_19980730': ('1998-07-30', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM-07'),
    'jp_hc_designates_kan_runoff_19980730': ('1998-07-30', 'designation_vote_house_of_councillors', 'JP-PM-07'),
    'jp_hc_hr_resolution_prevails_19980730': ('1998-07-30', 'house_of_representatives_resolution_prevails', 'JP-PM-07'),
    'jp_joint_committee_no_agreement_19980730': ('1998-07-30', 'joint_committee_no_agreement', 'JP-PM-07'),
    'jp_kantei_extraordinary_cabinet_resignation_item_19980730': ('1998-07-30', 'cabinet_resignation_agenda_item', 'JP-PM-07'),
    'jp_kantei_hashimoto_cabinet_resigned_19980730': ('1998-07-30', 'cabinet_resignation_statement', 'JP-PM-07'),
    'jp_kantei_hashimoto_recalls_taking_office_199601': (None, 'assumption_recalled_by_holder', 'JP-PM-06'),
    'jp_kantei_diary_obuchi_cabinet_19980730': ('1998-07-30', 'dated_photo_caption', 'JP-PM-07'),
    'jp_kantei_first_cabinet_meeting_19980731': ('1998-07-31', 'first_cabinet_meeting', 'JP-PM-07'),
    'jp_kantei_obuchi_statement_bears_office_19980731': ('1998-07-31', 'in_office_statement', 'JP-PM-07'),
    'jp_kantei_obuchi_press_conference_in_office_19980731': ('1998-07-31', 'in_office_attestation', 'JP-PM-07'),
    'jp_kantei_obuchi_new_cabinet_launched_19980730': ('1998-07-30', 'cabinet_formation_recalled_by_holder', 'JP-PM-07'),
    'jp_kantei_list_obuchi_cabinet_formed_19980730': (None, 'retrospective_cabinet_list', 'JP-PM-07'),
    'jp_obuchi_hospitalised_and_comatose_20000402': ('2000-04-02', 'prime_minister_incapacity_reported', 'JP-PM-08'),
    'jp_obuchi_instruction_to_aoki_1900_20000402': ('2000-04-02', 'acting_prime_minister_instruction_reported', 'JP-PM-08'),
    'jp_aoki_acting_pm_from_0900_announced_20000403': ('2000-04-03', 'acting_prime_minister_assumed', 'JP-PM-08'),
    'jp_aoki_reports_acting_designation_notified_20000403': ('2000-04-03', 'acting_prime_minister_designation_notified', 'JP-PM-08'),
    'jp_obuchi_cabinet_resignation_decided_20000404': ('2000-04-04', 'cabinet_resignation_decided', 'JP-PM-08'),
    'jp_member_asks_aoki_acting_pm_0900_20000403': ('2000-04-03', 'acting_prime_minister_question', 'JP-PM-08'),
    'jp_aoki_takes_acting_pm_0900_20000403': ('2000-04-03', 'acting_prime_minister_assumed', 'JP-PM-08'),
    'jp_aoki_states_acting_pm_legally_from_0900_20000403': ('2000-04-03', 'acting_prime_minister_assumed', 'JP-PM-08'),
    'jp_aoki_commenced_acting_duties_0900_20000403': ('2000-04-03', 'acting_prime_minister_assumed', 'JP-PM-08'),
    'jp_acting_designation_notice_sent_20000403': ('2000-04-03', 'acting_prime_minister_designation_notified', 'JP-PM-08'),
    'jp_obuchi_cabinet_resignation_art70_20000404': ('2000-04-04', 'cabinet_resignation_decided', 'JP-PM-08'),
    'jp_sangiin_receives_resignation_notice_from_acting_pm_20000404': ('2000-04-04', 'cabinet_resignation_notice', 'JP-PM-08'),
    'jp_shugiin_rules_reads_acting_pm_resignation_notice_20000404': ('2000-04-04', 'cabinet_resignation_notice', 'JP-PM-08'),
    'jp_shugiin_receives_resignation_notice_from_acting_pm_20000404': ('2000-04-04', 'cabinet_resignation_notice', 'JP-PM-08'),
    'jp_shugiin_designates_mori_20000405': ('2000-04-05', 'designation_vote_house_of_representatives', 'JP-PM-08'),
    'jp_sangiin_notice_cabinet_resigns_art70_20000404': ('2000-04-04', 'cabinet_resignation_notice', 'JP-PM-08'),
    'jp_sangiin_designates_mori_20000405': ('2000-04-05', 'designation_vote_house_of_councillors', 'JP-PM-08'),
    'jp_mori_appointed_pm_statement_20000405': ('2000-04-05', 'appointment_statement', 'JP-PM-08'),
    'jp_mori_shinninshiki_appointed_20000405': ('2000-04-05', 'imperial_appointment_ceremony', 'JP-PM-08'),
    'jp_mori_cabinet_formed_20000405': ('2000-04-05', 'cabinet_formation', 'JP-PM-08'),
    'jp_kunaicho_ceremony_20000405': ('2000-04-05', 'imperial_appointment_ceremony', 'JP-PM-08'),
    'jp_mori_profile_pm_career_entry_20000405': ('2000-04-05', 'appointment_career_entry', 'JP-PM-08'),
    'jp_sangiin_receives_mori_cabinet_resignation_notice_20000704': ('2000-07-04', 'cabinet_resignation_notice', 'JP-PM-08'),
    'jp_shugiin_designates_mori_20000704': ('2000-07-04', 'designation_vote_house_of_representatives', 'JP-PM-08'),
    'jp_sangiin_designates_mori_20000704': ('2000-07-04', 'designation_vote_house_of_councillors', 'JP-PM-08'),
    'jp_mori_resumes_pm_statement_20000704': ('2000-07-04', 'assumption_statement', 'JP-PM-08'),
    'jp_mori_shinninshiki_appointed_20000704': ('2000-07-04', 'imperial_appointment_ceremony', 'JP-PM-08'),
    'jp_mori_second_cabinet_formed_20000704': ('2000-07-04', 'cabinet_formation', 'JP-PM-08'),
    'jp_kunaicho_ceremony_20000704': ('2000-07-04', 'imperial_appointment_ceremony', 'JP-PM-08'),
    'jp_kantei_span_mori_85_20000405_20000704': (None, 'retrospective_term_span', 'JP-PM-08'),
    'jp_kantei_span_mori_86_20000704_20010426': (None, 'retrospective_term_span', 'JP-PM-08'),
    'jp_shugiin_rules_receives_mori_resignation_notice_20010426': ('2001-04-26', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_sangiin_rules_receives_mori_resignation_notice_20010426': ('2001-04-26', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_shugiin_receives_mori_cabinet_resignation_notice_20010426': ('2001-04-26', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_shugiin_designates_koizumi_20010426': ('2001-04-26', 'designation_vote_house_of_representatives', 'JP-PM-09'),
    'jp_sangiin_notice_mori_cabinet_resigns_20010426': ('2001-04-26', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_sangiin_designates_koizumi_20010426': ('2001-04-26', 'designation_vote_house_of_councillors', 'JP-PM-09'),
    'jp_mori_cabinet_resigned_statement_20010426': ('2001-04-26', 'cabinet_resignation_statement', 'JP-PM-09'),
    'jp_koizumi_appointed_pm_statement_20010426': ('2001-04-26', 'appointment_statement', 'JP-PM-09'),
    'jp_koizumi_shinninshiki_20010426': ('2001-04-26', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_koizumi_cabinet_formed_20010426': ('2001-04-26', 'cabinet_formation', 'JP-PM-09'),
    'jp_kunaicho_ceremony_20010426': ('2001-04-26', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_kantei_span_koizumi_87_20010426_20031119': (None, 'retrospective_term_span', 'JP-PM-09'),
    'jp_sangiin_receives_koizumi_cabinet_resignation_notice_20031119': ('2003-11-19', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_shugiin_designates_koizumi_20031119': ('2003-11-19', 'designation_vote_house_of_representatives', 'JP-PM-09'),
    'jp_sangiin_designates_koizumi_20031119': ('2003-11-19', 'designation_vote_house_of_councillors', 'JP-PM-09'),
    'jp_koizumi_resumes_pm_statement_20031119': ('2003-11-19', 'assumption_statement', 'JP-PM-09'),
    'jp_koizumi_shinninshiki_appointed_20031119': ('2003-11-19', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_koizumi_second_cabinet_formed_20031119': ('2003-11-19', 'cabinet_formation', 'JP-PM-09'),
    'jp_kunaicho_ceremony_20031119': ('2003-11-19', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_kantei_span_koizumi_88_20031119_20050921': (None, 'retrospective_term_span', 'JP-PM-09'),
    'jp_sangiin_receives_koizumi_cabinet_resignation_notice_20050921': ('2005-09-21', 'cabinet_resignation_notice', 'JP-PM-09'),
    'jp_shugiin_designates_koizumi_20050921': ('2005-09-21', 'designation_vote_house_of_representatives', 'JP-PM-09'),
    'jp_sangiin_designates_koizumi_20050921': ('2005-09-21', 'designation_vote_house_of_councillors', 'JP-PM-09'),
    'jp_koizumi_resumes_pm_statement_20050921': ('2005-09-21', 'assumption_statement', 'JP-PM-09'),
    'jp_koizumi_cabinet_resigned_morning_20050921': ('2005-09-21', 'cabinet_resignation_decided', 'JP-PM-09'),
    'jp_koizumi_shinninshiki_appointed_20050921': ('2005-09-21', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_koizumi_third_cabinet_formed_20050921': ('2005-09-21', 'cabinet_formation', 'JP-PM-09'),
    'jp_kunaicho_ceremony_20050921': ('2005-09-21', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_kunaicho_ceremony_koizumi_20050921': ('2005-09-21', 'imperial_appointment_ceremony', 'JP-PM-09'),
    'jp_koizumi_pm_final_interview_20060925': ('2006-09-25', 'in_office_attestation', 'JP-PM-10'),
    'jp_shugiin_rules_receives_koizumi_resignation_notice_20060926': ('2006-09-26', 'cabinet_resignation_notice', 'JP-PM-10'),
    'jp_sangiin_rules_receives_koizumi_resignation_notice_20060926': ('2006-09-26', 'cabinet_resignation_notice', 'JP-PM-10'),
    'jp_shugiin_receives_koizumi_cabinet_resignation_notice_20060926': ('2006-09-26', 'cabinet_resignation_notice', 'JP-PM-10'),
    'jp_shugiin_designates_abe_20060926': ('2006-09-26', 'designation_vote_house_of_representatives', 'JP-PM-10'),
    'jp_sangiin_notice_koizumi_cabinet_resigns_20060926': ('2006-09-26', 'cabinet_resignation_notice', 'JP-PM-10'),
    'jp_sangiin_designates_abe_20060926': ('2006-09-26', 'designation_vote_house_of_councillors', 'JP-PM-10'),
    'jp_koizumi_cabinet_resigned_statement_20060926': ('2006-09-26', 'cabinet_resignation_statement', 'JP-PM-10'),
    'jp_abe_appointed_pm_statement_20060926': ('2006-09-26', 'appointment_statement', 'JP-PM-10'),
    'jp_abe_shinninshiki_20060926': ('2006-09-26', 'imperial_appointment_ceremony', 'JP-PM-10'),
    'jp_abe_cabinet_formed_20060926': ('2006-09-26', 'cabinet_formation', 'JP-PM-10'),
    'jp_kunaicho_ceremony_20060926': ('2006-09-26', 'imperial_appointment_ceremony', 'JP-PM-10'),
    'jp_kunaicho_ceremony_abe_20060926': ('2006-09-26', 'imperial_appointment_ceremony', 'JP-PM-10'),
    'jp_kantei_span_koizumi_89_20050921_20060926': (None, 'retrospective_term_span', 'JP-PM-10'),
}
# The holder name each extract row carries; None where the source names no holder of this role in this packet.
ROW_HOLDERS = {
    'jp_hashimoto_acting_pm_signs_hr_answer_19900112': None,
    'jp_kaifu_signs_hc_written_answer_as_pm_19900119': '海部俊樹',
    'jp_kaifu_signs_hr_written_answer_as_pm_19900123': '海部俊樹',
    'jp_kantei_first_kaifu_span_19890810_19900228': '海部俊樹',
    'jp_kaifu_notifies_cabinet_resignation_hc_19900227': '海部俊樹',
    'jp_hr_designates_kaifu_19900227': '海部俊樹',
    'jp_hc_designates_kaifu_runoff_19900227': '海部俊樹',
    'jp_hc_precedents_hc_designates_kaifu_19900227': '海部俊樹',
    'jp_hc_precedents_hr_designates_kaifu_19900227': '海部俊樹',
    'jp_hc_precedents_kaifu_cabinet_resignation_19900227': '海部俊樹',
    'jp_hc_precedents_second_kaifu_cabinet_formed_19900228': '海部俊樹',
    'jp_hc_precedents_hc_designates_miyazawa_19911105': '宮澤喜一',
    'jp_hc_precedents_hr_designates_miyazawa_19911105': '宮澤喜一',
    'jp_hc_precedents_kaifu_cabinet_resignation_19911105': '海部俊樹',
    'jp_hc_precedents_miyazawa_cabinet_formed_19911105': '宮澤喜一',
    'jp_hc_precedents_hc_designates_hosokawa_19930806': '細川護煕',
    'jp_hc_precedents_hr_designates_hosokawa_19930806': '細川護煕',
    'jp_hc_precedents_miyazawa_cabinet_resignation_19930805': '宮澤喜一',
    'jp_hc_precedents_hosokawa_cabinet_formed_19930809': '細川護煕',
    'jp_kantei_second_kaifu_cabinet_formed_19900228': '海部俊樹',
    'jp_kantei_second_kaifu_span_19900228_19911105': '海部俊樹',
    'jp_kaifu_policy_speech_states_reappointment_19900302': '海部俊樹',
    'jp_kaifu_hc_policy_speech_states_reappointment_19900302': '海部俊樹',
    'jp_member_states_second_kaifu_cabinet_formed_19900228': '海部俊樹',
    'jp_kaifu_notifies_cabinet_resignation_hr_19911105': '海部俊樹',
    'jp_hc_receives_kaifu_resignation_notice_19911105': '海部俊樹',
    'jp_hr_speaker_reports_kaifu_resignation_notice_19911105': '海部俊樹',
    'jp_hr_designates_miyazawa_19911105': '宮澤喜一',
    'jp_hc_president_reports_kaifu_resignation_notice_19911105': '海部俊樹',
    'jp_hc_designates_miyazawa_runoff_19911105': '宮澤喜一',
    'jp_miyazawa_policy_speech_states_appointment_19911108': '宮澤喜一',
    'jp_miyazawa_hc_policy_speech_states_appointment_19911108': '宮澤喜一',
    'jp_minister_states_miyazawa_cabinet_formed_19911105': '宮澤喜一',
    'jp_kantei_miyazawa_cabinet_formed_19911105': '宮澤喜一',
    'jp_kantei_miyazawa_span_19911105_19930809': '宮澤喜一',
    'jp_miyazawa_notifies_cabinet_resignation_19930805': '宮澤喜一',
    'jp_hr_designates_hosokawa_19930806': '細川護煕',
    'jp_hc_designates_hosokawa_19930806': '細川護煕',
    'jp_hosokawa_policy_speech_states_appointment_19930823': '細川護煕',
    'jp_hosokawa_hc_policy_speech_states_appointment_19930823': '細川護煕',
    'jp_minister_states_hosokawa_cabinet_formed_19930809': '細川護煕',
    'jp_minister_recalls_pm_instruction_19930809': '細川護煕',
    'jp_member_recalls_pm_disaster_visit_19930813': '細川護煕',
    'jp_kantei_hosokawa_cabinet_formed_19930809': '細川護煕',
    'jp_kantei_hosokawa_span_19930809_19940428': '細川護煕',
    'jp_hr_receives_hosokawa_cabinet_resignation_notice_19940425': '細川護煕',
    'jp_hr_designates_hata_19940425': '羽田孜',
    'jp_hc_hosokawa_cabinet_resolves_resignation_19940425': '細川護煕',
    'jp_hc_designates_hata_19940425': '羽田孜',
    'jp_kantei_list_hata_cabinet_formed_19940428': '羽田孜',
    'jp_hata_states_appointed_pm_19940510': '羽田孜',
    'jp_hata_hr_policy_speech_states_appointment_19940510': '羽田孜',
    'jp_member_recalls_hata_cabinet_launch_19940428': '羽田孜',
    'jp_hc_receives_hata_resignation_notice_19940625': '羽田孜',
    'jp_hr_receives_hata_cabinet_resignation_notice_19940625': '羽田孜',
    'jp_hr_first_ballot_no_majority_19940629': '村山富市',
    'jp_hr_designates_murayama_runoff_19940629': '村山富市',
    'jp_hc_hata_cabinet_resolves_resignation_19940625': '羽田孜',
    'jp_hc_designates_murayama_19940629': '村山富市',
    'jp_kantei_list_murayama_cabinet_formed_19940630': '村山富市',
    'jp_hc_member_refers_to_murayama_cabinet_19940706': '村山富市',
    'jp_hr_member_refers_to_murayama_cabinet_19940706': '村山富市',
    'jp_hr_member_refers_to_prime_minister_murayama_19940706': '村山富市',
    'jp_murayama_speaks_as_pm_19940718': '村山富市',
    'jp_murayama_recalls_cabinet_formed_19940630': '村山富市',
    'jp_hr_receives_murayama_cabinet_resignation_notice_19960111': '村山富市',
    'jp_hr_designates_hashimoto_19960111': '橋本龍太郎',
    'jp_hc_murayama_cabinet_resolves_resignation_19960111': '村山富市',
    'jp_hc_designates_hashimoto_19960111': '橋本龍太郎',
    'jp_kantei_hashimoto_assumes_office_19960111': '橋本龍太郎',
    'jp_kantei_first_cabinet_meeting_19960111': '橋本龍太郎',
    'jp_kantei_list_hashimoto_cabinet_1_formed_19960111': '橋本龍太郎',
    'jp_hc_receives_hashimoto_cabinet_resignation_notice_19961107': '橋本龍太郎',
    'jp_kantei_extraordinary_cabinet_resignation_item_19961107': None,
    'jp_kantei_cabinet_statement_item_19961107': None,
    'jp_hr_designates_hashimoto_19961107': '橋本龍太郎',
    'jp_hc_designates_hashimoto_19961107': '橋本龍太郎',
    'jp_kantei_pm_assumes_office_again_19961107': None,
    'jp_kantei_hashimoto_designated_again_continues_19961107': '橋本龍太郎',
    'jp_kantei_hashimoto_press_conference_in_office_19961108': '橋本龍太郎',
    'jp_kantei_hashimoto_recalls_appointment_19960111': '橋本龍太郎',
    'jp_kantei_list_hashimoto_cabinet_2_formed_19961107': '橋本龍太郎',
    'jp_hr_receives_hashimoto_cabinet_resignation_notice_19980730': '橋本龍太郎',
    'jp_hr_designates_obuchi_19980730': '小渕恵三',
    'jp_hr_joint_committee_requested_19980730': None,
    'jp_hr_resolution_prevails_obuchi_19980730': '小渕恵三',
    'jp_hc_hashimoto_cabinet_resolves_resignation_19980730': '橋本龍太郎',
    'jp_hc_first_ballot_no_majority_19980730': '小渕恵三',
    'jp_hc_designates_kan_runoff_19980730': None,
    'jp_hc_hr_resolution_prevails_19980730': '小渕恵三',
    'jp_joint_committee_no_agreement_19980730': None,
    'jp_kantei_extraordinary_cabinet_resignation_item_19980730': None,
    'jp_kantei_hashimoto_cabinet_resigned_19980730': '橋本龍太郎',
    'jp_kantei_hashimoto_recalls_taking_office_199601': '橋本龍太郎',
    'jp_kantei_diary_obuchi_cabinet_19980730': '小渕恵三',
    'jp_kantei_first_cabinet_meeting_19980731': None,
    'jp_kantei_obuchi_statement_bears_office_19980731': None,
    'jp_kantei_obuchi_press_conference_in_office_19980731': '小渕恵三',
    'jp_kantei_obuchi_new_cabinet_launched_19980730': '小渕恵三',
    'jp_kantei_list_obuchi_cabinet_formed_19980730': '小渕恵三',
    'jp_obuchi_hospitalised_and_comatose_20000402': '小渕恵三',
    'jp_obuchi_instruction_to_aoki_1900_20000402': None,
    'jp_aoki_acting_pm_from_0900_announced_20000403': None,
    'jp_aoki_reports_acting_designation_notified_20000403': None,
    'jp_obuchi_cabinet_resignation_decided_20000404': '小渕恵三',
    'jp_member_asks_aoki_acting_pm_0900_20000403': None,
    'jp_aoki_takes_acting_pm_0900_20000403': None,
    'jp_aoki_states_acting_pm_legally_from_0900_20000403': None,
    'jp_aoki_commenced_acting_duties_0900_20000403': None,
    'jp_acting_designation_notice_sent_20000403': None,
    'jp_obuchi_cabinet_resignation_art70_20000404': '小渕恵三',
    'jp_sangiin_receives_resignation_notice_from_acting_pm_20000404': None,
    'jp_shugiin_rules_reads_acting_pm_resignation_notice_20000404': None,
    'jp_shugiin_receives_resignation_notice_from_acting_pm_20000404': None,
    'jp_shugiin_designates_mori_20000405': '森喜朗',
    'jp_sangiin_notice_cabinet_resigns_art70_20000404': None,
    'jp_sangiin_designates_mori_20000405': '森喜朗',
    'jp_mori_appointed_pm_statement_20000405': '森喜朗',
    'jp_mori_shinninshiki_appointed_20000405': '森喜朗',
    'jp_mori_cabinet_formed_20000405': '森喜朗',
    'jp_kunaicho_ceremony_20000405': None,
    'jp_mori_profile_pm_career_entry_20000405': '森喜朗',
    'jp_sangiin_receives_mori_cabinet_resignation_notice_20000704': '森喜朗',
    'jp_shugiin_designates_mori_20000704': '森喜朗',
    'jp_sangiin_designates_mori_20000704': '森喜朗',
    'jp_mori_resumes_pm_statement_20000704': None,
    'jp_mori_shinninshiki_appointed_20000704': '森喜朗',
    'jp_mori_second_cabinet_formed_20000704': '森喜朗',
    'jp_kunaicho_ceremony_20000704': None,
    'jp_kantei_span_mori_85_20000405_20000704': '森喜朗',
    'jp_kantei_span_mori_86_20000704_20010426': '森喜朗',
    'jp_shugiin_rules_receives_mori_resignation_notice_20010426': '森喜朗',
    'jp_sangiin_rules_receives_mori_resignation_notice_20010426': '森喜朗',
    'jp_shugiin_receives_mori_cabinet_resignation_notice_20010426': '森喜朗',
    'jp_shugiin_designates_koizumi_20010426': '小泉純一郎',
    'jp_sangiin_notice_mori_cabinet_resigns_20010426': '森喜朗',
    'jp_sangiin_designates_koizumi_20010426': '小泉純一郎',
    'jp_mori_cabinet_resigned_statement_20010426': '森喜朗',
    'jp_koizumi_appointed_pm_statement_20010426': None,
    'jp_koizumi_shinninshiki_20010426': '小泉純一郎',
    'jp_koizumi_cabinet_formed_20010426': '小泉純一郎',
    'jp_kunaicho_ceremony_20010426': None,
    'jp_kantei_span_koizumi_87_20010426_20031119': '小泉純一郎',
    'jp_sangiin_receives_koizumi_cabinet_resignation_notice_20031119': '小泉純一郎',
    'jp_shugiin_designates_koizumi_20031119': '小泉純一郎',
    'jp_sangiin_designates_koizumi_20031119': '小泉純一郎',
    'jp_koizumi_resumes_pm_statement_20031119': '小泉純一郎',
    'jp_koizumi_shinninshiki_appointed_20031119': '小泉純一郎',
    'jp_koizumi_second_cabinet_formed_20031119': '小泉純一郎',
    'jp_kunaicho_ceremony_20031119': None,
    'jp_kantei_span_koizumi_88_20031119_20050921': '小泉純一郎',
    'jp_sangiin_receives_koizumi_cabinet_resignation_notice_20050921': '小泉純一郎',
    'jp_shugiin_designates_koizumi_20050921': '小泉純一郎',
    'jp_sangiin_designates_koizumi_20050921': '小泉純一郎',
    'jp_koizumi_resumes_pm_statement_20050921': '小泉純一郎',
    'jp_koizumi_cabinet_resigned_morning_20050921': '小泉純一郎',
    'jp_koizumi_shinninshiki_appointed_20050921': '小泉純一郎',
    'jp_koizumi_third_cabinet_formed_20050921': '小泉純一郎',
    'jp_kunaicho_ceremony_20050921': None,
    'jp_kunaicho_ceremony_koizumi_20050921': '小泉純一郎',
    'jp_koizumi_pm_final_interview_20060925': '小泉純一郎',
    'jp_shugiin_rules_receives_koizumi_resignation_notice_20060926': '小泉純一郎',
    'jp_sangiin_rules_receives_koizumi_resignation_notice_20060926': '小泉純一郎',
    'jp_shugiin_receives_koizumi_cabinet_resignation_notice_20060926': '小泉純一郎',
    'jp_shugiin_designates_abe_20060926': None,
    'jp_sangiin_notice_koizumi_cabinet_resigns_20060926': '小泉純一郎',
    'jp_sangiin_designates_abe_20060926': None,
    'jp_koizumi_cabinet_resigned_statement_20060926': '小泉純一郎',
    'jp_abe_appointed_pm_statement_20060926': None,
    'jp_abe_shinninshiki_20060926': None,
    'jp_abe_cabinet_formed_20060926': None,
    'jp_kunaicho_ceremony_20060926': None,
    'jp_kunaicho_ceremony_abe_20060926': None,
    'jp_kantei_span_koizumi_89_20050921_20060926': '小泉純一郎',
}
NEW_SOURCES = list(RESPONSES)
ORIGINAL_SOURCES = ('jp_tokyo_pr_2025', 'jp_shugiin_groups_20260218', 'jp_shugiin_group_definition',
                    'jp_ldp_ishiba_elected_2024', 'jp_ldp_takaichi_elected_2025', 'jp_jcp_chairs_2024',
                    'jp_dpfp_tamaki_elected_2026')
GROUPS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
          'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
          'jp_shugiin_group_20260218_070']
PM = 'jp_pm'
T_PM = '内閣総理大臣 — Prime Minister of Japan'
SURNAMES = {'海部俊樹': '海部', '宮澤喜一': '宮澤', '細川護煕': '細川', '羽田孜': '羽田', '村山富市': '村山', '橋本龍太郎': '橋本',
            '小渕恵三': '小渕', '森喜朗': '森', '小泉純一郎': '小泉'}

# Exact holder observations of jp_pm: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('海部俊樹', '1990-01-19', None, None),
    ('海部俊樹', '1990-03-02', None, None),
    ('宮澤喜一', '1991-11-08', None, None),
    ('細川護煕', '1993-08-23', None, None),
    ('羽田孜', '1994-05-10', None, None),
    ('村山富市', '1994-07-18', None, None),
    ('橋本龍太郎', None, '1996-01-11', None),
    ('橋本龍太郎', '1996-11-08', None, None),
    ('小渕恵三', '1998-07-31', None, None),
    ('森喜朗', None, '2000-04-05', None),
    ('森喜朗', None, '2000-07-04', None),
    ('小泉純一郎', None, '2001-04-26', None),
    ('小泉純一郎', None, '2003-11-19', None),
    ('小泉純一郎', None, '2005-09-21', None),
]
HOLDER_CLAIMS = [
    ['jp_kaifu_signs_hc_written_answer_as_pm_19900119', 'jp_kaifu_signs_hr_written_answer_as_pm_19900123'],
    ['jp_kaifu_policy_speech_states_reappointment_19900302', 'jp_kaifu_hc_policy_speech_states_reappointment_19900302'],
    ['jp_miyazawa_policy_speech_states_appointment_19911108',
     'jp_miyazawa_hc_policy_speech_states_appointment_19911108'],
    ['jp_hosokawa_policy_speech_states_appointment_19930823',
     'jp_hosokawa_hc_policy_speech_states_appointment_19930823'],
    ['jp_hata_states_appointed_pm_19940510', 'jp_hata_hr_policy_speech_states_appointment_19940510'],
    ['jp_murayama_speaks_as_pm_19940718'],
    ['jp_kantei_hashimoto_assumes_office_19960111'],
    ['jp_kantei_hashimoto_press_conference_in_office_19961108'],
    ['jp_kantei_obuchi_press_conference_in_office_19980731'],
    ['jp_mori_appointed_pm_statement_20000405',
     'jp_mori_shinninshiki_appointed_20000405',
     'jp_mori_profile_pm_career_entry_20000405'],
    ['jp_mori_shinninshiki_appointed_20000704'],
    ['jp_koizumi_shinninshiki_20010426'],
    ['jp_koizumi_resumes_pm_statement_20031119', 'jp_koizumi_shinninshiki_appointed_20031119'],
    ['jp_koizumi_resumes_pm_statement_20050921',
     'jp_koizumi_shinninshiki_appointed_20050921',
     'jp_kunaicho_ceremony_koizumi_20050921',
     'jp_koizumi_pm_final_interview_20060925'],
]
# Event kinds that may bound or date a holder; every other kind never does.
FROM_KINDS = {'appointment_statement', 'assumption_statement', 'imperial_appointment_ceremony', 'appointment_career_entry'}
UNTIL_KINDS = {'end_of_office_stated'}  # no source in this packet states the day an office ended
ATTEST_KINDS = {'in_office_attestation'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | ATTEST_KINDS
DESIGNATION_KINDS = {'designation_vote_house_of_representatives', 'designation_vote_house_of_councillors',
                     'designation_ballot_no_majority_house_of_representatives',
                     'designation_ballot_no_majority_house_of_councillors', 'joint_committee_requested',
                     'joint_committee_no_agreement', 'house_of_representatives_resolution_prevails',
                     'retrospective_designation_record_house_of_councillors',
                     'retrospective_designation_record_house_of_representatives'}
RESIGNATION_KINDS = {'cabinet_resignation_notice', 'cabinet_resignation_decided', 'cabinet_resignation_statement',
                     'cabinet_resignation_agenda_item', 'retrospective_cabinet_resignation_record'}
ACTING_KINDS = {'acting_prime_minister_signature', 'acting_prime_minister_instruction_reported',
                'acting_prime_minister_assumed', 'acting_prime_minister_designation_notified',
                'acting_prime_minister_question'}
CONTINUATION_KINDS = {'continued_performance_of_duties'}  # art. 71: no source reviewed states it for any transition
RETROSPECTIVE_KINDS = {'retrospective_term_span', 'retrospective_cabinet_list', 'retrospective_cabinet_formation_record',
                       'retrospective_designation_record_house_of_councillors',
                       'retrospective_designation_record_house_of_representatives',
                       'retrospective_cabinet_resignation_record'}
UNDATED_KINDS = RETROSPECTIVE_KINDS | {'assumption_recalled_by_holder'}
# Claims that must never feed a holder observation, by category.
DESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in DESIGNATION_KINDS)
RESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RESIGNATION_KINDS)
ACTING = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ACTING_KINDS)
CONTINUATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in CONTINUATION_KINDS)
RETROSPECTIVE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items()
                      if kind in RETROSPECTIVE_KINDS or kind.endswith('_recalled') or kind.endswith('_recalled_by_holder'))
# The successor's designation, appointment and ceremony: the closing boundary, never a holder here and never an end.
BOUNDARY = ('jp_shugiin_designates_abe_20060926', 'jp_sangiin_designates_abe_20060926', 'jp_abe_appointed_pm_statement_20060926',
            'jp_abe_shinninshiki_20060926', 'jp_abe_cabinet_formed_20060926', 'jp_kunaicho_ceremony_20060926',
            'jp_kunaicho_ceremony_abe_20060926')
# Statements, agendas and ceremony records that print no name of the Prime Minister concerned.
UNNAMED = ('jp_kantei_extraordinary_cabinet_resignation_item_19961107', 'jp_kantei_cabinet_statement_item_19961107',
           'jp_kantei_pm_assumes_office_again_19961107', 'jp_kantei_extraordinary_cabinet_resignation_item_19980730',
           'jp_kantei_first_cabinet_meeting_19980731', 'jp_kantei_obuchi_statement_bears_office_19980731',
           'jp_mori_resumes_pm_statement_20000704', 'jp_koizumi_appointed_pm_statement_20010426',
           'jp_kunaicho_ceremony_20000405', 'jp_kunaicho_ceremony_20000704', 'jp_kunaicho_ceremony_20010426',
           'jp_kunaicho_ceremony_20031119', 'jp_kunaicho_ceremony_20050921')
NEVER_HOLDER = tuple(cid for cid in EVENTS if not any(cid in ids for ids in HOLDER_CLAIMS))
# Dates that are never any holder's attested_on, start or end: designation and resignation days, acting service,
# recollected formations and acts, members' references, the last attestation before the resignation, and the boundary.
NEVER_HOLDER_DATE = {
    '1990-01-12', '1990-02-27', '1990-02-28', '1991-11-05', '1993-08-05', '1993-08-06', '1993-08-09', '1993-08-13',
    '1994-04-25', '1994-04-28', '1994-06-25', '1994-06-29', '1994-06-30', '1994-07-06', '1996-11-07', '1998-07-30',
    '2000-04-02', '2000-04-03', '2000-04-04', '2006-09-25', '2006-09-26'}
STARTS = [(h[0], h[2]) for h in HOLDERS if h[2]]
# Leads, secondary sources and records not imported: never a source URL.
LEAD_URL_MARKERS = ('wikipedia', '111714330X00119900118', '111615254X01219891213', 't117003', 't121018', 't122001',
                    't127001', '111804006X00119900313', '19980128152510', 'souri/76', 'shinninshiki/index', 'km1107',
                    'setuji', 'murayama.html', 'danwa-122', '9807kakuryo', 'kaiken-1107', '112904024X01519940425',
                    '112905007X00319940427', '114305254X00319980807', '0004-1kakuryo/index', '0007-1kakuryo',
                    '114705254X02320000411', '114715261X01420000425', '114715254X02520000517', 'ninsyokan', 'press.html',
                    'ichiran.html', 'opensearch', 'ndlsearch', 'kanpo', '20000926015512')
# URL patterns of responses generated per request, cache-busting queries, searches or session state: never recorded.
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'OpenElement', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=',
                        'searchResult', 'opensearch', 'dl.ndl.go.jp')
# Claim ids, source ids and field names withdrawn or renamed after the checks; none may remain in the packet.
STALE_IDS = ('jp_hc_precedents_118th_designations_19900227', 'jp_hc_precedents_122nd_designations_19911105',
             'jp_hc_precedents_127th_designations_19930806', 'jp_sangiin_gyoseikanshi_20000403"', 'attested_period',
             'stated_span', 'retrospective_span"', 'reappointment_recalled_by_holder', 'diet_designation_record',
             'cabinet_resignation_en_masse', 'assumption_of_office_statement', 'reappointment_statement',
             'pm_incapacity_reported', 'acting_pm_assumed', 'cabinet_resignation_notified')
# This packet's closing boundary: its own holders are all dated before it, and later packets' holders on or after it.
CLOSING_BOUNDARY = '2006-09-26'
REPORT = research.RESEARCH / 'japan-prime-ministers-1990-2006-12.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-12.md'


def load_rows():
    packet = json.loads((research.ROOT / research.RESEARCH / 'japan.json').read_text(encoding='utf-8'))
    rows = {}
    for source in packet['sources'][len(ORIGINAL_SOURCES):len(ORIGINAL_SOURCES) + len(NEW_SOURCES)]:
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def pm_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister'], 'one prime-ministership after the groups'
    office = packet['institutions'][-1]
    assert office['kind'] == 'executive_institution' and office['lifecycle']['status'] == 'unknown'
    assert [r['id'] for r in office['roles']] == [PM], 'exactly one role'
    role = office['roles'][0]
    assert (role['title'], role['kind']) == (T_PM, 'head_of_government')
    heads = [r['id'] for e in packet['organizations'] + packet['institutions'] for r in e['roles']
             if r['kind'] == 'head_of_government']
    assert heads == [PM], 'no other head-of-government role'
    holders = role['holder_claims']
    previous = ''
    for holder in holders:
        dated = [d for d in (holder['attested_on'], holder['from']) if d]
        assert len(dated) == 1, (holder['name'], 'a holder is dated by exactly one of attested_on and from')
        assert dated[0] > previous, (holder['name'], 'holders stay in chronological order')
        previous = dated[0]
    # This packet's holders are the ones dated before its closing boundary; a later packet's holder never cites its claims.
    own = [h for h in holders if (h['attested_on'] or h['from']) < CLOSING_BOUNDARY]
    assert holders[:len(own)] == own, 'this packet\'s holders come first'
    for holder in holders[len(own):]:
        assert not set(holder['claim_ids']) & set(EVENTS), (holder['name'], 'a later holder cites this packet\'s claims')
    for holder in own:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        assert not {holder['attested_on'], holder['from'], holder['until']} & NEVER_HOLDER_DATE, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        kinds = {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            # A holder claim names its holder: the row's holder_name, and the surname printed in the claim text.
            assert row['holder_name'] == name and row['role_title'] == T_PM, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        attest_days = set().union(*(kinds.get(k, set()) for k in ATTEST_KINDS))
        # A start only where a same-day source that names the holder states the day of appointment or assumption.
        assert ({holder['from']} == from_days) if holder['from'] else not from_days, (name, 'start')
        # An end only where a source states the day the office ended; none in this packet does.
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['attested_on']:
            assert holder['attested_on'] in attest_days, (name, 'observation')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and (holder['from'] or holder['attested_on']) <= holder['until']
    # A named statement or ceremony record of a start is never left off its holder, and an unnamed one never dates one.
    for cid, row in rows.items():
        if row['event_kind'] in FROM_KINDS and row['holder_name'] is not None:
            assert any(cid in h['claim_ids'] and h['name'] == row['holder_name'] and h['from'] == row['attested_on']
                       for h in holders), cid
        assert row['event_kind'] not in UNTIL_KINDS | CONTINUATION_KINDS, cid
    # Designations, resignations, acting service, recollections and the boundary stay on the role; none is an end.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] not in FROM_KINDS or rows[cid]['holder_name'] is None, cid
    for cid in ACTING + BOUNDARY + UNNAMED:
        assert rows[cid]['holder_name'] is None, cid
    for cid in (c for c in office['claim_ids'] if c in rows):
        assert not {'period', 'attested_period', 'stated_span'} & set(claims[cid]), cid
        if rows[cid]['event_kind'] in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid], cid


def pm_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    pm_rules(packet, rows)
    role = packet['institutions'][-1]['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims']]
    assert got == HOLDERS + later.HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims']] == HOLDER_CLAIMS + later.HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'] if h['from']] == STARTS + later.STARTS
    assert not [h for h in role['holder_claims'][:len(HOLDERS)] if h['until']]
    # Distinct dated events stay distinct and keep their own days.
    for cid, (day, _kind, _obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class JapanPrimeMinistersTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.raw = (research.ROOT / research.RESEARCH / 'japan.json').read_text(encoding='utf-8')
        cls.packet = json.loads(cls.raw)
        cls.sources = {s['id']: s for s in cls.packet['sources']}
        cls.claims = {c['id']: c for s in cls.packet['sources'] for c in s['claims']}
        cls.claim_source = {c['id']: s['id'] for s in cls.packet['sources'] for c in s['claims']}
        cls.office = cls.packet['institutions'][-1]
        cls.role = cls.office['roles'][0]
        cls.extracts = {sid: json.loads((research.ROOT / cls.sources[sid]['snapshot']['path']).read_text(encoding='utf-8'))
                        for sid in NEW_SOURCES}
        cls.rows = load_rows()
        cls.new_claims = [c['id'] for sid in NEW_SOURCES for c in cls.sources[sid]['claims']]
        cls.report = (research.ROOT / REPORT).read_text(encoding='utf-8')

    def validate(self, packet=None):
        return research.validate(packet or self.packet, research.ROOT, {'Japan'}, {'Japan': set()})

    def section(self, heading):
        parts = self.report.split(f'\n## {heading}', 1)
        self.assertEqual(len(parts), 2, heading)
        return parts[1].split('\n## ', 1)[0]

    def test_new_records_are_bounded_and_every_claim_is_classified(self):
        ids = self.validate()
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), (119, 174))
        self.assertEqual([s['id'] for s in self.packet['sources']], list(ORIGINAL_SOURCES) + NEW_SOURCES + later.NEW_SOURCES)
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 5))
        self.assertEqual(len(self.packet['organizations']), 16)
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        holder_claims = {cid for ids_ in HOLDER_CLAIMS for cid in ids_}
        self.assertFalse(holder_claims & set(NEVER_HOLDER))
        self.assertEqual(holder_claims | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(holder_claims), len(NEVER_HOLDER)), (25, 149))
        self.assertEqual(set(EVENTS), set(self.new_claims))
        self.assertEqual((len(DESIGNATIONS), len(RESIGNATIONS), len(ACTING), len(CONTINUATION), len(RETROSPECTIVE)),
                         (40, 42, 9, 0, 40))
        # Every new claim and source is cited by the institution and its one role, and by no organization or group.
        self.assertEqual(self.office['claim_ids'], self.new_claims + later.NEW_CLAIMS)
        self.assertEqual(self.role['claim_ids'], self.new_claims + later.NEW_CLAIMS)
        self.assertEqual(self.office['sources'], NEW_SOURCES + later.NEW_SOURCES)
        self.assertEqual(self.role['sources'], NEW_SOURCES + later.NEW_SOURCES)
        for entry in self.packet['organizations'] + self.packet['institutions'][:-1]:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (JP-PM-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'JP-PM-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'JP-PM-{n:02d}' for n in range(1, 11)})
        for stale in STALE_IDS:
            self.assertNotIn(stale, self.raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract, ensure_ascii=False), stale)

    def test_holders_are_exactly_as_intended(self):
        pm_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims']:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = []
            for cid in holder['claim_ids']:
                if self.claim_source[cid] not in expected_sources:
                    expected_sources.append(self.claim_source[cid])
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
        for holder in self.role['holder_claims'][:len(HOLDERS)]:
            self.assertTrue(re.match(r'No (start and no )?end', holder['uncertainty']), holder['name'])
            self.assertRegex(holder['uncertainty'], r'No start and no end' if not holder['from'] else r'^No end')
        self.assertIn('earliest 1990 attestation pinned to a byte-stable response', self.role['holder_claims'][0]['note'])
        self.assertEqual(list(self.office), ['id', 'name', 'kind', 'represented_party_ids', 'reconciled_organization_id',
                                             'lifecycle', 'roles', 'sources', 'claim_ids', 'coverage'])
        self.assertEqual(self.office['represented_party_ids'], [])
        self.assertIsNone(self.office['reconciled_organization_id'])
        self.assertEqual(self.office['lifecycle'], {
            'status': 'unknown', 'from': None, 'until': None,
            'note': 'These observations do not establish founding, dissolution, legal continuity or exact terms of the institution.'})
        self.assertEqual(self.office['coverage']['status'], 'partial')
        self.assertEqual(self.office['coverage']['period'], {'from': '1990-01-01', 'through': '2026-09-07'})
        self.assertEqual(list(self.role), ['id', 'title', 'kind', 'sources', 'claim_ids', 'holder_claims', 'scope_note'])
        for cid in self.new_claims:
            self.assertEqual((self.rows[cid]['role_title'], self.rows[cid]['role_id']), (T_PM, PM), cid)
        # Row holder names are pinned; a non-null one is a holder of this role whose surname the claim text prints.
        self.assertEqual({cid: row['holder_name'] for cid, row in self.rows.items()}, ROW_HOLDERS)
        self.assertEqual({v for v in ROW_HOLDERS.values()} - {None}, set(SURNAMES))
        for cid, name in ROW_HOLDERS.items():
            if name:
                self.assertIn(SURNAMES[name], self.claims[cid]['text'], cid)
        # The acting prime ministers and the House of Councillors' 1998 designee are named only in text.
        for cid in ACTING:
            self.assertIsNone(ROW_HOLDERS[cid], cid)
            self.assertRegex(self.claims[cid]['uncertainty'], r'a claim only and never a holder', cid)
        self.assertIn('橋本龍太郎', self.claims['jp_hashimoto_acting_pm_signs_hr_answer_19900112']['text'])
        self.assertIsNone(ROW_HOLDERS['jp_hc_designates_kan_runoff_19980730'])
        self.assertIn('菅直人', self.claims['jp_hc_designates_kan_runoff_19980730']['text'])
        # One holder string per person: the House of Councillors' 熙 variant stays in text only.
        self.assertNotIn('細川護熙', {h['name'] for h in self.role['holder_claims']} | set(ROW_HOLDERS.values()) - {None})
        self.assertIn('細川護熙', self.claims['jp_hc_precedents_hc_designates_hosokawa_19930806']['text'])

    def test_starts_ends_and_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        # Each House's designation vote is its own claim; the House of Councillors' table is split by column.
        for cid in DESIGNATIONS:
            self.assertNotIn(rows[cid]['event_kind'], HOLDER_KINDS, cid)
        for house in ('house_of_councillors', 'house_of_representatives'):
            self.assertEqual(sum(rows[c]['event_kind'] == f'retrospective_designation_record_{house}' for c in rows), 3)
        self.assertEqual(sum(rows[c]['event_kind'] == 'house_of_representatives_resolution_prevails' for c in rows), 2)
        # Resignations en masse are never an end, and no continued performance of duties is recorded or implied.
        self.assertEqual(CONTINUATION, ())
        for cid in RESIGNATIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)not (an|the) end|no end|not the day', cid)
        # Acting service is claims only.
        self.assertEqual({rows[cid]['event_kind'] for cid in ACTING}, ACTING_KINDS)
        # Retrospective lists, spans and tables carry no structured date; recollections are dated by the day recalled.
        for cid, row in rows.items():
            if row['event_kind'] in UNDATED_KINDS:
                self.assertNotIn('attested_on', claims[cid], cid)
                self.assertIn('no structured date', claims[cid]['uncertainty'], cid)
        # A statement that prints no name never dates a start: the second Hashimoto holder is observed on 8 November.
        self.assertIsNone(ROW_HOLDERS['jp_kantei_pm_assumes_office_again_19961107'])
        self.assertEqual(self.role['holder_claims'][7]['attested_on'], '1996-11-08')
        self.assertIn('prints no name', claims['jp_kantei_pm_assumes_office_again_19961107']['uncertainty'])
        # Obuchi's spoken 'today' is not an assumption statement.
        self.assertIn('not read as an assumption statement',
                      claims['jp_kantei_obuchi_press_conference_in_office_19980731']['uncertainty'])
        # The Imperial Household Agency's dated ceremony records are imported; unnamed ones stay on the role.
        ceremonies = [cid for cid in rows if rows[cid]['event_kind'] == 'imperial_appointment_ceremony'
                      and 'kunaicho' in self.claim_source[cid]]
        self.assertEqual(len(ceremonies), 8)
        self.assertEqual([cid for cid in ceremonies if rows[cid]['holder_name']], ['jp_kunaicho_ceremony_koizumi_20050921'])
        # Holder wording: no end inferred from a successor, a designation or a resignation.
        for holder in self.role['holder_claims'][:len(HOLDERS)]:
            self.assertIsNone(holder['until'])
            self.assertNotRegex(holder['uncertainty'] + holder['note'], r'(?i)until the \d+ \w+ \d{4} (appointment|ceremony)')
        scope = self.role['scope_note']
        for text in ('never as a separate holder', 'procedure only, never a date', "a successor's", 'names the holder',
                     'no structured date'):
            self.assertIn(text, scope)
        unresolved = self.office['coverage']['unresolved']
        self.assertTrue(unresolved[0].startswith('Prime ministers 1990-2006 (CLAUDE-C01-12'))
        self.assertTrue(unresolved[1].startswith('Stated starts:'))
        self.assertIn('No stated end.', unresolved[1])
        self.assertTrue(unresolved[3].startswith('Holders from 26 September 2006'))
        self.assertTrue(unresolved[-1].startswith('Keep executive office distinct from party leadership'))
        coverage = self.packet['coverage']
        self.assertEqual(sum('CLAUDE-C01-12' in u for u in coverage['unresolved']), 1)
        self.assertTrue(coverage['unresolved'][8].startswith('Prime ministers 1990-2006 (CLAUDE-C01-12)'))
        self.assertTrue(coverage['unresolved'][9].startswith('Prime ministers 2006-2026 (CLAUDE-C01-13'))
        self.assertEqual(len(coverage['unresolved']), 10)
        self.assertEqual([r['records'] for r in coverage['bounded_registers']], [16, 7])

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], '2026-09-24', sid)
            self.assertIsNone(source['published_date'], sid)
            self.assertIs(extract['source_response_checked_in'], False)
            self.assertEqual((extract['source_response_bytes'], extract['source_response_sha256']), RESPONSES[sid])
            self.assertEqual(extract['source_response_content_encoding'], 'identity')
            self.assertTrue(extract['source_character_encoding'], sid)
            for phrase in ('not checked into this repository', 'derived factual extract', 'same byte count and SHA-256',
                           'identity-encoded body', 'this packet at'):
                self.assertIn(phrase, extract['provenance_note'], (sid, phrase))
            self.assertIn('No open license, portrait permission or likeness approval', extract['rights_note'])
            self.assertEqual(extract['visual_review']['pdf_pages_one_based'], PDF_PAGES.get(sid, []))
            self.assertTrue(source['source_type'].startswith('primary_') and source['scope_note'])
            snapshot = source['snapshot']
            self.assertTrue(snapshot['path'].startswith('docs/campaign-certification/C01/research/sources/japan-'))
            self.assertTrue(snapshot['path'].endswith('-facts.json'))
            data = (research.ROOT / snapshot['path']).read_bytes()
            self.assertEqual((len(data), hashlib.sha256(data).hexdigest()), (snapshot['bytes'], snapshot['sha256']))
            self.assertNotEqual(snapshot['sha256'], extract['source_response_sha256'])
            self.assertTrue(data.endswith(b'}\n') and b'\r' not in data)
            self.assertEqual(data.decode('utf-8'), json.dumps(extract, indent=2, ensure_ascii=False) + '\n')
            # Rows repeat the packet claims exactly, in order, keyed by claim_id; no row has a bare 'name' key.
            self.assertEqual([r['claim_id'] for r in extract['rows']], [c['id'] for c in source['claims']])
            for row, claim in zip(extract['rows'], source['claims']):
                self.assertEqual((row['text'], row['locator'], row['attested_on']),
                                 (claim['text'], claim['locator'], claim.get('attested_on')))
                self.assertEqual((row['observation_id'], row['role_id']), ('jp_prime_minister', PM))
                self.assertNotIn('name', row)
                self.assertEqual(list(row), ['claim_id', 'observation_id', 'review_observation', 'role_id', 'holder_name',
                                             'role_title', 'event_kind', 'attested_on', 'text', 'locator'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        self.assertEqual({cid: (row['attested_on'], row['event_kind'], row['review_observation'])
                          for cid, row in self.rows.items()}, EVENTS)
        for sid, stamp in ARCHIVED.items():
            source, extract = self.sources[sid], self.extracts[sid]
            url = urlsplit(source['url'])
            self.assertEqual(url.hostname, 'web.archive.org')
            self.assertTrue(url.path.startswith(f'/web/{stamp}id_/http'), sid)
            self.assertLess(stamp, '20260907')
            self.assertEqual(extract['original_url'], source['original_url'])
            self.assertEqual(source['url'].split('id_/', 1)[1].replace(':80/', '/', 1), source['original_url'])
            self.assertNotIn(':80', source['original_url'])
            self.assertEqual(extract['archive_capture_utc'].replace('-', '').replace(':', '').replace('T', '').rstrip('Z'),
                             stamp)
            self.assertIn('Raw Internet Archive capture', extract['provenance_note'])
            # The character encoding of every archived page is recorded (Shift_JIS, EUC-JP, ISO-2022-JP or UTF-8).
            if not source['url'].endswith('.pdf'):
                self.assertRegex(extract['source_character_encoding'], r'^(Shift_JIS|EUC-JP|ISO-2022-JP|UTF-8)')
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual((len(ARCHIVED), len(others)), (49, 70))
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others}, OFFICIAL_HOSTS)
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
            if urlsplit(self.sources[sid]['url']).hostname == 'kokkai.ndl.go.jp':
                record = self.extracts[sid]['diet_record']
                self.assertIn(record['issueID'], self.sources[sid]['url'])
                self.assertEqual(record['record_page_url'], f"https://kokkai.ndl.go.jp/txt/{record['issueID']}")
        # The House of Councillors' table: the recorded capture, with the byte-identical relocated live file beside it.
        table = self.extracts['jp_hc_precedents_pm_designations']['alternate_location']
        self.assertEqual((table['bytes'], table['sha256']), RESPONSES['jp_hc_precedents_pm_designations'])
        self.assertEqual(table['url'], 'https://www.sangiin.go.jp/jpn/shiryo/houki/09senrei/pdf/r5se-s-11.pdf')

    def test_response_identities_are_reproducible_urls(self):
        for sid in NEW_SOURCES:
            url = self.sources[sid]['url']
            for pattern in PER_REQUEST_PATTERNS:
                self.assertNotIn(pattern, url, url)
            parts = urlsplit(url)
            self.assertEqual(parts.scheme, 'https', url)
            if parts.hostname == 'web.archive.org':
                self.assertRegex(parts.path, r'^/web/\d{14}id_/https?://', url)
            else:
                self.assertIn(parts.hostname, OFFICIAL_HOSTS, url)
            # The Diet minutes are recorded only through the public API: a whole meeting or a pinned speech.
            if parts.hostname == 'kokkai.ndl.go.jp':
                self.assertRegex(url, r'^https://kokkai\.ndl\.go\.jp/api/(meeting\?issueID=\w+|speech\?(speechID=\w+|'
                                      r'issueID=\w+&speechNumber=\d+))&recordPacking=json$')
        # This packet's Kantei and Imperial Household Agency pages are recorded only through fixed archive captures; the
        # later packet's stored official pages are exactly the ones its own test pins.
        for source in self.packet['sources'][:len(ORIGINAL_SOURCES) + len(NEW_SOURCES)]:
            self.assertNotRegex(source['url'], r'^https://(www\.)?(kantei|kunaicho)\.go\.jp', source['id'])
        self.assertEqual([s['id'] for s in self.packet['sources']
                          if re.match(r'^https://(www\.)?(kantei|kunaicho)\.go\.jp', s['url'])], later.LIVE_OFFICIAL)

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for source in self.packet['sources'][:len(ORIGINAL_SOURCES) + len(NEW_SOURCES)]:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, source['url'], source['id'])
        # For the later packet's sources, the generic 'press.html' and 'kanpo' markers are re-expressed as the exact leads
        # they excluded here (the 2003, 2005 and 2006 press conferences and the Kantei-hosted Gazette contents); the later
        # packet records other press conferences and 2020-2026 Gazette issues as its own sources.
        exact = {'press.html': ('2003/11/19press', '2005/09/21press', '2006/09/26press'),
                 'kanpo': ('/jp/kanpo/',)}
        for source in self.packet['sources'][len(ORIGINAL_SOURCES) + len(NEW_SOURCES):]:
            for marker in LEAD_URL_MARKERS:
                for form in exact.get(marker, (marker,)):
                    self.assertNotIn(form, source['url'], source['id'])
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'nikkei', 'asahi.com', 'nhk.or.jp'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in ('111714330X00119900118', 't122001', '19990422184900', 'km1107ee', 'kaiken-1107', '0405setuji',
                       '114715261X01420000425', 'd04-13-ninsyokan', '112904024X01519940425'):
            self.assertIn(marker, leads, marker)
            self.assertNotIn(marker, added, marker)

    def test_packet_formatting_is_preserved(self):
        data = (research.ROOT / research.RESEARCH / 'japan.json').read_bytes()
        self.assertNotIn(b'\r', data)
        self.assertEqual(data.decode('utf-8'), json.dumps(self.packet, indent=2, ensure_ascii=False) + '\n')

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
            return packet['institutions'][-1]['roles'][0]

        def holder(packet, index):
            return role(packet)['holder_claims'][index]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = self.claim_source[cid]
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        validator_cases = [
            (lambda p: source(p, 'jp_hc_precedents_pm_designations')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'jp_kunaicho_photo_20050921')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'jp_hr_plenary_19900227')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, 13).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, 5).update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_koizumi_pm_final_interview_20060925').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_kantei_span_koizumi_89_20050921_20060926').update(
                period={'from': '2005-09-21', 'through': '2026-09-30'}), 'exceeds cutoff'),
            (lambda p: holder(p, 6).update(until='1995-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, 9)['claim_ids'].append('jp_koizumi_shinninshiki_20010426'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('jp_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def extra_holder(name, day, cid, start=None):
            return {'name': name, 'attested_on': None if start else day, 'from': day if start else None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        rule_cases = [
            # A successor's designation or appointment, or the same person's reappointment, used as an end.
            ('successor appointment used as an end (Koizumi 2005)', lambda p: holder(p, 13).update(until='2006-09-26')),
            ('successor appointment cited as an end (Koizumi 2005)',
             cite(13, 'jp_abe_appointed_pm_statement_20060926', until='2006-09-26')),
            ('successor ceremony cited as an end (Koizumi 2005)', cite(13, 'jp_kunaicho_ceremony_abe_20060926', until='2006-09-26')),
            ('successor appointment used as an end (Mori 2000)', lambda p: holder(p, 10).update(until='2001-04-26')),
            ('own reappointment used as an end (Hashimoto 1996)', lambda p: holder(p, 6).update(until='1996-11-07')),
            ('successor designation used as an end (Kaifu 1990)', lambda p: holder(p, 1).update(until='1991-11-05')),
            ('successor observation used as an end (Hosokawa)', lambda p: holder(p, 3).update(until='1994-05-10')),
            # A resignation en masse, its notice or a retrospective span used as an end.
            ('resignation notice cited as an end (Miyazawa)',
             cite(2, 'jp_miyazawa_notifies_cabinet_resignation_19930805', until='1993-08-05')),
            ('resignation statement cited as an end (Hashimoto 1998)',
             cite(7, 'jp_kantei_hashimoto_cabinet_resigned_19980730', until='1998-07-30')),
            ('art. 70 resignation used as an end (Obuchi)', lambda p: holder(p, 8).update(until='2000-04-04')),
            ('retrospective span used as an end (Koizumi 2005)',
             cite(13, 'jp_kantei_span_koizumi_89_20050921_20060926', until='2006-09-26')),
            ('last interview used as an end (Koizumi 2005)', lambda p: holder(p, 13).update(until='2006-09-25')),
            # A designation date or a retrospective formation used as a start.
            ('designation date used as a start (Obuchi)',
             lambda p: holder(p, 8).update({'attested_on': None, 'from': '1998-07-30'})),
            ('designation cited as a start (Obuchi)', cite(8, 'jp_hr_designates_obuchi_19980730')),
            ('designation date used as a start (Hata)', lambda p: holder(p, 4).update({'attested_on': None, 'from': '1994-04-25'})),
            ('prevailing resolution cited by a holder (Obuchi)', cite(8, 'jp_hr_resolution_prevails_obuchi_19980730')),
            ('retrospective list used as a start (Murayama)',
             lambda p: holder(p, 5).update({'attested_on': None, 'from': '1994-06-30'})),
            ('retrospective list cited by a holder (Hata)', cite(4, 'jp_kantei_list_hata_cabinet_formed_19940428')),
            ('member recollection used as an observation (Hosokawa)', lambda p: holder(p, 3).update(attested_on='1993-08-13')),
            ('unnamed statement used as a start (Hashimoto 1996-11)',
             lambda p: holder(p, 7).update({'attested_on': None, 'from': '1996-11-07'})),
            ('unnamed statement cited as a start (Hashimoto 1996-11)',
             cite(7, 'jp_kantei_pm_assumes_office_again_19961107', attested_on=None, **{'from': '1996-11-07'})),
            ('unnamed ceremony record cited by a holder (Mori 2000)', cite(9, 'jp_kunaicho_ceremony_20000405')),
            ('dated photo caption used as the observation (Obuchi)', lambda p: holder(p, 8).update(attested_on='1998-07-30')),
            ('named start left off its holder (Koizumi 2003)',
             lambda p: holder(p, 12)['claim_ids'].remove('jp_koizumi_shinninshiki_appointed_20031119')),
            # Acting service or continued duties added as a holder, or cited by one.
            ('acting prime minister added as a holder (Aoki)', lambda p: role(p)['holder_claims'].insert(
                9, extra_holder('青木幹雄', '2000-04-03', 'jp_aoki_takes_acting_pm_0900_20000403'))),
            ('acting prime minister added as a holder (Hashimoto 1990)', lambda p: role(p)['holder_claims'].insert(
                0, extra_holder('橋本龍太郎', '1990-01-12', 'jp_hashimoto_acting_pm_signs_hr_answer_19900112'))),
            ('acting service cited by a holder (Obuchi)', cite(8, 'jp_aoki_commenced_acting_duties_0900_20000403')),
            ('continued duties added as a holder (Hosokawa 1994)', lambda p: role(p)['holder_claims'].insert(
                4, extra_holder('細川護煕', '1994-04-25', 'jp_hc_hosokawa_cabinet_resolves_resignation_19940425'))),
            ('successor added as a holder (Abe)', lambda p: role(p)['holder_claims'].append(
                extra_holder('安倍晋三', '2006-09-26', 'jp_abe_appointed_pm_statement_20060926', start=True))),
            ('member reference cited by a holder (Murayama)', cite(5, 'jp_hc_member_refers_to_murayama_cabinet_19940706')),
            ('span given a structured date', lambda p: claim(p, 'jp_kantei_span_mori_85_20000405_20000704').update(
                attested_on='2000-04-05')),
            ('span stored as a structured period', lambda p: claim(p, 'jp_kantei_first_kaifu_span_19890810_19900228').update(
                attested_period={'from': '1989-08-10', 'through': '1990-02-28'})),
            ('list given a structured date', lambda p: claim(p, 'jp_kantei_list_obuchi_cabinet_formed_19980730').update(
                attested_on='1998-07-30')),
            ('second role', lambda p: p['institutions'][-1]['roles'].append(dict(copy.deepcopy(role(p)), id='jp_pm_2'))),
            ('second institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][-1]),
                                                                           id='jp_prime_minister_2'))),
            ('institution not after the groups', lambda p: p['institutions'].insert(0, p['institutions'].pop())),
            ('head of government moved onto an organization', lambda p: p['organizations'][0]['roles'].append(
                dict(copy.deepcopy(role(p)), id='jp_pm_org'))),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
        ]
        pm_rules(self.packet, self.rows)
        pm_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    pm_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError)):
                    pm_invariants(packet, self.rows)
        # Row-level mutations: an acting row or an unnamed start given a holder name is caught by the rules.
        for cid, name in (('jp_hashimoto_acting_pm_signs_hr_answer_19900112', '橋本龍太郎'),
                          ('jp_kantei_pm_assumes_office_again_19961107', '橋本龍太郎'),
                          ('jp_abe_appointed_pm_statement_20060926', '小泉純一郎')):
            rows = copy.deepcopy(self.rows)
            rows[cid]['holder_name'] = name
            with self.subTest(named=cid), self.assertRaises(AssertionError):
                pm_rules(self.packet, rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('jp_hr_designates_hashimoto_19961107', '1996-11-08'),
                         ('jp_kaifu_notifies_cabinet_resignation_hc_19900227', '1990-03-02'),
                         ('jp_aoki_takes_acting_pm_0900_20000403', '2000-04-05'),
                         ('jp_member_recalls_hata_cabinet_launch_19940428', '1994-05-27')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                pm_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        decisions = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted in part',
                     '04': 'Accepted in part', '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted in part',
                     '08': 'Accepted', '09': 'Accepted', '10': 'Accepted in part'}
        for number, decision in decisions.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| JP-PM-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 12)] + [f'B{n}' for n in range(1, 12)] + [f'C{n}' for n in range(1, 10)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('c2aafd22', 'ffe54b02', 'research-index.json', 'test_japan_research_s10d.py', 'test_campaign_census',
                     'Observed on'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'japan-prime-ministers-1990-2006-12.md', 'claude/c01-jp-12', 'c2aafd22',
                     'ffe54b02', 'test_japan_prime_ministers_c01_12.py'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 5))
        self.assertEqual({w['status'] for w in index['work_orders'] if w['nation'] == 'Japan'}, {'open'})
        self.assertFalse(index['c01_complete'])


if __name__ == '__main__':
    unittest.main()
