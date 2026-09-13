# S09 bounded startup diagnosis

Two separate, disposable Chrome sessions traced the frozen release binary reporting native `/api/build` revision `94d2c094b2c6`. These observations are diagnostic samples, not qualification or performance acceptance. No campaign was created or changed and no production code was edited.

- `chrome-46IXNu`: navigation committed and DOMContentLoaded arrived normally (approximately 155 ms from navigation start). The actual menu screenshot was captured.
- `chrome-ZXMCsN`: a second instrumented startup also reached the actual menu normally. This bounded investigation stopped after the second sample.

Each directory retains the raw CDP lifecycle and network records, console and page errors, a Chrome CPU profile, frame tree, initial screenshot, result metadata and own server log. CPU samples in both runs were mostly browser program/idle work with small map-initialization samples, not a sustained synchronous JavaScript hotspot. Startup script requests completed. The favicon returned 404 after DOMContentLoaded; it did not block parsing. The `pending_requests` field is the snapshot immediately after the profiler stopped; the complete final `requests` records show subsequent image/state requests finishing during screenshot capture.

The earlier 30-second failures were not reproduced by these two instrumented sessions. This provides no basis to dismiss those failures, to relax their timeout, or to claim a fixed startup issue. The diagnostic uses navigation commit followed by the same 30-second DOMContentLoaded observation; that split is not proposed as a production or test-harness workaround.

The next useful measurement is observational CDP/profiler instrumentation attached to the actual full browser harness before its unchanged `goto(...domcontentloaded)` call. A failing trace can then distinguish a pending document/script response from a renderer main-thread stall.

Both own browser instances were closed and only their respective own servers (PIDs 30100 and 33292) were terminated. Other game and qualification processes were untouched.
