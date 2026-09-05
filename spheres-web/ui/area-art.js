/* Original room paintings are presentation only: no state, network API or
   simulation values enter this allowlist. All text stays in the HTML UI. */
(function (root) {
  "use strict";
  const keys = Object.freeze(["cabinet", "treasury", "production", "research", "diplomacy", "military", "logistics", "resources", "history", "campaign"]);
  const sizes = Object.freeze(["hero", "compact", "banner", "campaign"]);
  function url(key) {
    return keys.includes(key) ? `/art/areas/${key}-v1.webp` : null;
  }
  function html(key, size = "hero") {
    const src = url(key);
    if (!src) return "";
    const frame = sizes.includes(size) ? size : "hero";
    return `<figure class="area-art area-art--${frame}" data-area="${key}" aria-hidden="true"><img src="${src}" alt="" width="1536" height="1024" loading="${key === "campaign" ? "eager" : "lazy"}" decoding="async" draggable="false"></figure>`;
  }
  function mount(scope) {
    scope.querySelectorAll("[data-area-art]").forEach(slot => {
      slot.innerHTML = html(slot.dataset.areaArt, slot.dataset.areaArtSize);
    });
  }
  const api = Object.freeze({ keys, url, html, mount });
  if (typeof module !== "undefined" && module.exports) module.exports = api;
  root.AreaArt = api;
  if (typeof document !== "undefined") mount(document);
})(typeof globalThis !== "undefined" ? globalThis : this);
