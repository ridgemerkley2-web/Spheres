# Spheres page artwork

Spheres uses **39 original painted scenes** across its menus, government rooms, economy and public-service pages, equipment workflow, and eight research domains. Ten existing area paintings are joined by 29 new page paintings. Their shared direction is a fictional 1990s world, tactile places, navy and teal shadows, individual natural lighting, and strong upper/right focal interest with a quiet left side for headings.

The [page manifest](spheres-web/ui/page-art/manifest.json) contains every new scene's complete generation prompt, original and display SHA-256 hashes, byte sizes, dimensions, conversion recipe, and absolute source/archive locations. Its display paths are relative to this repository and remain portable. The new WebPs retain their native **1672 × 941** dimensions, use quality 85/method 6 compression, and total **5,628,772 bytes**. Their unchanged PNG originals retain the generator's embedded provenance. The absolute archive locations are historical records; running the game does not require those folders.

The [existing area manifest](spheres-web/ui/area-art/manifest.json) records the ten earlier 1536 × 1024 paintings, their [original PNGs](tools/area-art/source), and [exact prompts](tools/area-art/prompts). These images remain unchanged. The following catalogue covers all 39 scenes; linked names open their display artwork.

| Scene | Page or purpose |
|---|---|
| [Campaign](spheres-web/ui/area-art/campaign-v1.webp) | Main menu and About: a cartographic study and a world of possibilities. |
| [Cabinet](spheres-web/ui/area-art/cabinet-v1.webp) | Government overview, cash flow, and shared decision header. |
| [Treasury](spheres-web/ui/area-art/treasury-v1.webp) | Budget and national economy. |
| [Production](spheres-web/ui/area-art/production-v1.webp) | Industry, domestic competition, and manufacturing. |
| [Research](spheres-web/ui/area-art/research-v1.webp) | Research menu and overview. |
| [Diplomacy](spheres-web/ui/area-art/diplomacy-v1.webp) | Negotiations and general diplomatic dossiers. |
| [Military](spheres-web/ui/area-art/military-v1.webp) | Conflict decisions. |
| [Logistics](spheres-web/ui/area-art/logistics-v1.webp) | Logistics and movement support. |
| [Resources](spheres-web/ui/area-art/resources-v1.webp) | Supply and national resource endowments. |
| [Oil and gas](spheres-web/ui/page-art/resource-oil-gas-v1.webp) | Petroleum resources: a coastal refinery, storage tanks, and terminal. |
| [History](spheres-web/ui/area-art/history-v1.webp) | Chronicle, charts, keyboard guide, and playtest notes. |
| [Nation selection](spheres-web/ui/page-art/nation-selection-v1.webp) | Choosing a nation and national overview: a sunlit globe room. |
| [Saved campaigns](spheres-web/ui/page-art/saved-campaigns-v1.webp) | Loading campaigns: an archive with expedition maps. |
| [Construction](spheres-web/ui/page-art/construction-v1.webp) | Funded construction: a growing city and bridge works. |
| [Trade](spheres-web/ui/page-art/trade-v1.webp) | International trade: a working container port at dawn. |
| [World markets](spheres-web/ui/page-art/world-markets-v1.webp) | The world economy: a luminous financial district. |
| [Influence](spheres-web/ui/page-art/influence-v1.webp) | Economic spheres and national world view: a circular conference hall. |
| [Military research](spheres-web/ui/page-art/military-research-v1.webp) | Component research and research decisions: a defense laboratory. |
| [Tank designer](spheres-web/ui/page-art/tank-designer-v1.webp) | Equipment design workspace; the interactive 3D model remains separate. |
| [Equipment library](spheres-web/ui/page-art/equipment-library-v1.webp) | Saved designs and certified model revisions. |
| [Proving ground](spheres-web/ui/page-art/proving-ground-v1.webp) | Paid development and model evaluation. |
| [Tank factory](spheres-web/ui/page-art/tank-factory-v1.webp) | Equipment production and delivery. |
| [Army service](spheres-web/ui/page-art/army-service-v1.webp) | Delivered equipment, maintenance, and refits. |
| [Global command](spheres-web/ui/page-art/global-command-v1.webp) | Global strategy and the domination overview. |
| [Decisions](spheres-web/ui/page-art/decisions-v1.webp) | Agency and national decisions: an evening cabinet room. |
| [Policy](spheres-web/ui/page-art/policy-v1.webp) | Cabinet policy: a legislative chamber in thoughtful golden light. |
| [Healthcare](spheres-web/ui/page-art/healthcare-v1.webp) | Public healthcare: a humane, sunlit hospital atrium and clinical wing. |
| [Community services](spheres-web/ui/page-art/community-services-v1.webp) | Local public services: a welcoming community center and garden. |
| [City life](spheres-web/ui/page-art/city-life-v1.webp) | Urban life: a walkable neighborhood, leafy park, apartments, and streetcar. |
| [League](spheres-web/ui/page-art/league-v1.webp) | International standings: a hopeful exhibition rotunda. |
| [Intelligence](spheres-web/ui/page-art/intelligence-v1.webp) | World Briefing and world affairs. |
| [Computing](spheres-web/ui/page-art/science-computing-v1.webp) | Computing research: mainframes and electronic systems. |
| [Communications](spheres-web/ui/page-art/science-communications-v1.webp) | Communications research: radio dishes and an observatory. |
| [Energy](spheres-web/ui/page-art/science-energy-v1.webp) | Energy research and infrastructure. |
| [Materials](spheres-web/ui/page-art/science-materials-v1.webp) | Materials research and industrial science. |
| [Aerospace](spheres-web/ui/page-art/science-aerospace-v1.webp) | Aerospace research and aeronautical engineering. |
| [Biotech](spheres-web/ui/page-art/science-biotech-v1.webp) | Biotechnology and laboratory research. |
| [Transport](spheres-web/ui/page-art/science-transport-v1.webp) | Transport research and movement networks. |
| [Agriculture](spheres-web/ui/page-art/science-agriculture-v1.webp) | Agricultural research and productive landscapes. |

