# Bauxite coverage source

`ds896-aluminum.xlsx` is the unchanged USGS Data Series 896 global aluminum
workbook, redistributed as a work of the U.S. Government in the public domain.
The existing district-resource provenance already identifies that license.

- [Official DS896 catalog](https://www.usgs.gov/centers/nmic/historical-global-statistics-mineral-and-material-commodities)
- [Original workbook](https://d9-wret.s3.us-west-2.amazonaws.com/assets/palladium/production/mineral-pubs/historical-statistics/global/ds896-aluminum.xlsx)
- Retrieved 22 September 2026; 67,751 bytes.
- SHA-256: `0d8ae069552229b306ce4795a29704d7eb7c859346cfb1058ecddf1e9d268214`.

These bytes match the source hash already recorded in the shipped artifact.
The coverage test reads the actual **Bauxite** sheet's named **1990** column
and published **metric tons** unit. Guinea reports 15,800,000 and Sierra Leone
1,430,000; both are outside the district roster. The World row is an aggregate,
not another omitted producer. The workbook is reporting evidence only. None of
its values are assigned to a new nation, district or runtime resource account.

The reporting pass covers bauxite only. It does not certify other commodities'
ignored source countries, or turn the MRDS location records into 1990 mine output.
