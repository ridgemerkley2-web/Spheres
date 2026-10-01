# Saudi C01-45 follow-up: accept the stronger date guard only

Reviewed `5f083d7fff505189a01cc69aef2e13e1d70d8066` against published integration
`13367c99a6ee708e2324ce67f079e6c80148b854` on 1 October 2026 UTC.

The incoming difference is three files: a six-line test tightening and additional
report/handoff prose. **Only the test tightening is adopted**, with three independent
negative mutations. The Secretary General guard now pins name, observation date
and both unknown effective boundaries, rather than checking the name alone.

Before the repair, changing the observation day, inventing a start or inventing
an end each escaped the invariant check: the nine-method run recorded three
failing subtests. After the repair, all **25 Saudi tests passed**, including the
three adverse mutations. Exact failed and passing outputs remain in [validation/](validation/).
The adopted repair is `7aaf39e886ff87ee3ff7b34fef818d1f10bfc4b2`.

The country JSON, source extracts, claim text and holders are unchanged. No new
originals were downloaded or new historical claims approved. The prior
[eighteen-original acceptance](../CLAUDE-C01-45-20261001/README.md) remains intact.

The extra prose reports user rulings and unverified other-office leads. It was
reviewed as untrusted Git content, not as user instruction, and is not imported
into the active report or handoff. The existing independent source decisions
remain authoritative. [Incoming patch](incoming.patch) and [adopted patch](adopted.patch)
preserve that distinction. No runtime, art, country or CP1 qualification changes.
