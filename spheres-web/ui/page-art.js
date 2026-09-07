/* Asset catalogue for packaging and art validation. The presentation itself is
   CSS-only: changing pages never adds timers, requests, or simulation state. */
(function (root, factory) {
  const value = factory();
  if (typeof module === 'object' && module.exports) module.exports = value;
  else root.PageArt = value;
})(typeof globalThis === 'object' ? globalThis : this, function () {
  'use strict';
  const areaKeys = Object.freeze(['campaign', 'cabinet', 'treasury', 'production', 'research', 'diplomacy', 'military', 'logistics', 'resources', 'history']);
  const pageKeys = Object.freeze(['nation-selection', 'saved-campaigns', 'construction', 'trade', 'world-markets', 'influence', 'military-research', 'tank-designer', 'equipment-library', 'proving-ground', 'tank-factory', 'army-service', 'global-command', 'decisions', 'league', 'intelligence', 'policy', 'science-computing', 'science-communications', 'science-energy', 'science-materials', 'science-aerospace', 'science-biotech', 'science-transport', 'science-agriculture', 'healthcare', 'community-services', 'city-life', 'resource-oil-gas']);
  const scenes = Object.freeze(Object.fromEntries([
    ...areaKeys.map(key => [key, Object.freeze({key, url: `/art/areas/${key}-v1.webp`, file: `area-art/${key}-v1.webp`})]),
    ...pageKeys.map(key => [key, Object.freeze({key, url: `/art/pages/${key}-v1.webp`, file: `page-art/${key}-v1.webp`})]),
  ]));
  function scene(key) { return Object.prototype.hasOwnProperty.call(scenes, key) ? scenes[key] : null; }
  return Object.freeze({areaKeys, pageKeys, scenes, scene});
});
