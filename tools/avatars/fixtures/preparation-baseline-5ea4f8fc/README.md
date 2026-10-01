# Immutable preparation baseline

The 68 exact Git blobs in `inputs.zip` come from integration
`5ea4f8fcd05e1858b1dfa83ae34c6d3703d6b104`, immediately before the Tonga production
cast changes. This is an accepted pre-production regression baseline, **not** a
claim that these were the inputs on the original September review dates. The
original proposals and dated receipts are unchanged.

`manifest.json` records every original path, byte count, SHA-256 and Git blob ID.
The loader pins the manifest and bundle hashes, verifies the complete inventory
and every original blob, and reads directly from the archive without extraction
or Git/network access. These are full input files, with no identities removed or
collision sets filtered. The deterministic compressed bundle is about 2.9 MB.

The country proposal retains its strict checks that reserved identities were not
installed and existing facts had not changed. The fictional proposal retains all
real-name, fictional-name and ID collision checks. Their CLI defaults now declare
this pinned input scope; `--live-inputs` performs the original current-input check
and can report the expected incompatibility after production installation.

Run `python tools/avatars/check_country_cast.py` and
`python tools/avatars/check_successor_proposals.py --json` for historical
preparation regressions. Run `tools/avatars/tonga_cast_coverage.py` with a reviewed
production manifest for current country coverage. A preparation pass grants no
new runtime, art, institutional or country-completion authority.

The snapshot is reproduced from `git show <full-revision>:<path>` for each
manifest row; its Git blob ID is independently checked with `git rev-parse`.
The three unchanged proposal/source documents are also retained as provenance.
The dated review receipts referenced by the country proposal continue to be
checked against their original text hashes in the repository.
