# USSR Democratic Russia and Soyuz 35: the co-chairs of the Democratic Russia movement and of the Soyuz deputies' group, 1990-1991

Packet: **CLAUDE-C01-35**. State: **ready_for_review** (not complete).
Owner: Claude. Integrator/reviewer: Codex. Branch `claude/c01-su-35`; claim commit `c1475ada` on `44098c5a`, the head of
`codex/campaign-certification` when the packet was claimed; **not stacked** on any pending packet. Research access: 29 September
2026 (UTC; 28 September local, UTC-7); every recorded download and both of this packet's download passes fall on that UTC day.
The historical cutoff stays **7 September 2026**.

This packet reviews ten observations, SU-DR-01 to SU-DR-05 and SU-SOYUZ-01 to SU-SOYUZ-05, in [ussr.json](ussr.json). It adds
two organization observations, each with one co-leadership role:

- `su_democratic_russia`, "Движение «Демократическая Россия» — Democratic Russia movement" (kind `political_movement`,
  jurisdiction USSR at republic level, RSFSR), with the role `su_dr_co_chair` ("Сопредседатель движения «Демократическая Россия»
  (Координационного совета) — Co-chair of the Democratic Russia movement", kind `party_leader`) and seven holder observations
  of five co-chairs;
- `su_soyuz_deputies_group`, "Депутатская группа «Союз» — Soyuz deputies' group" (kind `deputies_group`, union level), with the
  role `su_soyuz_co_chair` (kind `parliamentary_leader`) and one holder observation.

In all it adds 16 sources and 36 claims, each source with a checked-in derived extract (154,583 bytes for the 16 extracts), and
one packet coverage item. It changes no existing source, claim, extract, entry, role or holder, adds no institution, game
mapping, portrait or avatar, and does not touch `russia.json`. No holder has a `from` or an `until`. Co-chairs are recorded as
co-leadership: every holder observation is one person, and several co-chairs share the same days. The parent scope (C01, C06,
S23, WC1 and CP1) remains open.

The Democratic Russia part rests on a research dossier made by a sub-agent (21 sources and 40 claims proposed; 8 sources and 16
claims imported, the rest belonging to other identities or stating no office), the Soyuz part on this packet's own reading of the
USSR Congress and Supreme Soviet records. An independent adversarial check re-downloaded and re-read each part; all 14 defects
of the Soyuz check and all five of the Democratic Russia check are applied, and seven claims from primary records the Soyuz
check found are imported (see [Independent checks](#independent-checks)).

## Outcome

| ID | Question | Decision |
|---|---|---|
| SU-DR-01 | The movement's founding, its bodies and its separation from the 1990 bloc and the RSFSR deputies' group | **Accepted in part:** the RSFSR deputies' bloc calls for a movement (22 Jun 1990); bloc members are to elect the movement's Council of Representatives (7 Dec 1990); the movement's Coordinating Council applies for the march of 28 Mar 1991; a plenum of the Council of Representatives (15 Sep 1991). No primary record of the founding congress |
| SU-DR-02 | Leadership before the co-chairs are attested (the organizing committee) | **Accepted in part:** two USSR deputies call Мурашев "председатель оргкомитета движения «Демократическая Россия»" (17 and 19 Dec 1990); Заславский calls himself a member of its coordinating council. Claims on the organization only, never holders |
| SU-DR-03 | The co-chairs attested between 1990 and 1991 | **Accepted in part:** five co-chairs, seven observations: Дмитриев (5 Apr 1991, his own words at the RSFSR Congress), Афанасьев (15 Jul 1991, his own words; 12 Sep 1991), Пономарев (12 Sep and 12 Dec 1991), Мурашев (12 Sep 1991), Якунин (12 Dec 1991), from the Coordinating Council's letter signed by three co-chairs and the RSFSR President's office list; one intermediate signature (15 Sep 1991) is a role claim |
| SU-DR-04 | The second congress | **Accepted in part:** the Coordinating Council's invitation to the second congress, 9-10 November (year not printed); no record of its proceedings or elections |
| SU-DR-05 | The end of the movement or of any co-chair's office by 25 Dec 1991 | **Not found:** no reviewed record states an end; no `until` |
| SU-SOYUZ-01 | The group in the Congress's records | **Accepted:** a deputy speaks for the group (12 Mar 1990); its statement, membership (more than 300) and nominations (13-15 Mar 1990); its meetings' decisions, nominees and rotation list (17-27 Dec 1990), with the nominees' full names in resolution 1842-I. Claims on the organization only |
| SU-SOYUZ-02 | The co-chairs | **Accepted in part:** one co-chair attested, Анатолий Георгиевич Чехоев, in his own words "Как один из сопредседателей депутатской группы «Союз»" (20 Dec 1990); the other co-chairs are not named in any reviewed record |
| SU-SOYUZ-03 | Registration and size | **Accepted:** the presiding officer names the group first in the roll of groups registered by the Supreme Soviet, and 561 deputies register as its members (25 Dec 1990) |
| SU-SOYUZ-04 | Leaders named by other deputies | **Accepted in part:** "руководителя группы «Союз»" (18 Dec 1990), unnamed "лидеры" or "руководители" (26 Dec 1990, 26 Aug 1991, 3 Sep 1991) and an accusing proposal naming "ее лидеров ... Когана, Алксниса, Блохина, Чехоева и Петрушенко" (26 Aug 1991); Коган calls himself an active member. Role claims, never holders |
| SU-SOYUZ-05 | The group's end | **Not found:** the latest reviewed record is of 3 Sep 1991; no end is recorded |

### Holders

`su_dr_co_chair` (new role; co-leadership):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Виктор Владимирович Дмитриев | 1991-04-05 | null | null | `su_dr_dmitriev_one_of_co_chairs_19910405` |
| 2 | Юрий Николаевич Афанасьев | 1991-07-15 | null | null | `su_dr_afanasyev_co_chair_statement_19910715` |
| 3 | Юрий Николаевич Афанасьев | 1991-09-12 | null | null | `su_dr_afanasyev_signs_as_co_chair_19910912` |
| 4 | Лев Александрович Пономарев | 1991-09-12 | null | null | `su_dr_ponomarev_signs_as_co_chair_19910912` |
| 5 | А. Мурашев | 1991-09-12 | null | null | `su_dr_murashev_signs_as_co_chair_19910912` |
| 6 | Глеб Павлович Якунин | 1991-12-12 | null | null | `su_dr_yakunin_listed_as_co_chair_19911212` |
| 7 | Лев Александрович Пономарев | 1991-12-12 | null | null | `su_dr_ponomarev_listed_as_co_chair_19911212` |

`su_soyuz_co_chair` (new role; co-leadership):

| # | Name | `attested_on` | `from` | `until` | Claim |
|---|---|---|---|---|---|
| 1 | Анатолий Георгиевич Чехоев | 1990-12-20 | null | null | `su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220` |

### Co-leadership, starts and ends

The movement's co-chairs sign together ("Сопредседатели КС „Дем. России“" over three names), are listed together
("сопредседатели Координационного совета: Якунин Глеб Павлович, Пономарев Лев Александрович") and speak as "один из" or "как
сопредседатель"; the Soyuz co-chair speaks "как один из сопредседателей". So each observation is one person, never a list of
names, and none is recorded as the sole leader; three observations share 12 September 1991 and two share 12 December 1991. The
test pins that no holder name joins two people and that each observation rests on one claim.

A holder rests on a record printing the person in the office on that record's own day: his own words in an official stenogram
(Дмитриев, Афанасьев, Чехоев), the co-chairs' signatures on the Coordinating Council's letter, or the RSFSR President's office
list of 12 December 1991. Each person has an observation at the earliest and at the latest reviewed attestation; Пономарев's
signature on the list approved on 15 September 1991 lies between his two and is a role claim. `from` would need a source stating
the day a co-chair was elected or took office, and `until` one stating the day the office ended; none does, so every observation
carries `attested_on` only. Speaking or relaying for a group, other deputies' descriptions ("руководитель", "лидеры",
"координатор", "председатель оргкомитета"), membership statements, congresses, appeals and registrations are dated claims and
never feed a holder, and no end is taken from another co-chair's later attestation, the second congress or the Union's end.

Names. Holder names are the given-name, patronymic, surname form wherever a reviewed source prints the name in full (the
CLAUDE-C01-19 and C01-26 convention): "Юрий Николаевич Афанасьев" (the presiding officer on 15 July 1991), "Виктор Владимирович
Дмитриев" (the presiding officer's dative "Дмитриеву Виктору Владимировичу"), "Лев Александрович Пономарев" and "Глеб Павлович
Якунин" (the list of 12 December 1991) and "Анатолий Георгиевич Чехоев" (resolution 1842-I, whose description of him matches his
speaker line). Мурашев is printed only as "А. Мурашев" and "Мурашев А. Н.", so his holder name stays "А. Мурашев". Extract rows
keep the printed forms in `persons_named`; other people appear only there, with `holder_name` null.

### Separate identities

Three bodies called «Демократическая Россия» appear in the reviewed records and are not merged: the 1990 electoral bloc, the
deputies' group, bloc or faction in the RSFSR Congress of People's Deputies (registered on 25 May 1990 with 66 members, with its
own coordinating council; a faction coordinator is listed on 11 December 1991), and the movement. Only the movement is filed. The
bloc's appeal of 22 June 1990 to form "массовое общественно-политическое движение ”Демократическая Россия”" and the announcement
of 7 December 1990 that bloc members would elect "совет представителей движения" are filed on the movement because they bear on
it, with the bloc named as a separate body; the group and faction records are read, not imported. The Soyuz deputies' group is
not assumed to be any later movement or party called «Союз».

Neither organization is mapped to the simulation's party rows `USSR/su_dr` ("Democratic Russia") and `USSR/su_soyuz` ("Soyuz
group"): a matching name is not an identity, and `represented_party_ids` stays empty for the integrator.

### Movement and group offices and state offices

The two roles are party and deputies'-group offices. No claim or source of this packet feeds a state role (`su_president`,
`su_government_head`, `su_supreme_soviet_chair`, `su_congress_deputies`) or the CPSU roles, and no state claim feeds the two new
roles; the test pins both directions and that every existing USSR holder is unchanged. Records of the RSFSR President's office
and of the RSFSR and USSR Congresses are used only as records naming the movement's or group's officers. Nothing is added to
`russia.json`.

