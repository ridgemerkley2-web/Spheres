# Combined native verification

This receipt covers frozen commit `3b6cecc8f4d99e2e2bed483baf5c3d5b60fa273d`: the campaign-leader presentation change together with the incoming political correctness repairs and refreshed provenance. Earlier presentation-baseline attempts remain preserved in the neighboring directories; their results are not counted again here.

All commands used the dedicated `D:\spheres-opening-leader-art-20260930-target` target, two build jobs and `GIT_OPTIONAL_LOCKS=0`. The [result receipt](results.json) pins 293 native source/test/example and changed crate inputs. Their hashes, HEAD and the Git index remained unchanged throughout validation.

1. `cargo test --locked --release --workspace --no-fail-fast -- --skip tests::the_resource_pass_stays_under_budget --test-threads=4` passed: **1,994 passed, 0 failed, 115 ignored, 1 filtered**. Full output is retained in [workspace-tests.log](workspace-tests.log). The 115 ignored tests retain their existing fixture, long-run and qualification requirements.
2. After the workspace run ended, [process observations](process-observations.json) found no competing native compilation or campaign workload. Other agents had completed their CPU-heavy checks. The existing user web server was left untouched. `cargo test --locked --release -p spheres-sim --lib tests::the_resource_pass_stays_under_budget -- --exact --nocapture --test-threads=1` passed separately: **1 passed**, measured **0.0604 ms/month** against the unchanged **0.15 ms/month** acceptance limit. The benchmark also reports that its aspirational 0.05 ms/month reading was not met. See [resource-budget.log](resource-budget.log).
3. `cargo build --locked --release -p spheres-web` passed. See [web-build.log](web-build.log).

The resulting server is `D:\spheres-opening-leader-art-20260930-target\release\spheres-web.exe`, 366,672,574 bytes, SHA-256 `44f12747f3463710ccd382da5a89a87cc6a09840fe23b6c10a97d8245c1078eb`. Its complete build identity is retained in the JSON receipt.

These checks validate the combined native implementation and the stated resource benchmark. They do not certify a long campaign, CP1, A1 calibration, historical worldwide coverage or a human playtest. Existing saves and other workloads were not changed. Final browser evidence is recorded separately against this executable using a disposable campaign.
