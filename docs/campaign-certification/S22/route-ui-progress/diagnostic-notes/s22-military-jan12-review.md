# S22 January 12 military-review investigation

Read-only audit of the immutable actual Jan 1 2006 source and runtime code; no timing qualification or source mutation.

- Source: `s22-subsystem-2006-supportfix-01/input.json`, copied from `s22-preparation-active-01/campaigns/saves/s22-2006-01-01.json`.
- Preserved SHA-256 from diagnostic provenance: `7de5c6d0cb30506f9fa6024e6025d31649246e1d53c138d82d0482f447726325`.
- Measured subsystem diagnostic: Jan 12 military AI 420.0077 ms before AirSupport fix; 170.8973 ms after. Both are diagnostic results, not S22 qualification; latter ran concurrently with legacy regressions. Adjacent Jan 11/13 military times were 18.4737/16.9884 ms.
- Code inspected at integration `dc73474f5c71c0d164f42042010c25dd67b9dd67` (AirSupport runtime candidate `594c7ce1356a9c5a393e1079e4b7abff48cf093d`).

## Reconstructed due review and target-only work

The source military plans give `last_review_day = 5825` for Greece, New Zealand, Papua New Guinea, Tunisia and UK. Existing 30-day review scheduling therefore makes them due on absolute day 5855, January 12 2006. Of those five, only UK has a managed supplier. None has a delivered custom squadron or custom ground ammunition requirement in the input.

UK supplier 719 has four jobs, sorted exactly as `military_ai::rotate_supplier_work`:

| Product | Kind | Input target | January 12 rotation target |
| --- | --- | ---: | ---: |
| 730 | ground reconnaissance | 0 | 0 |
| 2215 | light attack aircraft | 0 | 0 |
| 2440 | mg_127 ammunition | 750000 | 0 |
| 3190 | unguided aircraft stores | 0 | 1800 |

`5855.div_euclid(30).rem_euclid(4) = 3`, selecting the aircraft-store row. Its native recipe produces 60 units/work-day, so the existing 30-work-day buffer is 1800. Both changed ammunition targets dispatch ordinary `CompanyOrder::AmmoInventory`.

Each such command takes `companies::trial`, which clones the entire retained world before executing a single target write. `set_ammo_inventory` checks bounds and domestic company/product identities before this write. The equipment `Inventory` arm similarly checks actor, bounds, identities and cancellation before its sole target write. Neither target-only arm settles money, prices a quote, starts production, appends transactions, or invokes fiscal charging.

This is a concrete remaining clone mechanism consistent with the spike, not yet a per-command measured attribution. No military policy or rotation change is proposed. The parent authorized a narrow dispatcher optimization for these two commands with unchanged native setter execution, only when the command has no standing bill or fiscal-policy entry. All other company commands retain their original transaction path. Exact whole-world/error tests and another actual-checkpoint diagnostic must verify it before accepting a speedup claim.

## Source locators

- `spheres-sim/src/military_ai.rs`: review line 986; scheduling line 1117; rotation line 501 and target-order dispatch lines 557/563.
- `spheres-sim/src/companies.rs`: actor line 275; world-cloning trial line 653; equipment Inventory line 863; AmmoInventory line 899 (pre-patch).
- `spheres-sim/src/companies_ammunition.rs`: domestic lookup line 83; target setter line 294.
- `spheres-sim/src/equipment_ammunition_production.rs`: unguided-store recipe line 250.

No native builds, browser runs or benchmark reruns were launched for this read-only investigation.
