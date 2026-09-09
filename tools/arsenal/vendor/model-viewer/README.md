# Locally bundled model-viewer

Unmodified `dist/model-viewer.min.js` from **@google/model-viewer 4.3.1**, used only by the optional, explicitly opened historical tank reference in the static art workshop. It is not loaded by the game equipment designer or arsenal renderer.

- Primary project: https://github.com/google/model-viewer
- Documentation: https://modelviewer.dev/docs/index.html
- Exact package: https://registry.npmjs.org/@google/model-viewer/-/model-viewer-4.3.1.tgz
- Registry SHA512 integrity: `GP+inXhAtY31E8rILVmByA6z8CZZjdlNajddppyI1/j1eIaSQiZcMRaUqTFe7+jv4mzRzwKIOiKBud0apiv+WQ==` (verified before extraction).
- Runtime size: **1,068,903 bytes**.
- Runtime SHA256: `283b0672384614b4847636c306fc93fe4b1fcadc76d668b4e47f0ca76bcf033b`.

The package's Apache-2.0 `LICENSE` and its embedded copyright notices are preserved. The bundled Three.js, Lit and gainmap-js dependency licenses are provided as `THREE-LICENSE`, `LIT-LICENSE`, and `GAINMAP-LICENSE`, copied from their primary repositories (Three.js r183, Lit main, MONOGRID/gainmap-js main) on 8 September 2026. No npm lifecycle script was run; only the exact compiled module and license were extracted from the verified package.

The workshop uses an ordinary self-contained, uncompressed GLB and the built-in neutral environment. No AR, Draco, KTX2, Lottie, external environment image or provider API is enabled. The local runtime is imported only after the user selects **Inspect the textured reference**. Its separate WebGL renderer and texture cost belong to that optional art reference, not to each game card. The source map is not distributed; original minified module bytes are retained.