### Date ledger

Each row is a separate dated fact with its own claim; two facts on one day stay two claims.

| Date | Events | Holder field and claims |
|---|---|---|
| 12 Mar 1990 | a deputy speaks for the Soyuz group | `su_snd3_deputy_speaks_for_soyuz_19900312` |
| 13 Mar 1990 | the group's statement; its pre-Congress meeting reported (the meeting undated); more than 300 members | `su_snd3_blokhin_delivers_soyuz_statement_19900313`, `su_snd3_blokhin_reports_soyuz_pre_congress_meeting_19900313`, `su_snd3_blokhin_soyuz_membership_19900313` |
| 14 Mar 1990 | the group's presidential nominations relayed; the presiding officer on its size | `su_snd3_alksnis_relays_soyuz_presidential_nominations_19900314`, `su_snd3_presiding_officer_soyuz_over_300_19900314` |
| 15 Mar 1990 | the group's nominations for Chairman of the Supreme Soviet relayed | `su_snd3_kim_relays_soyuz_chair_nominations_19900315` |
| 22 Jun 1990 | the RSFSR deputies' bloc calls for a movement | `su_dr_bloc_appeal_calls_for_movement_19900622` |
| 7 Dec 1990 | election to the movement's Council of Representatives announced | `su_dr_representatives_council_election_announced_19901207` |
| 17 Dec 1990 | the Soyuz group's meeting decision and nominees; the organizing-committee description of Мурашев; resolution 1842-I names the nominees | `su_snd4_gninenko_reports_soyuz_meeting_decision_19901217`, `su_snd4_soyuz_commission_nominees_general_meeting_19901217`, `su_snd4_golyakov_murashev_dr_orgcommittee_chair_19901217`, `su_snd4_res_1842i_names_soyuz_nominees_19901217` |
| 18 Dec 1990 | "руководителя группы «Союз»" | `su_snd4_bisher_refers_to_soyuz_leader_19901218` |
| 19 Dec 1990 | the organizing-committee description repeated; a coordinating council member | `su_snd4_zaslavsky_murashev_dr_orgcommittee_chair_19901219`, `su_snd4_zaslavsky_dr_council_member_19901219` |
| 20 Dec 1990 | a Soyuz co-chair in his own words | Soyuz holder `attested_on`; `su_snd4_chekhoev_one_of_soyuz_co_chairs_19901220` |
| 25 Dec 1990 | registration roll: Soyuz first, 561 deputies | `su_snd4_soyuz_registration_roll_19901225` |
| 26 Dec 1990 | unnamed "лидеры"; the group's rotation list | `su_snd4_sazonov_refers_to_soyuz_leaders_19901226`, `su_snd4_kogan_reports_soyuz_rotation_list_19901226` |
| 27 Dec 1990 | the group's meeting in a break reported | `su_snd4_chekhoev_reports_soyuz_meeting_19901227` |
| 28 Mar 1991 | the movement's Coordinating Council's application for the march | `su_dr_coordinating_council_march_application_19910328` |
| 5 Apr 1991 | a movement co-chair in his own words | movement holder `attested_on`; `su_dr_dmitriev_one_of_co_chairs_19910405` |
| 15 Jul 1991 | a movement co-chair in his own words | movement holder `attested_on`; `su_dr_afanasyev_co_chair_statement_19910715` |
| 26 Aug 1991 | unnamed "руководители"; the accusing proposal naming five "лидеры"; a member's reply | `su_vs26_bogdanov_refers_to_soyuz_leaders_19910826`, `su_vs26_boyars_proposal_names_soyuz_leaders_19910826`, `su_vs26_kogan_active_member_of_soyuz_19910826` |
| 3 Sep 1991 | unnamed "лидеров группы" (the latest Soyuz record) | `su_snd5_kraiko_refers_to_soyuz_leaders_19910903` |
| 12 Sep 1991 | the Coordinating Council's letter signed by three co-chairs (the date it bears; registered 12 Oct 1991) | three movement holders `attested_on`; `su_dr_afanasyev_signs_as_co_chair_19910912`, `su_dr_ponomarev_signs_as_co_chair_19910912`, `su_dr_murashev_signs_as_co_chair_19910912` |
| 15 Sep 1991 | plenum of the Council of Representatives; a co-chair signs the list it approved | `su_dr_representatives_council_plenum_19910915`, `su_dr_ponomarev_signs_delegation_list_as_co_chair_19910915` |
| 9-10 Nov (year not printed) | the second congress announced | `su_dr_second_congress_invitation_undated` (no structured date) |
| 12 Dec 1991 | the President's office list of the movement's co-chairs | two movement holders `attested_on`; `su_dr_yakunin_listed_as_co_chair_19911212`, `su_dr_ponomarev_listed_as_co_chair_19911212` |

