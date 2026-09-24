# Node applicability for the selected optimization

The selected runtime uses the smaller stage-one dyads change, SHA-256 `a9bc6689a343d1957130a3b38af4f020e2d22b4d8c2c9dfe77db4b3f89c2a98b`. The later stage-two change was not retained because its measured timing added no benefit. Native results and timing are recorded separately by the parent.

`selected-stage1/result.json` is a fresh, read-only comparison after restoration. The Node runner plus all 107 test files match their checkpoint26 raw hashes, and the test list remains unchanged. Of 1,933 broad source/data/UI inputs, all 1,932 other than dyads.rs remain raw-byte identical. Additional tools/ui and tools/arsenal tracked inputs are unchanged. Node remains v24.12.0. The recorded 1,701-pass, zero-failure, zero-skip Node result therefore remains applicable to these unchanged inputs; it is not described as a new test execution.

`earlier-stage2/` preserves the preceding comparison for the unselected dyads SHA `76bcc8dfdbe03939962a297d17e15d81ce0d03d6d3733ce7f2922495ee8529a8` without altering its bytes or claims. Its result was produced by the earlier inline comparison; `compare_node_inputs.py` packages that same verification approach and was executed for the selected-source receipt only.

The helper reads files, verifies historical receipt hashes, compares input bytes, and checks Node version. It does not run tests or generators. Its output must be a new directory, and it requires an explicit expected dyads SHA. Neither receipt transfers native, browser, political, or performance acceptance to a changed build.
