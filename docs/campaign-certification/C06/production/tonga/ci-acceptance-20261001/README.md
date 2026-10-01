# Tonga — three native acceptance checks

**1 October 2026: runtime date limits, save compatibility and institutional rules
passed; country acceptance remains open.**

The tested candidate is `6829ecc75600c3fa11a2df8136f7c4a513c4941e`.
[Linux native job 110424971157](https://github.com/ridgemerkley2-web/Spheres/actions/runs/36878791848/job/110424971157)
completed 2,006 release workspace tests with zero failures, 115 ignored and the
resource test filtered for separate execution. That separate assertion passed
at 0.0763 ms/month against the unchanged 0.15 limit. Release build and generic
native-browser steps passed. Both platform JavaScript/contracts and package
extraction/offline-smoke jobs passed. Windows native was still running at the
snapshot recorded no later than 15:08:08 UTC; no latest Windows pass is claimed.

All ten institutional-leadership tests pass. The source review and exact passing
names in [job-excerpts.json](job-excerpts.json) support these bounded decisions:

| Manifest check | Proven behavior |
|---|---|
| `boundary_dates` | No appointment from dates or read-only views; cutoff refusal on 7 September 2026, eligibility on 8 September, and no new fictional election at the 2036 endpoint. Saved incumbents remain. |
| `save_compatibility` | Legacy institutional absence, appointed-state roundtrip, exact authored heir after actual succession/reload, and rejection of tampered or unjournalled appointment identity. |
| `institutional_rules` | Explicit reform, election, PM recommendation, minister and vacancy actions; Crown/PM separation; role and foreign-actor refusals; costs, caretaker behavior, cabinet limit and takeover authority. Tonga government payload and portrait identity regressions also pass. |

These accept the implemented arcade institution contracts, not complete historical
office terms or 46-year campaign endurance. [receipt.json](receipt.json) carries
the decision and [source-pins.json](source-pins.json) pins every source required by
the unchanged country validator. Source bytes match both the tested Git revision
and the candidate checkout. The earlier Windows run at `64627c82` passed the
same native code, but is supplemental evidence rather than a latest-head pass.

## Requirements kept open

`production_browser` stays pending. The inspected `tools/ui/ci-browser.cjs`
selects USA and checks generic navigation, panels, command recovery and saves;
`ci-integrated.cjs` does not add a Tonga scenario. No Tonga Crown/PM card,
fictional eligibility or institutional-command journey has been certified in an
actual browser. Successful generic browser and package jobs cannot stand in for it.

`likeness_visual_review` and `country_signoff` also stay pending. Fatai Helu's
likeness, the 16 unresolved historical role dispositions and broader post-cutoff
appearance exposure remain recorded. No human signoff is invented. Both political
calibration jobs still fail the existing A1 concentration assertion, reported as
0.57 against `< 0.5`. C01, C06, S23, S25, G5 and CP1 remain open.

The candidate is still separate from the live integration branch. Merge it with
current accepted research using Git ancestry; preserve the C01-47–51 source,
correction and receipt commits. Resolve planning ownership explicitly and
regenerate shared research/census/gap/boundary outputs from the combined tree.
Do not replace current research packets with the older candidate's snapshots.
Revalidate these pins after integration, then run the affected combined checks and
the new Tonga browser scenario on a disposable CI server before publication.

## Evidence and reproduction

The old unexecuted/resource-stopped checkpoints remain in `validation-preserved`;
[previous-checks.json](previous-checks.json) preserves the six pending manifest
entries at the tested revision. This receipt is additive. The two one-shot CI
snapshots and complete decoded logs are retained externally; [external-evidence.json](external-evidence.json)
pins their exact bytes. No local native process or user campaign was started.

Run `python -B -X utf8 verify.py --repo PATH` in this directory to verify receipt
integrity, exact source blobs and external log evidence without network or native
execution. `--external-root PATH` supports relocation of the retained evidence.
This integrity check does not repeat tests or confer historical/visual acceptance.
