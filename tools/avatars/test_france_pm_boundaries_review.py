"""Review regression: dated PM instruments are not explicit effective term boundaries."""
import copy
import json
import unittest
import import_cnccfp_census as importer


def observation_only_contract(packet):
    role = next(i for i in packet['institutions'] if i['id'] == 'fr_prime_minister')['roles'][0]
    claims = {c['id']: c for source in packet['sources'] for c in source['claims']}
    for holder in role['holder_claims']:
        assert holder['from'] is None and holder['until'] is None, 'signature/publication is not an explicit effective boundary'
        assert holder['attested_on'] == claims[holder['claim_ids'][0]]['attested_on'], 'preserve actual dated observation'
        assert holder['attested_on'] is not None
    return role


class PrimeMinisterBoundaryReviewTests(unittest.TestCase):
    def setUp(self):
        self.supplement = json.loads((importer.ROOT / importer.SUPPLEMENT).read_text(encoding='utf-8'))

    def test_effective_boundaries_are_not_inferred_from_instrument_dates(self):
        role = observation_only_contract(self.supplement)
        # CLAUDE-C01-37: 16 observations of 10 people; CLAUDE-C01-38 appends 12 observations of 9 more people.
        self.assertEqual(len(role['holder_claims']), 16 + 12)
        self.assertEqual(len({h['name'] for h in role['holder_claims']}), 10 + 9)

    def test_signature_and_publication_promotion_is_rejected(self):
        observation_only_contract(self.supplement)
        claims = {c['id']: (source, c) for source in self.supplement['sources'] for c in source['claims']}
        role = self.supplement['institutions'][1]['roles'][0]
        cases = 0
        for index, holder in enumerate(role['holder_claims']):
            for cid in holder['claim_ids']:
                source, claim = claims[cid]
                for label, day in [('signature', claim['attested_on']), ('publication', source['published_date'])]:
                    for field in ['from', 'until']:
                        altered = copy.deepcopy(self.supplement)
                        altered['institutions'][1]['roles'][0]['holder_claims'][index][field] = day
                        with self.subTest(index=index, claim=cid, date=label, field=field):
                            with self.assertRaisesRegex(AssertionError, 'explicit effective boundary'):
                                observation_only_contract(altered)
                        cases += 1
        self.assertGreater(cases, 100)


if __name__ == '__main__':
    unittest.main()