## Observations

### SU-DR-01 — The movement's founding and bodies

Evidence:

- The First Congress of People's Deputies of the RSFSR, stenographic report vol. V (51st sitting, 22 June 1990, Yeltsin
  presiding): "Манохин А.Н." reads the "Обращение блока ”Демократическая Россия” к избирателям, народным депутатам всех уровней и
  общественно-политическим организациям", which "призывает ... объединиться в массовое общественно-политическое движение
  ”Демократическая Россия”" (printed pp. 336-337).
- The Second (extraordinary) Congress, vol. III (16th sitting, 7 December 1990): "Членов блока ”Демократическая Россия” просьба
  собраться в два часа на балконе. Будет рассматриваться вопрос о выборах в совет представителей движения ”Демократическая
  Россия”" (printed p. 134).
- The Third (extraordinary) Congress, vol. I (1st sitting, 28 March 1991): an appeal read out says the march of that day was held
  "по заявке координационного совета движения ”Демократическая Россия”, удовлетворенной Мосгорисполкомом" (printed p. 59).
- The Yeltsin Center facsimile of the movement's papers (Ф. 6. Оп. 1. Д. 104, leaf 5): a delegation list "Утвержден на Пленуме
  Совета Представителей I5.09.9I".

Decision: **accepted in part**, as claims on the organization. They show the bloc and the movement as distinct bodies and the
movement's Coordinating Council and Council of Representatives acting in 1990-1991. The bloc's call is not a founding act, and no
primary record of the founding congress (20-21 October 1990 in the leads) or of the movement's registration was obtained, so the
lifecycle has no dates.

### SU-DR-02 — The organizing committee

Evidence: the USSR Fourth Congress, vol. I: on 17 December 1990 "Голяков А. И." cites "Председатель оргкомитета движения
«Демократическая Россия» народный депутат СССР Мурашев в интервью еженедельнику «Аргументы и факты»" (printed p. 119); on 19
December "Заславский И. И." answers the "полемику с председателем оргкомитета движения «Демократическая Россия» депутатом
Мурашевым и членом координационного совета «Демократической России» депутатом Заславским" (printed p. 361).

Decision: **accepted in part**, as claims on the organization. Other deputies' descriptions of an organizing-committee
chairmanship (the second citing a newspaper interview) are not the co-chair office and state no date of office; Заславский's own
words make him a member, not a leader. Never holders.

### SU-DR-03 — The co-chairs

Evidence:

- The Third Congress of People's Deputies of the RSFSR, vol. V (16th sitting, 5 April 1991): "Дмитриев В.В., Колпинский
  территориальный избирательный округ, г. Ленинград": "Сам я принадлежу к движению "Демократическая Россия" и являюсь
  сопредседателем его координационного совета" (printed p. 53).
- The Fifth Congress of People's Deputies of the RSFSR, vol. I (9th sitting, 15 July 1991): "Афанасьев Ю.Н." rejects the presiding
  officer's "координатор фракции беспартийных" and ends "я как сопредседатель движения ”Демократическая Россия” обращаюсь к
  гражданам России" (printed pp. 310-311).
