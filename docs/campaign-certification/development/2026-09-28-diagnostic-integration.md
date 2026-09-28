# Additional diagnostic integration receipt

This integration adds the complete
[A1 geography/model review](../S27/preparation/a1-geography-20260928/README.md)
and [January 1994 native diagnostic](../S25/preparation/native-profile-1994-20260928/README.md).
They add evidence, not a political repair, completed full matrix or qualification.

The A1 wrapper manifest is SHA-256
`2947276f09a2e5dd1897c33686af5e14bb41c8fe7539a2da28f7fdf31655dec1`.
Its 47 payload files include the complete original geography analysis,
independent reproduction and design review, each with its original manifest.
The independent reviewer checked 28 original payloads, 14 input pins, six
source/Git pairs and 198 exact observation witnesses.

The native diagnostic manifest is SHA-256
`c6df787933ef8282c4f28e9baf4603e0e9a01da8c83912ab7cbd37a406aa51ac`.
After relocating the packet into integration, the standalone read-only verifier
passed all 28 payloads, the complete decoded 135,739,703-byte input, and 31
recorded native-world/headline comparisons. The input decoded SHA-256 is
`5480ccc296c2fe67ef2234871d1ef5fc9f098184ca1c83d885b07010a1368b20`.
This verifies recorded artifacts; no native replay was performed during copying.

Two packaging invocations failed before successful verification and remain
explicit: the generic copier first looked for a `files` manifest key, whereas
the native packet uses `payloads`, raising `KeyError: 'files'` before creating
the destination. The corrected byte-preserving copier retained all 30 files,
including the manifest and its detached digest. The first verifier invocation
then supplied `--expected-manifest-sha256` instead of its actual
`--manifest-sha256` argument and omitted the positional packet directory; it
exited without verifying. The subsequent invocation with the documented arguments
completed successfully. Neither invocation failure changed input or report bytes
or executed a game binary.

Scoped Git attributes preserve both packets' exact bytes across platforms.
The final index check verified all 78 packet files (32,253,138 bytes) against
their actual working bytes and all manifest payload pins; no file was omitted
by ignore rules or transformed by Git. Planning validation passed 44 canonical
markers and 33 separately tracked bounded tasks. These evidence changes require
no additional native run and do not supersede the outstanding A1 failure.
No native policy, coefficient, acceptance limit or frozen full-matrix input changed.
The live distributed matrix continues on its original `5d970f6d` candidate and
`70a51be54a7e3100a5267ca310bf5c7f05a3c388a389b7c54904e6365c477cc8` batch.
Its actual complete results and archive verification remain required.

Remote inventory was fetched again on 28 September. The four reviewed France,
Tonga, Saudi Arabia and India submission tips are unchanged. New
`claude/c01-br-34` at `c2ff9396` is a separate Brazilian-party research claim,
not a submission acceptance or an additional task in this eight-task order.
