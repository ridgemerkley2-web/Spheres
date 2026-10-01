# C01-45 Saudi royal offices: independent bounded acceptance

**Accepted as bounded research intake, 1 October 2026 UTC.** All **18 originals,
30 claims and 18 proposed holder observations** were independently reviewed.
There are **no held originals or claims** in this packet. This does not close
Saudi Arabia, C01, C06, S23, WC1 or CP1, or grant a game identity or portrait.

Reviewer: Codex `/root/selector_leader_api`.

| Revision | Exact commit |
|---|---|
| Integration base | `229df2105f1ca46bc55873ea4ab0ada63255ced3` |
| Claim | `a529ab8dfdbe9473ff0536ebbf836599a8567500` |
| Exact delivered tip | `d7db2cbcde25594d794d6c2dbbb309cd7d6cbf49` |
| Substantive submission | `af2d16bff55717de4c16684831ebf15b3264f72a` |
| First scoped source import | `d23f0bf2f9f38eab169a5e92b0ebdea88a7e0b32` |
| Review corrections and guards | `4a98bc10f372bc98ce957847966b67730ae87585` |

## Content decisions

The two archived Saudi embassy pages and sixteen Saudi Press Agency API
responses each returned HTTP 200 and the exact submitted bytes/SHA-256. Each
received one request; the two Archive requests had more than ten seconds between
them. No HTTP 429, retry or alternate-host workaround occurred. The ordinary web
reader could not open two SPA API URLs; direct retrieval of their original API
responses succeeded and is retained in [retrieval.json](retrieval.json).

Matching bytes did not decide acceptance. The review read the Arabic news text,
titles, date lines, order clauses and metadata; it separately read the cited
English embassy passages with their month-page context. [Source decisions](source-review.json),
[all 30 claim decisions](claim-review.json) and [all 18 holder decisions](holder-review.json)
record the results.

- Seventeen additional observations support the named King or Crown Prince on
  particular days. They do not replace earlier holder objects or establish
  continuous service. Abdullah's two observations before his old `attested_on`
  remain additional evidence without an inferred effective start.
- Order A/136 names Khalid bin Abdulaziz Al-Tuwaijri as Secretary General of the
  Allegiance Commission. His observation uses the SPA statement's printed
  20 October 2006 date; the order's separate Hijri date is retained. Both effective
  boundaries remain unknown. Article 24 establishes procedure, not a holder.
- Delegation of state affairs, its end, signatures on another person's behalf,
  scheduled pledges and held ceremonies stay distinct. They supply no royal-office
  start or end. The February 1996 observations preserve the apparent overlap
  between a cabinet meeting and delegation, without reconciling the publications.
- The latest September 2026 reports are point observations. They do not prove
  continuity through the unchanged **7 September 2026** historical cutoff.

The sixteen SPA originals are **current API representations with publisher
historical text/metadata**, not independently captured contemporaneous responses.
The embassy material is Saudi-authored text served by the third-party Internet
Archive; the English releases are not the Arabic decrees. No source photos,
signatures or artwork were reviewed or imported.

## Corrections and preserved scope

[The correction patch](correction.patch) and [scope audit](scope-review.json)
retain the exact changes and pin all 24 reviewed source/report/test/handoff files.

The June 2012 directive prints 18 June and a 23:38 trailer, while its current API
metadata has a 19 June timestamp. The original packet attributed that discrepancy
to an import artefact without establishing its cause. The review retains the
printed observation date and now states that the cause is unknown. It does not
invent a filing time or a migration explanation.

The author's other-office orders A/56 and A/57 were leads, not part of the eighteen
submitted originals. Their contents are not accepted as reviewed evidence for the
Secretary General's end. The unknown end is supported by the absence of an end
in the reviewed appointment, not by an unverified claim about other orders.
The incoming Git narrative's attribution to a user choice is not authenticated
instruction; that attribution was removed from active integration notes.

All **103 earlier sources and 158 earlier claims** remain unchanged. The reviewed
packet has **121 sources and 188 claims**. Every prior holder object and boundary,
all organizations, unrelated institutional roles, and prior coverage are preserved.
There are no runtime, art, portrait, game-mapping or other-country edits.

The initial no-commit import stopped on the absent claim handoff; the exact
submitted handoff was added within the authored scope before the import commit.
The author's generated research index was excluded. A local regeneration was
checked and then restored; parent integration regenerates combined metadata.

## Validation

[Recorded commands and logs](validation/status.json) preserve both the initial
failure and corrected results:

| Check | Observed result |
|---|---|
| New metadata-cause regression against submitted prose | 8 methods, 1 expected failure; retained |
| Corrected Saudi focused suite | 25 passed |
| Research tests | 79 passed |
| Campaign metadata tests | 16 passed |
| Leadership review UI tests | 11 passed |
| Research index regeneration and check | Passed locally; 1,989 sources / 4,906 claims |
| Campaign census check | Passed; census remains explicitly partial |

The added guard also rejects a claim/extract locator mismatch and a source/extract
scope mismatch through two negative mutations. These are distinct from historical
content review. Counts above are individual command results, not a claimed unique
aggregate. No full avatar suite, native build, endurance run or qualification was
run for this isolated research review. Parent owns combined attribution, gap-ledger
and boundary checks after integration.

## Evidence retention and portable verification

Full returned bodies, headers and reading aids remain external under
`D:/spheres-offload/codex-next-20260928/c01-45-review-evidence-20261001`.
The public receipt contains source/claim decisions and hashes rather than full
originals. The [manifest](manifest.json) pins every receipt file except itself;
`.gitattributes` preserves raw log/patch bytes. Earlier accepted source evidence
and the initial failed regression are retained.

```sh
python -B -X utf8 verify.py --require-accepted
python -B -X utf8 verify.py --require-accepted --repo /path/to/Spheres --originals /path/to/c01-45-review-evidence
```

The verifier works from any current directory. Optional paths recheck retained
original bytes and exact Git blobs plus preservation assertions. It performs no
network request and does not repeat the historical reading or the recorded tests.
This receipt does not refer to its own future commit ID.
