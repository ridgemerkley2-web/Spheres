/* Keep the menu responsive while the large map shader compiles on a cold GPU
   cache. Ordinary compile/link status queries can block; with the extension,
   poll only COMPLETION_STATUS_KHR until it is safe to inspect link status.
   This changes scheduling, not the shader or its total compilation workload.
   Khronos contract and recommended query order:
   https://registry.khronos.org/webgl/extensions/KHR_parallel_shader_compile/

   const mapProgram = await ShaderLoader.program(gl, GLSL_VS, GLSL_MAP, "map");
   The caller owns the returned program. All temporary shaders and failed
   programs are released; no current-program or rendering bindings are changed. */
(function () {
  "use strict";
  const POLL_MS = 20, MAX_WAIT_MS = 120000;

  async function program(gl, vertexSource, fragmentSource, label = "shader") {
    const shaders = [];
    let linkedProgram = null, succeeded = false;
    const checkContext = () => {
      if (gl.isContextLost()) throw new Error(label + " context lost while compiling");
    };
    try {
      checkContext();
      const parallel = gl.getExtension("KHR_parallel_shader_compile");
      for (const [type, source, stage] of [[gl.VERTEX_SHADER, vertexSource, "vs"], [gl.FRAGMENT_SHADER, fragmentSource, "fs"]]) {
        const shader = gl.createShader(type);
        if (!shader) throw new Error(label + "." + stage + " shader allocation failed");
        shaders.push({shader, stage, attached: false});
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
      }
      linkedProgram = gl.createProgram();
      if (!linkedProgram) throw new Error(label + " program allocation failed");
      for (const entry of shaders) {
        gl.attachShader(linkedProgram, entry.shader);
        entry.attached = true;
      }
      gl.linkProgram(linkedProgram);
      if (parallel) {
        const started = performance.now();
        while (true) {
          checkContext();
          const complete = gl.getProgramParameter(linkedProgram, parallel.COMPLETION_STATUS_KHR);
          // The extension reports true on context loss, so completion alone
          // must never be interpreted as a successfully linked program.
          checkContext();
          if (complete) break;
          if (performance.now() - started >= MAX_WAIT_MS)
            throw new Error(label + " shader compilation timed out");
          await new Promise(resolve => setTimeout(resolve, POLL_MS));
        }
      }
      checkContext();
      if (!gl.getProgramParameter(linkedProgram, gl.LINK_STATUS)) {
        const errors = [];
        // Only inspect individual compile results after a failed link; doing
        // this earlier defeats parallel compilation even with the extension.
        for (const entry of shaders) {
          if (!gl.getShaderParameter(entry.shader, gl.COMPILE_STATUS))
            errors.push(label + "." + entry.stage + " compile: " + (gl.getShaderInfoLog(entry.shader) || "(no info log)"));
        }
        errors.push(label + " link: " + (gl.getProgramInfoLog(linkedProgram) || "(no info log)"));
        throw new Error(errors.join("\n"));
      }
      succeeded = true;
      return linkedProgram;
    } finally {
      for (const entry of shaders) {
        if (entry.attached) gl.detachShader(linkedProgram, entry.shader);
        gl.deleteShader(entry.shader);
      }
      if (linkedProgram && !succeeded) gl.deleteProgram(linkedProgram);
    }
  }
  window.ShaderLoader = Object.freeze({program});
})();
