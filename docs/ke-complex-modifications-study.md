# KE-complex_modifications Study for Kar

Last updated: 2026-02-21

## Why this study

Goal: use real-world Karabiner usage patterns to improve `~/code/kar` for high-signal, low-latency telemetry and better RL training data for next-type and action-routing models.

Repo studied:
- `~/repos/pqrs-org/KE-complex_modifications`
- core submodule: `~/repos/pqrs-org/KE-complex_modifications/core`

## What was analyzed

### 1. Recent commit stream (latest non-merge)

From `git log --no-merges -n 20` in KE-complex_modifications:

- `50b8b709` (2026-02-20): Update core (`f163f30`)
- `e30f9a24` (2026-02-11): Update core (`25f2abc`)
- `74df5dc7` (2026-02-09): Add function keys rule
  - files: `public/json/fn_keys_as_standard_fn_keys_in_specific_apps.json`, generator in `src/json/*.json.js`
- `9253aeda` / `4cbb7201` (2026-02-06): Logitech R400 Keynote support
- `1c14fb24`, `5ce630f1`, `ce7bbad0`: Swiss layout iteration

Core submodule commits (latest):
- `f163f30`: update embedded `karabiner_cli` to 15.9.16
- `25f2abc`: update `karabiner_cli` 15.9.14
- recent core work is mostly validation and build pipeline hardening.

### 2. Corpus-level structural analysis

Dataset analyzed:
- `public/json/*.json` (814 files)

Aggregate counts:
- files: 814
- rules: 2,758
- manipulators: 15,358
- average rules/file: 3.39
- average manipulators/rule: 5.57

Manipulator/action pattern highlights:
- dominant type: `basic` (15,350 / 15,358)
- advanced timing/state constructs are common:
  - `to_after_key_up`: 984 manipulators
  - `to_if_alone`: 513
  - `to_if_held_down`: 219
  - `to_delayed_action`: 505
- action payloads:
  - `key_code`: 14,633
  - `set_variable`: 1,822
  - `shell_command`: 438
  - `set_notification_message`: 94

Condition pattern highlights:
- variable conditions dominate:
  - `variable_if`: 4,197
  - `variable_unless`: 2,805
- app scoping is common:
  - `frontmost_application_if`: 1,122
  - `frontmost_application_unless`: 1,002
- regex-heavy bundle matching is pervasive (`^...`, escaped dots)

Variable naming shape:
- many namespaced variable keys to avoid collisions, e.g.
  - `matias_ergo_pro.use_nav_keys_as_right_option`
  - `personal_mingaldrichgan.use_symbols_as_fn`

## What this means for Kar (`~/code/kar`)

### Directly transferable lessons

1. **Strict lint gates are essential**
- KE core has dedicated lints for source/generator/public outputs.
- Transfer to kar: enforce telemetry-readiness gates before deploy/train prep.

2. **App-scoped rules need richer condition syntax**
- KE routinely uses app regex lists and both `if` + `unless`.
- Single-string app match in kar is too narrow for realistic workflows.

3. **Variable namespace discipline matters for telemetry joins**
- KE variable names are explicit and collision-resistant.
- For RL, stable IDs and namespaced metadata are mandatory for attribution.

4. **Generator-first authoring scales better than manual repetition**
- KE source uses generators and compiles to canonical JSON.
- Kar already uses TS config, so we should lean harder into generated mapping blocks with explicit IDs/signals.

5. **Complex key semantics often require delayed/up events**
- `to_after_key_up` and `to_delayed_action` usage is high.
- These are valuable for modeling user intent transitions and correction patterns in training data.

## Changes already applied in Kar from this study

### A) Signal metadata in runtime command payloads (already implemented)

`send_user_command` object payloads now include:
- `_kar_signal.rule_id`
- `_kar_signal.mapping_id`
- `_kar_signal.signal` (if present)

Non-object payloads are unchanged.

Why this matters:
- deterministic attribution from key mapping -> downstream action/outcome in seq/ClickHouse.

### B) Richer app condition support (implemented in this pass)

Kar `UserCondition` now supports:
- `app` (existing)
- `apps` (new list)
- `app_unless` (new)
- `apps_unless` (new list)

These map to:
- `frontmost_application_if`
- `frontmost_application_unless`

Why this matters:
- parity with real-world KE patterns
- better targeting reduces noisy traces and improves training signal quality.

## Recommended next imports from KE approach (priority)

1. **Kar signal lint gate task (high priority)**
- Add `f signal-gate` that fails when:
  - observable action share below threshold
  - shell share above threshold
  - id/signal coverage below threshold on `send_user_command` mappings
- Equivalent to KE’s fail-fast lint culture.

Status:
- Implemented.
- `f signal-gate` now runs scan + threshold checks from one task.

2. **Namespace validator for IDs/signals (high priority)**
- Enforce pattern (e.g., `domain.feature.action`) for `rule.id`, `mapping.id`.
- Catch collisions and empty IDs pre-build.

3. **Support delayed/up semantics in high-level schema (medium)**
- Add declarative `to_after_key_up` and delayed actions in kar schema.
- Important for capturing correction and disambiguation signals.

4. **Generated config modules (medium)**
- Encourage per-domain TS generators in `config.ts` with shared helpers.
- Keep IDs/signals generated deterministically.

5. **Karabiner-cli compatibility check in CI (medium)**
- Borrow KE core idea: validate generated output against current `karabiner_cli`.

## Explicit non-goals (to protect latency)

- no sync network calls in key path
- no shell spawn in high-frequency path
- no per-keystroke heavy parsing

All metadata enrichment should remain build-time/config-time, not keystroke dispatch time.

## Commands used (for repeatability)

```bash
git -C ~/repos/pqrs-org/KE-complex_modifications log --no-merges --oneline -n 20
find ~/repos/pqrs-org/KE-complex_modifications/public/json -name '*.json' | wc -l
python3 <corpus-stats-script>
```
