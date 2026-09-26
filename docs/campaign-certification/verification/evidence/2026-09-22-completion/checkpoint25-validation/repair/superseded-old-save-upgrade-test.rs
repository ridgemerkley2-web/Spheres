    #[test]
    fn army_presence_old_saves_upgrade_once_without_overwriting_live_loyalty() {
        let mut w = world_1990(GameRules::default());
        let original = crate::save(&w);
        w = crate::load(&original).unwrap();
        for id in opening_institutions().iter().map(|r| r.nation) { government::ensure(&mut w, id); }
        assert_eq!(crate::save(&w), original, "default/off saves are byte stable");
        w.rules.ideology_blocs = true;
        for id in opening_institutions().iter().map(|r| r.nation) {
            let before = government::state(&w, id).unwrap().clone();
            government::ensure(&mut w, id);
            w.governments.states.iter_mut().find(|g| g.nation == id).unwrap().pillars.iter_mut()
                .find(|(p, _)| *p == Pillar::Army).unwrap().1 = 0.72;
            government::ensure(&mut w, id);
            let g = government::state(&w, id).unwrap();
            assert_eq!(g.pillars.iter().filter(|(p, _)| *p == Pillar::Army).count(), 1);
            assert_eq!(g.loyalty(Pillar::Army), 0.72);
            assert_eq!(g.support, before.support);
            assert_eq!(g.seats, before.seats);
            assert_eq!(g.coalition, before.coalition);
            assert_eq!(g.next_election, before.next_election);
            assert_eq!(g.elected, before.elected);
            assert_eq!(g.months_in_office, before.months_in_office);
            assert_eq!(g.coup_pressure, before.coup_pressure);
            assert_eq!(g.office_month_fraction, before.office_month_fraction);
            for (pillar, loyalty) in before.pillars {
                assert_eq!(g.loyalty(pillar), loyalty, "existing institution is not reseated");
            }
        }
        let saved = crate::save(&w);
        assert_eq!(crate::save(&crate::load(&saved).unwrap()), saved);
    }