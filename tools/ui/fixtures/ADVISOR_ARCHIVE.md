# Archived advisor reading

`advisor-archive.json` is a 32,618-byte projection of the existing real API
archive previously used by `check_advisor_model.cjs`. That test formerly skipped
when the 4,337,703-byte file outside the repository was absent. It now runs on
clean checkouts and keeps the same four outcome assertions: annual renewal and
a research choice are present; treasury and disabled government-route alarms
are absent.

The retained values are copied directly from the archived Japan reading dated
1 January 1990. No values, routes, defaults or dates were synthesized. All 137
nations keep their identity/name/alive values and original ordering; the player
also keeps the entire annual-budget, treasury and takeover values. Research,
conflicts, deployments, incoming agency requests and production/manufacturing
summaries are unchanged. The programme enabled/due flags and incoming resource
offers are retained. Unused data such as map geometry, other nations' economic
records, ministry details and the session identifier are omitted.

The [provenance record](advisor-archive.provenance.json) lists the precise
projection and SHA-256 of both the full original archive and this fixture. Before
writing, `assert.deepEqual` proved that the full archive and projection produce
identical complete `AdvisorModel.evaluate` results: the two expected cards. This
fixture concerns state-only advice, not first-hour outcome recognition.

The original archive's HTTP capture command and engine revision were not
recorded. Its filesystem timestamp is provided as metadata, not asserted as a
verified capture date. This evidence therefore does not claim compatibility with
every current native payload or certify the current simulation. Other served
API and guidance tests retain that responsibility.

To reproduce the projection from a copy of the original archive, first verify
its recorded byte count and digest. Copy the provenance's `full_top_level_fields`
when present; for programmes, resources and each nation, copy the listed fields
without changing nested values or array order. Preserve absent keys as absent.
Serialize with `JSON.stringify(value, null, 2) + '\n'`, using the top-level field
order in the provenance, then programmes, resources and nations. Its bytes must
match the fixture digest. Independently compare complete advisor output from
both inputs before accepting any changed extraction.
