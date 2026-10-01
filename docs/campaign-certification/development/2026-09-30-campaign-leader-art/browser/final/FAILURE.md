# Retained failed browser checkpoint

The expanded browser invocation returned exit 1: 107 test entries passed and 2 failed (the Greece redirect subtest and its enclosing journey). Greece was absent from the exported country-name dictionary because only countries with active people were retained. The five new artwork preview checks passed.

The original result.json is preserved unchanged. Its `passed: true` is an inherited receipt bug: the last network-only subtest marked the whole journey passed even after another subtest failed. This checkpoint is FAILED and must not be counted as acceptance. The checker now starts with `passed: false` and requires every named browser subjourney to complete before it may set true. A new run will retain the repair result in a separate directory.
