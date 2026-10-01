# Japan / seed 7 — isolated native diagnostic

Started **1 October 2026 at 03:26:08 UTC** (30 September, 8:26 PM Pacific).
The [startup snapshot](startup-snapshot.json) confirms the owned native process
was active at 03:28 UTC. This is a dated startup observation, not a live status
page or a successful campaign result. **S25, G5 and CP1 remain open.**

The [reviewed preparation](../../preparation/crash-capture-20261001/README.md),
commit `1edabe666baf206efdcf33256f91c15b2f73e925`, supplies the guarded launcher.
Its exact SHA-256 is
`e8138eefd9f2133056baeefb1cc8ff34ac4f1d72375a855a1336b36f02f5e427`.
Twelve launcher checks, seven dump checks and five disposable-process cases
passed before launch; the final independent review accepted this exact script.

The original executable and request are unchanged: candidate
`68ba0622ec709b78617aadd1f9198d18f532bb32`, compiled identifier `68ba0622ec70`,
Japan, seed 7, through 31 December 2035. The native opening comparison was matched
and its two initial validations were recorded. No settled-day completion,
terminal pass, crash reproduction or cause is claimed by this startup receipt.
The executable has embedded function symbols and unwind data, but no matching
source-line/local-variable symbols.

The [launch record](launch.json), [exact request](request.json),
[plan](plan.json) and [resource preflight](preflight.json) preserve fixed inputs.
The script and dump reader were copied to a new control directory before launch.
Only the owned wrapper, ProcDump monitor and native child are involved; the
existing user game was not attached to or restarted. Initial PIDs were wrapper
47896, monitor 39860 and native child 37804. Never use those historical PIDs alone
to control a process; validate its executable, start time and ownership first.

## Read the actual outcome here

- Control and outer stdout/stderr:
  `D:/spheres-offload/codex-next-20260928/japan-7-launch-20261001-01/`
- Diagnostic journal, terminal `result.json`, dump files and parsed exception:
  `D:/spheres-offload/codex-next-20260928/japan-7-diagnostic-20261001-01/`
- Native paired-campaign progress and native result: the diagnostic directory's
  `native/` subdirectory.

The wrapper records a terminal result when it finishes. The twelve-hour safety
limit is around **15:26 UTC / 8:26 AM Pacific on 1 October**. Resource limits may
stop it earlier; either a resource stop or a captured native failure is a failed
diagnostic execution, with evidence retained. No diagnostic outcome qualifies the
24-cell matrix. The old failed attempt and its paused monitor remain untouched;
no replacement matrix or dependent verifier was launched.

After termination, inspect the outer logs and wrapper result before interpreting
the native report. Verify any dump's completeness, original exception and owned
process identity, then investigate the faulting stack. If the crash does not
recur, retain that limited result without declaring the original defect fixed.
The [manifest](manifest.json) pins this startup receipt; the growing external
journal and future result are deliberately not frozen here.
