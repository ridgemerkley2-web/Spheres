# Broad 1990 sector profiles

Fresh browser campaigns enrich inherited accounts with frozen national profiles. Existing campaigns are not backfilled. This is an accounting breakdown: no GDP, cash, goods, demand, jobs or growth bonus is created.

The World Bank WDI agriculture, industry including construction, and services shares are downloaded for 1988–1992. Prefer 1990, then nearest year (earlier breaks ties). This artifact covers 88 countries with all three 1990 observations, 3 with nearby observations, and 46 using a disclosed median of five nearest opening-game GDP-per-capita peers for at least one missing series. Former federations are not silently assigned a modern successor's observations. Every selected observation, year, source URL and fallback peer is recorded. Source license: CC BY 4.0.

Manufacturing keeps the already frozen UNIDO-derived or explicitly modeled share. Agriculture, nonmanufacturing industry and services are proportionally reconciled into the remaining GDP. Net product taxes and statistical discrepancies are therefore allocated proportionally, not represented as a separate sector. Within nonmanufacturing industry the extraction/utilities/construction split is the explicit model ratio 6:4:7; within services transport/market/public is 10:30:15. These narrower splits are not historical observations.

Provincial mass is population multiplied by `1 + 0.1 × clamp(log(local density / national density), -2.5, 2.5)`, then normalized across the nation, with a last-row remainder. The bounded 0.75–1.25 multiplier is a game proxy for concentrated urban activity. It is not a census of establishments or a sourced productivity estimate. Missing area or population receives the population-only fallback. Area comes from the existing Natural Earth map geometry; population uses the existing frozen census. Both inherited factory equivalents and provincial background GDP use the same weights, which then travel with the land.

Rebuild offline: `node tools/industry/collect_sectors.cjs`. Refresh public observations explicitly: add `--fetch`. The raw source cache is tracked; the game never fetches live economic data.

Sources: [Agriculture](https://data.worldbank.org/indicator/NV.AGR.TOTL.ZS), [Industry](https://data.worldbank.org/indicator/NV.IND.TOTL.ZS), [Services](https://data.worldbank.org/indicator/NV.SRV.TOTL.ZS).
