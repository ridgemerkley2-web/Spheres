/* Independent NOAA elevation detail. The coast, lake, vegetation and occlusion
   textures keep their original resolution and decode contracts. */
(function () {
  "use strict";
  const WIDTH = 4800, HEIGHT = 2036, LOW = -1500, HIGH = 9000;
  const decodeShader = `#version 300 es
precision highp float;
precision highp sampler2D;
uniform sampler2D uEncoded;
out float heightMetres;
void main() {
  vec2 bytes = floor(texelFetch(uEncoded, ivec2(gl_FragCoord.xy), 0).rg * 255.0 + 0.5);
  heightMetres = dot(bytes, vec2(256.0, 1.0)) * (${HIGH - LOW}.0 / 65535.0) + (${LOW}.0);
}`;

  function decode(gl, program, source, width, height, levels) {
    const texture = gl.createTexture(), framebuffer = gl.createFramebuffer();
    try {
      if (!texture || !framebuffer) throw new Error("Detailed elevation allocation unavailable");
      gl.bindTexture(gl.TEXTURE_2D, texture);
      gl.texStorage2D(gl.TEXTURE_2D, levels, gl.R16F, width, height);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
      gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
      gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, texture, 0);
      if (gl.checkFramebufferStatus(gl.FRAMEBUFFER) !== gl.FRAMEBUFFER_COMPLETE)
        throw new Error("Detailed elevation framebuffer unavailable");
      gl.viewport(0, 0, width, height);
      gl.useProgram(program);
      gl.uniform1i(gl.getUniformLocation(program, "uEncoded"), 0);
      gl.activeTexture(gl.TEXTURE0);
      gl.bindTexture(gl.TEXTURE_2D, source);
      gl.drawArrays(gl.TRIANGLES, 0, 3);
      return { texture, framebuffer };
    } catch (error) {
      gl.deleteTexture(texture); gl.deleteFramebuffer(framebuffer);
      throw error;
    }
  }

  async function create(gl, helpers) {
    const viewportLimit = gl.getParameter(gl.MAX_VIEWPORT_DIMS);
    if (gl.getParameter(gl.MAX_TEXTURE_SIZE) < WIDTH || viewportLimit[0] < WIDTH || viewportLimit[1] < HEIGHT) return null;
    let bitmap, source, program, output, saved;
    try {
      bitmap = await helpers.bitmap("/height-detail.png");
      if (bitmap.width !== WIDTH || bitmap.height !== HEIGHT)
        throw new Error("Detailed elevation dimensions disagree with the bake");
      program = await helpers.program(gl, decodeShader, "height-detail");
      // Capture after the await so another draw during the fetch cannot leave
      // us restoring stale bindings. Only texture unit zero is borrowed.
      saved = {
        active: gl.getParameter(gl.ACTIVE_TEXTURE),
        program: gl.getParameter(gl.CURRENT_PROGRAM),
        drawFramebuffer: gl.getParameter(gl.DRAW_FRAMEBUFFER_BINDING),
        readFramebuffer: gl.getParameter(gl.READ_FRAMEBUFFER_BINDING),
        viewport: gl.getParameter(gl.VIEWPORT),
        colorMask: gl.getParameter(gl.COLOR_WRITEMASK),
        capabilities: [gl.BLEND, gl.DEPTH_TEST, gl.SCISSOR_TEST, gl.CULL_FACE, gl.RASTERIZER_DISCARD]
          .map(capability => [capability, gl.isEnabled(capability)]),
        unpack: [gl.UNPACK_COLORSPACE_CONVERSION_WEBGL, gl.UNPACK_PREMULTIPLY_ALPHA_WEBGL,
          gl.UNPACK_FLIP_Y_WEBGL, gl.UNPACK_ALIGNMENT].map(key => [key, gl.getParameter(key)]),
      };
      gl.activeTexture(gl.TEXTURE0);
      saved.texture = gl.getParameter(gl.TEXTURE_BINDING_2D);
      for (const [capability] of saved.capabilities) gl.disable(capability);
      gl.colorMask(true, true, true, true);
      // Separate pre-existing errors before our own work, never after a draw:
      // clearing after decode would conceal a failed draw as blank elevation.
      for (let i = 0; i < 16 && gl.getError() !== gl.NO_ERROR; i++) { /* bounded drain */ }
      source = helpers.upload(gl, bitmap, gl.RGB8, gl.RGB, gl.NEAREST);
      if (!program || !source) throw new Error("Detailed elevation source unavailable");
      const levels = 1 + Math.floor(Math.log2(WIDTH));
      output = decode(gl, program, source, WIDTH, HEIGHT, levels);
      if (gl.getError() !== gl.NO_ERROR) throw new Error("Detailed elevation decode unavailable");
      gl.bindFramebuffer(gl.FRAMEBUFFER, null);
      gl.deleteFramebuffer(output.framebuffer); output.framebuffer = null;
      gl.bindTexture(gl.TEXTURE_2D, output.texture);
      gl.generateMipmap(gl.TEXTURE_2D);
      if (gl.getError() !== gl.NO_ERROR) throw new Error("Detailed elevation mipmaps unavailable");
      gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
      if (gl.getError() !== gl.NO_ERROR) throw new Error("Detailed elevation filtering unavailable");
      return { texture: output.texture, maxLod: Math.min(8, levels - 1), scale: 2 };
    } catch (error) {
      if (output?.texture) gl.deleteTexture(output.texture);
      if (output?.framebuffer) gl.deleteFramebuffer(output.framebuffer);
      console.warn("[glmap] detailed elevation unavailable; using bundled base relief:", error.message);
      return null;
    } finally {
      if (bitmap?.close) bitmap.close();
      if (source) gl.deleteTexture(source);
      if (program) gl.deleteProgram(program);
      if (saved) {
        gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, saved.drawFramebuffer);
        gl.bindFramebuffer(gl.READ_FRAMEBUFFER, saved.readFramebuffer);
        gl.viewport(...saved.viewport);
        gl.useProgram(saved.program);
        gl.colorMask(...saved.colorMask);
        for (const [capability, enabled] of saved.capabilities) gl[enabled ? "enable" : "disable"](capability);
        for (const [key, value] of saved.unpack) gl.pixelStorei(key, value);
        gl.activeTexture(gl.TEXTURE0);
        gl.bindTexture(gl.TEXTURE_2D, saved.texture);
        gl.activeTexture(saved.active);
      }
    }
  }
  window.HeightDetail = Object.freeze({ create, decodeShader, width: WIDTH, height: HEIGHT,
    low: LOW, high: HIGH });
})();
