# S09 visual follow-up on the connection fix

The completed capture is `france-W3uT3z/result.json`. It used ordinary New France menus on 1 January 1990 with installed Chrome 152.0.7977.83. All thirteen PNGs were opened and visually inspected. This is a visual check only; native and Node qualification were running concurrently, and no latency or performance acceptance claim is made.

Before launch, the checkout was clean at `e5cfd1a0f18e4114073185b6b05352d236462610` and the game binary SHA-256 was independently verified as `43b5325b184fa1f3f259cc7df9f0fb3ac769b70c11bd0dd7cd7d75a35915a330`, matching `../S09-final-connection-visual-binary.json`. The own server's native `/api/build` reported `e5cfd1a0f18e`.

The tank rendered correctly on desktop and mobile. The first settled mobile canvas capture (330 × 256) shows the detailed tank, tracks, cannon, shadows and ground grid. It was observed after normal viewport resizing and scrolling, with no Reset view click, WebGL-context access, state grants or rendering substitutions. The desktop canvas also rendered on its first sample. This resolves the earlier review limitation: a settled current-build mobile frame is available, and no persistent blank-model defect was observed. The earlier immediate-resize blank capture remains preserved in the prior evidence.

The remaining views are readable at 1440 × 1000 and 390 × 844: two desktop suggestion columns become stacked mobile cards; all costs wrap without clipped amounts; the Current design heading separates active costs from proposals; and the research-part details show known status, installed-part costs and compatible platform exploration. The initial map-help banner expired normally before these captures. The mobile tabs and research stages use their existing horizontal scrolling containers; the page itself does not overflow.

No page errors, failed HTTP responses or game orders were observed. The only POST routes were ordinary campaign creation and the shipped read-only previews. No game simulation time was advanced. The helper closed its own Chrome instance and terminated only its own server, PID 31924. Source files were not modified.
