# CI command formatting follow-up — 30 September 2026

The reviewed intake and clean-dependency change was pushed at
`2ac34862b007868c556bbba8cc646e5d92c28485`. Its [GitHub Actions run](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36700816754)
failed before creating any jobs. The public annotation identifies invalid YAML
in `.github/workflows/verify.yml` at line 29. No test suite ran in that attempt.

The unquoted pip command contained `--only-binary=:all:` followed by a space;
YAML interpreted that colon as mapping syntax. The follow-up puts the same
command in a literal block scalar. The dependency versions, executable command,
forty-minute resource budget and all assertions are unchanged.

An independent local PyYAML 6.0.3 check reproduced the original `ScannerError`
at line 29, column 54. The earlier `3947d1f1` workflow parsed, and the corrected
workflow parses all six job definitions while preserving the exact pip command
apart from its terminal newline. `git diff --check` passes. This formatting fix
does not require rerunning the unchanged 589-test clean-environment suite;
actual GitHub execution remains a separate requirement.

The [sealed intake packet](../development/2026-09-30-intake-followup/README.md)
and its original status snapshot remain unchanged. Six of eight original bounded
tasks are complete; A1 still fails its unchanged threshold and the frozen full
24-cell campaign run is continuing separately. No CI, full-matrix, S25, G5 or
CP1 pass is claimed by this formatting repair.
