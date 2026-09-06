/* Map presentation controls. All mutations here belong to the local view;
   campaign state and the simulation clock remain owned by their existing APIs. */
(function () {
  "use strict";
  const modes = ["terrain", "political", "fronts"];
  const details = { relief: "Detailed terrain", borders: "Borders", provinces: "Provinces", cities: "Cities", labels: "Labels", features: "Physical names", grid: "Coordinate grid" };
  const defaults = { relief: true, borders: true, provinces: true, cities: true, labels: true, features: true, grid: false };
  let focusKey = null;
  let detailsOpen = false;
  let dockResizeObserver = null;
  let observedDock = null;
  let dockResizeBound = false;

  const escape = value => String(value).replace(/[&<>"']/g, char => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;"
  }[char]));
  const root = () => document.getElementById("mapControls");
  function reserveDockSpace() {
    const app = document.getElementById("app");
    const dock = document.getElementById("commandDock");
    if (!app || !dock) return;
    const height = dock.getBoundingClientRect().height;
    // A hidden campaign has no footprint yet. Keep the CSS fallback (or last
    // visible measurement) until the dock is laid out again.
    if (!(height > 0)) return;
    const bottom = parseFloat(window.getComputedStyle(dock).bottom) || 0;
    const space = `${Math.ceil(height + Math.max(0, bottom) + 8)}px`;
    if (app.style.getPropertyValue("--map-dock-space") !== space) {
      app.style.setProperty("--map-dock-space", space);
    }
  }
  function bindDockSizing() {
    const dock = document.getElementById("commandDock");
    if (!dock) return;
    if (typeof ResizeObserver === "function") {
      if (!dockResizeObserver) dockResizeObserver = new ResizeObserver(reserveDockSpace);
      if (observedDock !== dock) {
        if (observedDock) dockResizeObserver.disconnect();
        dockResizeObserver.observe(dock, { box: "border-box" });
        observedDock = dock;
      }
    }
    // Bottom offsets can change at a media breakpoint even when the dock's
    // border box stays the same size. This also covers older browsers.
    if (!dockResizeBound && typeof window.addEventListener === "function") {
      window.addEventListener("resize", reserveDockSpace, { passive: true });
      dockResizeBound = true;
    }
    reserveDockSpace();
  }
  function flags() {
    if (!ui.mapDetails) ui.mapDetails = {};
    for (const key of Object.keys(details)) {
      if (typeof ui.mapDetails[key] !== "boolean") ui.mapDetails[key] = defaults[key];
    }
    return ui.mapDetails;
  }
  function rememberFocus() {
    const controls = root();
    const active = document.activeElement;
    focusKey = controls && controls.contains(active) ? active?.dataset?.mapFocus || null : null;
    const menu = controls?.querySelector(".map-detail-menu");
    if (menu) detailsOpen = menu.open;
  }
  function html() {
    rememberFocus();
    const state = flags();
    return `<section class="map-controls" id="mapControls" aria-label="Map controls">
      <div class="map-view-controls">
        <div class="map-quick-modes" role="group" aria-label="Map view">${modes.map(key =>
          `<button type="button" data-map-mode="${key}" data-map-focus="mode-${key}" aria-pressed="${ui.mapMode === key}">${escape(MAP_MODES[key]?.label || key)}</button>`
        ).join("")}</div>
        <button type="button" data-map-action="tilt" data-map-focus="tilt" aria-label="Toggle 3D terrain view" aria-pressed="${ui.mapTilt !== false}" title="Angled terrain / top view">3D</button>
        <details class="map-detail-menu"${detailsOpen ? " open" : ""}>
          <summary data-map-focus="details">Details</summary>
          <div class="map-detail-options" role="group" aria-label="Map detail layers">${Object.entries(details).map(([key, label]) =>
            `<button type="button" data-map-detail="${key}" data-map-focus="detail-${key}" aria-pressed="${state[key]}"><span>${label}</span><span class="map-detail-state" aria-hidden="true">${state[key] ? "On" : "Off"}</span></button>`
          ).join("")}</div>
        </details>
      </div>
      <div class="map-camera-controls" role="group" aria-label="Globe camera">
        <button type="button" data-map-action="zoom-out" data-map-focus="zoom-out" aria-label="Zoom out" title="Zoom out (Shift+Z)">−</button>
        <output class="map-view-status" id="mapViewStatus" aria-label="Map view and zoom"></output>
        <button type="button" data-map-action="zoom-in" data-map-focus="zoom-in" aria-label="Zoom in" title="Zoom in (Z)">+</button>
        <button type="button" data-map-action="world" data-map-focus="world" aria-label="World view" title="World view (0)"><span aria-hidden="true">◎</span><span class="map-camera-caption">World</span></button>
        <button type="button" data-map-action="home" data-map-focus="home" aria-label="Center your nation" title="Center your nation"><span aria-hidden="true">⌂</span><span class="map-camera-caption">Home</span></button>
        <button type="button" data-map-action="west" data-map-focus="west" aria-label="Rotate globe west" title="Rotate west (Left arrow)">‹</button>
        <button type="button" data-map-action="east" data-map-focus="east" aria-label="Rotate globe east" title="Rotate east (Right arrow)">›</button>
      </div>
      <output id="terrainStatus" class="map-terrain-status" aria-live="polite"></output>
    </section>`;
  }
  function sync() {
    const controls = root();
    if (!controls) return;
    const state = flags();
    const zoom = Number(ui.cam?.k) || 1;
    const min = typeof Globe3D !== "undefined" ? Globe3D.ZOOM_MIN : 1;
    const max = typeof Globe3D !== "undefined" ? Globe3D.ZOOM_MAX : 32;
    controls.querySelectorAll("[data-map-mode]").forEach(button => {
      button.setAttribute("aria-pressed", String(button.dataset.mapMode === ui.mapMode));
    });
    controls.querySelectorAll("[data-map-detail]").forEach(button => {
      const enabled = state[button.dataset.mapDetail];
      button.setAttribute("aria-pressed", String(enabled));
      const status = button.querySelector(".map-detail-state");
      if (status) status.textContent = enabled ? "On" : "Off";
    });
    const label = MAP_MODES[ui.mapMode]?.label || "Map";
    const status = controls.querySelector("#mapViewStatus");
    if (status) status.textContent = `${label} · ${zoom.toFixed(1)}×`;
    const tilt = controls.querySelector('[data-map-action="tilt"]');
    if (tilt) { tilt.setAttribute('aria-pressed', String(ui.mapTilt !== false)); tilt.textContent = ui.mapTilt !== false ? '3D' : 'Top'; }
    const zoomOut = controls.querySelector('[data-map-action="zoom-out"]');
    const zoomIn = controls.querySelector('[data-map-action="zoom-in"]');
    if (zoomOut) zoomOut.disabled = zoom <= min + 0.0001;
    if (zoomIn) zoomIn.disabled = zoom >= max - 0.0001;
    const layersSummary = document.querySelector(".arc-map-tools > summary");
    if (layersSummary) layersSummary.textContent = `More layers · ${label}`;
    if (typeof GLOBE !== "undefined" && GLOBE?.options) {
      GLOBE.options.showCities = state.cities;
      GLOBE.options.showLabels = state.labels;
    }
  }
  function setMode(mode) {
    if (!Object.hasOwn(MAP_MODES, mode)) return false;
    rememberFocus();
    ui.mapMode = mode;
    POL.dirty = true;
    SEL.dirty = true;
    showTab("map");
    renderMap();
    return true;
  }
  function toggleDetail(key) {
    if (!Object.hasOwn(details, key)) return false;
    rememberFocus();
    const state = flags();
    state[key] = !state[key];
    POL.dirty = true;
    SEL.dirty = true;
    if (typeof GLOBE !== "undefined" && GLOBE?.options) {
      GLOBE.options.showCities = state.cities;
      GLOBE.options.showLabels = state.labels;
    }
    renderMap();
    sync();
    return true;
  }
  function camera(action) {
    if (typeof camUserInput === "function") camUserInput();
    switch (action) {
      case "zoom-in": mapZoom(1.35); break;
      case "zoom-out": mapZoom(1 / 1.35); break;
      case "world": resetCam(); break;
      case "home": homeNation(); break;
      case "west": globeNudge(-1, 0); break;
      case "east": globeNudge(1, 0); break;
      case "tilt":
        ui.mapTilt = ui.mapTilt === false || (Number(ui.cam?.k)||1) < 12;
        if (ui.mapTilt) { ui.mapMode = 'terrain'; flags().relief = true; }
        if (typeof GLOBE !== 'undefined' && GLOBE) GLOBE.setView(GLOBE.yaw, GLOBE.pitch, Math.max(24, GLOBE.zoom));
        renderMap();
        break;
      default: return;
    }
    sync();
  }
  function bind() {
    const controls = root();
    if (!controls) return false;
    document.getElementById("app")?.classList.add("map-controls-ready");
    bindDockSizing();
    controls.querySelectorAll("[data-map-mode]").forEach(button => {
      button.onclick = () => setMode(button.dataset.mapMode);
    });
    controls.querySelectorAll("[data-map-detail]").forEach(button => {
      button.onclick = () => toggleDetail(button.dataset.mapDetail);
    });
    controls.querySelectorAll("[data-map-action]").forEach(button => {
      button.onclick = () => { if (!button.disabled) camera(button.dataset.mapAction); };
    });
    const menu = controls.querySelector(".map-detail-menu");
    if (menu) menu.ontoggle = () => { detailsOpen = menu.open; };
    controls.onkeydown = event => {
      if (event.key === "Escape" && menu?.open) {
        event.preventDefault();
        event.stopPropagation();
        menu.open = false;
        detailsOpen = false;
        menu.querySelector("summary")?.focus({ preventScroll: true });
      }
    };
    sync();
    if (focusKey) {
      const target = [...controls.querySelectorAll("[data-map-focus]")].find(button => button.dataset.mapFocus === focusKey);
      if (target && !target.disabled) target.focus({ preventScroll: true });
      focusKey = null;
    }
    return true;
  }
  window.MapControls = Object.freeze({ html, sync, bind, install: bind, rememberFocus, setMode, toggleDetail });
})();