All new art was generated with the built-in image tool, one image per scene, and visually inspected. Source archives for the nine core/equipment and eight science scenes live in the original workspace's `outputs/page-art-originals`; the eight world/government scenes live in `outputs/page-art-world-originals`. The four public-service, city, and petroleum additions are preserved in `outputs/page-art-gap-originals`. Each exact absolute location is recorded in the page manifest. Full batch records were consolidated from the four `outputs/page-art-*-manifest.json` files, whose paths and hashes are retained for provenance.

To extend the collection:

1. Start from the relevant manifest prompt. Describe one coherent physical place, preserve the room's subject and 1990s setting, and keep bright focal details away from heading text. Request no UI, readable text, logos, or national flags.
2. Preserve the selected generated PNG unchanged, including its metadata. Create a new version such as `trade-v2.webp`; keep previous versions and originals. Record the full prompt, generation method, source/archive locations, dimensions, bytes, hashes, and actual conversion steps.
3. Use the existing WebP conversion as a baseline: Pillow, quality 85, method 6. The current page paintings require no resize, crop, or recolor. Record any future changes explicitly.
4. Add the versioned local file to the [page catalogue](spheres-web/ui/page-art.js), the relevant [CSS page selector](spheres-web/ui/page-art.css), and the packaged server asset allowlist. Rebuild `spheres-web`, because the executable embeds the art. Images load from local `/art/pages/` and `/art/areas/` routes and work offline.
5. Verify file hashes and decoded dimensions, then run `node --test tools/ui/check_page_art.cjs tools/ui/check_area_art.cjs`. Inspect the actual page at desktop and narrow widths, including readable headings, financial values, form controls, and focus states.

These paintings are decorative presentation assets. They do not change simulation state, prices, research progress, construction work, military strength, saved campaigns, or the geographic map. Text and interactive controls remain in the existing HTML interface.