- The Yeltsin Center facsimile Ф. 6. Оп. 1. Д. 104, leaf 1: the Coordinating Council's handwritten letter to the President of the
  RSFSR, signed under "Сопредседатели КС „Дем. России“" by "Ю. Афанасьев", "Л. Пономарев" and "А. Мурашев", dated "12.09.91" and
  registered "12,10,1991 № 03540"; leaf 5: a list approved on 15.09.91 signed over "Сопредседатедь [sic] Движения
  "Демократическая Россия"" by "ПОНОМАРЕВ Л.А.".
- The Yeltsin Center facsimile Ф. 6. Оп. 1. Д. 104, leaf 128: the RSFSR President's office list of participants of 12 December
  1991: "Движение "Демократическая Россия" сопредседатели Координационного совета: Якунин Глеб Павлович, Пономарев Лев
  Александрович; Боксер Владимир Оскарович-член КС".

Decision: **accepted in part**. Seven holder observations of five co-chairs (see [Holders](#holders)). The letter keeps the date it
bears: it is written lower left in the letter's hand, the registration mark is that of the meeting of 12 October 1991, and the
plenum of 15 September approving a delegation to the President fits a request made in mid-September; on either reading the letter
existed by 12 October. The list of 12 December 1991 is the President's office's record, not the movement's, and shows only those
attending. Other deputies' "координатор" for Пономарев (29 March 1991) is read but not imported, because the speaker calls the
organiser a "блок" and the identity is not established.

Limits: when each co-chair was elected, the full list of co-chairs at any date and any withdrawal are not in a reviewed primary
record; Попов Г. Х. appears as a co-chair only in leads.

### SU-DR-04 — The second congress

Evidence: the Yeltsin Center facsimile Ф. 6. Оп. 1. Д. 222, leaf 129: a printed invitation, "Уважаемый" completed by hand "Борис
Николаевич!": "Движение "Демократическая Россия" приглашает Вас принять участие в работе второго съезда движения. Съезд
состоится 9-10 ноября в помещении киноконцертного зала "Октябрь" (Новоарбатский проспект)", signed by hand under
"Координационный Совет Движения “Демократическая Россия”".

Decision: **accepted in part**, as an undated claim on the organization: the card prints no year and no date of issue, so the claim
carries no structured date. Whether the congress met, and whom it elected, is not in a reviewed record.

### SU-DR-05 — Ends

No reviewed record states the end of the movement or of any co-chair's office by 25 December 1991. No `until`, no lifecycle end.

### SU-SOYUZ-01 — The group in the Congress's records

Evidence: the USSR Third Congress's stenographic report, vol. I: on 12 March 1990 a deputy says "Я выступаю от имени депутатской
группы «Союз»" (printed p. 98); on 13 March the presiding officer gives the floor "от имени депутатской группы «Союз» — депутату
Блохину", who makes "заявление депутатской группы «Союз»", reports a meeting held "в преддверии Съезда" and says "Сейчас в
составе депутатской группы «Союз» более 300 человек" (printed pp. 133-137). Vol. III: Алкснис and Ким relay the nominations of
the group's "общего собрания" (printed pp. 4 and 82); the presiding officer confirms its size (printed p. 22). The Fourth
Congress, vols. I and III: Гниненко, Блохин, Коган and Чехоев relay the decisions of its meetings, its commission nominees and
its rotation list (17, 26 and 27 December 1990), and resolution 1842-I (17 December 1990) elects its nominees "Коган Евгений
Владимирович" and "Чехоев Анатолий Георгиевич" to the drafting commission on the Union Treaty.

Decision: **accepted**, as claims on the organization. They show a group of USSR people's deputies deciding through its general
meeting and speaking through several members; none prints an office in the group, and speaking or relaying for the group is never
a holder. 12 March 1990 is the earliest reviewed record, not a founding date; the pre-Congress meeting is undated.

### SU-SOYUZ-02 — The co-chairs

Evidence: the USSR Fourth Congress, vol. I, seventh sitting (20 December 1990, "Председательствует Президент Киргизской ССР А.
Акаев"): "Чехоев А. Г., член Комиссии Совета Национальностей по национальной политике и межнациональным отношениям", after the
salutation: "Как один из сопредседателей депутатской группы «Союз», я хотел бы довести до вас те соображения, которые были
высказаны на нашем общем собрании в преддверии настоящего Съезда в отношении нового Союзного Договора" (printed pp. 403-404).

Decision: **accepted in part**. One holder observation, Анатолий Георгиевич Чехоев, `attested_on` 20 December 1990; "один из"
records co-leadership, and the other co-chairs are not named in any reviewed record. His report of the group's meeting on 27
December 1990 gives him no title and is a claim, not a second observation. No `from`, no `until`.

Limits: the group's own documents (its statements, the choice of its co-chairs or of a coordinating council) were not found in an
archival or official publication; the Supreme Soviet's stenographic bulletins for 1990-1991 other than those of 26 August 1991 are
not available as scans.

### SU-SOYUZ-03 — Registration and size

Evidence: the USSR Fourth Congress, vol. II, fifteenth sitting (25 December 1990): "Первая депутатская группа, которая
зарегистрирована Верховным Советом СССР,— группа «Союз». ... Итак, в группе «Союз» — 561 депутат" (printed pp. 421-422).

Decision: **accepted**, as a claim on the organization. The presiding officer is calling a roll; whether Soyuz was the first group
ever registered is not established, and the registration act, its date and any leaders it named were not found.

### SU-SOYUZ-04 — Leaders named by other deputies

Evidence: "Бишер И. О." on 18 December 1990: "уважаемого руководителя группы «Союз»", in a passage about "полковник Алкснис"
(printed p. 187); "Сазонов Н.С." on 26 December 1990: "Некоторые лидеры группы ”Союз”" (vol. III, printed p. 88); "Богданов И. М."
on 26 August 1991: "В мае руководители группы «Союз» на весь автобус кричали" (bulletin No. 1, printed p. 22); "Боярс Ю. Р." on 26
August 1991, for 32 deputies: suspend the mandates of "ее лидеров народных депутатов СССР Когана, Алксниса, Блохина, Чехоева и
Петрушенко" (bulletin No. 2, printed p. 86), answered by "Коган Е. В.": "а я все-таки являюсь активным членом группы «Союз»"
(printed p. 88); "Крайко А. Н." at the Fifth Congress on 3 September 1991: "такова же позиция даже лидеров группы "Союз""
(bulletin No. 3, printed p. 24).

Decision: **accepted in part**, as role claims (Коган's reply as a claim on the organization). All are characterisations by
opponents or other deputies: none names an office, election or date of office, the first names nobody in its sentence, and one of
the five "лидеры" calls himself only a member. None is a holder; whether any should count as a leadership attestation is left to
the integrator.

### SU-SOYUZ-05 — The group's end

No reviewed record dates the group's end or dissolution. The latest record reviewed is of 3 September 1991; the Congress's last
session and the USSR's end in December 1991 are not the group's end, and an assembly of USSR deputies in March 1992 is after the
period. `lifecycle.until` stays null.

## Sources added

16 sources, each with a checked-in derived factual extract under [sources/](sources/) (`ussr-*-facts.json`, LF, format
`spheres-c01-derived-factual-table/v1`, with its own checksum in the packet). Each extract records the original response's URL,
byte count and SHA-256, the byte-identical live file where one exists, both of this packet's downloads, the edition and host, the
pages read and one row per claim (claim_id, observation, role, `holder_name` or null, `persons_named`, role title, the printed
title, event kind, date, text, locator). Original pages, PDFs and images are not checked in; no emblem, signature image or
photograph is republished. Every identity is a raw Internet Archive capture made before the cutoff.

| Source ID | What | Response identity (bytes, SHA-256) | Claims |
|---|---|---|---|
| `su_snd3_steno_vol1` | [USSR Third Congress, stenographic report vol. I (12-13 Mar 1990)](https://web.archive.org/web/20241206074206id_/https://snd.sssr.su/III/I.pdf) | IA 20241206074206, 14,627,192, `3339b40a…0acfdb`; live file identical | 4 |
| `su_snd3_steno_vol3` | [USSR Third Congress, vol. III (14-15 Mar 1990)](https://web.archive.org/web/20240903210942id_/https://snd.sssr.su/III/III.pdf) | IA 20240903210942, 12,922,126, `ae895c02…07b345`; live identical | 3 |
| `su_rsfsr_snd1_sten_v5` | [RSFSR First Congress, vol. V (18-22 Jun 1990)](https://web.archive.org/web/20220313025049id_/https://sten.snd.rsfsr-rf.ru/I/V.pdf) | IA 20220313025049, 18,684,503, `da3383db…9eeb24` | 1 |
| `su_rsfsr_snd2_sten_v3` | [RSFSR Second Congress, vol. III (6-7 Dec 1990)](https://web.archive.org/web/20220313024936id_/https://sten.snd.rsfsr-rf.ru/r2s3.pdf) | IA 20220313024936, 19,806,883, `ccb68d98…ab794a` | 1 |
| `su_snd4_steno_vol1` | [USSR Fourth Congress, vol. I (17-21 Dec 1990)](https://web.archive.org/web/20240902002244id_/https://snd.sssr.su/IV/I.pdf) | IA 20240902002244, 25,557,430, `a0e2b847…f3ace1`; live identical | 7 |
| `su_snd4_steno_vol3_soyuz` | [USSR Fourth Congress, vol. III (26-27 Dec 1990 and acts), recorded for the Soyuz group](https://web.archive.org/web/20241124015727id_/https://snd.sssr.su/IV/III.pdf) | IA 20241124015727, 12,653,342, `398d2fa5…49b897`; live identical | 4 |
| `su_snd4_steno_vol2` | [USSR Fourth Congress, vol. II (22-25 Dec 1990)](https://web.archive.org/web/20240901144454id_/https://snd.sssr.su/IV/II.pdf) | IA 20240901144454, 18,853,969, `ee347c29…820246`; live identical | 1 |
| `su_rsfsr_snd3_sten_v1` | [RSFSR Third Congress, vol. I (28-30 Mar 1991)](https://web.archive.org/web/20220313024757id_/https://sten.snd.rsfsr-rf.ru/r3s1.pdf) | IA 20220313024757, 18,252,813, `0b65f356…3ba076` | 1 |
| `su_rsfsr_snd3_sten_v5` | [RSFSR Third Congress, vol. V (5 Apr 1991)](https://web.archive.org/web/20220313030126id_/https://sten.snd.rsfsr-rf.ru/r3s5.pdf) | IA 20220313030126, 18,615,464, `65ff1f36…4eb54d` | 1 |
| `su_rsfsr_snd5_sten_v1` | [RSFSR Fifth Congress, vol. I (10-17 Jul 1991)](https://web.archive.org/web/20220313024839id_/https://sten.snd.rsfsr-rf.ru/r5s1.pdf) | IA 20220313024839, 36,840,143, `392cfe46…fc1805` | 1 |
| `su_vs_bulletin1_soyuz_19910826` | [USSR Supreme Soviet bulletin No. 1, 26 Aug 1991, recorded for the Soyuz group](https://web.archive.org/web/20250718143607id_/https://sten.vs.sssr.su/12/6/1.pdf) | IA 20250718143607, 2,252,696, `849b9dae…39776a`; live identical | 1 |
| `su_vs_bulletin2_soyuz_19910826` | [USSR Supreme Soviet bulletin No. 2, 26 Aug 1991, recorded for the Soyuz group](https://web.archive.org/web/20250718143607id_/https://sten.vs.sssr.su/12/6/2.pdf) | IA 20250718143607, 1,530,044, `22562b5b…3fd5bd`; live identical | 2 |
| `su_snd5_bulletin3_19910903` | [USSR Fifth Congress bulletin No. 3, 3 Sep 1991](https://web.archive.org/web/20240901234307id_/https://snd.sssr.su/V/3.pdf) | IA 20240901234307, 2,146,995, `8eeae473…d9ac26`; live identical | 1 |
| `su_yc_f6_d104_l1_18_19910912` | [Yeltsin Center, Ф. 6. Оп. 1. Д. 104. Л. 1-18: the movement's papers for the meeting of 12 Oct 1991](https://web.archive.org/web/20230622210558id_/https://yeltsin.ru/uploads/upload/2015/03/27/f6_o1_d104_001_aJ7dx2A.pdf) | IA 20230622210558, 4,143,217, `ed390472…b5e765`; live identical | 5 |
| `su_yc_f6_d222_l129_199111` | [Yeltsin Center, Ф. 6. Оп. 1. Д. 222. Л. 129: invitation to the second congress](https://web.archive.org/web/20230623225515id_/https://yeltsin.ru/uploads/upload/2015/08/20/f6_o1_d222_060_0WLtD2F.pdf) | IA 20230623225515, 106,744, `a553a66a…8819d2`; live identical | 1 |
| `su_yc_f6_d104_l128_134_19911212` | [Yeltsin Center, Ф. 6. Оп. 1. Д. 104. Л. 128-134: the RSFSR President's office papers of 12 Dec 1991](https://web.archive.org/web/20230622210559id_/https://yeltsin.ru/uploads/upload/2015/03/27/f6_o1_d104_006_oscl6gt.pdf) | IA 20230622210559, 4,402,082, `37c8a141…1293d8`; live identical | 2 |

Hosting. The USSR stenograms and bulletins are page-image scans of official publications served by the non-official SSSR.SU
project (snd.sssr.su, sten.vs.sssr.su), the host whose provenance the CLAUDE-C01-SOURCE-26 review documented; the RSFSR Congress
reports are page-image scans of the official editions (Издательство "Республика", 1992-1993) served by the non-official ISTNET /
RSFSR-RF.RU project (sten.snd.rsfsr-rf.ru), whose live certificate does not verify here, so only raw captures are used; the three
archival facsimiles are copies from the Archive of the President of the Russian Federation (fund 91) published by the Yeltsin
Presidential Center, a federal institution. Every source says its host in its publisher; whether these hosts meet the
official-facsimile standard is the integrator's decision, as for CLAUDE-C01-26 (C12). If the ruling goes against the SSSR.SU and
RSFSR-RF.RU hosts, SU-DR-03 keeps its three archival records (five of the seven observations) and SU-SOYUZ loses every claim.

Three sources repeat files already recorded for CLAUDE-C01-26 (the Fourth Congress vol. III and the two Supreme Soviet bulletins
of 26 August 1991), each as a pre-cutoff raw capture of the same bytes under a new ID, so that the CLAUDE-C01-26 extracts and their
reviewed copies stay unchanged; the new records carry only this packet's claims.

## Response identities and stability checks

Every recorded and attached response was downloaded twice by this packet with plain curl (curl's own User-Agent, an explicit
`Accept-Encoding: identity` header, no cookies, no cache-busting query): the Soyuz sources at 2026-09-29T00:15:11Z-00:15:39Z and
00:46:56Z-00:47:39Z, the three sources the check found at 02:11:28Z-02:11:52Z and 02:43:03Z-02:43:15Z, and the Democratic Russia
sources at 02:57:41Z-02:58:07Z and 03:29:56Z-03:31:08Z; each response's two downloads are 31 to 33 minutes apart, and all 27 pairs
returned identical bytes and SHA-256 (the times are in each extract's `downloads`). The discovery downloads, the research
dossier's downloads (two or more of every Democratic Russia response, 30 minutes or more apart) and each independent check's
downloads were identical as well. For every capture the base32 SHA-1 of the recorded bytes equals the Internet Archive CDX digest,
and every capture is served without Content-Encoding to a request without gzip; the recorded identity is the uncompressed body.

Points a reviewer needs:

- The live SSSR.SU and yeltsin.ru files are static (fixed Last-Modified and ETag) and byte-identical to the captures; they are
  attached as `live_file_response`. The live sten.snd.rsfsr-rf.ru host fails certificate verification (schannel
  SEC_E_WRONG_PRINCIPAL); it was not bypassed, and no live file is attached for the five RSFSR volumes.
- The yeltsin.ru item pages (`/archive/paperwork/<id>/`) and its search JSON are dynamic and were used for discovery only; no
  recorded URL is a search, listing or per-request page.
- The RSFSR II-V volumes and the Yeltsin Center scans have no text layer. They were searched with a word spotter (RSFSR) or read
  page by page (Yeltsin Center), and every cited page was read from the image; other mentions in the RSFSR volumes may be missed.
- Every quotation was read from the page image, not the OCR layer; quotation-mark glyphs are kept as printed (”...” in the RSFSR
  volumes and in the USSR Fourth Congress vol. III, «...» elsewhere), and the typed Latin "I" for 1 is kept in the archival lists.

## Leads not imported

- **Encyclopaedias and wikis**: the Wikipedia articles on Democratic Russia and on the Soyuz group (co-chairs of the group
  elected in February 1990: Блохин, Алкснис, Комаров, Чехоев; Блохин chairman from April 1991; six movement co-chairs, including
  Попов Г. Х., elected at a council session of "12 декабря"), the Панорама directory pages behind them and bigenc.ru. No date or
  holder is taken from them.
- **Directory**: nasledie.ru, "«СОЮЗ» — ВСЕСОЮЗНОЕ НАРОДНОЕ ДВИЖЕНИЕ «СОЮЗ»" (IA capture 20110927103551), naming four co-chairs of
  a later movement, which may not be the deputies' group.
- **Newspapers**: the movement's newspaper «Демократическая Россия» (1990-1991), seen only as private-collection masthead scans on
  Wikimedia Commons (thumbnails; the imprint images returned HTTP 429); the group's newspaper «Политика» (1991) and central
  newspapers (Izvestia, Pravda, Sovetskaya Rossiya, Аргументы и факты) were not obtained and would be leads in any case.
- **Yeltsin Center items**: 10635 (Ф. 6. Оп. 1. Д. 104. Л. 21-79, the stenogram of the President's meetings with the movement on 12
  October and 5 November 1991), catalogued without a file; 18126 and 18127 (notes and a list of the movement's coordinators, 10-11
  February 1992, after the period); photograph 62818 (a caption naming "Аркадий Мурашев" at the founding congress, a caption only);
  audio 8965 (the host's title for a speech at an enlarged plenum of the Council of Representatives, host date June 1991).
- **A news-agency report cited in debate**: at the RSFSR Third Congress (vol. IV, printed pp. 4-5) the USSR Prosecutor General cites
  a Postfactum report of a press conference of the "руководства движения "Демократическая Россия""; a news report, naming no
  officer.
- **After the period**: the document collection of the assembly of USSR deputies of 17 March 1992 (sssr.su/VIsnd.pdf, 568,448
  bytes, `ff8eb529…`), which prints the full names of several Soyuz deputies.
- **Registration**: the movement's registration by the RSFSR Ministry of Justice (23 April 1991 in the leads) is not in a primary
  record.

## Sources attempted

- Read but not imported (no office stated, or another identity): the RSFSR First Congress vols. II-IV (the deputies' group's
  registration of 25 May 1990 with 66 members, its coordinating council and spokesmen), the Second Congress vols. I, II, IV and V
  and the Fifth Congress vol. I speeches for the RSFSR group or faction, the Third Congress vol. I remark of 29 March 1991 calling
  Пономарев "координатор “Демократической России”" (identity not established), the Yeltsin Center items 10634, 10636 and 10637
  (notes of 5 November 1991; the faction coordinators' list of 11 December 1991), leaf 6 of Д. 104 (participants of 12 October 1991,
  no titles) and the Coordinating Council's appeal of December 1991 (its day not legible), the Д. 25 bloc paper of April 1990; the
  USSR Third Congress vol. II and Fourth Congress vol. IV (unspoken speeches), the Fifth Congress bulletins 1, 2 and 4-7 and
  Vedomosti 1991 Nos. 35-38 and 41 (no Soyuz or movement office).
- `sten.snd.rsfsr-rf.ru` live: certificate verification fails (not bypassed); r4s4, r5s4 and r5s5 have no usable capture.
- `sten.vs.sssr.su`: only 12/6/1.pdf and 12/6/2.pdf exist as scans for 1990-1991; 12/6/3.pdf to 8.pdf, 12/5/1.pdf and 12/4/1.pdf
  return 404; the HTML transcriptions on the same host and on sten.sr.vs.sssr.su were not used.
- yeltsin.ru archive search for the Soyuz group, Алкснис, Петрушенко, Чехоев, Коган and the group's names: no item in the period;
  docs.historyrussia.org, gaidar-arc.ru (708 titles of 1990-1991), vtoraya-literatura.com and the Internet Archive's item search:
  no Soyuz or movement document of 1990-1991. The RSFSR Third Congress vols. II-IV and Fourth and Fifth Congress volumes other
  than those recorded: polemical mentions and meeting notices only. memo.ru was not searched.
- Blocked or unavailable, not bypassed: elib.shpl.ru (HTTP 429, then 500), prlib.ru (bot challenge), rusneb.ru (403),
  catalog.osaarchivum.org (403), alexanderyakovlev.org (521), the sakharov-center archive (login). The Internet Archive CDX index
  was intermittently offline; queries were retried.

## Independent checks

Two adversarial checks were made on 29 September 2026 (UTC): the Soyuz check (01:30Z-02:02Z) re-downloaded all ten responses and
read every cited page from its own renders; the Democratic Russia check (03:02Z-03:16Z) re-downloaded the eight sources and the
Fourth Congress vol. I and did the same. Both confirmed every identity and the holder decisions.

| Check | Defect | Outcome |
|---|---|---|
| S | D1-D2 volume titles (sittings 1-10 and 11-16) | **Applied** |
| S | D3 an earlier record of the group (12 Mar 1990) | **Applied**: `su_snd3_deputy_speaks_for_soyuz_19900312` added |
| S | D4 a membership statement filed on the role | **Applied**: Коган's reply is a claim on the organization |
| S | D5 event kind of the presiding officer's size statement | **Applied**: `membership_size_reported` |
| S | D6 "first registered" overstated | **Applied**: renamed `su_snd4_soyuz_registration_roll_19901225`, neutral text |
| S | D7-D9 pages read and edition pages | **Applied**: pages and title pages added |
| S | D10 wording; the co-chair's full name exists | **Applied**: resolution 1842-I imported; the holder is "Анатолий Георгиевич Чехоев" |
| S | D11 a speech and an undated meeting in one claim | **Applied**: split into two claims |
| S | D12 genitive names | **Applied**: nominative with initials in `persons_named`, the printed form kept |
| S | D13 no Soyuz discovery record | **Applied**: see Sources attempted |
| S | D14 scope note | **Applied** |
| S | missing records M1-M5 | **Applied**: the records of 12 Mar 1990, 26 Dec 1990 (two), 27 Dec 1990, 26 Aug 1991 (bulletin No. 1) and 3 Sep 1991 imported; M6's member statement of 14 Mar 1990 read, not imported. The check dated the rotation-list statement 27 Dec; the page images place it in the eighteenth sitting of 26 Dec (the next speaker proposes to vote "завтра") |
| DR | D1 an identity-ambiguous "координатор" on the role | **Applied**: not imported |
| DR | D2 the printed salutation | **Applied** |
| DR | D3 imprints as printed | **Applied** |
| DR | D4 the sitting's end bound | **Applied**: contents PDF pp. 239-240 |
| DR | D5 evidence for the letter's date | **Applied**: in the uncertainty of the three letter claims |

## Suggested next work orders

These are proposals for the integrator. They are not created in `work-orders.json`.

- `C01-USSR-DR-001`: a primary record of the movement's founding congress (20-21 October 1990) and of the first election of its
  co-chairs, and of its second congress (9-10 November 1991); the Yeltsin Center stenogram of 12 October and 5 November 1991
  (item 10635) if it is published.
- `C01-USSR-DR-002`: a separate packet for the RSFSR deputies' group, bloc and faction «Демократическая Россия» and its
  coordinators (records read here and listed above), and an integrator decision on which identity, if any, the simulation row
  `USSR/su_dr` represents.
- `C01-USSR-SOYUZ-001`: the Soyuz group's own documents (statements, the choice of its co-chairs and coordinating council) in an
  archival publication, and the Supreme Soviet's registration of deputy groups (Vedomosti 1990).
- `C01-USSR-SOYUZ-002`: an integrator ruling on whether a Congress or Supreme Soviet record in which other deputies call named
  deputies a group's "лидеры" counts as a leadership attestation.
- `C01-USSR-GOV-008` (a CLAUDE-C01-26 follow-up found here): the USSR Third Congress vol. III, recorded here as
  `su_snd3_steno_vol3`, prints at the seventh sitting of 15 March 1990, after the break, "Председательствует М. С. Горбачев" and
  his words "В связи с избранием меня Президентом СССР мои полномочия Председателя Верховного Совета СССР прекращаются" (printed
  p. 69, PDF p. 71), and the Soyuz group's nominations for the new Chairman (printed p. 82); both bear on `su_supreme_soviet_chair`
  (a stated end of the 1990-03-14 observation, for the integrator) and are not imported here.

## Integration notes (outside this packet's file boundary)

- **Stacking.** The branch is **not stacked**: claim commit `c1475ada` sits directly on `44098c5a`. The CLAUDE-C01-SOURCE-26 repair
  is already integrated. `codex/campaign-certification` is fetched again before committing and merged if it moved.
- **Existing records changed.** None: no existing source, claim, extract, entry, role or holder of `ussr.json` is edited; two
  organizations, 16 sources and one coverage item are appended. No existing extract is edited.
- **Existing tests updated** (pinned counts, exact sets and access dates only; none loosened):
  - `test_ussr_research_s10h.py`: totals (5, 40, 95, 6) → (7, 56, 131, 8); organizations 1 → 3; the table-extract count 30 → 46,
    with the 30 CLAUDE-C01-26 extracts still pinned at positions 10-39; the PDF-page set adds the 13 scanned PDFs of this packet;
    access dates add 2026-09-29, pinned to positions 40-55; index organization observations 1 → 3, source claims 95 → 131,
    `mapping_pending` 5 → 7. The jurisdiction guard, which required union level for every entry, is re-expressed exactly: every
    entry is union-level except `su_democratic_russia`, pinned as level `republic`, RSFSR; every entry still has no successor
    mapping and no party mapping.
  - `test_ussr_government_supreme_soviet_c01_26.py`: its new-source list is pinned to positions 10-39 (`[10:40]`), the packet
    totals become (56, 131, 7, 8) and the index figures (4, 8, 131, 7). The pending CLAUDE-C01-28 also edits this file (its Russia
    holders guard), in other lines.
  - `test_ussr_russia_transition_c01_05.py`: unchanged.
- `research-index.json` is regenerated in a **separate commit**; it is the only file this packet shares with other pending
  packets (apart from the CLAUDE-C01-26 test above). New totals: 1,587 sources and 4,218 claims (from 1,571 and 4,182), 843
  organization observations (841); USSR's entries go from 5 to 7, role observations from 6 to 8, source claims from 95 to 131 and
  `mapping_pending` from 5 to 7; its one discovery batch now has seven members. If another packet lands first, regenerate the index
  rather than merging it.
- **Known failures outside the listed checks** (not fixed here):
  - `campaign_census.py --check` fails on the integration base itself: commit `262d5f61` changed `spheres-sim/src/government.rs`
    without regenerating `census.json`. Regenerating the census to a scratch directory shows that the only difference is
    `census.json`'s record of that input (848,551 → 849,546 bytes; SHA-256 `0e2dbdef…` → `3f846b4b…`), with
    `all_declared_inputs_current` false; the other four census outputs are identical. This packet does not touch the census.
  - `tools/avatars/test_certified_gap_ledger.py` already fails on the base (`test_committed_output_is_current`: `ledger.json`
    stale), and with this packet also errors on the new sources ("no pinned attribution") until Codex classifies this packet's
    commit in `COMMIT_PACKETS` at integration. `docs/campaign-certification/C01/gap-ledger/` is not touched.
  - `tools/avatars/test_certified_boundary_matrix.py` (S23) needs `spheres-web/src`, which the sparse checkout lacks; in a full
    checkout it reports the packet as `unclassified_packet` with stale boundary-matrix files, so Codex must list the packet and
    regenerate `docs/campaign-certification/S23/preparation/boundary-matrix/` on integration.
- **A process incident.** While stopping its own leftover log-tail processes, the Democratic Russia research sub-agent also
  killed one `tail -f` it had not started (on a task output file, started at 11:03 local time, not by this packet's session). It
  only stopped that output stream, not the task it watched.
- `research/README.md`, the C01 README totals and `docs/planning/ai-workstreams.json` are left for the integrator; this handoff is
  not registered there.
- The new test `test_ussr_democratic_russia_soyuz_c01_35.py` pins the two organizations and roles, the eight holder observations,
  every claim's date, kind, observation, role and organization, every row's `holder_name`, every response identity, live file,
  pair of downloads and pages read, the extracts, the separation from state offices, the CPSU and `russia.json`, and 47 mutations.
- In the atlas, the USSR gains the two organizations with one office each (seven and one observations). No UI code changed.

## Checks

```text
python -X utf8 tools/avatars/campaign_research.py
python -X utf8 tools/avatars/campaign_research.py --check
python -X utf8 tools/avatars/campaign_census.py --check
python -X utf8 -m unittest discover -s tools/avatars -p "test_ussr*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_russia*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_*research*.py"
python -X utf8 -m unittest discover -s tools/avatars -p "test_campaign*.py"
node --test tools/ui/check_leadership_research_review.cjs
python tools/planning/workboard.py --check
git diff --check (this packet's paths)
```

Results (29 September 2026 UTC) are recorded in the handoff's Checks paragraph.
