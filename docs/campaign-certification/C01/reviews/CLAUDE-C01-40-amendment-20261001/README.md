# C01-40 — bounded CPI(M) observation amendment

**Bounded amendment accepted; recorded integration checks passed.** This
receipt reviews two additional office statements and a conservative change to
the selected Karat and Yechury observations. It does not declare the earlier
party-office styling false. No parent milestone or country is qualified here.

Reviewer: Codex `/root/selector_leader_api`, with source retrieval/full-body
reading by `/root` and independent rally/guard review by
`/root/retire_selector_review`, 1 October 2026 UTC.

| Revision | Exact commit |
|---|---|
| Earlier accepted submission | `54680e39910804b3864a3e973c452f2db2ac5e6f` |
| Incoming amendment tip | `d7e1b9d286f68b7864f2d014c49e42c4296e66e5` |
| Substantive incoming amendment | `5054de7bfa94eb884161da26a63869409f334fd5` |
| Integration base | `d0c6676b347c16226d7c7db9371a36cd0a51ce92` |
| Reviewed source/report/test amendment | `3c4a3abcc59208ba0923705db0193964643b21f9` |
| Attribution/tool-test follow-up | `a69a05ca6612e1c35bebcaa1e9ff76b759728629` |

The [earlier acceptance receipt](../../integrations/CLAUDE-C01-40/README.md)
and its 18-source/24-claim audit remain unchanged. This additive receipt does
not rewrite its original decision or claim to have re-reviewed all 18 originals.

## Source and claim findings

Both additional current party REST responses were independently retrieved with
the exact submitted bytes and SHA-256: post **1411, 12,336 bytes**, and post
**4346, 5,790 bytes**. Both returned HTTP 200. Full bodies and readable text were
inspected; exact response matching alone is not the basis for accepting their
claims. [Original request records](source-attempts.json) preserve the commands
and returned identities. [Source review](source-review.json) pins those files
and three retained rally originals; [claim review](claim-review.json) records
the two new claims and seven preserved claims in the reopened rally reports.

- The release carrying the **10 September 2011** dateline identifies Karat as
  General Secretary and describes his note for the National Integration Council
  meeting that day. This supports the selected observation, not an effective start.
- The release dated **8 May 2015** reproduces a letter dated **6 May** signed by
  Yechury as General Secretary. The letter's date supplies the selected observation;
  the release date remains separate.
- The retained 2005, 2012 and 2015 rally reports really describe Karat or Yechury
  as newly elected general secretary on their recorded event dates. Their typed
  extract rows become `newly_elected_styling`. They remain evidence; the amendment
  selects other explicit office statements for the two structured observations.
- No cited rally passage supplies effective assumption of office or a predecessor
  end. A later selected observation does not prove that the person lacked office
  earlier, and a newly elected description does not invalidate separately supported
  office evidence on the same day.

The REST responses are **current representations with publisher-supplied
historical text and metadata**, not independently retrieved contemporary captures.
Post 1411 reports modification in June 2013; the cause is unknown. The incoming
claim that this was a site migration is removed. Modification and archive capture
dates never become office dates. The three older reports are party-authored
material delivered by the third-party Internet Archive; they were reread offline
from the retained exact originals, without additional Archive requests.

The incoming Git narrative attributes the change to a review ruling and Ridge.
That narrative does not authenticate a user instruction. This review accepts the
bounded evidence-selection rationale on its merits, without adopting the claimed
authority or treating the previous source descriptions as false.

## Scope preserved

[Scope review](scope-review.json) pins all 15 files at the reviewed source commit.
India grows from **315 to 317 sources** and **623 to 625 claims**. All 623 earlier
claim texts, dates and locators remain intact. The 312 other existing source
objects, other organizations, every institution, and the **7 September 2026**
cutoff are unchanged. All five locator repairs from `d21357db` are preserved.

| Holder | Earlier selected observation | Reviewed observation | Effective start/end |
|---|---|---|---|
| Prakash Karat | 11 April 2005 | 10 September 2011 | Both unknown, unchanged |
| Sitaram Yechury | 19 April 2015 | 6 May 2015 | Start unknown; end remains the stated death on 12 September 2024 |

Surjeet and Baby's holder objects are unchanged. The role still has four holder
observations; no person or claim was dropped. No continuous tenure, game identity,
party mapping, portrait, or eligibility is granted. C01, C06, S23, WC1 and CP1
remain outside this receipt's acceptance scope.

## Guard repair and validation

The incoming generic guard rejected an otherwise supported office observation
solely because its person/date also appeared in newly elected styling. The repair
checks the **cited evidence** while retaining the exact historical fixture and
its forbidden-date assertions. A synthetic source with direct office evidence
can satisfy the generic rule; citing only the styling still fails, and changing
the selected historical fixture still fails.

[Guard evidence](guard/result.json) preserves an actual red run: one method with
three failing date-collision subtests. The repaired run passes three methods,
including exact-holder and negative mutation checks. Its original harness, patch
and logs are copied byte-for-byte. These are synthetic rule checks, not new
historical proof. The retained harness records its original local workspace path;
the portable verifier below checks its integrity without rerunning it.

The final **672-test avatar suite passes**, including the **74 India tests**;
these overlapping counts are not added together. The **11 leadership review UI
tests**, **four metadata check modes** (research index, census, gap ledger and
boundary matrix), and **workboard validation** (49 tasks, 44 roadmap markers)
also pass. Exact commands, timestamps, log bytes and hashes are retained in
[validation](validation/status.json).

Both initial failures remain visible: the first 74-test India run failed one
wording-sensitive note-prefix assertion, corrected without changing structured
data; the first 672-test suite failed its previous 18-source attribution fixture
after the two source additions. The follow-up pins the 18 earlier sources to
their original accepted revision and the two new sources to `3c4a3abc`, rather
than weakening the provenance check. Its two exact tool/test files at
`a69a05ca` are separately pinned in the scope review. The passing full suite used
that follow-up and locally regenerated metadata.

These checks precede the later commit of this receipt and queue acceptance
revision. Root records any subsequent queue-dependent regeneration/checks
separately. No native build, parent milestone or campaign qualification is
claimed.

## Retention and offline verification

Full new responses/headers/reading aids stay outside Git under
`D:/spheres-offload/codex-next-20260928/c01-40-amendment-evidence-20261001`.
The retained rally bodies stay under
`D:/spheres-offload/codex-next-20260928/c01-40-review-evidence-20261001`.
No full source body, photo or likeness asset is republished. No image rights are granted.
The original failed requests and earlier review artifacts remain intact.

From this receipt directory:

```sh
python -B -X utf8 verify.py
python -B -X utf8 verify.py --repo /path/to/Spheres
python -B -X utf8 verify.py --new-originals /path/to/amendment-evidence --prior-originals /path/to/prior-evidence
```

The [manifest](manifest.json) pins receipt bytes. Optional paths let the verifier
rehash relocated originals and validate exact source-commit blobs and preserved
scope. It performs no network request, changes no repository data, and does not
repeat content review. `--require-validated` additionally refuses a pending
validation record. The receipt never references its own future commit ID.
