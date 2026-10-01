"""New CPSU citations cannot hide invented holders from the prior-history guard."""
import copy
import json
import unittest

import campaign_research as research
from test_ussr_democratic_russia_soyuz_c01_35 import c35_invariants, roles_of


class PriorHolderGuard(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.ussr = json.loads((research.ROOT / research.RESEARCH / 'ussr.json').read_text(encoding='utf-8'))
        cls.russia = json.loads((research.ROOT / research.RESEARCH / 'russia.json').read_text(encoding='utf-8'))

    def test_new_cpsu_citation_cannot_hide_extra_state_office_holder(self):
        packet = copy.deepcopy(self.ussr)
        roles = roles_of(packet)
        fabricated = copy.deepcopy(roles['su_cpsu_general_secretary'][1]['holder_claims'][1])
        fabricated['name'] = 'Invented additional president'
        roles['su_president'][1]['holder_claims'].append(fabricated)
        with self.assertRaises(AssertionError):
            c35_invariants(packet, self.russia)

    def test_mixed_old_and_new_citations_cannot_hide_extra_party_holder(self):
        packet = copy.deepcopy(self.ussr)
        role = roles_of(packet)['su_cpsu_general_secretary'][1]
        fabricated = copy.deepcopy(role['holder_claims'][1])
        fabricated['name'] = 'Invented additional general secretary'
        fabricated['sources'].append('su_japan_diplomatic_bluebook_1990')
        role['holder_claims'].append(fabricated)
        with self.assertRaises(AssertionError):
            c35_invariants(packet, self.russia)


if __name__ == '__main__':
    unittest.main()
