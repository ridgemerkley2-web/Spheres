# S08 raw evidence

This package records evidence only; it does not award campaign certification. Failed developmental attempts remain alongside the final-candidate records. See inventory.json for each source path, phase, byte count and SHA256.

Small records and screenshots are exact byte copies. Large records and genuine supplier campaign snapshots are deduplicated only when the entire original SHA256 matches. Each compressed object is an ordered sequence of ZIP members, each containing at most 64 MiB of original bytes. Every ZIP is below 90 MiB. The collector verified each extracted member and their full concatenation against the original hashes.

To restore a snapshot into a new file, use its logical_path from inventory.json:

    python restore-record.py inventory.json "S08-genuine-supplier-LABEL/export/2130-purchased.campaign.json" "restored.campaign.json"

The example path is illustrative; actual dates and filenames come from the inventory. The helper refuses an existing output and verifies every archive/member and the complete reconstructed source. No executable or unrelated legacy campaign bytes are included. Their original paths and hashes remain in external_references or the original qualification proof records. Current-run process samples and timings do not imply browser throughput.
