# S22 intermediate checkpoint storage records

**Local storage maintenance only; qualification: false.** This packet preserves
both attempts and their original paths, hashes, byte counts and timestamps. The
large compressed checkpoint bodies remain local, outside Git. Original
preparation reports and canonical qualification inputs were not rewritten.

The first attempt compressed and removed **zero files**. A timestamp type mismatch
incorrectly rejected all 34 candidates as refreshed; its inventory, rejection
journal and result remain in `attempt-01`. After correcting the timestamp
comparison, `attempt-02` retained all 34 annual checkpoints losslessly:

| Preparation | Retained intermediate annual dates | Count |
| --- | --- | ---: |
| active-01 | January 1, 2000–2005 | 6 |
| active-02 | January 1, 2007–2014 | 8 |
| active-03 | January 1, 2016–2035 | 20 |

Each new adjacent `.json.gz` was decompressed and matched to the original SHA-256
and byte count before that single redundant raw file was removed. The durable
journal records verification before removal. Original payload was 4,396,021,113
bytes; retained gzip payload is 793,461,528 bytes, recovering 3,602,559,585 payload
bytes. Free disk space at that operation's boundaries rose from 4,488,826,880 to
8,091,283,456 bytes. These are historical observations, not current free space.

The adopted 1999, actual 2006, actual 2015, final November 30, 2035 and both
resume-input saves remain raw and unchanged. Packaging freshly rehashed these
six saves, the original S19 source, and all 34 retained gzip files. See
`protected-input-verification.json`. Packaging did not repeat decompression; the
recorded roundtrips are the checks performed before removal.

To restore one intermediate checkpoint:

1. Select its entry in `attempt-02/result.json`, and verify the recorded
   `gzip_path` against `gzip_bytes` and `gzip_sha256`.
2. Require that `original_path` does not already exist. Decompress into a new
   file at that exact original path with create-new semantics; do not overwrite
   a save or substitute it for a canonical input.
3. Verify the restored file against `original_bytes` and `original_sha256`.
   Keep the gzip until this verification succeeds. The recorded creation,
   last-write and last-access UTC values can then restore its timestamps.

No restoration was executed during packaging. The original reports still name
the raw intermediate paths; this maintenance record maps each to its recoverable
adjacent gzip. No external/user data, binaries or frozen qualification outputs
were selected. This work does not earn S22, G5 or CP1.
