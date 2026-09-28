# Human playtest records

`s26_playtest.py` is a local, standard-library Python 3.10+ tool. It reads
observations and verifies their file identities; it does not operate the game,
contact people, collect personal contact data or award campaign qualification.

Start with the [S26 facilitator kit](../../docs/campaign-certification/S26/preparation/README.md).

```text
python -X utf8 tools/playtests/s26_playtest.py init work/s26-study-001
python -X utf8 tools/playtests/s26_playtest.py report work/s26-study-001/records.json --output work/s26-report-001
python -X utf8 -m unittest discover -s tools/playtests -p "test_*.py" -v
```

Use a new study/report directory each time. `init` and `report --output` refuse
existing output directories. Reporting does not alter records or evidence.

Report exits: **2** means invalid records or evidence; **1** means valid records
with missing human coverage; **0** means recorded criteria are ready for human
review. Zero never marks S26 complete. The shipped empty plan returns 1.

To add a local artifact, print its exact raw identity and paste that object into
the record's `evidence` list. Paths are relative to the evidence root; default
report evidence root is the record's directory. Use `--evidence-root` if the
record and evidence are stored separately. The tool refuses path escapes.

```text
python -X utf8 tools/playtests/s26_playtest.py pin work/s26-study-001/notes/S001.txt --root work/s26-study-001 --id E001 --kind notes
```

Allowed kinds: `notes`, `image`, `video`, `save`, `build`. The package SHA-256
must match its `build` artifact. Successful save steps and later-session starting
saves require pinned `save` artifacts. The tool verifies bytes and hashes, not
the internal native meaning of a save or a human's identity; independent review
must check those against the recorded build and campaign context.

Tests construct explicitly synthetic records in temporary directories. Passing
them demonstrates bookkeeping behavior, not human usability success.
