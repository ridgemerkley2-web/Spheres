from copy import deepcopy
import unittest
from export_leadership_review import portrait_at, enrich_board, build


class StaticReviewTests(unittest.TestCase):
    def art(self, name, first='1990-01-01', last='1995-01-01'):
        return {'asset':'spheres-web/ui/person-portraits/'+name+'.png','from':first,'to':last,'style':'cartoon'}

    def test_exact_half_open_era_and_ambiguous_art_never_fall_back(self):
        records=[self.art('first'),self.art('later','1995-01-01','2000-01-01')]
        self.assertIsNone(portrait_at(records,'1989-12-31'))
        self.assertEqual(portrait_at(records,'1990-01-01')['url'],'/art/people/first.png')
        self.assertEqual(portrait_at(records,'1995-01-01')['url'],'/art/people/later.png')
        self.assertIsNone(portrait_at(records,'2000-01-01'))
        self.assertIsNone(portrait_at(records+[records[0]],'1990-01-01'))
        self.assertIsNone(portrait_at([self.art('../escape')],'1990-01-01'))

    def test_campaign_and_future_dates_are_separate_and_source_is_immutable(self):
        real={'id':'real','name':'Historical person'}
        fiction={'id':'fiction','name':'Invented person','fiction':{'origin':'fictional_successor'}}
        board={'date':'2030-01-01','campaign_date':'1990-01-01','executive_person':real,
            'parties':[{'campaign':[{'person':real}], 'historical':[{'person':real}],
                'future_preview':[{'person':fiction}],'future_candidates':[{'person':fiction}]}]}
        original=deepcopy(board)
        enriched=enrich_board(board,{'real':[self.art('real')]},{'fiction':[self.art('fiction','2026-09-08','2036-01-01')]})
        party=enriched['parties'][0]
        self.assertEqual(party['campaign'][0]['person']['portrait']['url'],'/art/people/real.png')
        self.assertIsNone(party['historical'][0]['person']['portrait'])
        self.assertEqual(party['future_preview'][0]['person']['portrait']['preview_date'],'2026-09-08')
        self.assertNotIn('preview_date',party['future_candidates'][0]['person']['portrait'])
        self.assertEqual(board,original)

    def test_fiction_cannot_borrow_real_identity_art_and_invalid_native_contract_fails(self):
        person={'id':'fiction','name':'Invented person','fiction':{'origin':'fictional_successor'}}
        board={'date':'1990-01-01','campaign_date':'1990-01-01','future_preview':[{'person':person}]}
        self.assertIsNone(enrich_board(board,{'fiction':[self.art('real')]},{})['future_preview'][0]['person']['portrait'])
        with self.assertRaises(ValueError): build({'read_only':False},{},{})


if __name__=='__main__': unittest.main()
