# S09 visual review

This is a bounded visual inspection, not campaign qualification or a performance measurement. All files are outside the source checkout. No game orders were issued.

## Reviewed capture

`france-DBu7gA/result.json` records the successful capture using the corrected Windows release binary. Its native `/api/build` revision is `94d2c094b2c6` (expected full candidate `94d2c094b2c69a70613af2258acbf49e10d9e871`). The separate server opened a fresh France campaign through the ordinary menus on 1 January 1990. Browser: installed Chrome 152.0.7977.83. Desktop viewport: 1440 × 1000; mobile viewport: 390 × 844.

All eleven PNGs in `france-DBu7gA` were opened and visually inspected:

- Desktop top/model: the detailed tank renders, the specification groups and native review ratings are readable, and the unsaved status explains how to retain the design before campaign loading.
- Desktop guidance: lower-cost and advanced cards align in two columns. Native ratings, development, manufacturer capital, later acquisition and daily maintenance are legible. Conditional purchase wording is visible.
- Current costs: the explicit “Current design · Main battle tank” heading clearly separates the active design from both suggestions at desktop and mobile widths.
- Mobile guidance: cards stack without clipped amounts or overlapping labels. Long conditions and cost descriptions wrap. Each card requires vertical scrolling, but its contents remain readable.
- Research component: the native daylight-optics detail displays installed-part costs, known status, a compatible-platform selector and the Explore action. The research-stage columns and top navigation intentionally scroll horizontally on mobile; document and room widths remain 390 pixels, rather than widening the page. The selected component itself is readable.

No page errors, HTTP errors or `/api/command` requests were observed. Document and room scroll widths equal their viewport widths in all four recorded layout checks. The reported offscreen navigation/stage items belong to the horizontal scrolling containers; they are not evidence of page overflow.

## Limits and follow-up

The mobile model PNG was taken immediately after a viewport resize and shows a black canvas while its status still reads “3D model ready.” The desktop model PNG renders normally. This capture does not establish whether the mobile image remains blank after the renderer settles; verify a settled mobile frame during the main browser journey before treating this as a runtime defect. No rendering code was changed.

The opening map-help banner overlays the upper part of several early captures. The shipped banner expires after 5.2 seconds; these were rapid screenshots after ordinary campaign creation. It is not a permanent content obstruction, although retiring map-only guidance when entering the equipment room would improve that initial transition.

A separate settled-frame follow-up (`france-cnVL13`) passed build identity but timed out waiting 30 seconds for initial DOMContentLoaded, before the New France flow. It produced no additional reviewed UI evidence. The failed attempt and its logs remain intact; no timeout was raised and no successful qualification is claimed.

## Preserved initial failure and cleanup

`france-t7nX4J` stopped before browser launch because the first binary reported `94d2c094b2c6-modified`. Root identified Windows long-path Git status as the cause, corrected repository-local Git configuration and rebuilt; the successful capture used the subsequent exact clean stamp. The original failure remains intact.

Each helper closed its own browser and terminated only its own server (PIDs 8988, 26332 and 4772 respectively). The main review server and other qualification processes were not touched.
