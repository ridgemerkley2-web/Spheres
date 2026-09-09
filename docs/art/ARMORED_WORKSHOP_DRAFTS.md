# Armored workshop browser drafts

The standalone armored workshop remembers separate visual drafts for the IFV,
APC, reconnaissance vehicle, self-propelled artillery and air-defense vehicle.
Each draft includes its component selections, finish, condition and detail level.
Reloading restores the last selected family and validates its components before
the first model is mounted.

These are browser-local visual previews. They do not create campaign equipment,
spend funds, start research or development, buy stock, or modify a campaign save.
Camera position and selected inspection part are not persisted.

## Storage and recovery

The storage key is `spheres.armored-workshop.drafts`; its schema version is `1`.
Records are stored with `activeFamily` and a `drafts` object keyed by the five
supported platform IDs. Storage belongs to the current browser origin: another
port, browser profile or device has separate drafts. Clearing site data removes
the drafts. Simultaneous tabs use the last successful whole-workshop save;
there is no cross-tab merge protocol.

Opening the page performs no storage write. A component, family, finish,
condition or detail change saves the current normalized records. Reset preset
resets only the current family's components, retaining its finish, condition and
detail level and all other families' drafts.

The reader allowlists families, slots and component values and reconstructs
required weapon/mount/ammunition and sensor pairings. Unknown or obsolete values
return to compatible defaults. Invalid records show a recovery message; the
original stored bytes remain untouched until an explicit user change saves a
replacement. An unsupported positive schema version is preserved throughout the
visit, with changes held only in memory.

When storage is unavailable, denied or full, editing and switching families
continue within the current visit. The status message explains that the current
changes cannot be saved. Such changes are not guaranteed to survive a reload.

## Focused verification

`node --test tools/ui/check_armored_workshop.cjs` passes 18 checks. The original
nine retain native component compatibility, actual controller part selection,
GLB content and export cleanup coverage. Nine added checks exercise schema and
accessor safety, independent family reloads, first-mount pairing repairs,
unavailable and quota-limited storage, preservation of future schemas, and
current-family reset isolation. The controller tests execute the actual workshop
script with an in-memory DOM/storage fixture, actual equipment controller and
mesh/export modules; they do not replace browser visual QA.
