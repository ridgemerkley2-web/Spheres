# S22 camera predicate: independent source/evidence review

Reviewed `97ad0527ad020b535d127c3759c1bdedec60b1cf` and the subsequent
`7762aca3` regression assertion correction. No runtime changes, builds or live
browser runs were performed for this review. The retained map preflight was not
rewritten or promoted.

**Recommendation:** accept this bounded correction to the control-effect
predicate before freezing a qualification pair. No blocking findings.

The native UI's `mapZoom` changes `ui.cam.k` and calls `applyCam`. `applyCam`
recovers yaw/pitch through `Globe3D.unprojectFree`, which bisects a 172.8-degree
latitude interval 24 times and returns the final midpoint. Its latitude error
bound is `172.8 / 2^25 * pi / 180 = 8.988168679017428e-8` radians. The new
`1e-7` angular bound therefore has a direct source-based numerical justification.
The fallback also requires identical finite raw Robinson `cx/cy`, the requested
zoom direction, and all existing ordered/trusted controls. Larger angle changes,
changed map centers, or a stationary zoom remain refusals.

Read-only checks of
`s22-browser-map-guards-preflight/browser-Mm2QB7/result.json` found:

| Preset | Trusted ordered controls | Raw p95 | Rounded controls |
| --- | ---: | ---: | ---: |
| standard | 31 | 45.29999999701977 ms | first zoom-in only |
| low | 31 | 50.099999994039536 ms | first zoom-in only |

Each first zoom-in changed pitch by `1.9947076768112026e-8` radians and yaw by
`-3.037214924006548e-10` radians while both raw map-center coordinates were exactly
unchanged. Executing only the actual source's pure `unprojectFree` function on
those coordinates yielded `48.85660114288331` degrees latitude and
`2.35220001740185` degrees longitude. The resulting pitch exactly equals the
recorded post-control pitch. Applying `setView`'s existing yaw normalization to
the recovered yaw exactly reproduces the recorded post-control yaw too. These
are the expected coordinate round-trip values, not evidence of an unintended
pan.

Both current predicates accepted the two retained control sequences when called
directly as read-only diagnostic checks. The original result still records
`qualification:false`, `passed:false`, and runtime revision
`1a8c07d722de219f94f3f0ace9115c39b2a204fb`; it cannot fill a new qualification
cell. All frame-rate, latency, memory, workload, identity and confirmation
requirements remain unchanged.

The follow-up test assertion `assert.doesNotThrow(() => q.inputs(rows))` is
appropriate: that validator returns a summary with `samples`, not `count`, and
the test concerns acceptance of this camera sequence.

Minor documentation clarification: missing center observations fail when the
rounded-angle fallback is needed. Exactly unchanged globe angles retain the
previous predicate behavior and do not require that fallback.
