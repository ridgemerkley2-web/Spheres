"""CLAUDE-C01-13: Japan's prime ministers, 2006-2026, keep each House's designation, the Imperial appointment
ceremony, the cabinet's formation, its resignation en masse and continued duties apart, state a start only where a
same-day source that names the holder does, and an end only where the Official Gazette states the day an office was lost."""
import copy
import hashlib
import json
import re
import unittest
from urllib.parse import urlsplit

import campaign_research as research


# Original response identity recorded in each extract: (bytes, sha256), of the identity-encoded body. Every new source is
# reproducible: a raw Internet Archive capture, an official Diet minutes API response or a stored official page or PDF.
RESPONSES = {
    'jp_kantei_abe_danwa_named_20060926':
        (4910, '8ae3f73ecc88af4cd9695dcb1a0d0f0c8420f90732a1dc5d201d9791d944d886'),
    'jp_kantei_abe_hossoku_named_20060926':
        (5538, '370ae14d206d380de78296b2a214df8756375e1fed3f9311393dbeb8197f8bbf'),
    'jp_kunaicho_photo_abe_named_20060926':
        (1399, '416637aeddc3e58db190599f518f2ba2abde8e8827b95653e9fd097fc8e64136'),
    'jp_kantei_abe_kaiken_20070912':
        (11978, 'eaacc123071a3791bc48d5382ca0ea639ffb294f56746c5eb11828a4f14d7b8d'),
    'jp_kantei_ccs_press_20070913_am':
        (2416, '47cc2b8fadc4e180eb4638053d570986753e33e2b7624c558243683713eae123'),
    'jp_kantei_abe_kaiken_20070924':
        (10129, '99ffe0e22edfb853ec9577281950a2b0b4bea9c688415c32bb675165b72555ce'),
    'jp_kantei_abe_sojishoku_danwa_20070925':
        (2657, '13832132ab7a981400cf1055e259c949709d0db877ccf297d38ecfc4b4f79b57'),
    'jp_kantei_abe_jisyoku_20070925':
        (2902, 'c42391d8140555c45415c1a5e8f6b4e8c441cfac92176e6743a8bf22fba05514'),
    'jp_shugiin_giun_20070925':
        (13216, 'd4b000ea092812c4c74010c3a23a1f76503620143d5dd5d1015139f430d3e135'),
    'jp_sangiin_giun_20070925':
        (19740, 'c2c668fc7c15251eaa9452b3623bc297dc093014ad069337123fd88f1cf16e19'),
    'jp_shugiin_honkaigi_20070925':
        (30262, 'a92d9fc2613be80a664a0a6408a4bb715d3d36701858ad6ba2a52272b62b5c45'),
    'jp_sangiin_honkaigi_20070925':
        (15388, '41400cdb41544e25180f4f8a54c8ae6cdc2f7f1b37be1edce694bc2b5fe5e0ca'),
    'jp_ryoin_kyogikai_20070925':
        (21704, '15d18d73dfd375df9ad40d580808b517ed43498980350b3b7df0a21e7f155625'),
    'jp_kantei_ccs_press_20070925_am':
        (2042, '8da4dcc5306f7942c73d15eb471519a4bef02b9839e83a3697ac8afee76a3d13'),
    'jp_kantei_rekidai_090':
        (28838, '3cf8487d36139725dc97e03bb9c3775347906377bdb99a6e05735ff630b89cbe'),
    'jp_kantei_fukuda_kaiken_20070925':
        (16992, 'ef685ca58301b4d2fbf84717041e6a0e409f20aa09e73ad46d747e27227e7d70'),
    'jp_kantei_ccs_press_20070925_pm':
        (2406, 'a750b12c7219a743d3383fff663b48f4982fc62ad7db05d0cac5c858009e6d47'),
    'jp_kantei_fukuda_hossoku_20070926':
        (4282, '0a4787142bcb4af8b00f0d4ba8a6bedca33475117401822d7eff4ed188e2bfb0'),
    'jp_kantei_fukuda_danwa_20070926':
        (3629, 'a03099ffe7d4185a9919c98057280cb38cef5e3a8e219754fbacbe0a5261596a'),
    'jp_kunaicho_schedule_2007_h2':
        (49199, '55f08b6f6d654005b7b52d14d0c17f4cf0e9d8db75e1af425af894e9bc066d34'),
    'jp_kunaicho_photo_20070926':
        (6368, '425306b9ab548b4a2823092ccb01266d324bf23c4453fda493fd0281f3845ecc'),
    'jp_kantei_ccs_press_20070926_am':
        (5350, 'acd3970d9488501f5ffc2cb332cfaefd1ffbb362dd6deedafed53b8916306d1f'),
    'jp_kantei_fukuda_kaiken_20080901':
        (12859, 'f3b6f2cd913cf72629c7dbe75c64d01f4c303d0d6f77b7ea47cb98a9b84b659d'),
    'jp_kantei_fukuda_sojishoku_danwa_20080924':
        (3116, '4d9425a5d215dc16882dc258d1b00b096a450ae525230c3c0f987943be8ad8e6'),
    'jp_kantei_fukuda_jisyoku_20080924':
        (2545, 'be08035349c4c182d7b9a2937a3335a8e3324e5fb51adc27caf1ffd26057a198'),
    'jp_shugiin_giun_20080924':
        (34524, '566ef27c2b80a7c0cfc6aa5ab4b24d4d2eca36e336ce4391d6e4b95e70f2ac02'),
    'jp_sangiin_giun_20080924':
        (30512, 'ae0f5586094f442e80d612cfb9ddbffb39b5d6942c6225cbf376cf211c977189'),
    'jp_shugiin_honkaigi_20080924':
        (34785, 'b8021c5f421d0cf2732ea596c75487b09b1c91d8a4edd105d640816256223844'),
    'jp_sangiin_honkaigi_20080924':
        (22134, '652c9632b1db619563afecdc3cb923f7853dfbca825147355eff7f6c3d506700'),
    'jp_kantei_ccs_press_20080924_am':
        (4661, '238f390a09275dfb19a9295718f43d4ccfc43d2bcd2710875c45686630eca014'),
    'jp_kantei_rekidai_091':
        (26041, '519c2c756fb4e3cfb70ba207a57610b01ad2bfd8477041a82df37cfc95445148'),
    'jp_ryoin_kyogikai_20080924':
        (20729, 'e46fc671f999a7b44ae3f8f79a702e13a0807158c8e85d0a8496ac9a461da0fd'),
    'jp_kantei_aso_hossoku_20080924':
        (4596, '6d02091e465e7c0923b86e4ab3ca8e110d047cacdad8eebb5f76945dbac06af1'),
    'jp_kunaicho_schedule_2008_h2':
        (50261, 'bcdbed9038e6e88036c204cbad185cbd7815bfd6dc3ff535a77ac01304f55f50'),
    'jp_kunaicho_photo_20080924':
        (1374, 'afc7578a08ad46adb0db172df84ffcec64882199ab50a558f6e7a2a52158cc2e'),
    'jp_kantei_ccs_press_20080925':
        (4896, '5c031e141fc25a7729a91e925327a19921c14719c8c7f95e547ef11a66e52cda'),
    'jp_kantei_aso_kaiken_20090916':
        (8072, '06f1e46a55b4452d10fbdf217302bc308d7e62ee50d85fc878ebab1ab01f507f'),
    'jp_kantei_aso_jisyoku_20090916':
        (5848, '29996fb877467e0ba3b2217a47e94e1efafafdfb680598bcdfa2f84c5ecad51c'),
    'jp_sangiin_giun_20090916':
        (27105, '84ff5f02fcd6a18f0153eaa21da0fff1c5fe999d5fa940650e910ec90525046d'),
    'jp_kantei_rekidai_092':
        (19314, 'bfcdce324760bf7d2b4a3bc4cf6afa48eea7ae7a0c108ea6e575d5e5a8dde82d'),
    'jp_shugiin_honkaigi_20090916':
        (69025, 'a7591492e2e540a21dbca9abb2924b00c49bbf2a9d0e8c15d7ef7d4b150191a9'),
    'jp_sangiin_honkaigi_20090916':
        (21124, 'a09db976b2560585e7dcf59a6f19fe9874bdb305df5153f636b91be3b31d4c62'),
    'jp_kantei_hatoyama_hossoku_20090916':
        (7876, '7386aac30ea6aa39647ee22bad42faa020b9f174d6d65cefb9a2248230905a0f'),
    'jp_kunaicho_schedule_2009_h2':
        (53863, '328d85d3cce00f595a700336f627a067560b0163b02aff345ae6c945e757cbbc'),
    'jp_kunaicho_photo_20090916':
        (6689, '395bad47d1f64fac0dcd6218e548b3855d814327f940945caffd85acaef51ebb'),
    'jp_kantei_ccs_press_20090916_pm':
        (4594, '3d15f88d33deb874a3022ab85ef2d1846f901fcfff4821b435da48be46850bf8'),
    'jp_kantei_ccs_press_20100602_am':
        (20726, '50e1736be735cfa597c89ad177a8fcb43eb04b1e2ade3fcd22b9cb9454953988'),
    'jp_kantei_ccs_press_20100602_pm':
        (22082, 'fd8e24eac00aeddb1cca232dab183faccc7839f155ae602632835142dda28602'),
    'jp_kantei_hatoyama_hatsugen_20100604':
        (116842, 'fd493fffe4a78e5bac6adb46c74e3c01765d9d4ee4d4fea960618c86726740cb'),
    'jp_kantei_hatoyama_statement_index_201006':
        (16349, '6b961423c3d8091ddc6ebc6c182eed0cecaab4915c1c1de549fc5b732daa6234'),
    'jp_kantei_hatoyama_sojishoku_danwa_20100604':
        (4641, '8a1d43d763a678838e10dbb2c1b637b4a531ea6793344fb752348deadb4d4b13'),
    'jp_kantei_hatoyama_soujisyoku_20100604':
        (33665, '7fd14d3b3817223bf9faff7443eb6738e567827355a8e4dd81f36894e012f26e'),
    'jp_shugiin_giun_20100604':
        (6481, '5661133c528380db46a32a9cf6946b0be833b2a3f96a167a25c26d981b976e58'),
    'jp_sangiin_giun_20100604':
        (7995, '3be7ff85858effb11d74271e93dc42a2ff748888331730fdb20ee643c2a444e7'),
    'jp_shugiin_honkaigi_20100604':
        (20798, 'a8d6b302e097bb4071d0e86a2157f53c725cf3f08bfd261541de8bfd95f291d5'),
    'jp_sangiin_honkaigi_20100604':
        (6008, '92025d9c9a7dc90b7b36a9577e5ff203a092ad68cd5dab00049e05131be84fb5'),
    'jp_kantei_ccs_press_20100604_am':
        (26843, '499a3e64716a5e22c179396acd50ac316008a1da0e79f9bb74f6813e5c8eabb7'),
    'jp_sangiin_honkaigi_20100615_s004':
        (33991, '6baa619bd7c40e826d8e626be9997ff163bd61405712fbf465940ae80a8c51eb'),
    'jp_sangiin_honkaigi_20100615_s005':
        (28869, '1c7a8082916438ac099132949573bf6f5a969b90cf3567de4457adea0ba77ca6'),
    'jp_kantei_rekidai_093':
        (20961, '8b82f66c27268cb6522c01c388678b9bb4d7d6d83d3f464e5bc320161321a616'),
    'jp_kantei_kan_shimei_20100604':
        (12851, '7946fef10bd0fcf652b9e25f0eb9962e32553befbf4cf72c6a31fc988109ea71'),
    'jp_kunaicho_schedule_2010_q2':
        (92768, '7877d61a685fcb0fe3d7e7716580d9ef6c58991702e6f21b558f90a938cc115d'),
    'jp_kunaicho_photo_20100608':
        (6848, 'c6efd730446c6bc871ea4aeabe3cc309e70462b12e712fcd5932790a390e898f'),
    'jp_kantei_kan_kaiken_20100608':
        (37934, '4f556ce00846715f2435a943711fd66e9c6576ee95fb59382c103f7748a3d572'),
    'jp_kantei_kan_hossoku_20100608':
        (17434, 'f1b653fb2d91880e27b87671d2a418c90d99c464941665d92f056d8095017d2a'),
    'jp_kantei_ccs_press_20100608_pm':
        (21626, '34a6ecba1fbfa7d4b2947e3227796d94ec28cad7eea4bfb44157f2794ef22563'),
    'jp_shugiin_honkaigi_kan_speech_20100611':
        (1838, 'd12c6b39420fbfb58198a9b84e631df0d8e7e119e59692cecf410c6765ab65de'),
    'jp_kantei_kan_kaiken_20110826':
        (26303, '48b2aa965738108afa5de56e4554b560fd5e371ef20e7cb70dcb5ee92c8c9f04'),
    'jp_shugiin_giun_20110830':
        (6980, '86f4b11bfcd802ad3572db7c98716063d01808c9001fb59da330510e24ee15b0'),
    'jp_shugiin_honkaigi_20110830':
        (21275, '522c09dd8f6851a792cefa348fddf4d7de4c4a972eef1f66b9ba796e685af50e'),
    'jp_sangiin_giun_20110830':
        (8022, '2c2d0af7d8b6e8c0bf3dd0c24d6efffc49f6a34ecc553499bbeea9016df2cbcd'),
    'jp_sangiin_honkaigi_20110830':
        (9135, '8a239d08ea274063a87e629233eda5549af46ed0940c7bd3d1eec292213b979d'),
    'jp_kantei_kan_sojishoku_danwa_20110830':
        (3633, 'd78770727ca146806c4eabea8e8b37f7acc3211e5dac307c4916b928d3edf675'),
    'jp_kantei_kan_bousai_20110901':
        (59181, '0c8d3df84c20f5103e8328cb1ce34ba0b4e2210483c6d6ab21e65ad30f5b87b1'),
    'jp_kantei_kan_sojishoku_20110902':
        (56136, '1392b41c292a1fd3c577e12a144fadbc4ca4db8facd2dc0dd5d4bde25139859d'),
    'jp_sangiin_yosan_20110928_s112':
        (2147, 'f29f84f547d1400edbae6171e0dea63dd303af5e085aa2759dc5944d49be671b'),
    'jp_sangiin_yosan_20110928_s113':
        (1636, 'f5295a72a0b0652633b1bf4dd3d8cec2d5d8fc5959316f9f322004b6721484dc'),
    'jp_sangiin_yosan_20110928_s115':
        (1200, 'c37e04da8418915e7d759f7dcd6eba98c8f39f8c47c1aa6e7f783bde24b51ff3'),
    'jp_kantei_rekidai_094':
        (43705, 'cb3d28f2479ef1c889a30840c39e2b6578b535083c31e2cdb4df19503858fb91'),
    'jp_kunaicho_schedule_2011_q3':
        (91105, '45fe2de56875cdc46ac20908e854f50972e3e753402d6fb6b8291b5cfe3932fa'),
    'jp_kunaicho_photo_20110902':
        (6873, '6b166201772c35ca712c04195f32f813aa1ecb3f1a62339287328a4359dd966f'),
    'jp_kantei_noda_hossoku_20110902':
        (16888, '583e8de4d2fd466643cf705a29a892e2a92a3ed800994ecf8e9f333df43d571d'),
    'jp_kantei_noda_kaiken_20110902':
        (36456, '135937c14e0065fd0eed2e63ba5a37b0e4e9b93a9d5db1aaf73963d73d37d808'),
    'jp_kantei_noda_siji_20110902':
        (22460, '9ae7737d0810cc10d587a9e0b2f42842f9e06df70f22a8cc5ec872cccf335a71'),
    'jp_shugiin_honkaigi_noda_speech_20110913':
        (3606, '53172a0b9a33a26553338c0f207aa5dcf256542ccb4a060f8dbef84cb8680432'),
    'jp_sangiin_yosan_20110928_s119':
        (1338, '44a42e4f2ded443bebf52bb1a6500a4e5618a6727f82a6e1046ee89ccae58e2f'),
    'jp_sangiin_giun_20121226':
        (30068, 'b643ae687f64c8af4923f10ef507ec078f3bd139f6d0ddbdd8b3cf665fdcb34b'),
    'jp_kantei_noda_sojishoku_20121226':
        (16317, 'dd2eabb7cf9ed6f99f1e2d823d6d720dca1d23f94d9e9636ef9658d2d4e0ff1c'),
    'jp_kantei_noda_sojishoku_danwa_20121226':
        (15799, '5b0af54e4218008136ed80b5206c5ac791571492376ac3f408406fc42f4a7926'),
    'jp_kantei_rekidai_095':
        (58157, '95a458c91735ec95c42454d3282a7f1daa14bb006943c5367652504876e91caf'),
    'jp_shugiin_honkaigi_20121226':
        (69616, 'ae6c621d5abfa2af293c4950ab4665f880a4c208ef70600f3cf774028cb64f34'),
    'jp_sangiin_honkaigi_20121226':
        (27489, 'a0cfc74d6b9ca1cd70d9848b52a7b4682641d376a204b9b6c74560eec192247d'),
    'jp_kunaicho_schedule_2012_q4':
        (113891, '5b5eef31a0bb8271e59ffe0e096b60bd1e36e5ffb6bf2f8424d65bd969ab9426'),
    'jp_kunaicho_photo_20121226':
        (6858, '5bfe99768d3c8b8d4fa7e097fea1b4d054ee172564c918c6cbb6d16f62663b6e'),
    'jp_kantei_abe_danwa_20121226':
        (19811, '83bcd2fd9f0ad6217ef4b85a14a4f1de632fa5f13350f15aaf4811f28114d89f'),
    'jp_kantei_abe_hossoku_20121226':
        (24333, '814d31c1a93f65df12870d9512b887fe1a2a3ebfe78d390b66a2f772b12427d6'),
    'jp_kantei_abe_kaiken_20121226':
        (43441, 'af9f9f46e876173c2cbd118ae20174753b5e0cacccecf6450fb6073f6efc0ea7'),
    'jp_kantei_abe_designation_20121226':
        (21392, '9ebdc605f11b9f6ff332ce064cf33bfeb29ad657eb3120014b753663e6e2663a'),
    'jp_shugiin_honkaigi_abe_speech_20130128':
        (15385, 'b17196d4b46ee16e451bda8b5826c78db9d2d2127bde4656cce1ed58d44c8e3f'),
    'jp_sangiin_giun_20141224':
        (26007, '298d5b66515afa8e8eae747e1a126a8254552af70bf27e66e06b7416602c1ed3'),
    'jp_shugiin_honkaigi_20141224':
        (69311, '74a5b662a324d462b108b694299cc3b7c364a8fc8e41aad1c40ab9582574ada3'),
    'jp_sangiin_honkaigi_20141224':
        (11613, '783d31221ee90c4110ba132ca74dbb74e8c836f99e680e2739f102b41e480ebf'),
    'jp_kunaicho_schedule_2014_q4':
        (105206, '344fcb463831777911995284bfdf505421137c6c8927ebf5efff8f1e35ae952e'),
    'jp_kunaicho_photo_20141224':
        (6845, '735f834f67044135d2cc1055b5fe08c1d44932a12995633a06d360fbb6e32915'),
    'jp_kantei_abe_hossoku_20141224':
        (16738, 'b23ba888225a575642d78fab3bcb806fd3d18676be71d07c93ac563776966ba2'),
    'jp_kantei_abe_kaiken_20141224':
        (33673, '315cf95d110c47f615bd42d8e8d579d0b111a72e46f2507237be0164d9c279ae'),
    'jp_kantei_abe_designation_20141224':
        (13763, '9e33a5c1434481adbaa83b7ee269b04b740756744f7f521e8a61c92b556676b9'),
    'jp_shugiin_honkaigi_abe_speech_20150212':
        (38562, '6f627d9bdc853eb0311f5b945817ea71a4702f5d38cf50e44e03af2fb84cd8df'),
    'jp_sangiin_giun_20171101':
        (21659, '04b451138b946a704e0fae09b9d1ac9d3cbeaa14bcdad694270a27b0536f7286'),
    'jp_shugiin_honkaigi_20171101':
        (69250, '0511b93adcf4cda0817dafefb7e4009e2fd41079ea5308140334cdabde76edf5'),
    'jp_sangiin_honkaigi_20171101':
        (11246, '1f7912f9a82efbb472df21827371a75cb2439e8391a06305413c1a7016adb5dc'),
    'jp_kunaicho_schedule_2017_q4':
        (107791, 'cfb8239c6a5550fed48c98474075a554c00ef512949bd47add56b81ae26f5417'),
    'jp_kunaicho_photo_20171101':
        (6850, '81e9cac165dd0179ce40bb97cc7ac438dc337bf58fa50c64bb51a34204916deb'),
    'jp_kantei_abe_hossoku_20171101':
        (13255, '358055449888c3e83f55e5cf8b45541a7a366708b81c97777ee4c9d1b29a86fc'),
    'jp_kantei_abe_kaiken_20171101':
        (30141, '8b8b96ea141d2fdef03ac6a5d8d55420be859ef9843c12571dcea79c6a10d710'),
    'jp_kantei_abe_designation_20171101':
        (9378, '5d86abb9c8b5b8f6b34a9ffd0a6be5d3a23c7907e298f3d88efd83ba08aeef54'),
    'jp_shugiin_honkaigi_abe_speech_20171117':
        (11605, 'f98ba62851cfedcfd180dbc646b22fc986e73c9725510b91608236c3ba5996eb'),
    'jp_kantei_abe_kaiken_20200828':
        (62923, '1bee7ee383dc5a3c02cf10061dacbed2b0b742ea8a64875ee71049bee1ed31c7'),
    'jp_shugiin_giun_20200916':
        (18970, 'e48a623493bd12abfdb8932d4c9e77be6f8e5babf496df1a593966fbcf5dacba'),
    'jp_shugiin_honkaigi_20200916':
        (21827, '5e0b66751c83450766b098f35e39947eaefd6b79f64704bd5ef1fb22bd698cdd'),
    'jp_sangiin_giun_20200916':
        (30559, '8e6465ec3bdbf9527d149eb26bc63d85591b0d78148e97212a59221097e9e024'),
    'jp_sangiin_honkaigi_20200916':
        (17519, 'c0d20dbb3ec6248150d6bb69a0968aab288887f0ce41b9b4d20785b8fcf1cbca'),
    'jp_kantei_abe_sojishoku_20200916':
        (8674, 'c3a8db1bf67f941b77094290086091137b4f8818338c32a76080104bb357f437'),
    'jp_kantei_abe_sojishoku_danwa_20200916':
        (8106, 'aa2e117a50296be4f74e021b9ecfaf316728eff262d05032b138f68cb76ff993'),
    'jp_kanpo_gogai_toku99_20200916_p2':
        (308013, '8f7729b138150b73c7fd82e4e9784e84f03741af6c8abac18e8bd0845ee0a744'),
    'jp_kantei_rekidai_096':
        (85785, 'afeb71fde9c1219cea24fb88dcec12d86c185f92a25842704557f7b31f183fab'),
    'jp_kantei_rekidai_097':
        (169110, 'a5037eed2a14d72c5eaee20c81f27a973df8d96f5dba45f4b024176d4e988a49'),
    'jp_kantei_rekidai_098':
        (159667, 'f433d6442ccbc6cb67e4e96f857b41bc0ed59a4abe948b0d3996d6f48f222e5f'),
    'jp_kantei_suga_designation_20200916':
        (10667, '55065f6ce5f8f94c2631fbafc7621abbf0ea985ca9c129877a99e6e9b89383d1'),
    'jp_kantei_suga_cabinet_launch_20200916':
        (15329, 'fe8bd4514b05e574a5f5e9ee48fddd142dc811cef8ccfe2511df072c10457b9e'),
    'jp_kantei_pm_statement_20200916':
        (7104, '90455bbeb1dd4d67c33b8e42e30bf990731cc39e325f7f5d4b7d6f3fc8ba0d32'),
    'jp_kantei_suga_press_conference_20200916':
        (33531, 'c9d509ccbcd8cb50eb679537cc40538cd1a50808c11e010174bb14ba60ce637c'),
    'jp_kunaicho_schedule_entry_20200916':
        (7702, '3aefa0e939fddf02448e24853c45a5c0db43254da1d203be7343049a076d3903'),
    'jp_kantei_cabinet_minutes_20200916_first':
        (365264, '79133cfc8cfd312ee766809192d4c8a8873c96d2ec2b3f42aeaaa97d9627c0b3'),
    'jp_shugiin_giun_20211004':
        (19535, 'b321d64999b3c231848b86b2e30f8d7221769a36ec49fec7e4d1d703ccbb1793'),
    'jp_sangiin_giun_20211004':
        (28076, 'e98e6bb98ea59d1e6e1df604036efb7f04044419b3898670b69a6efb0cf5bb29'),
    'jp_shugiin_honkaigi_20211004':
        (22423, '2f5816ef662b511980bb18249d266a034c388763b7b6fe393e33a1092b2e4cd8'),
    'jp_sangiin_honkaigi_20211004':
        (16543, '83e3000441c2511ad07d5b404c62327b8f2a9b0661aa02c9f09cffa479e915bb'),
    'jp_kantei_suga_cabinet_resignation_20211004':
        (8608, '38e701ed3a37c472c20c34f60ee7288e42259dbc076facae8853c82563487574'),
    'jp_kantei_suga_resignation_statement_20211004':
        (10268, 'c9257e57563cb907381db3e2fd1e2fcdc3791f99db0baec4bcad5282bf5a2b34'),
    'jp_kanpo_gogai_toku83_20211004_p2':
        (297975, '695e627611fdbb60e341c6545402ba9f8531988ed62bbd0abf703084700cde02'),
    'jp_kantei_cabinet_minutes_20211004_resignation':
        (353635, '8b347a6b6367d21eafbbaf70df21724126da9e01a1e15042f4ef292901d7b2af'),
    'jp_kantei_kishida_designation_20211004':
        (10588, '5170affd0037a1ef52ce34b988df415a12d3610bdb3324970bc886eed9405960'),
    'jp_kantei_kishida_cabinet_launch_20211004':
        (12716, '46ee6cbe65f157fc2b65068df6288211d244e07021e38fa8d9e35d97f1d41a9d'),
    'jp_kantei_pm_statement_20211004':
        (6738, '80e4198136da0657552c4dcb016f200613260dc5020f72ccf9fb90a3a94de009'),
    'jp_kunaicho_schedule_entry_20211004':
        (7706, 'ff7246283679f28909b8510190f46a036bc21ad405a5e85340bb222f4db946e2'),
    'jp_kanpo_gogai_toku83_20211004':
        (250230, '324e48c8d0bd2bbf545208215135f631be8cf8a056d8b19158e471e0017aacde'),
    'jp_kantei_cabinet_minutes_20211004_first':
        (413376, 'c4bcebc795690ebb25391cc10834d73d0ac11aaed6951519d0593a933b681e9e'),
    'jp_sangiin_giun_20211110':
        (30637, 'dfbb3b21e9b56c85da1bd18af0d33fa3b127f1b39f8b50933afa9faa68c73572'),
    'jp_shugiin_honkaigi_20211110':
        (67903, '22474ba4b073f2ee55f93e28d4cb1a9bbaf4759617ee8bd1b78893c5e52e2441'),
    'jp_sangiin_honkaigi_20211110':
        (19518, 'f0cb15dc80af88851e0619f35781b28c2674ef42a0e68aa25da9a2c02d7dfb30'),
    'jp_kantei_kishida_designation_20211110':
        (10792, '68f88b36553809b75a6e87b02d4225919aae4a2d58978b3225d4054030274dc3'),
    'jp_kantei_kishida_cabinet_launch_20211110':
        (11264, 'a4290d12462525df0c61533d27a3de8792c2a54dfa6d5106eff2109868d6fc89'),
    'jp_kantei_pm_statement_20211110':
        (7407, 'f7de9e3b30855ae36e6af2b788c09b4a25077ac009d095387de1d28c8bbb196b'),
    'jp_kunaicho_schedule_entry_20211110':
        (7707, '712dd8a13e782bf1eedc696e9f8d458027f8763ee5950c36d3bc5209cdbdf4ab'),
    'jp_kanpo_gogai_toku88_20211110':
        (257370, 'a8aa962c9f3d372bd7aaaa976fee53b8da9f9f1710a6b312201754588bb5562e'),
    'jp_kanpo_gogai_toku88_20211110_p2':
        (306712, '4185c6b54abb3424afde6127ffa115b03236352df9619a958fee2b5916d8c328'),
    'jp_kantei_cabinet_minutes_20211110_resignation':
        (242978, '0370d1ea83556f913e5016727a37d5c42510c809aedd3719af79ebfbc9b91159'),
    'jp_kantei_cabinet_minutes_20211110_first':
        (367143, '693d6a5b5937f194d064aed8b8879520c7b8ee44047fdfdf62f148f0e166234f'),
    'jp_shugiin_giun_20241001':
        (51168, '26257185680262c0982c3ac7912fc7862627942bcfe876f32f1b8420b760b470'),
    'jp_sangiin_giun_20241001':
        (40603, '632ff7359336965fdf51e24421d4d3c88f9d03f85c98065703fb1d139fb95039'),
    'jp_shugiin_honkaigi_20241001':
        (34514, '438c99bebd796b58eea8fb35b50f45a6977aa206b48832c4fc56e69be03ccc98'),
    'jp_sangiin_honkaigi_20241001':
        (39904, 'da887504827592590b19878eec5dba0513b94ebfb22c3b92c095bb5d24add626'),
    'jp_kantei_kishida_cabinet_resignation_20241001':
        (12961, '217ddea14d6a5541dc5c0858879622b50acaad6873ed0b910bce63f301ac58ee'),
    'jp_kantei_kishida_resignation_statement_20241001':
        (12639, 'fc1b3ff22ecd0013299413916fe54660edd8d0652f37f9dbd21763707458082e'),
    'jp_kanpo_gogai_toku45_20241001':
        (517482, 'e4db13ba6a8373f42bb5ee2756721dd1f1e0956131b4fd1708efd6256a6dddf1'),
    'jp_kantei_cabinet_minutes_20241001_resignation':
        (165404, '48cb136c55514122f50add8cf56ef6000c8cf262c1cce65f0832e6b73bb7c761'),
    'jp_kantei_ishiba_designation_20241001':
        (13950, 'c92bf4001e52263c8f2ada7e15bf2bd40132668e7c480666bc7a8b8fb2225e2f'),
    'jp_kantei_ishiba_cabinet_launch_20241001':
        (16982, 'c37bc56180e6903d621915fbe8f3e18620324c2cf5da30ee6fa6a7296f3e5355'),
    'jp_kantei_pm_statement_20241001':
        (8548, '48e63a19535468d12f39714e850f07e5ec14d2794fe4fbf369322c8b920d3a3a'),
    'jp_kantei_ishiba_press_conference_20241001':
        (53768, 'edeed5f7bc70e16d7af6a624d1184f1c7293ec93650bf3374712160049d3aec5'),
    'jp_kunaicho_schedule_entry_20241001':
        (7847, 'ac9e70484413d6cd62582052e29f7eafe7ec1c628b7596948a099baf9976d9ad'),
    'jp_kantei_cabinet_minutes_20241001_first':
        (285754, '65c4a182b738ba9810f3dbacf11a264394cbe78511de3810d2393182152a65c4'),
    'jp_sangiin_giun_20241111':
        (32129, '6064ebf5cfc480dd487918995070bbc926c0c9686e8019574c3b173b362a9df5'),
    'jp_shugiin_honkaigi_20241111':
        (86948, 'acb7c5e8007e04105bed559b9b714dc17b5b3d53970dc45a09f9a62c62ee2cd9'),
    'jp_sangiin_honkaigi_20241111':
        (37390, '0b4d7a395ab25664579fc4183bca7d51a38ffc13f7e3f473e9e1a581af1cf214'),
    'jp_kantei_ishiba_designation_20241111':
        (14102, '9cf429657d3b092a7acd6470e94735f486266283544cd6d3248183ad785306ce'),
    'jp_kantei_ishiba_cabinet_launch_20241111':
        (15416, '9b586e745e9d8c542331abdd0de543ea0241c653ce6771ee146ac8edcb88eb25'),
    'jp_kantei_ishiba_press_conference_20241111':
        (51315, 'e1037a8013f9b8558dba1ecd387afc30c43c235740efa87f035b1ae712d8d254'),
    'jp_kunaicho_schedule_202411':
        (78199, '79e8e33ffa8cda2cc445f6454607bd7e4666495eb067fb2ab6bd2c4de19a686a'),
    'jp_kanpo_gogai_toku52_20241111':
        (519828, '76bd3b3d1901e6549ea66e8d59f488350f19fb675ae000647d4c7024ed9b4beb'),
    'jp_kantei_cabinet_minutes_20241111_resignation':
        (190589, '812690497cc131cd0d0e2c8e7e26f9c195e95fc683007816f59fcacb0d1736fe'),
    'jp_kantei_cabinet_minutes_20241111_first':
        (268401, '0777e5fcc9e4199aad1c59193ab232b7414a7e010ebe1ca644b13ae6b6b072dc'),
    'jp_kantei_pm_statement_20241111':
        (7961, '6ae0b7e2085c7688d3c41377ac72f8f0fd714f2f443744ce8d88b39245cadd0b'),
    'jp_shugiin_giun_20251021':
        (36580, 'c5175bb0d3c34ff25a03a566bb2639e304c722f358d9486f8dc05a08a2f95f57'),
    'jp_sangiin_giun_20251021':
        (39246, '851d9f253aacaa3481c77e50040a25e57ee1c62f7ae3b1c10bb250a0c61c2cb4'),
    'jp_shugiin_honkaigi_20251021':
        (31062, '1dfcf5da41e80f9d6212d00d42a42ce26cb81c4b73f91f6e3414aeccf5d40853'),
    'jp_sangiin_honkaigi_20251021':
        (36653, 'eed5fdf37c3eaf3dd1570b293c21f9f274365746c1b3d70ba4e4563f9f80609d'),
    'jp_kantei_ishiba_cabinet_resignation_20251021':
        (14605, '120a9720c4f3cdc8ece877915e5e993238c7516a163f59c4a41a48c00c025548'),
    'jp_kantei_ishiba_resignation_statement_20251021':
        (15801, 'ca4bd3cdd8c48a5e956a5cbb73921662b3ede510c9d0662f4c9c685ce7958a02'),
    'jp_kantei_takaichi_designation_20251021':
        (14440, 'eb85c976ef63710f2e547e04af67c8e7c4e9d4e77daf1e8bda8d66eb1eba19a6'),
    'jp_kantei_takaichi_cabinet_launch_20251021':
        (16509, '0750e51b6a8c66817397ba0d3ae0fe329f7f2d150ce29e52b97d9ce3b3fd5f92'),
    'jp_kantei_pm_statement_20251021':
        (8019, '64005495794b70d5efc6bad6259c01a1d47b105c5c2dc1be6496dda79f9ad308'),
    'jp_kantei_takaichi_press_conference_20251021':
        (51266, 'c48bfa07bd96f1fa7cc0493790e2d59216eb405e3cefc054394e7bc6814e91c3'),
    'jp_kunaicho_schedule_entry_20251021':
        (7848, '29a52012284a5e295eabca105a9c4828219f6bf208216ecbf5209990d8aca51f'),
    'jp_kanpo_gogai_toku28_20251021':
        (231567, '6f72a1d1ba17c1d18e455e0a076261e320bf9d9578d39fbbcc04841d27b7ae85'),
    'jp_kantei_cabinet_minutes_20251021_resignation':
        (288323, 'f4e6176c8d08f2c4125da24318195937f65b856065dd2e1c20e3d0b15bffc8b8'),
    'jp_kantei_cabinet_minutes_20251021_first':
        (282368, '2875316d6027501b542ad57058c01e056659f0098e3bde0d89ed4b982cead753'),
    'jp_sangiin_giun_20260218':
        (26539, '9656ba58ed9dcb81967e5518d9fcf624a2367a944704ccf257178f8f0d575604'),
    'jp_shugiin_honkaigi_20260218':
        (67561, '87f77bebda5283dedd23a74d9063edd93581f5fcf686a393c6969e2f565c2d75'),
    'jp_sangiin_honkaigi_20260218':
        (24218, '1ba084282bff73980203ec511c188ff99a485d4c8f3a7b186821e8e97c168abf'),
    'jp_kantei_takaichi_designation_20260218':
        (15150, '4d10fa6a7e8bb11cc35a5046d65204c288ba1d113fa200732134bb5987662707'),
    'jp_kantei_takaichi_cabinet_launch_20260218':
        (15491, 'b790e6790c098c90a90a48436ccb67acf29b9bda762113f4e2c38b70b9acde87'),
    'jp_kantei_pm_statement_20260218':
        (8363, '544e03d2b56fcd77ace2206dce67b2efded68d87165ec8582025f3cab351fbb1'),
    'jp_kantei_takaichi_press_conference_20260218':
        (41640, '7396028759bb4095237c29f690dff218ff1280b32d6ea7a161100afcbdca82dc'),
    'jp_kunaicho_schedule_entry_20260218':
        (7860, '7f22216f8a541f2f793c6122abd5a43e0c1824bc9c94fc7b54751acc8ecd32ce'),
    'jp_kanpo_gogai_toku9_20260218':
        (224239, 'a25c3b3ad59709dd614afb8d8c64d3f9c9dcdea835f03c2ca45aa044ae91e3ec'),
    'jp_kantei_cabinet_minutes_20260218_resignation':
        (117991, '2470987c925cf41f1fa6d32bb604ef36aafeaf92e553df32b5e631a3a4406b64'),
    'jp_kantei_cabinet_minutes_20260218_first':
        (206407, '34ccbadc58f59579cb62c94ae0e67dc6144d4e5d1832ea9330f2f1f794a11827'),
    'jp_shugiin_budget_committee_20260727':
        (9337, '4b8d890afb79820d7e7ee6474e5263ed48bf036a3f6cc8c7a7e4239d6347c141'),
    'jp_kantei_takaichi_recovery_hq_20260904':
        (16218, 'a1a8963358e79918e598577b8938a7da60797b9fe6cd1304cb5dddf4293e67d2'),
}
# Raw Internet Archive captures (id_ form), all made before the cutoff: source id -> capture timestamp.
ARCHIVED = {
    'jp_kantei_abe_danwa_named_20060926': '20061004122720',
    'jp_kantei_abe_hossoku_named_20060926': '20061004144947',
    'jp_kunaicho_photo_abe_named_20060926': '20061010232654',
    'jp_kantei_abe_kaiken_20070912': '20070914174513',
    'jp_kantei_ccs_press_20070913_am': '20071120030242',
    'jp_kantei_abe_kaiken_20070924': '20071013100620',
    'jp_kantei_abe_sojishoku_danwa_20070925': '20071013100625',
    'jp_kantei_abe_jisyoku_20070925': '20071119201651',
    'jp_kantei_ccs_press_20070925_am': '20071120030240',
    'jp_kantei_rekidai_090': '20260610214217',
    'jp_kantei_fukuda_kaiken_20070925': '20071011030824',
    'jp_kantei_ccs_press_20070925_pm': '20071120030246',
    'jp_kantei_fukuda_hossoku_20070926': '20071013000202',
    'jp_kantei_fukuda_danwa_20070926': '20071012042407',
    'jp_kunaicho_schedule_2007_h2': '20071026140203',
    'jp_kunaicho_photo_20070926': '20110322234659',
    'jp_kantei_ccs_press_20070926_am': '20071120030220',
    'jp_kantei_fukuda_kaiken_20080901': '20080905052903',
    'jp_kantei_fukuda_sojishoku_danwa_20080924': '20080926193417',
    'jp_kantei_fukuda_jisyoku_20080924': '20080927032752',
    'jp_kantei_ccs_press_20080924_am': '20081013183954',
    'jp_kantei_rekidai_091': '20260419141132',
    'jp_kantei_aso_hossoku_20080924': '20080927032720',
    'jp_kunaicho_schedule_2008_h2': '20081014021828',
    'jp_kunaicho_photo_20080924': '20081013083445',
    'jp_kantei_ccs_press_20080925': '20081013183956',
    'jp_kantei_aso_kaiken_20090916': '20091005013745',
    'jp_kantei_aso_jisyoku_20090916': '20090924025406',
    'jp_kantei_rekidai_092': '20260604054954',
    'jp_kantei_hatoyama_hossoku_20090916': '20090923134419',
    'jp_kunaicho_schedule_2009_h2': '20100110015156',
    'jp_kunaicho_photo_20090916': '20110322230830',
    'jp_kantei_ccs_press_20090916_pm': '20090925024706',
    'jp_kantei_ccs_press_20100602_am': '20100614153714',
    'jp_kantei_ccs_press_20100602_pm': '20100614153804',
    'jp_kantei_hatoyama_hatsugen_20100604': '20100614171223',
    'jp_kantei_hatoyama_statement_index_201006': '20100614153845',
    'jp_kantei_hatoyama_sojishoku_danwa_20100604': '20100607041925',
    'jp_kantei_hatoyama_soujisyoku_20100604': '20100607041921',
    'jp_kantei_ccs_press_20100604_am': '20100607041947',
    'jp_kantei_rekidai_093': '20251010102827',
    'jp_kantei_kan_shimei_20100604': '20100611012819',
    'jp_kantei_kan_kaiken_20100608': '20100614152822',
    'jp_kantei_kan_hossoku_20100608': '20100614153156',
    'jp_kantei_ccs_press_20100608_pm': '20100614153607',
    'jp_kantei_kan_kaiken_20110826': '20110901042107',
    'jp_kantei_kan_sojishoku_danwa_20110830': '20110901060817',
    'jp_kantei_kan_bousai_20110901': '20110908015959',
    'jp_kantei_kan_sojishoku_20110902': '20110908015954',
    'jp_kantei_rekidai_094': '20250725121907',
    'jp_kantei_noda_hossoku_20110902': '20110908015957',
    'jp_kantei_noda_kaiken_20110902': '20110908020055',
    'jp_kantei_noda_siji_20110902': '20120611203452',
    'jp_kantei_noda_sojishoku_20121226': '20130102013110',
    'jp_kantei_noda_sojishoku_danwa_20121226': '20130104150240',
    'jp_kantei_rekidai_095': '20260608195943',
    'jp_kantei_abe_danwa_20121226': '20130105152056',
    'jp_kantei_abe_hossoku_20121226': '20121231035515',
    'jp_kantei_abe_kaiken_20121226': '20121231214801',
    'jp_kantei_abe_designation_20121226': '20121230182813',
    'jp_kantei_abe_hossoku_20141224': '20150324002621',
    'jp_kantei_abe_kaiken_20141224': '20150202040924',
    'jp_kantei_abe_designation_20141224': '20150111000202',
    'jp_kantei_abe_hossoku_20171101': '20200829020441',
    'jp_kantei_abe_kaiken_20171101': '20171102081509',
    'jp_kantei_abe_designation_20171101': '20190717030204',
    'jp_kantei_abe_kaiken_20200828': '20200829033901',
    'jp_kantei_abe_sojishoku_20200916': '20200930143141',
    'jp_kantei_abe_sojishoku_danwa_20200916': '20200930160518',
    'jp_kanpo_gogai_toku99_20200916_p2': '20200917033144',
    'jp_kantei_rekidai_096': '20260421195324',
    'jp_kantei_rekidai_097': '20260419052815',
    'jp_kantei_rekidai_098': '20260513214818',
    'jp_kantei_suga_designation_20200916': '20200930151734',
    'jp_kantei_suga_cabinet_launch_20200916': '20200918031818',
    'jp_kantei_pm_statement_20200916': '20200930145703',
    'jp_kantei_suga_press_conference_20200916': '20200917000152',
    'jp_kantei_cabinet_minutes_20200916_first': '20240815232655',
    'jp_kantei_suga_cabinet_resignation_20211004': '20211004051814',
    'jp_kantei_suga_resignation_statement_20211004': '20211004044700',
    'jp_kanpo_gogai_toku83_20211004_p2': '20211005000918',
    'jp_kantei_cabinet_minutes_20211004_resignation': '20220519091808',
    'jp_kantei_kishida_designation_20211004': '20211004124431',
    'jp_kantei_kishida_cabinet_launch_20211004': '20211004200549',
    'jp_kantei_pm_statement_20211004': '20211004194353',
    'jp_kanpo_gogai_toku83_20211004': '20211004145916',
    'jp_kantei_cabinet_minutes_20211004_first': '20220519080647',
    'jp_kantei_kishida_designation_20211110': '20211110095724',
    'jp_kantei_kishida_cabinet_launch_20211110': '20211110185042',
    'jp_kantei_pm_statement_20211110': '20211110183129',
    'jp_kanpo_gogai_toku88_20211110': '20211111030850',
    'jp_kanpo_gogai_toku88_20211110_p2': '20211111141235',
    'jp_kantei_cabinet_minutes_20211110_resignation': '20220519083624',
    'jp_kantei_cabinet_minutes_20211110_first': '20220519081346',
    'jp_kantei_kishida_cabinet_resignation_20241001': '20241001062143',
    'jp_kantei_kishida_resignation_statement_20241001': '20241001035200',
    'jp_kanpo_gogai_toku45_20241001': '20241001155622',
    'jp_kantei_ishiba_designation_20241001': '20241001101146',
    'jp_kantei_ishiba_cabinet_launch_20241001': '20241001190358',
    'jp_kantei_pm_statement_20241001': '20241001190718',
    'jp_kantei_ishiba_designation_20241111': '20241111131209',
    'jp_kantei_ishiba_cabinet_launch_20241111': '20241111215410',
    'jp_kanpo_gogai_toku52_20241111': '20241111144126',
    'jp_kantei_ishiba_cabinet_resignation_20251021': '20251021060531',
    'jp_kantei_ishiba_resignation_statement_20251021': '20251021012053',
    'jp_kantei_takaichi_designation_20251021': '20251021074816',
    'jp_kantei_takaichi_cabinet_launch_20251021': '20251021202324',
    'jp_kantei_pm_statement_20251021': '20251021202317',
    'jp_kanpo_gogai_toku28_20251021': '20251021150239',
    'jp_kantei_takaichi_designation_20260218': '20260218093115',
    'jp_kantei_takaichi_cabinet_launch_20260218': '20260218195047',
    'jp_kantei_pm_statement_20260218': '20260218162146',
    'jp_kanpo_gogai_toku9_20260218': '20260218143823',
}
# Stored official pages and PDFs on the Kantei's and the Imperial Household Agency's own hosts, each with a Last-Modified
# before the cutoff and a matching cache-busting download; no other live page of those hosts is a source.
LIVE_OFFICIAL = [
    'jp_kunaicho_schedule_2010_q2',
    'jp_kunaicho_photo_20100608',
    'jp_kunaicho_schedule_2011_q3',
    'jp_kunaicho_photo_20110902',
    'jp_kunaicho_schedule_2012_q4',
    'jp_kunaicho_photo_20121226',
    'jp_kunaicho_schedule_2014_q4',
    'jp_kunaicho_photo_20141224',
    'jp_kunaicho_schedule_2017_q4',
    'jp_kunaicho_photo_20171101',
    'jp_kunaicho_schedule_entry_20200916',
    'jp_kunaicho_schedule_entry_20211004',
    'jp_kunaicho_schedule_entry_20211110',
    'jp_kantei_cabinet_minutes_20241001_resignation',
    'jp_kantei_ishiba_press_conference_20241001',
    'jp_kunaicho_schedule_entry_20241001',
    'jp_kantei_cabinet_minutes_20241001_first',
    'jp_kantei_ishiba_press_conference_20241111',
    'jp_kunaicho_schedule_202411',
    'jp_kantei_cabinet_minutes_20241111_resignation',
    'jp_kantei_cabinet_minutes_20241111_first',
    'jp_kantei_pm_statement_20241111',
    'jp_kantei_takaichi_press_conference_20251021',
    'jp_kunaicho_schedule_entry_20251021',
    'jp_kantei_cabinet_minutes_20251021_resignation',
    'jp_kantei_cabinet_minutes_20251021_first',
    'jp_kantei_takaichi_press_conference_20260218',
    'jp_kunaicho_schedule_entry_20260218',
    'jp_kantei_cabinet_minutes_20260218_resignation',
    'jp_kantei_cabinet_minutes_20260218_first',
    'jp_kantei_takaichi_recovery_hq_20260904',
]
# PDF pages viewed (rendered, or read from the text layer) for each PDF source.
PDF_PAGES = {
    'jp_kantei_hatoyama_hatsugen_20100604': [1, 2, 3],
    'jp_kanpo_gogai_toku99_20200916_p2': [1],
    'jp_kantei_cabinet_minutes_20200916_first': [1, 2],
    'jp_kanpo_gogai_toku83_20211004_p2': [1],
    'jp_kantei_cabinet_minutes_20211004_resignation': [1, 2, 3, 5],
    'jp_kanpo_gogai_toku83_20211004': [1],
    'jp_kantei_cabinet_minutes_20211004_first': [1, 2],
    'jp_kanpo_gogai_toku88_20211110': [1],
    'jp_kanpo_gogai_toku88_20211110_p2': [1],
    'jp_kantei_cabinet_minutes_20211110_resignation': [1, 2, 3],
    'jp_kantei_cabinet_minutes_20211110_first': [1, 2],
    'jp_kanpo_gogai_toku45_20241001': [1, 2],
    'jp_kantei_cabinet_minutes_20241001_resignation': [1, 2, 3],
    'jp_kantei_cabinet_minutes_20241001_first': [1, 2],
    'jp_kanpo_gogai_toku52_20241111': [1, 2],
    'jp_kantei_cabinet_minutes_20241111_resignation': [1, 2, 3],
    'jp_kantei_cabinet_minutes_20241111_first': [1, 2],
    'jp_kanpo_gogai_toku28_20251021': [1, 2],
    'jp_kantei_cabinet_minutes_20251021_resignation': [1, 2, 4, 5],
    'jp_kantei_cabinet_minutes_20251021_first': [1, 2],
    'jp_kanpo_gogai_toku9_20260218': [1, 2],
    'jp_kantei_cabinet_minutes_20260218_resignation': [1, 2],
    'jp_kantei_cabinet_minutes_20260218_first': [1, 2],
}
ACCESSED = {
    'jp_kantei_abe_danwa_named_20060926': '2026-09-24',
    'jp_kantei_abe_hossoku_named_20060926': '2026-09-24',
    'jp_kunaicho_photo_abe_named_20060926': '2026-09-24',
    'jp_kantei_abe_kaiken_20070912': '2026-09-24',
    'jp_kantei_ccs_press_20070913_am': '2026-09-24',
    'jp_kantei_abe_kaiken_20070924': '2026-09-24',
    'jp_kantei_abe_sojishoku_danwa_20070925': '2026-09-24',
    'jp_kantei_abe_jisyoku_20070925': '2026-09-24',
    'jp_shugiin_giun_20070925': '2026-09-24',
    'jp_sangiin_giun_20070925': '2026-09-24',
    'jp_shugiin_honkaigi_20070925': '2026-09-24',
    'jp_sangiin_honkaigi_20070925': '2026-09-24',
    'jp_ryoin_kyogikai_20070925': '2026-09-24',
    'jp_kantei_ccs_press_20070925_am': '2026-09-24',
    'jp_kantei_rekidai_090': '2026-09-24',
    'jp_kantei_fukuda_kaiken_20070925': '2026-09-24',
    'jp_kantei_ccs_press_20070925_pm': '2026-09-24',
    'jp_kantei_fukuda_hossoku_20070926': '2026-09-24',
    'jp_kantei_fukuda_danwa_20070926': '2026-09-24',
    'jp_kunaicho_schedule_2007_h2': '2026-09-24',
    'jp_kunaicho_photo_20070926': '2026-09-24',
    'jp_kantei_ccs_press_20070926_am': '2026-09-24',
    'jp_kantei_fukuda_kaiken_20080901': '2026-09-24',
    'jp_kantei_fukuda_sojishoku_danwa_20080924': '2026-09-24',
    'jp_kantei_fukuda_jisyoku_20080924': '2026-09-24',
    'jp_shugiin_giun_20080924': '2026-09-24',
    'jp_sangiin_giun_20080924': '2026-09-24',
    'jp_shugiin_honkaigi_20080924': '2026-09-24',
    'jp_sangiin_honkaigi_20080924': '2026-09-24',
    'jp_kantei_ccs_press_20080924_am': '2026-09-24',
    'jp_kantei_rekidai_091': '2026-09-24',
    'jp_ryoin_kyogikai_20080924': '2026-09-24',
    'jp_kantei_aso_hossoku_20080924': '2026-09-24',
    'jp_kunaicho_schedule_2008_h2': '2026-09-24',
    'jp_kunaicho_photo_20080924': '2026-09-24',
    'jp_kantei_ccs_press_20080925': '2026-09-24',
    'jp_kantei_aso_kaiken_20090916': '2026-09-24',
    'jp_kantei_aso_jisyoku_20090916': '2026-09-24',
    'jp_sangiin_giun_20090916': '2026-09-24',
    'jp_kantei_rekidai_092': '2026-09-24',
    'jp_shugiin_honkaigi_20090916': '2026-09-24',
    'jp_sangiin_honkaigi_20090916': '2026-09-24',
    'jp_kantei_hatoyama_hossoku_20090916': '2026-09-24',
    'jp_kunaicho_schedule_2009_h2': '2026-09-24',
    'jp_kunaicho_photo_20090916': '2026-09-24',
    'jp_kantei_ccs_press_20090916_pm': '2026-09-24',
    'jp_kantei_ccs_press_20100602_am': '2026-09-24',
    'jp_kantei_ccs_press_20100602_pm': '2026-09-24',
    'jp_kantei_hatoyama_hatsugen_20100604': '2026-09-24',
    'jp_kantei_hatoyama_statement_index_201006': '2026-09-24',
    'jp_kantei_hatoyama_sojishoku_danwa_20100604': '2026-09-24',
    'jp_kantei_hatoyama_soujisyoku_20100604': '2026-09-24',
    'jp_shugiin_giun_20100604': '2026-09-24',
    'jp_sangiin_giun_20100604': '2026-09-24',
    'jp_shugiin_honkaigi_20100604': '2026-09-24',
    'jp_sangiin_honkaigi_20100604': '2026-09-24',
    'jp_kantei_ccs_press_20100604_am': '2026-09-24',
    'jp_sangiin_honkaigi_20100615_s004': '2026-09-24',
    'jp_sangiin_honkaigi_20100615_s005': '2026-09-24',
    'jp_kantei_rekidai_093': '2026-09-24',
    'jp_kantei_kan_shimei_20100604': '2026-09-24',
    'jp_kunaicho_schedule_2010_q2': '2026-09-24',
    'jp_kunaicho_photo_20100608': '2026-09-24',
    'jp_kantei_kan_kaiken_20100608': '2026-09-24',
    'jp_kantei_kan_hossoku_20100608': '2026-09-24',
    'jp_kantei_ccs_press_20100608_pm': '2026-09-24',
    'jp_shugiin_honkaigi_kan_speech_20100611': '2026-09-24',
    'jp_kantei_kan_kaiken_20110826': '2026-09-24',
    'jp_shugiin_giun_20110830': '2026-09-24',
    'jp_shugiin_honkaigi_20110830': '2026-09-24',
    'jp_sangiin_giun_20110830': '2026-09-24',
    'jp_sangiin_honkaigi_20110830': '2026-09-24',
    'jp_kantei_kan_sojishoku_danwa_20110830': '2026-09-24',
    'jp_kantei_kan_bousai_20110901': '2026-09-24',
    'jp_kantei_kan_sojishoku_20110902': '2026-09-24',
    'jp_sangiin_yosan_20110928_s112': '2026-09-24',
    'jp_sangiin_yosan_20110928_s113': '2026-09-24',
    'jp_sangiin_yosan_20110928_s115': '2026-09-24',
    'jp_kantei_rekidai_094': '2026-09-25',
    'jp_kunaicho_schedule_2011_q3': '2026-09-24',
    'jp_kunaicho_photo_20110902': '2026-09-24',
    'jp_kantei_noda_hossoku_20110902': '2026-09-24',
    'jp_kantei_noda_kaiken_20110902': '2026-09-24',
    'jp_kantei_noda_siji_20110902': '2026-09-25',
    'jp_shugiin_honkaigi_noda_speech_20110913': '2026-09-24',
    'jp_sangiin_yosan_20110928_s119': '2026-09-25',
    'jp_sangiin_giun_20121226': '2026-09-24',
    'jp_kantei_noda_sojishoku_20121226': '2026-09-24',
    'jp_kantei_noda_sojishoku_danwa_20121226': '2026-09-25',
    'jp_kantei_rekidai_095': '2026-09-25',
    'jp_shugiin_honkaigi_20121226': '2026-09-24',
    'jp_sangiin_honkaigi_20121226': '2026-09-24',
    'jp_kunaicho_schedule_2012_q4': '2026-09-24',
    'jp_kunaicho_photo_20121226': '2026-09-24',
    'jp_kantei_abe_danwa_20121226': '2026-09-24',
    'jp_kantei_abe_hossoku_20121226': '2026-09-24',
    'jp_kantei_abe_kaiken_20121226': '2026-09-24',
    'jp_kantei_abe_designation_20121226': '2026-09-25',
    'jp_shugiin_honkaigi_abe_speech_20130128': '2026-09-24',
    'jp_sangiin_giun_20141224': '2026-09-24',
    'jp_shugiin_honkaigi_20141224': '2026-09-24',
    'jp_sangiin_honkaigi_20141224': '2026-09-24',
    'jp_kunaicho_schedule_2014_q4': '2026-09-24',
    'jp_kunaicho_photo_20141224': '2026-09-24',
    'jp_kantei_abe_hossoku_20141224': '2026-09-24',
    'jp_kantei_abe_kaiken_20141224': '2026-09-24',
    'jp_kantei_abe_designation_20141224': '2026-09-25',
    'jp_shugiin_honkaigi_abe_speech_20150212': '2026-09-24',
    'jp_sangiin_giun_20171101': '2026-09-24',
    'jp_shugiin_honkaigi_20171101': '2026-09-24',
    'jp_sangiin_honkaigi_20171101': '2026-09-24',
    'jp_kunaicho_schedule_2017_q4': '2026-09-24',
    'jp_kunaicho_photo_20171101': '2026-09-24',
    'jp_kantei_abe_hossoku_20171101': '2026-09-24',
    'jp_kantei_abe_kaiken_20171101': '2026-09-24',
    'jp_kantei_abe_designation_20171101': '2026-09-25',
    'jp_shugiin_honkaigi_abe_speech_20171117': '2026-09-24',
    'jp_kantei_abe_kaiken_20200828': '2026-09-24',
    'jp_shugiin_giun_20200916': '2026-09-24',
    'jp_shugiin_honkaigi_20200916': '2026-09-24',
    'jp_sangiin_giun_20200916': '2026-09-24',
    'jp_sangiin_honkaigi_20200916': '2026-09-24',
    'jp_kantei_abe_sojishoku_20200916': '2026-09-24',
    'jp_kantei_abe_sojishoku_danwa_20200916': '2026-09-25',
    'jp_kanpo_gogai_toku99_20200916_p2': '2026-09-25',
    'jp_kantei_rekidai_096': '2026-09-25',
    'jp_kantei_rekidai_097': '2026-09-25',
    'jp_kantei_rekidai_098': '2026-09-25',
    'jp_kantei_suga_designation_20200916': '2026-09-24',
    'jp_kantei_suga_cabinet_launch_20200916': '2026-09-24',
    'jp_kantei_pm_statement_20200916': '2026-09-24',
    'jp_kantei_suga_press_conference_20200916': '2026-09-24',
    'jp_kunaicho_schedule_entry_20200916': '2026-09-24',
    'jp_kantei_cabinet_minutes_20200916_first': '2026-09-25',
    'jp_shugiin_giun_20211004': '2026-09-24',
    'jp_sangiin_giun_20211004': '2026-09-24',
    'jp_shugiin_honkaigi_20211004': '2026-09-24',
    'jp_sangiin_honkaigi_20211004': '2026-09-24',
    'jp_kantei_suga_cabinet_resignation_20211004': '2026-09-24',
    'jp_kantei_suga_resignation_statement_20211004': '2026-09-24',
    'jp_kanpo_gogai_toku83_20211004_p2': '2026-09-25',
    'jp_kantei_cabinet_minutes_20211004_resignation': '2026-09-25',
    'jp_kantei_kishida_designation_20211004': '2026-09-24',
    'jp_kantei_kishida_cabinet_launch_20211004': '2026-09-24',
    'jp_kantei_pm_statement_20211004': '2026-09-24',
    'jp_kunaicho_schedule_entry_20211004': '2026-09-24',
    'jp_kanpo_gogai_toku83_20211004': '2026-09-24',
    'jp_kantei_cabinet_minutes_20211004_first': '2026-09-25',
    'jp_sangiin_giun_20211110': '2026-09-24',
    'jp_shugiin_honkaigi_20211110': '2026-09-24',
    'jp_sangiin_honkaigi_20211110': '2026-09-24',
    'jp_kantei_kishida_designation_20211110': '2026-09-24',
    'jp_kantei_kishida_cabinet_launch_20211110': '2026-09-24',
    'jp_kantei_pm_statement_20211110': '2026-09-24',
    'jp_kunaicho_schedule_entry_20211110': '2026-09-24',
    'jp_kanpo_gogai_toku88_20211110': '2026-09-24',
    'jp_kanpo_gogai_toku88_20211110_p2': '2026-09-25',
    'jp_kantei_cabinet_minutes_20211110_resignation': '2026-09-25',
    'jp_kantei_cabinet_minutes_20211110_first': '2026-09-25',
    'jp_shugiin_giun_20241001': '2026-09-24',
    'jp_sangiin_giun_20241001': '2026-09-24',
    'jp_shugiin_honkaigi_20241001': '2026-09-24',
    'jp_sangiin_honkaigi_20241001': '2026-09-24',
    'jp_kantei_kishida_cabinet_resignation_20241001': '2026-09-24',
    'jp_kantei_kishida_resignation_statement_20241001': '2026-09-24',
    'jp_kanpo_gogai_toku45_20241001': '2026-09-24',
    'jp_kantei_cabinet_minutes_20241001_resignation': '2026-09-25',
    'jp_kantei_ishiba_designation_20241001': '2026-09-24',
    'jp_kantei_ishiba_cabinet_launch_20241001': '2026-09-24',
    'jp_kantei_pm_statement_20241001': '2026-09-24',
    'jp_kantei_ishiba_press_conference_20241001': '2026-09-24',
    'jp_kunaicho_schedule_entry_20241001': '2026-09-24',
    'jp_kantei_cabinet_minutes_20241001_first': '2026-09-25',
    'jp_sangiin_giun_20241111': '2026-09-24',
    'jp_shugiin_honkaigi_20241111': '2026-09-24',
    'jp_sangiin_honkaigi_20241111': '2026-09-24',
    'jp_kantei_ishiba_designation_20241111': '2026-09-24',
    'jp_kantei_ishiba_cabinet_launch_20241111': '2026-09-24',
    'jp_kantei_ishiba_press_conference_20241111': '2026-09-24',
    'jp_kunaicho_schedule_202411': '2026-09-24',
    'jp_kanpo_gogai_toku52_20241111': '2026-09-24',
    'jp_kantei_cabinet_minutes_20241111_resignation': '2026-09-25',
    'jp_kantei_cabinet_minutes_20241111_first': '2026-09-25',
    'jp_kantei_pm_statement_20241111': '2026-09-25',
    'jp_shugiin_giun_20251021': '2026-09-24',
    'jp_sangiin_giun_20251021': '2026-09-24',
    'jp_shugiin_honkaigi_20251021': '2026-09-24',
    'jp_sangiin_honkaigi_20251021': '2026-09-24',
    'jp_kantei_ishiba_cabinet_resignation_20251021': '2026-09-24',
    'jp_kantei_ishiba_resignation_statement_20251021': '2026-09-24',
    'jp_kantei_takaichi_designation_20251021': '2026-09-24',
    'jp_kantei_takaichi_cabinet_launch_20251021': '2026-09-24',
    'jp_kantei_pm_statement_20251021': '2026-09-24',
    'jp_kantei_takaichi_press_conference_20251021': '2026-09-24',
    'jp_kunaicho_schedule_entry_20251021': '2026-09-24',
    'jp_kanpo_gogai_toku28_20251021': '2026-09-24',
    'jp_kantei_cabinet_minutes_20251021_resignation': '2026-09-25',
    'jp_kantei_cabinet_minutes_20251021_first': '2026-09-25',
    'jp_sangiin_giun_20260218': '2026-09-24',
    'jp_shugiin_honkaigi_20260218': '2026-09-24',
    'jp_sangiin_honkaigi_20260218': '2026-09-24',
    'jp_kantei_takaichi_designation_20260218': '2026-09-24',
    'jp_kantei_takaichi_cabinet_launch_20260218': '2026-09-24',
    'jp_kantei_pm_statement_20260218': '2026-09-24',
    'jp_kantei_takaichi_press_conference_20260218': '2026-09-24',
    'jp_kunaicho_schedule_entry_20260218': '2026-09-24',
    'jp_kanpo_gogai_toku9_20260218': '2026-09-24',
    'jp_kantei_cabinet_minutes_20260218_resignation': '2026-09-25',
    'jp_kantei_cabinet_minutes_20260218_first': '2026-09-25',
    'jp_shugiin_budget_committee_20260727': '2026-09-24',
    'jp_kantei_takaichi_recovery_hq_20260904': '2026-09-24',
}
SOURCE_TYPES = {
    'jp_kantei_abe_danwa_named_20060926': 'primary_kantei_statement_archived',
    'jp_kantei_abe_hossoku_named_20060926': 'primary_kantei_account_archived',
    'jp_kunaicho_photo_abe_named_20060926': 'primary_imperial_household_photo_page_archived',
    'jp_kantei_abe_kaiken_20070912': 'primary_kantei_press_conference_archived',
    'jp_kantei_ccs_press_20070913_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_abe_kaiken_20070924': 'primary_kantei_press_conference_archived',
    'jp_kantei_abe_sojishoku_danwa_20070925': 'primary_kantei_statement_archived',
    'jp_kantei_abe_jisyoku_20070925': 'primary_kantei_account_archived',
    'jp_shugiin_giun_20070925': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20070925': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20070925': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20070925': 'primary_diet_minutes_api_json',
    'jp_ryoin_kyogikai_20070925': 'primary_diet_minutes_api_json',
    'jp_kantei_ccs_press_20070925_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_rekidai_090': 'primary_kantei_page_archived_retrospective',
    'jp_kantei_fukuda_kaiken_20070925': 'primary_kantei_press_conference_archived',
    'jp_kantei_ccs_press_20070925_pm': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_fukuda_hossoku_20070926': 'primary_kantei_account_archived',
    'jp_kantei_fukuda_danwa_20070926': 'primary_kantei_statement_archived',
    'jp_kunaicho_schedule_2007_h2': 'primary_imperial_household_schedule_archived',
    'jp_kunaicho_photo_20070926': 'primary_imperial_household_photo_page_archived',
    'jp_kantei_ccs_press_20070926_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_fukuda_kaiken_20080901': 'primary_kantei_press_conference_archived',
    'jp_kantei_fukuda_sojishoku_danwa_20080924': 'primary_kantei_statement_archived',
    'jp_kantei_fukuda_jisyoku_20080924': 'primary_kantei_account_archived',
    'jp_shugiin_giun_20080924': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20080924': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20080924': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20080924': 'primary_diet_minutes_api_json',
    'jp_kantei_ccs_press_20080924_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_rekidai_091': 'primary_kantei_page_archived_retrospective',
    'jp_ryoin_kyogikai_20080924': 'primary_diet_minutes_api_json',
    'jp_kantei_aso_hossoku_20080924': 'primary_kantei_account_archived',
    'jp_kunaicho_schedule_2008_h2': 'primary_imperial_household_schedule_archived',
    'jp_kunaicho_photo_20080924': 'primary_imperial_household_photo_page_archived',
    'jp_kantei_ccs_press_20080925': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_aso_kaiken_20090916': 'primary_kantei_press_conference_archived',
    'jp_kantei_aso_jisyoku_20090916': 'primary_kantei_account_archived',
    'jp_sangiin_giun_20090916': 'primary_diet_minutes_api_json',
    'jp_kantei_rekidai_092': 'primary_kantei_page_archived_retrospective',
    'jp_shugiin_honkaigi_20090916': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20090916': 'primary_diet_minutes_api_json',
    'jp_kantei_hatoyama_hossoku_20090916': 'primary_kantei_account_archived',
    'jp_kunaicho_schedule_2009_h2': 'primary_imperial_household_schedule_archived',
    'jp_kunaicho_photo_20090916': 'primary_imperial_household_photo_page_archived',
    'jp_kantei_ccs_press_20090916_pm': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_ccs_press_20100602_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_ccs_press_20100602_pm': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_kantei_hatoyama_hatsugen_20100604': 'primary_kantei_statement_pdf_archived',
    'jp_kantei_hatoyama_statement_index_201006': 'primary_kantei_index_archived',
    'jp_kantei_hatoyama_sojishoku_danwa_20100604': 'primary_kantei_statement_archived',
    'jp_kantei_hatoyama_soujisyoku_20100604': 'primary_kantei_account_archived',
    'jp_shugiin_giun_20100604': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20100604': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20100604': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20100604': 'primary_diet_minutes_api_json',
    'jp_kantei_ccs_press_20100604_am': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_sangiin_honkaigi_20100615_s004': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20100615_s005': 'primary_diet_minutes_api_json',
    'jp_kantei_rekidai_093': 'primary_kantei_page_archived_retrospective',
    'jp_kantei_kan_shimei_20100604': 'primary_kantei_account_archived',
    'jp_kunaicho_schedule_2010_q2': 'primary_imperial_household_schedule_official',
    'jp_kunaicho_photo_20100608': 'primary_imperial_household_photo_page_official',
    'jp_kantei_kan_kaiken_20100608': 'primary_kantei_press_conference_archived',
    'jp_kantei_kan_hossoku_20100608': 'primary_kantei_account_archived',
    'jp_kantei_ccs_press_20100608_pm': 'primary_kantei_chief_cabinet_secretary_press_archived',
    'jp_shugiin_honkaigi_kan_speech_20100611': 'primary_diet_minutes_api_json',
    'jp_kantei_kan_kaiken_20110826': 'primary_kantei_press_conference_archived',
    'jp_shugiin_giun_20110830': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20110830': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20110830': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20110830': 'primary_diet_minutes_api_json',
    'jp_kantei_kan_sojishoku_danwa_20110830': 'primary_kantei_statement_archived',
    'jp_kantei_kan_bousai_20110901': 'primary_kantei_account_archived',
    'jp_kantei_kan_sojishoku_20110902': 'primary_kantei_account_archived',
    'jp_sangiin_yosan_20110928_s112': 'primary_diet_minutes_api_json',
    'jp_sangiin_yosan_20110928_s113': 'primary_diet_minutes_api_json',
    'jp_sangiin_yosan_20110928_s115': 'primary_diet_minutes_api_json',
    'jp_kantei_rekidai_094': 'primary_kantei_page_archived_retrospective',
    'jp_kunaicho_schedule_2011_q3': 'primary_imperial_household_schedule_official',
    'jp_kunaicho_photo_20110902': 'primary_imperial_household_photo_page_official',
    'jp_kantei_noda_hossoku_20110902': 'primary_kantei_account_archived',
    'jp_kantei_noda_kaiken_20110902': 'primary_kantei_press_conference_archived',
    'jp_kantei_noda_siji_20110902': 'primary_kantei_statement_archived',
    'jp_shugiin_honkaigi_noda_speech_20110913': 'primary_diet_minutes_api_json',
    'jp_sangiin_yosan_20110928_s119': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20121226': 'primary_diet_minutes_api_json',
    'jp_kantei_noda_sojishoku_20121226': 'primary_kantei_account_archived',
    'jp_kantei_noda_sojishoku_danwa_20121226': 'primary_kantei_statement_archived',
    'jp_kantei_rekidai_095': 'primary_kantei_page_archived_retrospective',
    'jp_shugiin_honkaigi_20121226': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20121226': 'primary_diet_minutes_api_json',
    'jp_kunaicho_schedule_2012_q4': 'primary_imperial_household_schedule_official',
    'jp_kunaicho_photo_20121226': 'primary_imperial_household_photo_page_official',
    'jp_kantei_abe_danwa_20121226': 'primary_kantei_statement_archived',
    'jp_kantei_abe_hossoku_20121226': 'primary_kantei_account_archived',
    'jp_kantei_abe_kaiken_20121226': 'primary_kantei_press_conference_archived',
    'jp_kantei_abe_designation_20121226': 'primary_kantei_account_archived',
    'jp_shugiin_honkaigi_abe_speech_20130128': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20141224': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20141224': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20141224': 'primary_diet_minutes_api_json',
    'jp_kunaicho_schedule_2014_q4': 'primary_imperial_household_schedule_official',
    'jp_kunaicho_photo_20141224': 'primary_imperial_household_photo_page_official',
    'jp_kantei_abe_hossoku_20141224': 'primary_kantei_account_archived',
    'jp_kantei_abe_kaiken_20141224': 'primary_kantei_press_conference_archived',
    'jp_kantei_abe_designation_20141224': 'primary_kantei_account_archived',
    'jp_shugiin_honkaigi_abe_speech_20150212': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20171101': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20171101': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20171101': 'primary_diet_minutes_api_json',
    'jp_kunaicho_schedule_2017_q4': 'primary_imperial_household_schedule_official',
    'jp_kunaicho_photo_20171101': 'primary_imperial_household_photo_page_official',
    'jp_kantei_abe_hossoku_20171101': 'primary_kantei_account_archived',
    'jp_kantei_abe_kaiken_20171101': 'primary_kantei_press_conference_archived',
    'jp_kantei_abe_designation_20171101': 'primary_kantei_account_archived',
    'jp_shugiin_honkaigi_abe_speech_20171117': 'primary_diet_minutes_api_json',
    'jp_kantei_abe_kaiken_20200828': 'primary_kantei_press_conference_archived',
    'jp_shugiin_giun_20200916': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20200916': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20200916': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20200916': 'primary_diet_minutes_api_json',
    'jp_kantei_abe_sojishoku_20200916': 'primary_kantei_account_archived',
    'jp_kantei_abe_sojishoku_danwa_20200916': 'primary_kantei_statement_archived',
    'jp_kanpo_gogai_toku99_20200916_p2': 'primary_official_gazette_pdf_archived',
    'jp_kantei_rekidai_096': 'primary_kantei_page_archived_retrospective',
    'jp_kantei_rekidai_097': 'primary_kantei_page_archived_retrospective',
    'jp_kantei_rekidai_098': 'primary_kantei_page_archived_retrospective',
    'jp_kantei_suga_designation_20200916': 'primary_kantei_account_archived',
    'jp_kantei_suga_cabinet_launch_20200916': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20200916': 'primary_kantei_statement_archived',
    'jp_kantei_suga_press_conference_20200916': 'primary_kantei_press_conference_archived',
    'jp_kunaicho_schedule_entry_20200916': 'primary_imperial_household_schedule_official',
    'jp_kantei_cabinet_minutes_20200916_first': 'primary_cabinet_minutes_pdf_archived',
    'jp_shugiin_giun_20211004': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20211004': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20211004': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20211004': 'primary_diet_minutes_api_json',
    'jp_kantei_suga_cabinet_resignation_20211004': 'primary_kantei_account_archived',
    'jp_kantei_suga_resignation_statement_20211004': 'primary_kantei_statement_archived',
    'jp_kanpo_gogai_toku83_20211004_p2': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20211004_resignation': 'primary_cabinet_minutes_pdf_archived',
    'jp_kantei_kishida_designation_20211004': 'primary_kantei_account_archived',
    'jp_kantei_kishida_cabinet_launch_20211004': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20211004': 'primary_kantei_statement_archived',
    'jp_kunaicho_schedule_entry_20211004': 'primary_imperial_household_schedule_official',
    'jp_kanpo_gogai_toku83_20211004': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20211004_first': 'primary_cabinet_minutes_pdf_archived',
    'jp_sangiin_giun_20211110': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20211110': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20211110': 'primary_diet_minutes_api_json',
    'jp_kantei_kishida_designation_20211110': 'primary_kantei_account_archived',
    'jp_kantei_kishida_cabinet_launch_20211110': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20211110': 'primary_kantei_statement_archived',
    'jp_kunaicho_schedule_entry_20211110': 'primary_imperial_household_schedule_official',
    'jp_kanpo_gogai_toku88_20211110': 'primary_official_gazette_pdf_archived',
    'jp_kanpo_gogai_toku88_20211110_p2': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20211110_resignation': 'primary_cabinet_minutes_pdf_archived',
    'jp_kantei_cabinet_minutes_20211110_first': 'primary_cabinet_minutes_pdf_archived',
    'jp_shugiin_giun_20241001': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20241001': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20241001': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20241001': 'primary_diet_minutes_api_json',
    'jp_kantei_kishida_cabinet_resignation_20241001': 'primary_kantei_account_archived',
    'jp_kantei_kishida_resignation_statement_20241001': 'primary_kantei_statement_archived',
    'jp_kanpo_gogai_toku45_20241001': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20241001_resignation': 'primary_cabinet_minutes_pdf_official',
    'jp_kantei_ishiba_designation_20241001': 'primary_kantei_account_archived',
    'jp_kantei_ishiba_cabinet_launch_20241001': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20241001': 'primary_kantei_statement_archived',
    'jp_kantei_ishiba_press_conference_20241001': 'primary_kantei_press_conference_official',
    'jp_kunaicho_schedule_entry_20241001': 'primary_imperial_household_schedule_official',
    'jp_kantei_cabinet_minutes_20241001_first': 'primary_cabinet_minutes_pdf_official',
    'jp_sangiin_giun_20241111': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20241111': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20241111': 'primary_diet_minutes_api_json',
    'jp_kantei_ishiba_designation_20241111': 'primary_kantei_account_archived',
    'jp_kantei_ishiba_cabinet_launch_20241111': 'primary_kantei_account_archived',
    'jp_kantei_ishiba_press_conference_20241111': 'primary_kantei_press_conference_official',
    'jp_kunaicho_schedule_202411': 'primary_imperial_household_schedule_official',
    'jp_kanpo_gogai_toku52_20241111': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20241111_resignation': 'primary_cabinet_minutes_pdf_official',
    'jp_kantei_cabinet_minutes_20241111_first': 'primary_cabinet_minutes_pdf_official',
    'jp_kantei_pm_statement_20241111': 'primary_kantei_statement_official',
    'jp_shugiin_giun_20251021': 'primary_diet_minutes_api_json',
    'jp_sangiin_giun_20251021': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20251021': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20251021': 'primary_diet_minutes_api_json',
    'jp_kantei_ishiba_cabinet_resignation_20251021': 'primary_kantei_account_archived',
    'jp_kantei_ishiba_resignation_statement_20251021': 'primary_kantei_statement_archived',
    'jp_kantei_takaichi_designation_20251021': 'primary_kantei_account_archived',
    'jp_kantei_takaichi_cabinet_launch_20251021': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20251021': 'primary_kantei_statement_archived',
    'jp_kantei_takaichi_press_conference_20251021': 'primary_kantei_press_conference_official',
    'jp_kunaicho_schedule_entry_20251021': 'primary_imperial_household_schedule_official',
    'jp_kanpo_gogai_toku28_20251021': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20251021_resignation': 'primary_cabinet_minutes_pdf_official',
    'jp_kantei_cabinet_minutes_20251021_first': 'primary_cabinet_minutes_pdf_official',
    'jp_sangiin_giun_20260218': 'primary_diet_minutes_api_json',
    'jp_shugiin_honkaigi_20260218': 'primary_diet_minutes_api_json',
    'jp_sangiin_honkaigi_20260218': 'primary_diet_minutes_api_json',
    'jp_kantei_takaichi_designation_20260218': 'primary_kantei_account_archived',
    'jp_kantei_takaichi_cabinet_launch_20260218': 'primary_kantei_account_archived',
    'jp_kantei_pm_statement_20260218': 'primary_kantei_statement_archived',
    'jp_kantei_takaichi_press_conference_20260218': 'primary_kantei_press_conference_official',
    'jp_kunaicho_schedule_entry_20260218': 'primary_imperial_household_schedule_official',
    'jp_kanpo_gogai_toku9_20260218': 'primary_official_gazette_pdf_archived',
    'jp_kantei_cabinet_minutes_20260218_resignation': 'primary_cabinet_minutes_pdf_official',
    'jp_kantei_cabinet_minutes_20260218_first': 'primary_cabinet_minutes_pdf_official',
    'jp_shugiin_budget_committee_20260727': 'primary_diet_minutes_api_json',
    'jp_kantei_takaichi_recovery_hq_20260904': 'primary_kantei_account_official',
}
# Byte-identical live copies recorded beside a capture (later versions, not the recorded identity).
ALTERNATES = {
    'jp_kantei_rekidai_094': ('https://www.kantei.go.jp/jp/rekidainaikaku/094.html', 43714, 'cd1bf17165e9cbd9bb8930be59d3226247140efe95064d75d41791335f699f43'),
    'jp_kantei_ishiba_cabinet_launch_20241111': ('https://www.kantei.go.jp/jp/103/actions/202411/11ishibanaikaku2.html', 12080, 'd92ac82383e225187bcc9069dc013544ed5e7cd25de56ebd88ee1bb41528e08e'),
    'jp_kantei_takaichi_cabinet_launch_20251021': ('https://www.kantei.go.jp/jp/104/actions/202510/21takaichinaikaku.html', 14093, '82f3c6e7bfda32e19a930520657fabb4a7efd1ef88e047cc5857caa0a917846f'),
}
# The three 2006 responses the earlier packet records as its closing boundary, re-recorded here so rows can name the
# holder: new source id -> the earlier packet's source id with the same URL, bytes and SHA-256.
REPEATED_RESPONSES = {
    'jp_kantei_abe_danwa_named_20060926': 'jp_kantei_abe_danwa_20060926',
    'jp_kantei_abe_hossoku_named_20060926': 'jp_kantei_abe_hossoku_20060926',
    'jp_kunaicho_photo_abe_named_20060926': 'jp_kunaicho_photo_20060926',
}
# Every new claim's (attested_on, event_kind, review observation), exactly: distinct dated events are never re-dated,
# relabelled or moved to another observation. Retrospective spans and undated recollections carry no structured date.
EVENTS = {
    'jp_abe_named_statement_appointed_pm_20060926': ('2006-09-26', 'appointment_statement', 'JP-PM06-01'),
    'jp_abe_named_shinninshiki_20060926': ('2006-09-26', 'imperial_appointment_ceremony', 'JP-PM06-01'),
    'jp_kunaicho_ceremony_abe_named_20060926': ('2006-09-26', 'imperial_appointment_ceremony', 'JP-PM06-01'),
    'jp_abe_announces_resignation_decision_20070912': ('2007-09-12', 'resignation_intent_announced', 'JP-PM06-01'),
    'jp_ccs_reports_abe_hospital_examination_20070913': ('2007-09-13', 'hospital_visit_reported', 'JP-PM06-01'),
    'jp_abe_in_office_press_conference_20070924': ('2007-09-24', 'in_office_attestation', 'JP-PM06-01'),
    'jp_abe_hospitalised_since_20070913': ('2007-09-13', 'hospitalisation_recalled_by_holder', 'JP-PM06-01'),
    'jp_abe_no_acting_pm_designated_20070924': ('2007-09-24', 'acting_prime_minister_not_designated', 'JP-PM06-01'),
    'jp_abe_states_he_will_leave_office_tomorrow_20070924': ('2007-09-24', 'prospective_resignation_statement', 'JP-PM06-01'),
    'jp_abe_cabinet_resigned_statement_20070925': ('2007-09-25', 'cabinet_resignation_statement', 'JP-PM06-01'),
    'jp_abe_cabinet_resignation_decided_20070925': ('2007-09-25', 'cabinet_resignation_decided', 'JP-PM06-01'),
    'jp_shugiin_rules_receives_abe_resignation_notice_20070925': ('2007-09-25', 'cabinet_resignation_notice', 'JP-PM06-01'),
    'jp_sangiin_rules_receives_abe_resignation_notice_20070925': ('2007-09-25', 'cabinet_resignation_notice', 'JP-PM06-01'),
    'jp_shugiin_receives_abe_cabinet_resignation_notice_20070925': ('2007-09-25', 'cabinet_resignation_notice', 'JP-PM06-01'),
    'jp_shugiin_designates_fukuda_20070925': ('2007-09-25', 'designation_vote_house_of_representatives', 'JP-PM06-02'),
    'jp_shugiin_receives_joint_committee_request_20070925': ('2007-09-25', 'joint_committee_requested', 'JP-PM06-02'),
    'jp_shugiin_hr_resolution_prevails_fukuda_20070925': ('2007-09-25', 'house_of_representatives_resolution_prevails', 'JP-PM06-02'),
    'jp_sangiin_notice_abe_cabinet_resigns_20070925': ('2007-09-25', 'cabinet_resignation_notice', 'JP-PM06-01'),
    'jp_sangiin_first_ballot_no_majority_20070925': ('2007-09-25', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-02'),
    'jp_sangiin_designates_ozawa_runoff_20070925': ('2007-09-25', 'designation_vote_house_of_councillors', 'JP-PM06-02'),
    'jp_sangiin_joint_committee_requested_20070925': ('2007-09-25', 'joint_committee_requested', 'JP-PM06-02'),
    'jp_sangiin_hr_resolution_prevails_20070925': ('2007-09-25', 'house_of_representatives_resolution_prevails', 'JP-PM06-02'),
    'jp_joint_committee_no_agreement_20070925': ('2007-09-25', 'joint_committee_no_agreement', 'JP-PM06-02'),
    'jp_member_recalls_abe_resignation_announcement_20070912': ('2007-09-12', 'resignation_intent_recalled', 'JP-PM06-01'),
    'jp_ccs_reports_abe_cabinet_resignation_decided_20070925': ('2007-09-25', 'cabinet_resignation_decided', 'JP-PM06-01'),
    'jp_kantei_span_abe_90_20060926_20070926': (None, 'retrospective_term_span', 'JP-PM06-01'),
    'jp_fukuda_says_appointed_on_eve_20070925': ('2007-09-25', 'designation_recalled_by_holder', 'JP-PM06-02'),
    'jp_ccs_schedules_fukuda_ceremony_20070925': ('2007-09-25', 'imperial_appointment_ceremony_scheduled', 'JP-PM06-02'),
    'jp_fukuda_shinninshiki_appointed_20070926': ('2007-09-26', 'imperial_appointment_ceremony', 'JP-PM06-02'),
    'jp_fukuda_cabinet_formed_20070926': ('2007-09-26', 'cabinet_formation', 'JP-PM06-02'),
    'jp_kantei_pm_statement_appointed_20070926': ('2007-09-26', 'appointment_statement', 'JP-PM06-02'),
    'jp_kunaicho_ceremony_20070926': ('2007-09-26', 'imperial_appointment_ceremony', 'JP-PM06-02'),
    'jp_kunaicho_ceremony_fukuda_20070926': ('2007-09-26', 'imperial_appointment_ceremony', 'JP-PM06-02'),
    'jp_ccs_reports_fukuda_ceremony_0830_20070926': ('2007-09-26', 'imperial_appointment_ceremony', 'JP-PM06-02'),
    'jp_ccs_reports_fukuda_acting_order_designated_20070926': ('2007-09-26', 'acting_prime_minister_order_designated', 'JP-PM06-02'),
    'jp_fukuda_announces_resignation_decision_20080901': ('2008-09-01', 'resignation_intent_announced', 'JP-PM06-02'),
    'jp_fukuda_recalls_taking_office_20070926': (None, 'assumption_recalled_by_holder', 'JP-PM06-02'),
    'jp_kantei_pm_statement_resigned_20080924': ('2008-09-24', 'cabinet_resignation_statement', 'JP-PM06-02'),
    'jp_fukuda_cabinet_resignation_decided_20080924': ('2008-09-24', 'cabinet_resignation_decided', 'JP-PM06-02'),
    'jp_shugiin_rules_receives_fukuda_resignation_notice_20080924': ('2008-09-24', 'cabinet_resignation_notice', 'JP-PM06-02'),
    'jp_sangiin_rules_receives_fukuda_resignation_notice_20080924': ('2008-09-24', 'cabinet_resignation_notice', 'JP-PM06-02'),
    'jp_shugiin_receives_fukuda_cabinet_resignation_notice_20080924': ('2008-09-24', 'cabinet_resignation_notice', 'JP-PM06-02'),
    'jp_shugiin_designates_aso_20080924': ('2008-09-24', 'designation_vote_house_of_representatives', 'JP-PM06-03'),
    'jp_shugiin_receives_joint_committee_request_20080924': ('2008-09-24', 'joint_committee_requested', 'JP-PM06-03'),
    'jp_shugiin_hr_resolution_prevails_aso_20080924': ('2008-09-24', 'house_of_representatives_resolution_prevails', 'JP-PM06-03'),
    'jp_sangiin_notice_fukuda_cabinet_resigns_20080924': ('2008-09-24', 'cabinet_resignation_notice', 'JP-PM06-02'),
    'jp_sangiin_first_ballot_no_majority_20080924': ('2008-09-24', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-03'),
    'jp_sangiin_designates_ozawa_runoff_20080924': ('2008-09-24', 'designation_vote_house_of_councillors', 'JP-PM06-03'),
    'jp_sangiin_joint_committee_requested_20080924': ('2008-09-24', 'joint_committee_requested', 'JP-PM06-03'),
    'jp_sangiin_hr_resolution_prevails_20080924': ('2008-09-24', 'house_of_representatives_resolution_prevails', 'JP-PM06-03'),
    'jp_ccs_reports_fukuda_cabinet_resignation_decided_20080924': ('2008-09-24', 'cabinet_resignation_decided', 'JP-PM06-02'),
    'jp_ccs_arranges_emergency_ministers_until_attestation_20080924': ('2008-09-24', 'continued_performance_of_duties_arranged', 'JP-PM06-02'),
    'jp_kantei_span_fukuda_91_20070926_20080924': (None, 'retrospective_term_span', 'JP-PM06-02'),
    'jp_joint_committee_no_agreement_20080924': ('2008-09-24', 'joint_committee_no_agreement', 'JP-PM06-03'),
    'jp_aso_shinninshiki_appointed_20080924': ('2008-09-24', 'imperial_appointment_ceremony', 'JP-PM06-03'),
    'jp_aso_cabinet_formed_20080924': ('2008-09-24', 'cabinet_formation', 'JP-PM06-03'),
    'jp_kunaicho_ceremony_20080924': ('2008-09-24', 'imperial_appointment_ceremony', 'JP-PM06-03'),
    'jp_kunaicho_ceremony_aso_20080924': ('2008-09-24', 'imperial_appointment_ceremony', 'JP-PM06-03'),
    'jp_ccs_reports_aso_acting_order_designated_20080925': ('2008-09-25', 'acting_prime_minister_order_designated', 'JP-PM06-03'),
    'jp_aso_states_cabinet_resigned_20090916': ('2009-09-16', 'cabinet_resignation_statement', 'JP-PM06-03'),
    'jp_aso_recalls_cabinet_launched_20080924': ('2008-09-24', 'cabinet_formation_recalled_by_holder', 'JP-PM06-03'),
    'jp_aso_cabinet_resignation_decided_20090916': ('2009-09-16', 'cabinet_resignation_decided', 'JP-PM06-03'),
    'jp_sangiin_rules_receives_aso_resignation_notice_20090916': ('2009-09-16', 'cabinet_resignation_notice', 'JP-PM06-03'),
    'jp_kantei_span_aso_92_20080924_20090916': (None, 'retrospective_term_span', 'JP-PM06-03'),
    'jp_shugiin_designates_hatoyama_20090916': ('2009-09-16', 'designation_vote_house_of_representatives', 'JP-PM06-04'),
    'jp_sangiin_designates_hatoyama_20090916': ('2009-09-16', 'designation_vote_house_of_councillors', 'JP-PM06-04'),
    'jp_hatoyama_shinninshiki_appointed_20090916': ('2009-09-16', 'imperial_appointment_ceremony', 'JP-PM06-04'),
    'jp_hatoyama_cabinet_formed_20090916': ('2009-09-16', 'cabinet_formation', 'JP-PM06-04'),
    'jp_kunaicho_ceremony_20090916': ('2009-09-16', 'imperial_appointment_ceremony', 'JP-PM06-04'),
    'jp_kunaicho_ceremony_hatoyama_20090916': ('2009-09-16', 'imperial_appointment_ceremony', 'JP-PM06-04'),
    'jp_ccs_schedules_hatoyama_ceremony_20090916': ('2009-09-16', 'imperial_appointment_ceremony_scheduled', 'JP-PM06-04'),
    'jp_ccs_reports_hatoyama_resignation_announcement_20100602': ('2010-06-02', 'resignation_intent_reported', 'JP-PM06-04'),
    'jp_ccs_government_to_continue_until_successor_20100602': ('2010-06-02', 'continued_performance_of_duties_arranged', 'JP-PM06-04'),
    'jp_hatoyama_transition_instruction_20100602': ('2010-06-02', 'continued_performance_of_duties_arranged', 'JP-PM06-04'),
    'jp_hatoyama_last_cabinet_meeting_remarks_20100604': ('2010-06-04', 'final_cabinet_meeting_remarks', 'JP-PM06-04'),
    'jp_kantei_index_lists_hatoyama_resignation_items_20100604': ('2010-06-04', 'statement_index_listing', 'JP-PM06-04'),
    'jp_hatoyama_cabinet_resigned_statement_20100604': ('2010-06-04', 'cabinet_resignation_statement', 'JP-PM06-04'),
    'jp_hatoyama_cabinet_resignation_decided_20100604': ('2010-06-04', 'cabinet_resignation_decided', 'JP-PM06-04'),
    'jp_shugiin_rules_receives_hatoyama_resignation_notice_20100604': ('2010-06-04', 'cabinet_resignation_notice', 'JP-PM06-04'),
    'jp_sangiin_rules_receives_hatoyama_resignation_notice_20100604': ('2010-06-04', 'cabinet_resignation_notice', 'JP-PM06-04'),
    'jp_shugiin_receives_hatoyama_cabinet_resignation_notice_20100604': ('2010-06-04', 'cabinet_resignation_notice', 'JP-PM06-04'),
    'jp_shugiin_designates_kan_20100604': ('2010-06-04', 'designation_vote_house_of_representatives', 'JP-PM06-05'),
    'jp_sangiin_notice_hatoyama_cabinet_resigns_20100604': ('2010-06-04', 'cabinet_resignation_notice', 'JP-PM06-04'),
    'jp_sangiin_designates_kan_20100604': ('2010-06-04', 'designation_vote_house_of_councillors', 'JP-PM06-05'),
    'jp_ccs_reports_hatoyama_cabinet_resignation_decided_20100604': ('2010-06-04', 'cabinet_resignation_decided', 'JP-PM06-04'),
    'jp_ccs_reports_hatoyama_final_cabinet_meeting_remarks_20100604': ('2010-06-04', 'final_cabinet_meeting_remarks', 'JP-PM06-04'),
    'jp_ccs_states_hatoyama_cabinet_span_262_days_20100604': (None, 'cabinet_span_stated_by_chief_cabinet_secretary', 'JP-PM06-04'),
    'jp_member_states_hatoyama_caretaker_cabinet_20100604_20100608': (None, 'continued_performance_of_duties_recalled', 'JP-PM06-04'),
    'jp_member_recalls_kan_designation_20100604': ('2010-06-04', 'designation_recalled_by_member', 'JP-PM06-05'),
    'jp_pm_kan_confirms_caretaker_cabinet_20100615': (None, 'continued_performance_of_duties_recalled', 'JP-PM06-04'),
    'jp_kantei_span_hatoyama_93_20090916_20100608': (None, 'retrospective_term_span', 'JP-PM06-04'),
    'jp_kantei_kan_designated_94th_20100604': ('2010-06-04', 'designation_account', 'JP-PM06-05'),
    'jp_kunaicho_speakers_report_20100608': ('2010-06-08', 'presiding_officers_report_to_emperor', 'JP-PM06-05'),
    'jp_kunaicho_ceremony_20100608': ('2010-06-08', 'imperial_appointment_ceremony', 'JP-PM06-05'),
    'jp_kunaicho_ceremony_kan_20100608': ('2010-06-08', 'imperial_appointment_ceremony', 'JP-PM06-05'),
    'jp_kan_press_conference_to_assume_office_that_evening_20100608': ('2010-06-08', 'prospective_assumption_statement', 'JP-PM06-05'),
    'jp_kan_shinninshiki_appointed_20100608': ('2010-06-08', 'imperial_appointment_ceremony', 'JP-PM06-05'),
    'jp_kan_cabinet_formed_20100608': ('2010-06-08', 'cabinet_formation', 'JP-PM06-05'),
    'jp_ccs_schedules_kan_ceremony_20100608': ('2010-06-08', 'imperial_appointment_ceremony_scheduled', 'JP-PM06-05'),
    'jp_kan_policy_speech_bears_office_20100611': ('2010-06-11', 'in_office_attestation', 'JP-PM06-05'),
    'jp_kan_announces_intent_to_resign_as_pm_20110826': ('2011-08-26', 'resignation_intent_announced', 'JP-PM06-05'),
    'jp_kan_recalls_taking_office_20100608': (None, 'assumption_recalled_by_holder', 'JP-PM06-05'),
    'jp_shugiin_giun_receives_kan_resignation_notice_20110830': ('2011-08-30', 'cabinet_resignation_notice', 'JP-PM06-05'),
    'jp_shugiin_honkaigi_kan_resignation_notice_20110830': ('2011-08-30', 'cabinet_resignation_notice', 'JP-PM06-05'),
    'jp_shugiin_designates_noda_20110830': ('2011-08-30', 'designation_vote_house_of_representatives', 'JP-PM06-06'),
    'jp_sangiin_giun_receives_kan_resignation_notice_20110830': ('2011-08-30', 'cabinet_resignation_notice', 'JP-PM06-05'),
    'jp_sangiin_honkaigi_kan_resignation_notice_20110830': ('2011-08-30', 'cabinet_resignation_notice', 'JP-PM06-05'),
    'jp_sangiin_first_ballot_no_majority_20110830': ('2011-08-30', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-06'),
    'jp_sangiin_designates_noda_runoff_20110830': ('2011-08-30', 'designation_vote_house_of_councillors', 'JP-PM06-06'),
    'jp_kan_cabinet_resigned_statement_20110830': ('2011-08-30', 'cabinet_resignation_statement', 'JP-PM06-05'),
    'jp_kan_leads_disaster_drill_continued_duties_20110901': ('2011-09-01', 'continued_performance_of_duties', 'JP-PM06-05'),
    'jp_kantei_kan_cabinet_resignation_decided_20110830': ('2011-08-30', 'cabinet_resignation_decided', 'JP-PM06-05'),
    'jp_kan_leaves_kantei_20110902': ('2011-09-02', 'departure_from_prime_ministers_office_recorded', 'JP-PM06-05'),
    'jp_member_states_kan_cabinet_continued_duties_20110830_20110902': (None, 'continued_performance_of_duties_recalled', 'JP-PM06-05'),
    'jp_member_states_noda_cabinet_first_meeting_20110902': ('2011-09-02', 'cabinet_formation_recalled', 'JP-PM06-06'),
    'jp_noda_states_cabinet_formation_completed_20110902': ('2011-09-02', 'cabinet_formation_recalled_by_holder', 'JP-PM06-06'),
    'jp_noda_states_kan_asked_to_continue_duties_20110830_20110902': (None, 'continued_performance_of_duties_recalled', 'JP-PM06-05'),
    'jp_kan_instructs_ministers_continued_duties_20110901': ('2011-09-01', 'continued_performance_of_duties_act_recalled', 'JP-PM06-05'),
    'jp_kantei_span_kan_94_20100608_20110902': (None, 'retrospective_term_span', 'JP-PM06-05'),
    'jp_kunaicho_speakers_report_20110902': ('2011-09-02', 'presiding_officers_report_to_emperor', 'JP-PM06-06'),
    'jp_kunaicho_ceremony_20110902': ('2011-09-02', 'imperial_appointment_ceremony', 'JP-PM06-06'),
    'jp_kunaicho_ceremony_noda_20110902': ('2011-09-02', 'imperial_appointment_ceremony', 'JP-PM06-06'),
    'jp_noda_shinninshiki_appointed_20110902': ('2011-09-02', 'imperial_appointment_ceremony', 'JP-PM06-06'),
    'jp_noda_cabinet_formed_20110902': ('2011-09-02', 'cabinet_formation', 'JP-PM06-06'),
    'jp_noda_states_assumed_office_today_20110902': ('2011-09-02', 'assumption_statement', 'JP-PM06-06'),
    'jp_kantei_first_cabinet_meeting_instruction_20110902': ('2011-09-02', 'first_cabinet_meeting', 'JP-PM06-06'),
    'jp_noda_policy_speech_states_appointment_20110913': ('2011-09-13', 'in_office_attestation', 'JP-PM06-06'),
    'jp_noda_recalls_continued_duties_period_20110928': (None, 'continued_performance_of_duties_recalled', 'JP-PM06-06'),
    'jp_sangiin_giun_receives_noda_resignation_notice_20121226': ('2012-12-26', 'cabinet_resignation_notice', 'JP-PM06-06'),
    'jp_noda_cabinet_resignation_decided_20121226': ('2012-12-26', 'cabinet_resignation_decided', 'JP-PM06-06'),
    'jp_noda_leaves_kantei_20121226': ('2012-12-26', 'departure_from_prime_ministers_office_recorded', 'JP-PM06-06'),
    'jp_noda_cabinet_resigned_statement_20121226': ('2012-12-26', 'cabinet_resignation_statement', 'JP-PM06-06'),
    'jp_kantei_span_noda_95_20110902_20121226': (None, 'retrospective_term_span', 'JP-PM06-06'),
    'jp_shugiin_designates_abe_20121226': ('2012-12-26', 'designation_vote_house_of_representatives', 'JP-PM06-07'),
    'jp_sangiin_first_ballot_no_majority_20121226': ('2012-12-26', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-07'),
    'jp_sangiin_designates_abe_runoff_20121226': ('2012-12-26', 'designation_vote_house_of_councillors', 'JP-PM06-07'),
    'jp_kunaicho_speakers_report_20121226': ('2012-12-26', 'presiding_officers_report_to_emperor', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_20121226': ('2012-12-26', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_abe_20121226': ('2012-12-26', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_statement_appointed_again_20121226': ('2012-12-26', 'appointment_statement', 'JP-PM06-07'),
    'jp_abe_shinninshiki_appointed_20121226': ('2012-12-26', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_second_cabinet_formed_20121226': ('2012-12-26', 'cabinet_formation', 'JP-PM06-07'),
    'jp_abe_press_conference_appointed_96th_today_20121226': ('2012-12-26', 'appointment_statement', 'JP-PM06-07'),
    'jp_kantei_abe_designated_96th_20121226': ('2012-12-26', 'designation_account', 'JP-PM06-07'),
    'jp_abe_policy_speech_states_96th_pm_20130128': ('2013-01-28', 'in_office_attestation', 'JP-PM06-07'),
    'jp_sangiin_giun_receives_abe_resignation_notice_20141224': ('2014-12-24', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_shugiin_designates_abe_20141224': ('2014-12-24', 'designation_vote_house_of_representatives', 'JP-PM06-07'),
    'jp_sangiin_designates_abe_20141224': ('2014-12-24', 'designation_vote_house_of_councillors', 'JP-PM06-07'),
    'jp_kunaicho_speakers_report_20141224': ('2014-12-24', 'presiding_officers_report_to_emperor', 'JP-PM06-07'),
    'jp_kunaicho_pm_report_20141224': ('2014-12-24', 'prime_minister_report_to_emperor', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_20141224': ('2014-12-24', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_abe_20141224': ('2014-12-24', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_shinninshiki_appointed_20141224': ('2014-12-24', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_third_cabinet_formed_20141224': ('2014-12-24', 'cabinet_formation', 'JP-PM06-07'),
    'jp_abe_press_conference_continues_office_today_20141224': ('2014-12-24', 'assumption_statement', 'JP-PM06-07'),
    'jp_kantei_second_abe_cabinet_resigned_20141224': ('2014-12-24', 'cabinet_resignation_decided', 'JP-PM06-07'),
    'jp_kantei_abe_designated_97th_20141224': ('2014-12-24', 'designation_account', 'JP-PM06-07'),
    'jp_abe_policy_speech_continues_office_20150212': ('2015-02-12', 'in_office_attestation', 'JP-PM06-07'),
    'jp_sangiin_giun_receives_abe_resignation_notice_20171101': ('2017-11-01', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_shugiin_designates_abe_20171101': ('2017-11-01', 'designation_vote_house_of_representatives', 'JP-PM06-07'),
    'jp_sangiin_designates_abe_20171101': ('2017-11-01', 'designation_vote_house_of_councillors', 'JP-PM06-07'),
    'jp_kunaicho_speakers_report_20171101': ('2017-11-01', 'presiding_officers_report_to_emperor', 'JP-PM06-07'),
    'jp_kunaicho_pm_report_20171101': ('2017-11-01', 'prime_minister_report_to_emperor', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_20171101': ('2017-11-01', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_kunaicho_ceremony_abe_20171101': ('2017-11-01', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_shinninshiki_appointed_20171101': ('2017-11-01', 'imperial_appointment_ceremony', 'JP-PM06-07'),
    'jp_abe_fourth_cabinet_formed_20171101': ('2017-11-01', 'cabinet_formation', 'JP-PM06-07'),
    'jp_abe_press_conference_98th_pm_today_20171101': ('2017-11-01', 'assumption_statement', 'JP-PM06-07'),
    'jp_kantei_third_abe_cabinet_resigned_20171101': ('2017-11-01', 'cabinet_resignation_decided', 'JP-PM06-07'),
    'jp_kantei_abe_designated_98th_20171101': ('2017-11-01', 'designation_account', 'JP-PM06-07'),
    'jp_abe_policy_speech_continues_office_20171117': ('2017-11-17', 'in_office_attestation', 'JP-PM06-07'),
    'jp_abe_announces_intent_to_resign_as_pm_20200828': ('2020-08-28', 'resignation_intent_announced', 'JP-PM06-07'),
    'jp_shugiin_giun_receives_abe_resignation_notice_20200916': ('2020-09-16', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_shugiin_honkaigi_abe_resignation_notice_20200916': ('2020-09-16', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_shugiin_designates_suga_20200916': ('2020-09-16', 'designation_vote_house_of_representatives', 'JP-PM06-08'),
    'jp_sangiin_giun_receives_abe_resignation_notice_20200916': ('2020-09-16', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_sangiin_honkaigi_abe_resignation_notice_20200916': ('2020-09-16', 'cabinet_resignation_notice', 'JP-PM06-07'),
    'jp_sangiin_designates_suga_20200916': ('2020-09-16', 'designation_vote_house_of_councillors', 'JP-PM06-08'),
    'jp_abe_cabinet_resignation_decided_20200916': ('2020-09-16', 'cabinet_resignation_decided', 'JP-PM06-07'),
    'jp_abe_leaves_kantei_20200916': ('2020-09-16', 'departure_from_prime_ministers_office_recorded', 'JP-PM06-07'),
    'jp_abe_cabinet_resigns_statement_20200916': ('2020-09-16', 'cabinet_resignation_statement', 'JP-PM06-07'),
    'jp_kanpo_suga_appointed_pm_20200916': ('2020-09-16', 'appointment_notice_official_gazette', 'JP-PM06-08'),
    'jp_kanpo_abe_office_lost_20200916': ('2020-09-16', 'end_of_office_stated', 'JP-PM06-07'),
    'jp_kantei_span_abe_96_20121226_20141224': (None, 'retrospective_term_span', 'JP-PM06-07'),
    'jp_kantei_span_abe_97_20141224_20171101': (None, 'retrospective_term_span', 'JP-PM06-07'),
    'jp_kantei_span_abe_98_20171101_20200916': (None, 'retrospective_term_span', 'JP-PM06-07'),
    'jp_kantei_suga_designated_99th_20200916': ('2020-09-16', 'designation_account', 'JP-PM06-08'),
    'jp_kantei_suga_shinninshiki_20200916': ('2020-09-16', 'imperial_appointment_ceremony', 'JP-PM06-08'),
    'jp_kantei_suga_cabinet_formed_20200916': ('2020-09-16', 'cabinet_formation', 'JP-PM06-08'),
    'jp_kantei_pm_statement_assumes_office_20200916': ('2020-09-16', 'appointment_statement', 'JP-PM06-08'),
    'jp_kantei_suga_press_conference_as_pm_20200916': ('2020-09-16', 'in_office_attestation', 'JP-PM06-08'),
    'jp_kunaicho_ceremony_suga_20200916': ('2020-09-16', 'imperial_appointment_ceremony', 'JP-PM06-08'),
    'jp_cabinet_minutes_suga_appointment_statement_20200916': ('2020-09-16', 'appointment_statement', 'JP-PM06-08'),
    'jp_suga_notifies_cabinet_resignation_hr_20211004': ('2021-10-04', 'cabinet_resignation_notice', 'JP-PM06-08'),
    'jp_sangiin_receives_suga_resignation_notice_20211004': ('2021-10-04', 'cabinet_resignation_notice', 'JP-PM06-08'),
    'jp_shugiin_receives_suga_cabinet_resignation_notice_20211004': ('2021-10-04', 'cabinet_resignation_notice', 'JP-PM06-08'),
    'jp_shugiin_designates_kishida_20211004': ('2021-10-04', 'designation_vote_house_of_representatives', 'JP-PM06-09'),
    'jp_sangiin_notice_suga_cabinet_resigns_20211004': ('2021-10-04', 'cabinet_resignation_notice', 'JP-PM06-08'),
    'jp_sangiin_designates_kishida_20211004': ('2021-10-04', 'designation_vote_house_of_councillors', 'JP-PM06-09'),
    'jp_kantei_suga_cabinet_resigned_20211004': ('2021-10-04', 'cabinet_resignation_decided', 'JP-PM06-08'),
    'jp_kantei_suga_cabinet_resignation_statement_20211004': ('2021-10-04', 'cabinet_resignation_statement', 'JP-PM06-08'),
    'jp_kanpo_suga_office_lost_20211004': ('2021-10-04', 'end_of_office_stated', 'JP-PM06-08'),
    'jp_cabinet_minutes_suga_cabinet_resignation_decided_20211004': ('2021-10-04', 'cabinet_resignation_decided', 'JP-PM06-08'),
    'jp_cabinet_minutes_suga_cabinet_emergency_arrangement_20211004': ('2021-10-04', 'continued_performance_of_duties_arranged', 'JP-PM06-08'),
    'jp_kantei_kishida_designated_100th_20211004': ('2021-10-04', 'designation_account', 'JP-PM06-09'),
    'jp_kantei_kishida_shinninshiki_20211004': ('2021-10-04', 'imperial_appointment_ceremony', 'JP-PM06-09'),
    'jp_kantei_kishida_cabinet_formed_20211004': ('2021-10-04', 'cabinet_formation', 'JP-PM06-09'),
    'jp_kantei_pm_statement_assumes_office_20211004': ('2021-10-04', 'appointment_statement', 'JP-PM06-09'),
    'jp_kunaicho_ceremony_kishida_20211004': ('2021-10-04', 'imperial_appointment_ceremony', 'JP-PM06-09'),
    'jp_kanpo_kishida_appointed_pm_20211004': ('2021-10-04', 'appointment_notice_official_gazette', 'JP-PM06-09'),
    'jp_cabinet_minutes_kishida_appointment_statement_20211004': ('2021-10-04', 'appointment_statement', 'JP-PM06-09'),
    'jp_sangiin_receives_kishida_resignation_notice_20211110': ('2021-11-10', 'cabinet_resignation_notice', 'JP-PM06-09'),
    'jp_shugiin_designates_kishida_20211110': ('2021-11-10', 'designation_vote_house_of_representatives', 'JP-PM06-09'),
    'jp_sangiin_designates_kishida_20211110': ('2021-11-10', 'designation_vote_house_of_councillors', 'JP-PM06-09'),
    'jp_kantei_kishida_cabinet_resigned_morning_20211110': ('2021-11-10', 'cabinet_resignation_decided', 'JP-PM06-09'),
    'jp_kantei_kishida_designated_101st_20211110': ('2021-11-10', 'designation_account', 'JP-PM06-09'),
    'jp_kantei_kishida_shinninshiki_20211110': ('2021-11-10', 'imperial_appointment_ceremony', 'JP-PM06-09'),
    'jp_kantei_kishida_second_cabinet_formed_20211110': ('2021-11-10', 'cabinet_formation', 'JP-PM06-09'),
    'jp_kantei_pm_statement_assumes_office_again_20211110': ('2021-11-10', 'assumption_statement', 'JP-PM06-09'),
    'jp_kunaicho_ceremony_kishida_20211110': ('2021-11-10', 'imperial_appointment_ceremony', 'JP-PM06-09'),
    'jp_kanpo_kishida_appointed_pm_20211110': ('2021-11-10', 'appointment_notice_official_gazette', 'JP-PM06-09'),
    'jp_kanpo_kishida_office_lost_20211110': ('2021-11-10', 'end_of_office_stated', 'JP-PM06-09'),
    'jp_cabinet_minutes_kishida_cabinet_resignation_decided_20211110': ('2021-11-10', 'cabinet_resignation_decided', 'JP-PM06-09'),
    'jp_cabinet_minutes_kishida_assumption_statement_20211110': ('2021-11-10', 'assumption_statement', 'JP-PM06-09'),
    'jp_kishida_notifies_cabinet_resignation_hr_20241001': ('2024-10-01', 'cabinet_resignation_notice', 'JP-PM06-09'),
    'jp_sangiin_receives_kishida_resignation_notice_20241001': ('2024-10-01', 'cabinet_resignation_notice', 'JP-PM06-09'),
    'jp_shugiin_receives_kishida_cabinet_resignation_notice_20241001': ('2024-10-01', 'cabinet_resignation_notice', 'JP-PM06-09'),
    'jp_shugiin_designates_ishiba_20241001': ('2024-10-01', 'designation_vote_house_of_representatives', 'JP-PM06-10'),
    'jp_sangiin_notice_kishida_cabinet_resigns_20241001': ('2024-10-01', 'cabinet_resignation_notice', 'JP-PM06-09'),
    'jp_sangiin_designates_ishiba_20241001': ('2024-10-01', 'designation_vote_house_of_councillors', 'JP-PM06-10'),
    'jp_kantei_kishida_cabinet_resigned_20241001': ('2024-10-01', 'cabinet_resignation_decided', 'JP-PM06-09'),
    'jp_kantei_kishida_cabinet_resignation_statement_20241001': ('2024-10-01', 'cabinet_resignation_statement', 'JP-PM06-09'),
    'jp_kanpo_ishiba_appointed_pm_20241001': ('2024-10-01', 'appointment_notice_official_gazette', 'JP-PM06-10'),
    'jp_kanpo_kishida_office_lost_20241001': ('2024-10-01', 'end_of_office_stated', 'JP-PM06-09'),
    'jp_cabinet_minutes_kishida_cabinet_resignation_decided_20241001': ('2024-10-01', 'cabinet_resignation_decided', 'JP-PM06-09'),
    'jp_cabinet_minutes_kishida_cabinet_emergency_arrangement_20241001': ('2024-10-01', 'continued_performance_of_duties_arranged', 'JP-PM06-09'),
    'jp_kantei_ishiba_designated_102nd_20241001': ('2024-10-01', 'designation_account', 'JP-PM06-10'),
    'jp_kantei_ishiba_shinninshiki_20241001': ('2024-10-01', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kantei_ishiba_cabinet_formed_20241001': ('2024-10-01', 'cabinet_formation', 'JP-PM06-10'),
    'jp_kantei_pm_statement_assumes_office_20241001': ('2024-10-01', 'appointment_statement', 'JP-PM06-10'),
    'jp_kantei_ishiba_press_conference_as_pm_20241001': ('2024-10-01', 'in_office_attestation', 'JP-PM06-10'),
    'jp_kunaicho_ceremony_20241001': ('2024-10-01', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_cabinet_minutes_ishiba_appointment_statement_20241001': ('2024-10-01', 'appointment_statement', 'JP-PM06-10'),
    'jp_sangiin_receives_ishiba_resignation_notice_20241111': ('2024-11-11', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_shugiin_first_ballot_no_majority_20241111': ('2024-11-11', 'designation_ballot_no_majority_house_of_representatives', 'JP-PM06-10'),
    'jp_shugiin_designates_ishiba_runoff_20241111': ('2024-11-11', 'designation_vote_house_of_representatives', 'JP-PM06-10'),
    'jp_sangiin_designates_ishiba_20241111': ('2024-11-11', 'designation_vote_house_of_councillors', 'JP-PM06-10'),
    'jp_kantei_ishiba_designated_103rd_20241111': ('2024-11-11', 'designation_account', 'JP-PM06-10'),
    'jp_kantei_ishiba_shinninshiki_20241111': ('2024-11-11', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kantei_ishiba_second_cabinet_formed_20241111': ('2024-11-11', 'cabinet_formation', 'JP-PM06-10'),
    'jp_kantei_ishiba_press_conference_as_pm_20241111': ('2024-11-11', 'in_office_attestation', 'JP-PM06-10'),
    'jp_kunaicho_ceremony_20241111': ('2024-11-11', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kanpo_ishiba_appointed_pm_20241111': ('2024-11-11', 'appointment_notice_official_gazette', 'JP-PM06-10'),
    'jp_kanpo_ishiba_office_lost_20241111': ('2024-11-11', 'end_of_office_stated', 'JP-PM06-10'),
    'jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20241111': ('2024-11-11', 'cabinet_resignation_decided', 'JP-PM06-10'),
    'jp_cabinet_minutes_ishiba_assumption_statement_20241111': ('2024-11-11', 'assumption_statement', 'JP-PM06-10'),
    'jp_kantei_pm_statement_assumes_office_again_20241111': ('2024-11-11', 'assumption_statement', 'JP-PM06-10'),
    'jp_ishiba_notifies_cabinet_resignation_hr_20251021': ('2025-10-21', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_sangiin_receives_ishiba_resignation_notice_20251021': ('2025-10-21', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_shugiin_receives_ishiba_cabinet_resignation_notice_20251021': ('2025-10-21', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_shugiin_designates_takaichi_20251021': ('2025-10-21', 'designation_vote_house_of_representatives', 'JP-PM06-10'),
    'jp_sangiin_notice_ishiba_cabinet_resigns_20251021': ('2025-10-21', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_sangiin_first_ballot_no_majority_20251021': ('2025-10-21', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-10'),
    'jp_sangiin_designates_takaichi_runoff_20251021': ('2025-10-21', 'designation_vote_house_of_councillors', 'JP-PM06-10'),
    'jp_kantei_ishiba_cabinet_resigned_20251021': ('2025-10-21', 'cabinet_resignation_decided', 'JP-PM06-10'),
    'jp_kantei_ishiba_cabinet_resignation_statement_20251021': ('2025-10-21', 'cabinet_resignation_statement', 'JP-PM06-10'),
    'jp_kantei_takaichi_designated_104th_20251021': ('2025-10-21', 'designation_account', 'JP-PM06-10'),
    'jp_kantei_takaichi_shinninshiki_20251021': ('2025-10-21', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kantei_takaichi_cabinet_formed_20251021': ('2025-10-21', 'cabinet_formation', 'JP-PM06-10'),
    'jp_kantei_pm_statement_assumes_office_20251021': ('2025-10-21', 'appointment_statement', 'JP-PM06-10'),
    'jp_kantei_takaichi_press_conference_as_pm_20251021': ('2025-10-21', 'in_office_attestation', 'JP-PM06-10'),
    'jp_kunaicho_ceremony_20251021': ('2025-10-21', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kanpo_takaichi_appointed_pm_20251021': ('2025-10-21', 'appointment_notice_official_gazette', 'JP-PM06-10'),
    'jp_kanpo_ishiba_office_lost_20251021': ('2025-10-21', 'end_of_office_stated', 'JP-PM06-10'),
    'jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20251021': ('2025-10-21', 'cabinet_resignation_decided', 'JP-PM06-10'),
    'jp_cabinet_minutes_ishiba_cabinet_emergency_arrangement_20251021': ('2025-10-21', 'continued_performance_of_duties_arranged', 'JP-PM06-10'),
    'jp_cabinet_minutes_takaichi_appointment_statement_20251021': ('2025-10-21', 'appointment_statement', 'JP-PM06-10'),
    'jp_sangiin_receives_takaichi_resignation_notice_20260218': ('2026-02-18', 'cabinet_resignation_notice', 'JP-PM06-10'),
    'jp_shugiin_designates_takaichi_20260218': ('2026-02-18', 'designation_vote_house_of_representatives', 'JP-PM06-10'),
    'jp_sangiin_first_ballot_no_majority_20260218': ('2026-02-18', 'designation_ballot_no_majority_house_of_councillors', 'JP-PM06-10'),
    'jp_sangiin_designates_takaichi_runoff_20260218': ('2026-02-18', 'designation_vote_house_of_councillors', 'JP-PM06-10'),
    'jp_kantei_takaichi_designated_105th_20260218': ('2026-02-18', 'designation_account', 'JP-PM06-10'),
    'jp_kantei_takaichi_shinninshiki_20260218': ('2026-02-18', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kantei_takaichi_second_cabinet_formed_20260218': ('2026-02-18', 'cabinet_formation', 'JP-PM06-10'),
    'jp_kantei_pm_statement_assumes_office_again_20260218': ('2026-02-18', 'assumption_statement', 'JP-PM06-10'),
    'jp_kantei_takaichi_press_conference_as_pm_20260218': ('2026-02-18', 'in_office_attestation', 'JP-PM06-10'),
    'jp_kunaicho_ceremony_20260218': ('2026-02-18', 'imperial_appointment_ceremony', 'JP-PM06-10'),
    'jp_kanpo_takaichi_appointed_pm_20260218': ('2026-02-18', 'appointment_notice_official_gazette', 'JP-PM06-10'),
    'jp_kanpo_takaichi_office_lost_20260218': ('2026-02-18', 'end_of_office_stated', 'JP-PM06-10'),
    'jp_cabinet_minutes_takaichi_cabinet_resignation_decided_20260218': ('2026-02-18', 'cabinet_resignation_decided', 'JP-PM06-10'),
    'jp_cabinet_minutes_takaichi_assumption_statement_20260218': ('2026-02-18', 'assumption_statement', 'JP-PM06-10'),
    'jp_takaichi_attends_hr_budget_committee_as_pm_20260727': ('2026-07-27', 'in_office_attestation', 'JP-PM06-10'),
    'jp_kantei_takaichi_in_office_20260904': ('2026-09-04', 'in_office_attestation', 'JP-PM06-10'),
}
# The holder name each extract row carries; None where the source names no holder of this role.
ROW_HOLDERS = {
    'jp_abe_named_statement_appointed_pm_20060926': '安倍晋三',
    'jp_abe_named_shinninshiki_20060926': '安倍晋三',
    'jp_kunaicho_ceremony_abe_named_20060926': '安倍晋三',
    'jp_abe_announces_resignation_decision_20070912': '安倍晋三',
    'jp_ccs_reports_abe_hospital_examination_20070913': '安倍晋三',
    'jp_abe_in_office_press_conference_20070924': '安倍晋三',
    'jp_abe_hospitalised_since_20070913': '安倍晋三',
    'jp_abe_no_acting_pm_designated_20070924': None,
    'jp_abe_states_he_will_leave_office_tomorrow_20070924': '安倍晋三',
    'jp_abe_cabinet_resigned_statement_20070925': '安倍晋三',
    'jp_abe_cabinet_resignation_decided_20070925': '安倍晋三',
    'jp_shugiin_rules_receives_abe_resignation_notice_20070925': '安倍晋三',
    'jp_sangiin_rules_receives_abe_resignation_notice_20070925': '安倍晋三',
    'jp_shugiin_receives_abe_cabinet_resignation_notice_20070925': '安倍晋三',
    'jp_shugiin_designates_fukuda_20070925': '福田康夫',
    'jp_shugiin_receives_joint_committee_request_20070925': None,
    'jp_shugiin_hr_resolution_prevails_fukuda_20070925': '福田康夫',
    'jp_sangiin_notice_abe_cabinet_resigns_20070925': '安倍晋三',
    'jp_sangiin_first_ballot_no_majority_20070925': '福田康夫',
    'jp_sangiin_designates_ozawa_runoff_20070925': None,
    'jp_sangiin_joint_committee_requested_20070925': None,
    'jp_sangiin_hr_resolution_prevails_20070925': None,
    'jp_joint_committee_no_agreement_20070925': None,
    'jp_member_recalls_abe_resignation_announcement_20070912': '安倍晋三',
    'jp_ccs_reports_abe_cabinet_resignation_decided_20070925': '安倍晋三',
    'jp_kantei_span_abe_90_20060926_20070926': '安倍晋三',
    'jp_fukuda_says_appointed_on_eve_20070925': '福田康夫',
    'jp_ccs_schedules_fukuda_ceremony_20070925': None,
    'jp_fukuda_shinninshiki_appointed_20070926': '福田康夫',
    'jp_fukuda_cabinet_formed_20070926': '福田康夫',
    'jp_kantei_pm_statement_appointed_20070926': None,
    'jp_kunaicho_ceremony_20070926': None,
    'jp_kunaicho_ceremony_fukuda_20070926': '福田康夫',
    'jp_ccs_reports_fukuda_ceremony_0830_20070926': '福田康夫',
    'jp_ccs_reports_fukuda_acting_order_designated_20070926': None,
    'jp_fukuda_announces_resignation_decision_20080901': '福田康夫',
    'jp_fukuda_recalls_taking_office_20070926': '福田康夫',
    'jp_kantei_pm_statement_resigned_20080924': None,
    'jp_fukuda_cabinet_resignation_decided_20080924': '福田康夫',
    'jp_shugiin_rules_receives_fukuda_resignation_notice_20080924': '福田康夫',
    'jp_sangiin_rules_receives_fukuda_resignation_notice_20080924': '福田康夫',
    'jp_shugiin_receives_fukuda_cabinet_resignation_notice_20080924': '福田康夫',
    'jp_shugiin_designates_aso_20080924': '麻生太郎',
    'jp_shugiin_receives_joint_committee_request_20080924': None,
    'jp_shugiin_hr_resolution_prevails_aso_20080924': '麻生太郎',
    'jp_sangiin_notice_fukuda_cabinet_resigns_20080924': '福田康夫',
    'jp_sangiin_first_ballot_no_majority_20080924': '麻生太郎',
    'jp_sangiin_designates_ozawa_runoff_20080924': None,
    'jp_sangiin_joint_committee_requested_20080924': None,
    'jp_sangiin_hr_resolution_prevails_20080924': None,
    'jp_ccs_reports_fukuda_cabinet_resignation_decided_20080924': '福田康夫',
    'jp_ccs_arranges_emergency_ministers_until_attestation_20080924': None,
    'jp_kantei_span_fukuda_91_20070926_20080924': '福田康夫',
    'jp_joint_committee_no_agreement_20080924': None,
    'jp_aso_shinninshiki_appointed_20080924': '麻生太郎',
    'jp_aso_cabinet_formed_20080924': '麻生太郎',
    'jp_kunaicho_ceremony_20080924': None,
    'jp_kunaicho_ceremony_aso_20080924': '麻生太郎',
    'jp_ccs_reports_aso_acting_order_designated_20080925': None,
    'jp_aso_states_cabinet_resigned_20090916': '麻生太郎',
    'jp_aso_recalls_cabinet_launched_20080924': '麻生太郎',
    'jp_aso_cabinet_resignation_decided_20090916': '麻生太郎',
    'jp_sangiin_rules_receives_aso_resignation_notice_20090916': '麻生太郎',
    'jp_kantei_span_aso_92_20080924_20090916': '麻生太郎',
    'jp_shugiin_designates_hatoyama_20090916': '鳩山由紀夫',
    'jp_sangiin_designates_hatoyama_20090916': '鳩山由紀夫',
    'jp_hatoyama_shinninshiki_appointed_20090916': '鳩山由紀夫',
    'jp_hatoyama_cabinet_formed_20090916': '鳩山由紀夫',
    'jp_kunaicho_ceremony_20090916': None,
    'jp_kunaicho_ceremony_hatoyama_20090916': '鳩山由紀夫',
    'jp_ccs_schedules_hatoyama_ceremony_20090916': '鳩山由紀夫',
    'jp_ccs_reports_hatoyama_resignation_announcement_20100602': '鳩山由紀夫',
    'jp_ccs_government_to_continue_until_successor_20100602': '鳩山由紀夫',
    'jp_hatoyama_transition_instruction_20100602': '鳩山由紀夫',
    'jp_hatoyama_last_cabinet_meeting_remarks_20100604': '鳩山由紀夫',
    'jp_kantei_index_lists_hatoyama_resignation_items_20100604': '鳩山由紀夫',
    'jp_hatoyama_cabinet_resigned_statement_20100604': '鳩山由紀夫',
    'jp_hatoyama_cabinet_resignation_decided_20100604': '鳩山由紀夫',
    'jp_shugiin_rules_receives_hatoyama_resignation_notice_20100604': '鳩山由紀夫',
    'jp_sangiin_rules_receives_hatoyama_resignation_notice_20100604': '鳩山由紀夫',
    'jp_shugiin_receives_hatoyama_cabinet_resignation_notice_20100604': '鳩山由紀夫',
    'jp_shugiin_designates_kan_20100604': '菅直人',
    'jp_sangiin_notice_hatoyama_cabinet_resigns_20100604': '鳩山由紀夫',
    'jp_sangiin_designates_kan_20100604': '菅直人',
    'jp_ccs_reports_hatoyama_cabinet_resignation_decided_20100604': '鳩山由紀夫',
    'jp_ccs_reports_hatoyama_final_cabinet_meeting_remarks_20100604': '鳩山由紀夫',
    'jp_ccs_states_hatoyama_cabinet_span_262_days_20100604': '鳩山由紀夫',
    'jp_member_states_hatoyama_caretaker_cabinet_20100604_20100608': '鳩山由紀夫',
    'jp_member_recalls_kan_designation_20100604': '菅直人',
    'jp_pm_kan_confirms_caretaker_cabinet_20100615': '鳩山由紀夫',
    'jp_kantei_span_hatoyama_93_20090916_20100608': '鳩山由紀夫',
    'jp_kantei_kan_designated_94th_20100604': '菅直人',
    'jp_kunaicho_speakers_report_20100608': None,
    'jp_kunaicho_ceremony_20100608': None,
    'jp_kunaicho_ceremony_kan_20100608': '菅直人',
    'jp_kan_press_conference_to_assume_office_that_evening_20100608': '菅直人',
    'jp_kan_shinninshiki_appointed_20100608': '菅直人',
    'jp_kan_cabinet_formed_20100608': '菅直人',
    'jp_ccs_schedules_kan_ceremony_20100608': '菅直人',
    'jp_kan_policy_speech_bears_office_20100611': '菅直人',
    'jp_kan_announces_intent_to_resign_as_pm_20110826': '菅直人',
    'jp_kan_recalls_taking_office_20100608': '菅直人',
    'jp_shugiin_giun_receives_kan_resignation_notice_20110830': '菅直人',
    'jp_shugiin_honkaigi_kan_resignation_notice_20110830': '菅直人',
    'jp_shugiin_designates_noda_20110830': '野田佳彦',
    'jp_sangiin_giun_receives_kan_resignation_notice_20110830': '菅直人',
    'jp_sangiin_honkaigi_kan_resignation_notice_20110830': '菅直人',
    'jp_sangiin_first_ballot_no_majority_20110830': '野田佳彦',
    'jp_sangiin_designates_noda_runoff_20110830': '野田佳彦',
    'jp_kan_cabinet_resigned_statement_20110830': '菅直人',
    'jp_kan_leads_disaster_drill_continued_duties_20110901': '菅直人',
    'jp_kantei_kan_cabinet_resignation_decided_20110830': '菅直人',
    'jp_kan_leaves_kantei_20110902': '菅直人',
    'jp_member_states_kan_cabinet_continued_duties_20110830_20110902': '菅直人',
    'jp_member_states_noda_cabinet_first_meeting_20110902': '野田佳彦',
    'jp_noda_states_cabinet_formation_completed_20110902': '野田佳彦',
    'jp_noda_states_kan_asked_to_continue_duties_20110830_20110902': '菅直人',
    'jp_kan_instructs_ministers_continued_duties_20110901': '菅直人',
    'jp_kantei_span_kan_94_20100608_20110902': '菅直人',
    'jp_kunaicho_speakers_report_20110902': None,
    'jp_kunaicho_ceremony_20110902': None,
    'jp_kunaicho_ceremony_noda_20110902': '野田佳彦',
    'jp_noda_shinninshiki_appointed_20110902': '野田佳彦',
    'jp_noda_cabinet_formed_20110902': '野田佳彦',
    'jp_noda_states_assumed_office_today_20110902': '野田佳彦',
    'jp_kantei_first_cabinet_meeting_instruction_20110902': None,
    'jp_noda_policy_speech_states_appointment_20110913': '野田佳彦',
    'jp_noda_recalls_continued_duties_period_20110928': '野田佳彦',
    'jp_sangiin_giun_receives_noda_resignation_notice_20121226': '野田佳彦',
    'jp_noda_cabinet_resignation_decided_20121226': '野田佳彦',
    'jp_noda_leaves_kantei_20121226': '野田佳彦',
    'jp_noda_cabinet_resigned_statement_20121226': '野田佳彦',
    'jp_kantei_span_noda_95_20110902_20121226': '野田佳彦',
    'jp_shugiin_designates_abe_20121226': '安倍晋三',
    'jp_sangiin_first_ballot_no_majority_20121226': '安倍晋三',
    'jp_sangiin_designates_abe_runoff_20121226': '安倍晋三',
    'jp_kunaicho_speakers_report_20121226': None,
    'jp_kunaicho_ceremony_20121226': None,
    'jp_kunaicho_ceremony_abe_20121226': '安倍晋三',
    'jp_abe_statement_appointed_again_20121226': '安倍晋三',
    'jp_abe_shinninshiki_appointed_20121226': '安倍晋三',
    'jp_abe_second_cabinet_formed_20121226': '安倍晋三',
    'jp_abe_press_conference_appointed_96th_today_20121226': '安倍晋三',
    'jp_kantei_abe_designated_96th_20121226': '安倍晋三',
    'jp_abe_policy_speech_states_96th_pm_20130128': '安倍晋三',
    'jp_sangiin_giun_receives_abe_resignation_notice_20141224': '安倍晋三',
    'jp_shugiin_designates_abe_20141224': '安倍晋三',
    'jp_sangiin_designates_abe_20141224': '安倍晋三',
    'jp_kunaicho_speakers_report_20141224': None,
    'jp_kunaicho_pm_report_20141224': None,
    'jp_kunaicho_ceremony_20141224': None,
    'jp_kunaicho_ceremony_abe_20141224': '安倍晋三',
    'jp_abe_shinninshiki_appointed_20141224': '安倍晋三',
    'jp_abe_third_cabinet_formed_20141224': '安倍晋三',
    'jp_abe_press_conference_continues_office_today_20141224': '安倍晋三',
    'jp_kantei_second_abe_cabinet_resigned_20141224': '安倍晋三',
    'jp_kantei_abe_designated_97th_20141224': '安倍晋三',
    'jp_abe_policy_speech_continues_office_20150212': '安倍晋三',
    'jp_sangiin_giun_receives_abe_resignation_notice_20171101': '安倍晋三',
    'jp_shugiin_designates_abe_20171101': '安倍晋三',
    'jp_sangiin_designates_abe_20171101': '安倍晋三',
    'jp_kunaicho_speakers_report_20171101': None,
    'jp_kunaicho_pm_report_20171101': None,
    'jp_kunaicho_ceremony_20171101': None,
    'jp_kunaicho_ceremony_abe_20171101': '安倍晋三',
    'jp_abe_shinninshiki_appointed_20171101': '安倍晋三',
    'jp_abe_fourth_cabinet_formed_20171101': '安倍晋三',
    'jp_abe_press_conference_98th_pm_today_20171101': '安倍晋三',
    'jp_kantei_third_abe_cabinet_resigned_20171101': '安倍晋三',
    'jp_kantei_abe_designated_98th_20171101': '安倍晋三',
    'jp_abe_policy_speech_continues_office_20171117': '安倍晋三',
    'jp_abe_announces_intent_to_resign_as_pm_20200828': '安倍晋三',
    'jp_shugiin_giun_receives_abe_resignation_notice_20200916': '安倍晋三',
    'jp_shugiin_honkaigi_abe_resignation_notice_20200916': '安倍晋三',
    'jp_shugiin_designates_suga_20200916': '菅義偉',
    'jp_sangiin_giun_receives_abe_resignation_notice_20200916': '安倍晋三',
    'jp_sangiin_honkaigi_abe_resignation_notice_20200916': '安倍晋三',
    'jp_sangiin_designates_suga_20200916': '菅義偉',
    'jp_abe_cabinet_resignation_decided_20200916': '安倍晋三',
    'jp_abe_leaves_kantei_20200916': '安倍晋三',
    'jp_abe_cabinet_resigns_statement_20200916': '安倍晋三',
    'jp_kanpo_suga_appointed_pm_20200916': '菅義偉',
    'jp_kanpo_abe_office_lost_20200916': '安倍晋三',
    'jp_kantei_span_abe_96_20121226_20141224': '安倍晋三',
    'jp_kantei_span_abe_97_20141224_20171101': '安倍晋三',
    'jp_kantei_span_abe_98_20171101_20200916': '安倍晋三',
    'jp_kantei_suga_designated_99th_20200916': '菅義偉',
    'jp_kantei_suga_shinninshiki_20200916': '菅義偉',
    'jp_kantei_suga_cabinet_formed_20200916': '菅義偉',
    'jp_kantei_pm_statement_assumes_office_20200916': None,
    'jp_kantei_suga_press_conference_as_pm_20200916': '菅義偉',
    'jp_kunaicho_ceremony_suga_20200916': '菅義偉',
    'jp_cabinet_minutes_suga_appointment_statement_20200916': '菅義偉',
    'jp_suga_notifies_cabinet_resignation_hr_20211004': '菅義偉',
    'jp_sangiin_receives_suga_resignation_notice_20211004': '菅義偉',
    'jp_shugiin_receives_suga_cabinet_resignation_notice_20211004': '菅義偉',
    'jp_shugiin_designates_kishida_20211004': '岸田文雄',
    'jp_sangiin_notice_suga_cabinet_resigns_20211004': '菅義偉',
    'jp_sangiin_designates_kishida_20211004': '岸田文雄',
    'jp_kantei_suga_cabinet_resigned_20211004': '菅義偉',
    'jp_kantei_suga_cabinet_resignation_statement_20211004': '菅義偉',
    'jp_kanpo_suga_office_lost_20211004': '菅義偉',
    'jp_cabinet_minutes_suga_cabinet_resignation_decided_20211004': '菅義偉',
    'jp_cabinet_minutes_suga_cabinet_emergency_arrangement_20211004': '菅義偉',
    'jp_kantei_kishida_designated_100th_20211004': '岸田文雄',
    'jp_kantei_kishida_shinninshiki_20211004': '岸田文雄',
    'jp_kantei_kishida_cabinet_formed_20211004': '岸田文雄',
    'jp_kantei_pm_statement_assumes_office_20211004': None,
    'jp_kunaicho_ceremony_kishida_20211004': '岸田文雄',
    'jp_kanpo_kishida_appointed_pm_20211004': '岸田文雄',
    'jp_cabinet_minutes_kishida_appointment_statement_20211004': '岸田文雄',
    'jp_sangiin_receives_kishida_resignation_notice_20211110': '岸田文雄',
    'jp_shugiin_designates_kishida_20211110': '岸田文雄',
    'jp_sangiin_designates_kishida_20211110': '岸田文雄',
    'jp_kantei_kishida_cabinet_resigned_morning_20211110': '岸田文雄',
    'jp_kantei_kishida_designated_101st_20211110': '岸田文雄',
    'jp_kantei_kishida_shinninshiki_20211110': '岸田文雄',
    'jp_kantei_kishida_second_cabinet_formed_20211110': '岸田文雄',
    'jp_kantei_pm_statement_assumes_office_again_20211110': None,
    'jp_kunaicho_ceremony_kishida_20211110': '岸田文雄',
    'jp_kanpo_kishida_appointed_pm_20211110': '岸田文雄',
    'jp_kanpo_kishida_office_lost_20211110': '岸田文雄',
    'jp_cabinet_minutes_kishida_cabinet_resignation_decided_20211110': '岸田文雄',
    'jp_cabinet_minutes_kishida_assumption_statement_20211110': '岸田文雄',
    'jp_kishida_notifies_cabinet_resignation_hr_20241001': '岸田文雄',
    'jp_sangiin_receives_kishida_resignation_notice_20241001': '岸田文雄',
    'jp_shugiin_receives_kishida_cabinet_resignation_notice_20241001': '岸田文雄',
    'jp_shugiin_designates_ishiba_20241001': '石破茂',
    'jp_sangiin_notice_kishida_cabinet_resigns_20241001': '岸田文雄',
    'jp_sangiin_designates_ishiba_20241001': '石破茂',
    'jp_kantei_kishida_cabinet_resigned_20241001': '岸田文雄',
    'jp_kantei_kishida_cabinet_resignation_statement_20241001': '岸田文雄',
    'jp_kanpo_ishiba_appointed_pm_20241001': '石破茂',
    'jp_kanpo_kishida_office_lost_20241001': '岸田文雄',
    'jp_cabinet_minutes_kishida_cabinet_resignation_decided_20241001': '岸田文雄',
    'jp_cabinet_minutes_kishida_cabinet_emergency_arrangement_20241001': '岸田文雄',
    'jp_kantei_ishiba_designated_102nd_20241001': '石破茂',
    'jp_kantei_ishiba_shinninshiki_20241001': '石破茂',
    'jp_kantei_ishiba_cabinet_formed_20241001': '石破茂',
    'jp_kantei_pm_statement_assumes_office_20241001': None,
    'jp_kantei_ishiba_press_conference_as_pm_20241001': '石破茂',
    'jp_kunaicho_ceremony_20241001': None,
    'jp_cabinet_minutes_ishiba_appointment_statement_20241001': '石破茂',
    'jp_sangiin_receives_ishiba_resignation_notice_20241111': '石破茂',
    'jp_shugiin_first_ballot_no_majority_20241111': '石破茂',
    'jp_shugiin_designates_ishiba_runoff_20241111': '石破茂',
    'jp_sangiin_designates_ishiba_20241111': '石破茂',
    'jp_kantei_ishiba_designated_103rd_20241111': '石破茂',
    'jp_kantei_ishiba_shinninshiki_20241111': '石破茂',
    'jp_kantei_ishiba_second_cabinet_formed_20241111': '石破茂',
    'jp_kantei_ishiba_press_conference_as_pm_20241111': '石破茂',
    'jp_kunaicho_ceremony_20241111': None,
    'jp_kanpo_ishiba_appointed_pm_20241111': '石破茂',
    'jp_kanpo_ishiba_office_lost_20241111': '石破茂',
    'jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20241111': '石破茂',
    'jp_cabinet_minutes_ishiba_assumption_statement_20241111': '石破茂',
    'jp_kantei_pm_statement_assumes_office_again_20241111': None,
    'jp_ishiba_notifies_cabinet_resignation_hr_20251021': '石破茂',
    'jp_sangiin_receives_ishiba_resignation_notice_20251021': '石破茂',
    'jp_shugiin_receives_ishiba_cabinet_resignation_notice_20251021': '石破茂',
    'jp_shugiin_designates_takaichi_20251021': '高市早苗',
    'jp_sangiin_notice_ishiba_cabinet_resigns_20251021': '石破茂',
    'jp_sangiin_first_ballot_no_majority_20251021': '高市早苗',
    'jp_sangiin_designates_takaichi_runoff_20251021': '高市早苗',
    'jp_kantei_ishiba_cabinet_resigned_20251021': '石破茂',
    'jp_kantei_ishiba_cabinet_resignation_statement_20251021': '石破茂',
    'jp_kantei_takaichi_designated_104th_20251021': '高市早苗',
    'jp_kantei_takaichi_shinninshiki_20251021': '高市早苗',
    'jp_kantei_takaichi_cabinet_formed_20251021': '高市早苗',
    'jp_kantei_pm_statement_assumes_office_20251021': None,
    'jp_kantei_takaichi_press_conference_as_pm_20251021': '高市早苗',
    'jp_kunaicho_ceremony_20251021': None,
    'jp_kanpo_takaichi_appointed_pm_20251021': '高市早苗',
    'jp_kanpo_ishiba_office_lost_20251021': '石破茂',
    'jp_cabinet_minutes_ishiba_cabinet_resignation_decided_20251021': '石破茂',
    'jp_cabinet_minutes_ishiba_cabinet_emergency_arrangement_20251021': '石破茂',
    'jp_cabinet_minutes_takaichi_appointment_statement_20251021': '高市早苗',
    'jp_sangiin_receives_takaichi_resignation_notice_20260218': '高市早苗',
    'jp_shugiin_designates_takaichi_20260218': '高市早苗',
    'jp_sangiin_first_ballot_no_majority_20260218': '高市早苗',
    'jp_sangiin_designates_takaichi_runoff_20260218': '高市早苗',
    'jp_kantei_takaichi_designated_105th_20260218': '高市早苗',
    'jp_kantei_takaichi_shinninshiki_20260218': '高市早苗',
    'jp_kantei_takaichi_second_cabinet_formed_20260218': '高市早苗',
    'jp_kantei_pm_statement_assumes_office_again_20260218': None,
    'jp_kantei_takaichi_press_conference_as_pm_20260218': '高市早苗',
    'jp_kunaicho_ceremony_20260218': None,
    'jp_kanpo_takaichi_appointed_pm_20260218': '高市早苗',
    'jp_kanpo_takaichi_office_lost_20260218': '高市早苗',
    'jp_cabinet_minutes_takaichi_cabinet_resignation_decided_20260218': '高市早苗',
    'jp_cabinet_minutes_takaichi_assumption_statement_20260218': '高市早苗',
    'jp_takaichi_attends_hr_budget_committee_as_pm_20260727': '高市早苗',
    'jp_kantei_takaichi_in_office_20260904': '高市早苗',
}
# Exact holder observations of jp_pm added here: (name, attested_on, from, until), in chronological order.
HOLDERS = [
    ('安倍晋三', None, '2006-09-26', None),
    ('福田康夫', None, '2007-09-26', None),
    ('麻生太郎', None, '2008-09-24', None),
    ('鳩山由紀夫', None, '2009-09-16', None),
    ('菅直人', None, '2010-06-08', None),
    ('野田佳彦', None, '2011-09-02', None),
    ('安倍晋三', None, '2012-12-26', None),
    ('安倍晋三', None, '2014-12-24', None),
    ('安倍晋三', None, '2017-11-01', '2020-09-16'),
    ('菅義偉', None, '2020-09-16', '2021-10-04'),
    ('岸田文雄', None, '2021-10-04', '2021-11-10'),
    ('岸田文雄', None, '2021-11-10', '2024-10-01'),
    ('石破茂', None, '2024-10-01', '2024-11-11'),
    ('石破茂', None, '2024-11-11', '2025-10-21'),
    ('高市早苗', None, '2025-10-21', '2026-02-18'),
    ('高市早苗', None, '2026-02-18', None),
]
HOLDER_CLAIMS = [
    ['jp_abe_named_statement_appointed_pm_20060926', 'jp_abe_named_shinninshiki_20060926', 'jp_kunaicho_ceremony_abe_named_20060926', 'jp_abe_in_office_press_conference_20070924'],
    ['jp_fukuda_shinninshiki_appointed_20070926', 'jp_kunaicho_ceremony_fukuda_20070926', 'jp_ccs_reports_fukuda_ceremony_0830_20070926'],
    ['jp_aso_shinninshiki_appointed_20080924', 'jp_kunaicho_ceremony_aso_20080924'],
    ['jp_hatoyama_shinninshiki_appointed_20090916', 'jp_kunaicho_ceremony_hatoyama_20090916'],
    ['jp_kunaicho_ceremony_kan_20100608', 'jp_kan_shinninshiki_appointed_20100608', 'jp_kan_policy_speech_bears_office_20100611'],
    ['jp_kunaicho_ceremony_noda_20110902', 'jp_noda_shinninshiki_appointed_20110902', 'jp_noda_states_assumed_office_today_20110902', 'jp_noda_policy_speech_states_appointment_20110913'],
    ['jp_kunaicho_ceremony_abe_20121226', 'jp_abe_statement_appointed_again_20121226', 'jp_abe_shinninshiki_appointed_20121226', 'jp_abe_press_conference_appointed_96th_today_20121226', 'jp_abe_policy_speech_states_96th_pm_20130128'],
    ['jp_kunaicho_ceremony_abe_20141224', 'jp_abe_shinninshiki_appointed_20141224', 'jp_abe_press_conference_continues_office_today_20141224', 'jp_abe_policy_speech_continues_office_20150212'],
    ['jp_kunaicho_ceremony_abe_20171101', 'jp_abe_shinninshiki_appointed_20171101', 'jp_abe_press_conference_98th_pm_today_20171101', 'jp_abe_policy_speech_continues_office_20171117', 'jp_kanpo_abe_office_lost_20200916'],
    ['jp_kanpo_suga_appointed_pm_20200916', 'jp_kantei_suga_shinninshiki_20200916', 'jp_kantei_suga_press_conference_as_pm_20200916', 'jp_kunaicho_ceremony_suga_20200916', 'jp_cabinet_minutes_suga_appointment_statement_20200916', 'jp_kanpo_suga_office_lost_20211004'],
    ['jp_kantei_kishida_shinninshiki_20211004', 'jp_kunaicho_ceremony_kishida_20211004', 'jp_kanpo_kishida_appointed_pm_20211004', 'jp_cabinet_minutes_kishida_appointment_statement_20211004', 'jp_kanpo_kishida_office_lost_20211110'],
    ['jp_kantei_kishida_shinninshiki_20211110', 'jp_kunaicho_ceremony_kishida_20211110', 'jp_kanpo_kishida_appointed_pm_20211110', 'jp_cabinet_minutes_kishida_assumption_statement_20211110', 'jp_kanpo_kishida_office_lost_20241001'],
    ['jp_kanpo_ishiba_appointed_pm_20241001', 'jp_kantei_ishiba_shinninshiki_20241001', 'jp_kantei_ishiba_press_conference_as_pm_20241001', 'jp_cabinet_minutes_ishiba_appointment_statement_20241001', 'jp_kanpo_ishiba_office_lost_20241111'],
    ['jp_kantei_ishiba_shinninshiki_20241111', 'jp_kantei_ishiba_press_conference_as_pm_20241111', 'jp_kanpo_ishiba_appointed_pm_20241111', 'jp_cabinet_minutes_ishiba_assumption_statement_20241111', 'jp_kanpo_ishiba_office_lost_20251021'],
    ['jp_kantei_takaichi_shinninshiki_20251021', 'jp_kantei_takaichi_press_conference_as_pm_20251021', 'jp_kanpo_takaichi_appointed_pm_20251021', 'jp_cabinet_minutes_takaichi_appointment_statement_20251021', 'jp_kanpo_takaichi_office_lost_20260218'],
    ['jp_kantei_takaichi_shinninshiki_20260218', 'jp_kantei_takaichi_press_conference_as_pm_20260218', 'jp_kanpo_takaichi_appointed_pm_20260218', 'jp_cabinet_minutes_takaichi_assumption_statement_20260218', 'jp_takaichi_attends_hr_budget_committee_as_pm_20260727', 'jp_kantei_takaichi_in_office_20260904'],
]
# Every stated end in the role, all from the Official Gazette's notices of loss of office.
ENDS = [('安倍晋三', '2020-09-16'), ('菅義偉', '2021-10-04'), ('岸田文雄', '2021-11-10'), ('岸田文雄', '2024-10-01'), ('石破茂', '2024-11-11'), ('石破茂', '2025-10-21'), ('高市早苗', '2026-02-18')]
# The LDP presidency keeps its two observations unchanged; party office never feeds jp_pm.
LDP_HOLDERS = [('石破茂', '2024-09-27', None, None), ('高市早苗', '2025-10-04', None, None)]
# Dates that are never a holder's start or end: designation-only days, announcements, hospital, continued duties,
# recollections, the Chief Cabinet Secretary's span and later attestations.
NEVER_HOLDER_DATE = {'2007-09-12', '2007-09-13', '2007-09-24', '2007-09-25', '2008-09-01', '2010-06-02', '2010-06-04',
                     '2010-06-11', '2010-06-15', '2011-08-26', '2011-08-29', '2011-08-30', '2011-09-01', '2011-09-13',
                     '2011-09-28', '2013-01-28', '2015-02-12', '2017-11-17', '2020-08-28', '2026-07-27', '2026-09-04'}
# On each transition day these events stay separate claims.
TRANSITIONS = {
    '2006-09-26': ['imperial_appointment_ceremony'],
    '2007-09-25': ['cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives'],
    '2007-09-26': ['cabinet_formation', 'imperial_appointment_ceremony'],
    '2008-09-24': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'continued_performance_of_duties_arranged', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'imperial_appointment_ceremony'],
    '2009-09-16': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'imperial_appointment_ceremony'],
    '2010-06-04': ['cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives'],
    '2010-06-08': ['cabinet_formation', 'imperial_appointment_ceremony'],
    '2011-08-30': ['cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives'],
    '2011-09-02': ['cabinet_formation', 'imperial_appointment_ceremony'],
    '2012-12-26': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'imperial_appointment_ceremony'],
    '2014-12-24': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'imperial_appointment_ceremony'],
    '2017-11-01': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'imperial_appointment_ceremony'],
    '2020-09-16': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2021-10-04': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'continued_performance_of_duties_arranged', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2021-11-10': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2024-10-01': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'continued_performance_of_duties_arranged', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2024-11-11': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2025-10-21': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'continued_performance_of_duties_arranged', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
    '2026-02-18': ['cabinet_formation', 'cabinet_resignation_decided', 'cabinet_resignation_notice', 'designation_vote_house_of_councillors', 'designation_vote_house_of_representatives', 'end_of_office_stated', 'imperial_appointment_ceremony'],
}
INSTITUTION_UNRESOLVED = [
    'Prime ministers 2006-2026 (CLAUDE-C01-13, JP-PM06-01..10) ex',
    'Still open for 2006-2026: the Official Gazette of 2006-2017,',
    'Continued performance of duties after a resignation en masse',
]
DECISIONS = {'01': 'Accepted in part', '02': 'Accepted in part', '03': 'Accepted in part', '04': 'Accepted in part',
             '05': 'Accepted in part', '06': 'Accepted in part', '07': 'Accepted in part', '08': 'Accepted',
             '09': 'Accepted', '10': 'Accepted'}
# Leads and records not imported: never a source URL here, and named in the report's leads section.
LEAD_URL_MARKERS = ('wikipedia', 'asospeech/2008/09/24kaiken', 'hatoyama/statement/200909/16kaiken', '/jp/kakugi/',
                    'gonittei01.html', '20110322201419', '20090917022504', 'hatoyama/actions/index',
                    '10syosin', 'tyoukanpress/201006/3_p', 'tyoukanpress/201006/index', '__icsFiles',
                    'rireki/2007/09/13_p', '118204024X00120121226', '118804024X00120141224', '119504024X00120171101',
                    '120604024X00120211110', '121504024X00120241111', '122104024X00120260218', 'speechNumber=5',
                    '1004kaiken', '1110kaiken', '01bousai', '17takaichinaikaku2', 'rekidainaikaku/099',
                    'rekidainaikaku/10', 'warp.ndl.go.jp', '20200916021157', 'kanpo.go.jp/old')
LEAD_REPORT_MARKERS = ('asospeech/2008/09/24kaiken', 'kakugi-2024100101', '__icsFiles', '3_p.html',
                       '118204024X00120121226', '01bousai', '17takaichinaikaku2', 'rekidainaikaku/099',
                       'gonittei01.html', '1004kaiken')
COUNTS = {
    'sources_claims': (211, 293),
    'holder_never': (69, 224),
    'categories': (65, 83, 13, 3, 17),
    'archived_others': (113, 98),
}
NEW_SOURCES = list(RESPONSES)
NEW_CLAIMS = list(EVENTS)
ORIGINAL_COUNT = 7
EARLIER_SOURCES = 119  # CLAUDE-C01-12's sources, between the originals and this packet's
EARLIER_HOLDERS = 14  # CLAUDE-C01-12's holders, all dated before CLOSING_BOUNDARY
CLOSING_BOUNDARY = '2006-09-26'
GROUPS = ['jp_shugiin_group_20260218_011', 'jp_shugiin_group_20260218_020', 'jp_shugiin_group_20260218_030',
          'jp_shugiin_group_20260218_040', 'jp_shugiin_group_20260218_050', 'jp_shugiin_group_20260218_060',
          'jp_shugiin_group_20260218_070']
PM = 'jp_pm'
T_PM = '内閣総理大臣 — Prime Minister of Japan'
SURNAMES = {'安倍晋三': '安倍', '福田康夫': '福田', '麻生太郎': '麻生', '鳩山由紀夫': '鳩山', '菅直人': '菅', '野田佳彦': '野田',
            '菅義偉': '菅', '岸田文雄': '岸田', '石破茂': '石破', '高市早苗': '高市'}
STARTS = [(h[0], h[2]) for h in HOLDERS]
FROM_KINDS = {'appointment_statement', 'assumption_statement', 'imperial_appointment_ceremony',
              'appointment_notice_official_gazette'}
UNTIL_KINDS = {'end_of_office_stated'}  # only the Official Gazette's notices of loss of office (退官)
ATTEST_KINDS = {'in_office_attestation'}
HOLDER_KINDS = FROM_KINDS | UNTIL_KINDS | ATTEST_KINDS
DESIGNATION_KINDS = {'designation_vote_house_of_representatives', 'designation_vote_house_of_councillors',
                     'designation_ballot_no_majority_house_of_representatives',
                     'designation_ballot_no_majority_house_of_councillors', 'joint_committee_requested',
                     'joint_committee_no_agreement', 'house_of_representatives_resolution_prevails', 'designation_account',
                     'presiding_officers_report_to_emperor', 'designation_recalled_by_holder', 'designation_recalled_by_member'}
RESIGNATION_KINDS = {'cabinet_resignation_notice', 'cabinet_resignation_decided', 'cabinet_resignation_statement',
                     'resignation_intent_announced', 'resignation_intent_reported', 'resignation_intent_recalled',
                     'prospective_resignation_statement', 'final_cabinet_meeting_remarks',
                     'departure_from_prime_ministers_office_recorded'}
CONTINUATION_KINDS = {'continued_performance_of_duties', 'continued_performance_of_duties_arranged',
                      'continued_performance_of_duties_recalled', 'continued_performance_of_duties_act_recalled'}
ACTING_KINDS = {'acting_prime_minister_not_designated', 'acting_prime_minister_order_designated'}
FORMATION_KINDS = {'cabinet_formation', 'first_cabinet_meeting', 'cabinet_formation_recalled',
                   'cabinet_formation_recalled_by_holder'}
RETROSPECTIVE_KINDS = {'retrospective_term_span'}
UNDATED_KINDS = {'retrospective_term_span', 'assumption_recalled_by_holder', 'continued_performance_of_duties_recalled',
                 'cabinet_span_stated_by_chief_cabinet_secretary'}
OTHER_KINDS = {'imperial_appointment_ceremony_scheduled', 'prospective_assumption_statement', 'hospital_visit_reported',
               'hospitalisation_recalled_by_holder', 'prime_minister_report_to_emperor', 'statement_index_listing',
               'cabinet_span_stated_by_chief_cabinet_secretary', 'assumption_recalled_by_holder'}
ALL_KINDS = (HOLDER_KINDS | DESIGNATION_KINDS | RESIGNATION_KINDS | CONTINUATION_KINDS | ACTING_KINDS | FORMATION_KINDS |
             RETROSPECTIVE_KINDS | OTHER_KINDS)
HOLDER_CLAIM_SET = {cid for ids in HOLDER_CLAIMS for cid in ids}
NEVER_HOLDER = tuple(cid for cid in EVENTS if cid not in HOLDER_CLAIM_SET)
DESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in DESIGNATION_KINDS)
RESIGNATIONS = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in RESIGNATION_KINDS)
CONTINUATION = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in CONTINUATION_KINDS)
ACTING = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in ACTING_KINDS)
RETROSPECTIVE = tuple(cid for cid, (_d, kind, _o) in EVENTS.items() if kind in UNDATED_KINDS)
PER_REQUEST_PATTERNS = ('cb=', '_cb', 'cbx=', 'OpenElement', 'any=', 'token=', 'sessionid', 'X-Amz-', 'Signature=',
                        'searchResult', 'opensearch', 'dl.ndl.go.jp', 'warp.ndl.go.jp', 'fullcontents')
OFFICIAL_HOSTS = {'kokkai.ndl.go.jp', 'www.kantei.go.jp', 'www.kunaicho.go.jp'}
STALE_IDS = ('attested_period', 'stated_span', '"statement_of_assumption_unnamed"', '"press_conference_in_office"',
             '"designation_vote_first_ballot_no_majority"', '_runoff"', '"continued_duties_', '"resignation_announcement',
             '"appointment_statement_before_ceremony"', '"holder_hospitalisation_reported"', '"statement_listing"',
             '"assumption_statement_prospective"', '"cabinet_resignation"',
             'jp_shugiin_giun_receives_hatoyama_resignation_notice_20100604',
             'jp_shugiin_honkaigi_hatoyama_resignation_notice_20100604',
             'jp_sangiin_giun_receives_hatoyama_resignation_notice_20100604',
             'jp_sangiin_honkaigi_hatoyama_resignation_notice_20100604',
             'jp_member_recalls_hatoyama_cabinet_continued_duties_20100604_20100608',
             'jp_kan_recalls_serving_in_continued_duties_cabinet_2010', 'Cabinet Office / National Printing Bureau')
REPORT = research.RESEARCH / 'japan-prime-ministers-2006-2026-13.md'
HANDOFF = 'docs/planning/ai-handoffs/CLAUDE-C01-13.md'


def later_sources(packet):
    return packet['sources'][ORIGINAL_COUNT + EARLIER_SOURCES:]


def load_rows(packet):
    rows = {}
    for source in later_sources(packet):
        extract = json.loads((research.ROOT / source['snapshot']['path']).read_text(encoding='utf-8'))
        rows.update({row['claim_id']: row for row in extract['rows']})
    return rows


def pm_rules(packet, rows):
    """Rule-based checks that hold without the pinned holder list; raise AssertionError, KeyError or IndexError."""
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    source_of = {c['id']: s for s in packet['sources'] for c in s['claims']}
    assert [i['id'] for i in packet['institutions']] == GROUPS + ['jp_prime_minister'], 'one prime-ministership after the groups'
    office = packet['institutions'][-1]
    assert office['kind'] == 'executive_institution' and office['lifecycle']['status'] == 'unknown'
    assert [r['id'] for r in office['roles']] == [PM], 'exactly one role, no new role'
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
    earlier = [h for h in holders if (h['attested_on'] or h['from']) < CLOSING_BOUNDARY]
    assert holders[:len(earlier)] == earlier and len(earlier) == EARLIER_HOLDERS, 'the earlier packet\'s holders come first'
    for holder in earlier:
        assert not set(holder['claim_ids']) & set(rows), (holder['name'], 'an earlier holder cites this packet\'s claims')
    for holder in holders[len(earlier):]:
        name = holder['name']
        assert isinstance(holder, dict) and name in SURNAMES, name
        assert holder['attested_on'] is None and holder['from'], (name, 'every holder here has a stated start')
        assert not {holder['from'], holder['until']} & NEVER_HOLDER_DATE, name
        assert not set(holder['claim_ids']) & set(NEVER_HOLDER), name
        kinds = {}
        for cid in holder['claim_ids']:
            row = rows[cid]
            assert cid in role['claim_ids'], cid
            # A holder claim names its holder: the row's holder_name, and the surname printed in the claim text.
            assert row['holder_name'] == name and row['role_title'] == T_PM and row['role_id'] == PM, (name, cid)
            assert row['event_kind'] in HOLDER_KINDS, (name, cid)
            assert SURNAMES[name] in claims[cid]['text'], (name, cid)
            kinds.setdefault(row['event_kind'], set()).add(claims[cid].get('attested_on'))
            if row['event_kind'] in UNTIL_KINDS:
                # An end is stated only by the Official Gazette's notice of loss of office.
                assert source_of[cid]['source_type'] == 'primary_official_gazette_pdf_archived', (name, cid)
                assert 'その地位を失った' in claims[cid]['text'], (name, cid)
            if row['event_kind'] in ATTEST_KINDS:
                day = claims[cid]['attested_on']
                assert holder['from'] <= day and (holder['until'] is None or day <= holder['until']), (name, cid)
        from_days = set().union(*(kinds.get(k, set()) for k in FROM_KINDS))
        until_days = set().union(*(kinds.get(k, set()) for k in UNTIL_KINDS))
        assert {holder['from']} == from_days, (name, 'start')
        assert ({holder['until']} == until_days) if holder['until'] else not until_days, (name, 'end')
        if holder['until']:
            assert holder['until'] <= research.CUTOFF and holder['from'] <= holder['until'], name
    # Every named start, end and attestation row is cited by its holder; no unnamed row ever is.
    for cid, row in rows.items():
        kind, name, day = row['event_kind'], row['holder_name'], row['attested_on']
        if kind in FROM_KINDS and name:
            assert sum(cid in h['claim_ids'] and h['name'] == name and h['from'] == day for h in holders) == 1, cid
        if kind in UNTIL_KINDS:
            assert name and sum(cid in h['claim_ids'] and h['name'] == name and h['until'] == day for h in holders) == 1, cid
        if kind in ATTEST_KINDS and name:
            assert sum(cid in h['claim_ids'] and h['name'] == name for h in holders) == 1, cid
        if not name:
            assert not any(cid in h['claim_ids'] for h in holders), cid
        assert kind in ALL_KINDS, (cid, kind)
    # Designations, resignations, continued duties, acting designations and retrospective lists stay on the role.
    for cid in NEVER_HOLDER:
        assert cid in role['claim_ids'], cid
        assert rows[cid]['event_kind'] not in HOLDER_KINDS or rows[cid]['holder_name'] is None, cid
    for cid in ACTING:
        assert rows[cid]['holder_name'] is None, cid
    for cid in (c for c in office['claim_ids'] if c in rows):
        assert not {'period', 'attested_period', 'stated_span'} & set(claims[cid]), cid
        if rows[cid]['event_kind'] in UNDATED_KINDS:
            assert 'attested_on' not in claims[cid], cid
        else:
            assert claims[cid]['attested_on'] <= research.CUTOFF, cid
    # Party office never feeds the prime-ministership: the LDP presidency keeps its two observations, with no boundaries.
    ldp = next(r for o in packet['organizations'] for r in o['roles'] if r['id'] == 'jp_ldp_party_president')
    assert [(h['name'], h['attested_on'], h['from'], h['until']) for h in ldp['holder_claims']] == LDP_HOLDERS
    assert not set(ldp['claim_ids']) & set(role['claim_ids']) and not set(ldp['sources']) & set(role['sources'])


def pm_invariants(packet, rows):
    """The rules plus the exact pinned holder list this packet intends."""
    pm_rules(packet, rows)
    role = packet['institutions'][-1]['roles'][0]
    claims = {c['id']: c for s in packet['sources'] for c in s['claims']}
    got = [(h['name'], h['attested_on'], h['from'], h['until']) for h in role['holder_claims'][EARLIER_HOLDERS:]]
    assert got == HOLDERS, got
    assert [h['claim_ids'] for h in role['holder_claims'][EARLIER_HOLDERS:]] == HOLDER_CLAIMS
    assert [(h['name'], h['from']) for h in role['holder_claims'][EARLIER_HOLDERS:]] == STARTS
    assert [(h['name'], h['until']) for h in role['holder_claims'] if h['until']] == ENDS
    for cid, (day, _kind, _obs) in EVENTS.items():
        assert claims[cid].get('attested_on') == day, cid


class JapanPrimeMinisters2006Tests(unittest.TestCase):
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
        cls.rows = load_rows(cls.packet)
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
        self.assertEqual((len(NEW_SOURCES), len(self.new_claims)), COUNTS['sources_claims'])
        self.assertEqual([s['id'] for s in later_sources(self.packet)], NEW_SOURCES)
        self.assertEqual(len(self.packet['sources']), ORIGINAL_COUNT + EARLIER_SOURCES + len(NEW_SOURCES))
        self.assertEqual((len(ids['entries']), len(ids['roles'])), (24, 5))
        self.assertEqual((len(self.packet['organizations']), len(self.packet['institutions'])), (16, 8))
        self.assertEqual(self.new_claims, NEW_CLAIMS)
        self.assertEqual(set(EVENTS), set(self.rows))
        # Every new claim is either a holder claim or a claim that never feeds a holder, never both.
        self.assertFalse(HOLDER_CLAIM_SET & set(NEVER_HOLDER))
        self.assertEqual(HOLDER_CLAIM_SET | set(NEVER_HOLDER), set(self.new_claims))
        self.assertEqual((len(HOLDER_CLAIM_SET), len(NEVER_HOLDER)), COUNTS['holder_never'])
        self.assertEqual((len(DESIGNATIONS), len(RESIGNATIONS), len(CONTINUATION), len(ACTING), len(RETROSPECTIVE)),
                         COUNTS['categories'])
        # The institution and its one role cite every new claim and source after the earlier packet's, and nothing else does.
        self.assertEqual(self.office['claim_ids'][-len(NEW_CLAIMS):], NEW_CLAIMS)
        self.assertEqual(self.role['claim_ids'][-len(NEW_CLAIMS):], NEW_CLAIMS)
        self.assertEqual(self.office['sources'][-len(NEW_SOURCES):], NEW_SOURCES)
        self.assertEqual(self.role['sources'][-len(NEW_SOURCES):], NEW_SOURCES)
        self.assertEqual(len(self.office['claim_ids']), len(set(self.office['claim_ids'])))
        for entry in self.packet['organizations'] + self.packet['institutions'][:-1]:
            self.assertFalse(set(self.new_claims) & set(entry['claim_ids']), entry['id'])
            self.assertFalse(set(NEW_SOURCES) & set(entry['sources']), entry['id'])
        # At most ten observations, each reported and each carrying rows.
        observations = re.findall(r'^### (JP-PM06-\d\d)\b', self.report, re.M)
        self.assertEqual(observations, [f'JP-PM06-{n:02d}' for n in range(1, 11)])
        self.assertEqual({row['review_observation'] for row in self.rows.values()},
                         {f'JP-PM06-{n:02d}' for n in range(1, 11)})
        later_raw = json.dumps(later_sources(self.packet) + self.role['holder_claims'][EARLIER_HOLDERS:], ensure_ascii=False)
        for stale in STALE_IDS:
            self.assertNotIn(stale, later_raw, stale)
            for extract in self.extracts.values():
                self.assertNotIn(stale, json.dumps(extract, ensure_ascii=False), stale)

    def test_holders_are_exactly_as_intended(self):
        pm_invariants(self.packet, self.rows)
        for holder in self.role['holder_claims'][EARLIER_HOLDERS:]:
            self.assertEqual(list(holder), ['name', 'attested_on', 'from', 'until', 'sources', 'claim_ids', 'note',
                                            'uncertainty'])
            self.assertTrue(holder['note'] and holder['uncertainty'], holder['name'])
            expected_sources = list(dict.fromkeys(self.claim_source[cid] for cid in holder['claim_ids']))
            self.assertEqual(holder['sources'], expected_sources, holder['name'])
            if holder['until']:
                self.assertTrue(holder['uncertainty'].startswith('The end is stated by the Official Gazette'), holder['name'])
                self.assertIn(f"Until {int(holder['until'][8:])} ", holder['note'])
            else:
                self.assertTrue(holder['uncertainty'].startswith('No end'), holder['name'])
            self.assertNotRegex(holder['note'] + holder['uncertainty'], r'(?i)until the \d+ \w+ \d{4} (appointment|ceremony)')
        for cid in self.new_claims:
            self.assertEqual((self.rows[cid]['role_title'], self.rows[cid]['role_id']), (T_PM, PM), cid)
        # Row holder names are pinned; a non-null one is a holder of this role whose surname the claim text prints.
        self.assertEqual({cid: row['holder_name'] for cid, row in self.rows.items()}, ROW_HOLDERS)
        self.assertEqual({v for v in ROW_HOLDERS.values()} - {None}, set(SURNAMES))
        for cid, name in ROW_HOLDERS.items():
            if name:
                self.assertIn(SURNAMES[name], self.claims[cid]['text'], cid)
        # The House of Councillors' designee of 2007 and 2008 is named only in text.
        for cid in ('jp_sangiin_designates_ozawa_runoff_20070925', 'jp_sangiin_designates_ozawa_runoff_20080924'):
            self.assertIsNone(ROW_HOLDERS[cid], cid)
            self.assertIn('小沢一郎', self.claims[cid]['text'], cid)
        # The earlier packet's holders and rows are untouched: its 2006 closing-boundary rows still name no one.
        self.assertEqual(len([h for h in self.role['holder_claims'] if (h['attested_on'] or h['from']) < CLOSING_BOUNDARY]),
                         EARLIER_HOLDERS)
        self.assertEqual([h['from'] for h in self.role['holder_claims'] if h['from'] == CLOSING_BOUNDARY], [CLOSING_BOUNDARY])

    def test_starts_ends_and_claims_that_never_feed_a_holder(self):
        claims, rows = self.claims, self.rows
        # Each House's designation vote, the Imperial appointment ceremony, the cabinet's formation, its resignation en
        # masse and continued duties are distinct dated claims on every transition day.
        for day, expected in TRANSITIONS.items():
            kinds = sorted({rows[cid]['event_kind'] for cid, (d, _k, _o) in EVENTS.items() if d == day})
            for kind in expected:
                self.assertIn(kind, kinds, (day, kind))
        for cid in DESIGNATIONS:
            self.assertNotIn(rows[cid]['event_kind'], HOLDER_KINDS, cid)
        self.assertEqual(sum(rows[c]['event_kind'] == 'house_of_representatives_resolution_prevails' for c in rows), 4)
        self.assertEqual(sum(rows[c]['event_kind'] == 'joint_committee_no_agreement' for c in rows), 2)
        # Resignations en masse, announcements and departures are never an end.
        for cid in RESIGNATIONS:
            self.assertRegex(claims[cid]['uncertainty'], r'(?i)not (an|the) end|no end|not used as an end|never an end|states no day|never a boundary', cid)
            self.assertNotIn(cid, HOLDER_CLAIM_SET)
        # Continued performance of duties and acting designations are claims only, never holders.
        for cid in CONTINUATION + ACTING:
            self.assertNotIn(cid, HOLDER_CLAIM_SET)
            self.assertRegex(claims[cid]['uncertainty'], r'claim only|claims only', cid)
        # Retrospective spans and undated recollections carry no structured date.
        for cid in RETROSPECTIVE:
            self.assertNotIn('attested_on', claims[cid], cid)
            self.assertIn('no structured date', claims[cid]['uncertainty'], cid)
        # Ends come only from the Official Gazette, and every Gazette end is the day that notice states.
        ends = [cid for cid in rows if rows[cid]['event_kind'] in UNTIL_KINDS]
        self.assertEqual(sorted((rows[c]['holder_name'], rows[c]['attested_on']) for c in ends), sorted(ENDS))
        for cid in ends:
            self.assertTrue(self.claim_source[cid].startswith('jp_kanpo_gogai_'), cid)
            self.assertRegex(claims[cid]['text'], r'本月[一二三四五六七八九十]+日')
        # The 2006 start rests on re-recorded rows that name the holder; the earlier packet's rows are not cited.
        abe = self.role['holder_claims'][EARLIER_HOLDERS]
        self.assertEqual((abe['name'], abe['from']), ('安倍晋三', CLOSING_BOUNDARY))
        for sid, earlier in REPEATED_RESPONSES.items():
            self.assertEqual((self.sources[sid]['url'], RESPONSES[sid]),
                             (self.sources[earlier]['url'], (self.extracts[sid]['source_response_bytes'],
                                                             self.extracts[sid]['source_response_sha256'])))
            earlier_extract = json.loads((research.ROOT / self.sources[earlier]['snapshot']['path']).read_text(encoding='utf-8'))
            self.assertEqual(earlier_extract['source_response_sha256'], RESPONSES[sid][1])
            self.assertTrue(all(r['holder_name'] is None for r in earlier_extract['rows']), earlier)
            self.assertFalse({r['claim_id'] for r in earlier_extract['rows']} & set(abe['claim_ids']))
        # The Hatoyama end conflict stays unresolved: neither 4 June nor 8 June 2010 is an end.
        hatoyama = next(h for h in self.role['holder_claims'] if h['name'] == '鳩山由紀夫')
        self.assertIsNone(hatoyama['until'])
        self.assertIn('conflict', hatoyama['uncertainty'])
        scope = self.role['scope_note']
        for text in ('never as a separate holder', 'procedure only, never a date', 'CLAUDE-C01-13 extends the role',
                     '内閣総理大臣及び国務大臣退官', 'never an end'):
            self.assertIn(text, scope)
        unresolved = self.office['coverage']['unresolved']
        self.assertTrue(unresolved[-1].startswith('Keep executive office distinct from party leadership'))
        self.assertEqual([u[:40] for u in unresolved[-4:-1]], [u[:40] for u in INSTITUTION_UNRESOLVED])
        self.assertTrue(self.packet['coverage']['unresolved'][-1].startswith('Prime ministers 2006-2026 (CLAUDE-C01-13'))
        self.assertEqual(sum('CLAUDE-C01-13' in u for u in self.packet['coverage']['unresolved']), 1)

    def test_extracts_match_packet_claims_and_record_original_responses(self):
        for sid in NEW_SOURCES:
            source, extract = self.sources[sid], self.extracts[sid]
            self.assertEqual(extract['format'], 'spheres-c01-derived-factual-table/v1')
            self.assertEqual((extract['source_id'], extract['source_url']), (sid, source['url']))
            for key in ('scope_note', 'access_method', 'published_date', 'rights_note', 'accessed_date'):
                self.assertEqual(extract[key], source[key], (sid, key))
            self.assertEqual(source['accessed_date'], ACCESSED[sid], sid)
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
            self.assertEqual(source['source_type'], SOURCE_TYPES[sid], sid)
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
                self.assertEqual(list(claim), ['id', 'text'] + (['attested_on'] if 'attested_on' in claim else []) +
                                 ['locator', 'uncertainty'])
        self.assertEqual(len({self.sources[s]['snapshot']['path'] for s in NEW_SOURCES}), len(NEW_SOURCES))
        earlier_paths = {s['snapshot']['path'] for s in self.packet['sources'][:ORIGINAL_COUNT + EARLIER_SOURCES] if 'snapshot' in s}
        self.assertFalse(earlier_paths & {self.sources[s]['snapshot']['path'] for s in NEW_SOURCES})
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
            if not source['url'].endswith('.pdf'):
                self.assertRegex(extract['source_character_encoding'], r'^(Shift_JIS|EUC-JP|ISO-2022-JP|UTF-8)')
        others = [sid for sid in NEW_SOURCES if sid not in ARCHIVED]
        self.assertEqual((len(ARCHIVED), len(others)), COUNTS['archived_others'])
        self.assertEqual({urlsplit(self.sources[s]['url']).hostname for s in others}, OFFICIAL_HOSTS)
        for sid in others:
            self.assertNotIn('original_url', self.sources[sid])
            self.assertNotIn('archive_capture_utc', self.extracts[sid])
            if urlsplit(self.sources[sid]['url']).hostname == 'kokkai.ndl.go.jp':
                record = self.extracts[sid]['diet_record']
                self.assertIn(record['issueID'], self.sources[sid]['url'])
                self.assertEqual(record['record_page_url'], f"https://kokkai.ndl.go.jp/txt/{record['issueID']}")
            else:
                # A stored official page: its Last-Modified precedes the cutoff and a cache-busting request matched.
                self.assertIn(sid, LIVE_OFFICIAL)
                note = self.extracts[sid]['provenance_note']
                self.assertIn('(cache-busting query)', note, sid)
                modified = re.search(r'fixed Last-Modified \(\w{3}, (\d\d) (\w{3}) (\d{4})', note)
                month = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'].index(modified.group(2)) + 1
                self.assertLess(f'{modified.group(3)}-{month:02d}-{modified.group(1)}', research.CUTOFF, sid)
        for sid, (url, size, sha) in ALTERNATES.items():
            alternate = self.extracts[sid]['alternate_location']
            self.assertEqual((alternate['url'], alternate['bytes'], alternate['sha256']), (url, size, sha))
            self.assertNotEqual(sha, RESPONSES[sid][1])

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
            if parts.hostname == 'kokkai.ndl.go.jp':
                self.assertRegex(url, r'^https://kokkai\.ndl\.go\.jp/api/(meeting\?issueID=\w+|speech\?(speechID=\w+|'
                                      r'issueID=\w+&speechNumber=\d+))&recordPacking=json$')
        # Live Kantei and Imperial Household Agency pages are exactly the pinned stored files.
        self.assertEqual([s for s in NEW_SOURCES if re.match(r'^https://www\.(kantei|kunaicho)\.go\.jp', self.sources[s]['url'])],
                         LIVE_OFFICIAL)
        # One response, one source, except the three deliberately re-recorded 2006 responses.
        seen = {}
        for source in self.packet['sources']:
            seen.setdefault(source['url'], []).append(source['id'])
        repeated = {tuple(v) for v in seen.values() if len(v) > 1}
        self.assertEqual(repeated, {(earlier, sid) for sid, earlier in REPEATED_RESPONSES.items()})

    def test_secondary_and_unimported_leads_stay_out_of_the_packet(self):
        for sid in NEW_SOURCES:
            for marker in LEAD_URL_MARKERS:
                self.assertNotIn(marker, self.sources[sid]['url'], sid)
        lowered = self.raw.lower()
        for marker in ('wikipedia', 'britannica', 'nikkei', 'asahi.com', 'nhk.or.jp', 'yomiuri', 'mainichi', 'kotobank'):
            self.assertNotIn(marker, lowered, marker)
        leads, added = self.section('Leads not imported'), self.section('Sources added')
        for marker in LEAD_REPORT_MARKERS:
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
            return role(packet)['holder_claims'][EARLIER_HOLDERS + index]

        def cite(index, cid, **dates):
            def change(packet):
                holder(packet, index).update(dates)
                holder(packet, index)['claim_ids'].append(cid)
                sid = next(s['id'] for s in packet['sources'] for c in s['claims'] if c['id'] == cid)
                if sid not in holder(packet, index)['sources']:
                    holder(packet, index)['sources'].append(sid)
            return change

        index = {(h[0], h[2]): i for i, h in enumerate(HOLDERS)}
        abe06, fukuda, aso, hatoyama = index[('安倍晋三', '2006-09-26')], index[('福田康夫', '2007-09-26')], \
            index[('麻生太郎', '2008-09-24')], index[('鳩山由紀夫', '2009-09-16')]
        kan, abe17, suga = index[('菅直人', '2010-06-08')], index[('安倍晋三', '2017-11-01')], index[('菅義偉', '2020-09-16')]
        takaichi2 = index[('高市早苗', '2026-02-18')]
        validator_cases = [
            (lambda p: source(p, 'jp_kanpo_gogai_toku99_20200916_p2')['snapshot'].update(sha256='0' * 64), 'checksum mismatch'),
            (lambda p: source(p, 'jp_shugiin_honkaigi_20070925')['snapshot'].update(bytes=1), 'checksum mismatch'),
            (lambda p: source(p, 'jp_kantei_rekidai_090')['snapshot'].update(path=REPORT.as_posix()), 'escapes'),
            (lambda p: holder(p, takaichi2).update(until='2026-09-08'), 'exceeds cutoff'),
            (lambda p: claim(p, 'jp_kantei_takaichi_in_office_20260904').update(attested_on='2026-09-08'), 'exceeds cutoff'),
            (lambda p: holder(p, suga).update(until='2020-01-01'), 'Reversed historical interval'),
            (lambda p: holder(p, fukuda)['claim_ids'].append('jp_kunaicho_ceremony_aso_20080924'), 'cited source'),
            (lambda p: role(p)['claim_ids'].append('jp_does_not_exist'), 'Unknown'),
        ]
        for change, message in validator_cases:
            with self.subTest(message=message), self.assertRaisesRegex(ValueError, message):
                self.validate(mutated(change))

        def extra_holder(name, day, cid, start=True):
            return {'name': name, 'attested_on': None if start else day, 'from': day if start else None, 'until': None,
                    'sources': [self.claim_source[cid]], 'claim_ids': [cid], 'note': 'x', 'uncertainty': 'x'}

        rule_cases = [
            # A successor's designation, appointment or ceremony used as an end.
            ('successor start used as an end (Aso)', lambda p: holder(p, aso).update(until='2009-09-16')),
            ('successor ceremony cited as an end (Aso)',
             cite(aso, 'jp_kunaicho_ceremony_hatoyama_20090916', until='2009-09-16')),
            ('successor start used as an end (Abe 2006)', lambda p: holder(p, abe06).update(until='2007-09-26')),
            ('successor appointment cited as an end (Fukuda)', cite(fukuda, 'jp_aso_shinninshiki_appointed_20080924',
                                                                    until='2008-09-24')),
            ('successor start used as an end (Takaichi 2026)', lambda p: holder(p, takaichi2).update(until='2026-09-07')),
            # A resignation, an announcement, a departure or a retrospective span used as an end.
            ('resignation notice cited as an end (Kan)', cite(kan, 'jp_shugiin_honkaigi_kan_resignation_notice_20110830',
                                                              until='2011-08-30')),
            ('departure cited as an end (Kan)', cite(kan, 'jp_kan_leaves_kantei_20110902', until='2011-09-02')),
            ('retrospective span used as an end (Hatoyama)', cite(hatoyama, 'jp_kantei_span_hatoyama_93_20090916_20100608',
                                                                   until='2010-06-08')),
            ("the Chief Cabinet Secretary's span used as an end (Hatoyama)",
             cite(hatoyama, 'jp_ccs_states_hatoyama_cabinet_span_262_days_20100604', until='2010-06-04')),
            ("'today' remarks used as an end (Hatoyama)", cite(hatoyama, 'jp_hatoyama_last_cabinet_meeting_remarks_20100604',
                                                               until='2010-06-04')),
            ('Gazette end dropped but the until kept (Abe 2017)',
             lambda p: holder(p, abe17)['claim_ids'].remove('jp_kanpo_abe_office_lost_20200916')),
            ('Gazette end left off its holder (Suga)', lambda p: holder(p, suga).update(until=None)),
            # A designation date used as a start.
            ('designation date used as a start (Kan)', lambda p: holder(p, kan).update({'from': '2010-06-04'})),
            ('designation cited as a start (Kan)', cite(kan, 'jp_shugiin_designates_kan_20100604')),
            ('designation date used as a start (Fukuda)', lambda p: holder(p, fukuda).update({'from': '2007-09-25'})),
            ('prevailing resolution cited by a holder (Aso)', cite(aso, 'jp_shugiin_hr_resolution_prevails_aso_20080924')),
            ('unnamed statement cited as a start (Suga)', cite(suga, 'jp_kantei_pm_statement_assumes_office_20200916')),
            ('unnamed schedule row cited as a start (Fukuda)', cite(fukuda, 'jp_kunaicho_ceremony_20070926')),
            ('named start left off its holder (Suga)',
             lambda p: holder(p, suga)['claim_ids'].remove('jp_cabinet_minutes_suga_appointment_statement_20200916')),
            # Acting service, an acting designation or continued duties added as a holder, or cited by one.
            ('acting designation added as a holder (2007)', lambda p: role(p)['holder_claims'].insert(
                EARLIER_HOLDERS + fukuda + 1, extra_holder('町村信孝', '2007-09-27',
                                                            'jp_ccs_reports_fukuda_acting_order_designated_20070926'))),
            ('acting designation cited by a holder (Aso)', cite(aso, 'jp_ccs_reports_aso_acting_order_designated_20080925')),
            ('continued duties added as a holder (Kan 2011)', lambda p: role(p)['holder_claims'].insert(
                EARLIER_HOLDERS + kan + 1, extra_holder('菅直人', '2011-09-01', 'jp_kan_leads_disaster_drill_continued_duties_20110901',
                                                        start=False))),
            ('continued duties cited by a holder (Hatoyama)',
             cite(hatoyama, 'jp_member_states_hatoyama_caretaker_cabinet_20100604_20100608')),
            # A cross-role or cross-institution holder.
            ('party-president holder cited by jp_pm', lambda p: holder(p, takaichi2)['claim_ids'].append(
                next(r for o in p['organizations'] for r in o['roles'] if r['id'] == 'jp_ldp_party_president')['holder_claims'][1]['claim_ids'][0])),
            ('jp_pm holder moved onto the party presidency', lambda p: next(
                r for o in p['organizations'] for r in o['roles'] if r['id'] == 'jp_ldp_party_president')['holder_claims'].append(
                    copy.deepcopy(holder(p, takaichi2)))),
            ('parliamentary-group claim cited by a holder', lambda p: holder(p, suga)['claim_ids'].append('jp_house_group_not_party')),
            ('earlier packet claim cited by a later holder', cite(abe06, 'jp_abe_appointed_pm_statement_20060926')),
            ('second role', lambda p: p['institutions'][-1]['roles'].append(dict(copy.deepcopy(role(p)), id='jp_pm_2'))),
            ('second institution', lambda p: p['institutions'].append(dict(copy.deepcopy(p['institutions'][-1]),
                                                                           id='jp_prime_minister_2'))),
            # Structured dates that are never holder dates, spans given a date, order.
            ('recollection used as a start (Fukuda)', lambda p: holder(p, fukuda).update({'from': '2007-09-25'})),
            ('span given a structured date', lambda p: claim(p, 'jp_kantei_span_abe_98_20171101_20200916').update(
                attested_on='2017-11-01')),
            ('span stored as a structured period', lambda p: claim(p, 'jp_kantei_span_kan_94_20100608_20110902').update(
                period={'from': '2010-06-08', 'through': '2011-09-02'})),
            ('holders out of order', lambda p: role(p)['holder_claims'].reverse()),
            ('beyond-cutoff end', lambda p: holder(p, takaichi2).update(until='2026-09-08')),
        ]
        pm_rules(self.packet, self.rows)
        pm_invariants(self.packet, self.rows)
        for label, change in rule_cases:
            with self.subTest(label=label):
                packet = mutated(change)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    pm_rules(packet, self.rows)
                with self.assertRaises((AssertionError, KeyError, IndexError, ValueError, StopIteration)):
                    pm_invariants(packet, self.rows)
        # Row-level mutations: an acting row or an unnamed start given a holder name, or an end with no Gazette source.
        for cid, name in (('jp_ccs_reports_fukuda_acting_order_designated_20070926', '福田康夫'),
                          ('jp_kantei_pm_statement_assumes_office_20200916', '菅義偉'),
                          ('jp_kunaicho_ceremony_20241001', '石破茂')):
            rows = copy.deepcopy(self.rows)
            rows[cid]['holder_name'] = name
            with self.subTest(named=cid), self.assertRaises(AssertionError):
                pm_rules(self.packet, rows)
        rows = copy.deepcopy(self.rows)
        rows['jp_kantei_span_hatoyama_93_20090916_20100608']['event_kind'] = 'end_of_office_stated'
        with self.subTest(end='retrospective span relabelled as an end'), self.assertRaises(AssertionError):
            pm_rules(self.packet, rows)
        # Event collapses are caught by the pinned events.
        for cid, day in (('jp_shugiin_designates_fukuda_20070925', '2007-09-26'),
                         ('jp_hatoyama_cabinet_resignation_decided_20100604', '2010-06-08'),
                         ('jp_kanpo_suga_office_lost_20211004', '2021-10-05'),
                         ('jp_kan_leads_disaster_drill_continued_duties_20110901', '2011-09-02')):
            with self.subTest(collapsed=cid), self.assertRaises(AssertionError):
                pm_invariants(mutated(lambda p: claim(p, cid).update(attested_on=day)), self.rows)

    def test_report_and_handoff_close_no_parent_gate(self):
        self.assertIn('ready_for_review', self.report)
        table = self.section('Outcome')
        for number, decision in DECISIONS.items():
            row, = [line for line in table.splitlines() if line.startswith(f'| JP-PM06-{number} ')]
            self.assertIn(f'**{decision}:**', row)
        defects = self.section('Checker defects')
        rows = [line for line in defects.splitlines() if re.match(r'\| [ABC]\d+ ', line)]
        self.assertEqual([re.match(r'\| ([ABC]\d+) ', r).group(1) for r in rows],
                         [f'A{n}' for n in range(1, 16)] + [f'B{n}' for n in range(1, 7)] + [f'C{n}' for n in range(1, 14)])
        for row in rows:
            self.assertRegex(row, r'\*\*(Applied|Resolved|Declined)')
        for marker in ('C01', 'C06', 'S23', 'WC1', 'CP1'):
            self.assertNotRegex(self.report, rf'\b{marker}\b[^.\n]*\bis (now )?complete\b')
        notes = self.section('Integration notes')
        for text in ('3bcd6e6c', 'claude/c01-jp-12', 'da358dbf', 'research-index.json', 'test_japan_research_s10d.py',
                     'test_japan_prime_ministers_c01_12.py', 'test_campaign_census', 'Observed on', 'only file'):
            self.assertIn(text, notes)
        identities = self.section('Response identities and stability checks')
        for sid, (size, sha) in RESPONSES.items():
            self.assertIn(f'`{sid}`', identities, sid)
            self.assertIn(sha[:12], identities, sid)
        handoff = (research.ROOT / HANDOFF).read_text(encoding='utf-8')
        for text in ('ready_for_review', 'japan-prime-ministers-2006-2026-13.md', 'claude/c01-jp-13', '3bcd6e6c',
                     'test_japan_prime_ministers_c01_13.py', 'claude/c01-jp-12'):
            self.assertIn(text, handoff)
        index = research.build()
        country = next(p for p in index['countries'] if p['nation'] == 'Japan')
        self.assertFalse(country['country_census_complete'])
        self.assertIsNone(country['unrepresented_organization_count'])
        self.assertEqual((country['institution_observations'], country['role_observations']), (8, 5))


if __name__ == '__main__':
    unittest.main()
