# `kar` Feature Pack From `karabiner.ts` + Goku

This document is the detailed implementation guide for features ported into
`~/code/kar` from:

- `~/repos/yqrashawn/GokuRakuJoudo`
- `~/repos/evan-liu/karabiner.ts`

with an explicit focus on seq-friendly low-latency paths.

## Design Goal

Keep runtime behavior fast and predictable:

- prefer Karabiner-native `socket_command` for seq app actions
- prefer compile-time native key events for short text payloads
- avoid extra shell/process hops unless explicitly needed

## Ported Features

## 1) Rich condition coverage

Rule, layer, and mapping conditions support:

- app scopes: `app`, `apps`, `app_unless`, `apps_unless`
- variable scopes: `variable`, `variable_unless`
- device scopes: `device`, `devices`, `device_unless`, `devices_unless`
- device-exists scopes: `device_exists`, `devices_exists`, `device_exists_unless`, `devices_exists_unless`
- input source scopes: `input_source`, `input_sources`, `input_source_unless`, `input_sources_unless`
- keyboard type scopes: `keyboard_type`, `keyboard_types`, `keyboard_type_unless`, `keyboard_types_unless`

Condition merge order:

1. rule condition
2. layer condition
3. mapping condition

All are applied together.

## 2) Simlayer extensions

Simlayer now supports:

- `mode: "simultaneous"` (default)
- `mode: "hold"`
- `modifiers` / `optional`
- `alone`
- `condition`
- `delay_ms` (hold mode)
- `leader` mode (hold mode)

Leader mode:

- `leader: true` enables leader behavior with default escape keys:
  `escape`, `caps_lock`
- object form:
  - `leader: { sticky?: boolean, escape?: KeyCode[] }`
- non-sticky mode deactivates after the next layer action
- sticky mode keeps layer active until escape key is pressed

## 3) Double tap from-key

New from-key shape:

```ts
{ double_tap: "q", modifiers?: ..., optional?: ... }
```

Mapping options:

- `double_tap_delay_ms`
- `to_if_single_tap`

Compiler behavior:

- emits action manipulator gated by `variable_if`
- emits toggle manipulator with `to_delayed_action`
- no shell process required

## 4) Mapping delayed/up semantics

Mapping supports:

- `to_after_key_up`
- `to_delayed: { invoked, canceled }`
- `parameters`:
  - `simultaneous_threshold_ms`
  - `to_if_alone_timeout_ms`
  - `to_if_held_down_threshold_ms`
  - `to_delayed_action_delay_ms`

This exposes core Karabiner timing semantics directly in `kar` config.

## 5) Imports

Top-level `imports` supports:

- `importJson(path)` -> load rules from JSON file
- `importProfile(profile, karabiner_json?)` -> load rules from a profile

Accepted JSON import shapes:

- object with `rules`
- rules array
- single rule object with `manipulators`

Paths are resolved relative to config file if not absolute.

## 6) Seq low-latency bindings in `types/index.ts`

Added helpers:

- `seqOpenApp(app, endpoint?)`
- `seqOpenAppToggle(app, endpoint?)`
- `sendUserCommand(payload, endpoint?)`
- `seqPasteText(text, endpoint?)`
- `seqEnterText(text, endpoint?)`
- `typeSequence(text, endpoint?)`

Latency behavior for text helpers:

- short ASCII: compiled to native key events (fast path)
- non-ASCII/long text: falls back to bridge payload path (reliable path)

## 7) Utility helpers

- `doubleTap(...)` (from-key helper)
- `duoLayer(...)` (two-key layer helper)
- `withMapper(...)`
- `withCondition(...)`
- `importJson(...)`
- `importProfile(...)`

## Example: Full config slice

```ts
import type { Config } from "../types/index.ts"
import {
  seqOpenApp,
  seqPasteText,
  importJson,
  importProfile,
  doubleTap,
  duoLayer,
} from "../types/index.ts"

export default {
  profile: { alone: 80, sim: 200 },

  imports: [
    importJson("./rules/vi_mode.json"),
    importProfile("legacy-profile"),
  ],

  simlayers: {
    "r-mode": {
      key: "r",
      mode: "hold",
      delay_ms: 140,
      leader: { sticky: false, escape: ["escape", "spacebar"] },
      condition: { app: "^dev\\.zed\\.Zed$" },
    },
  },

  rules: [
    duoLayer(
      "nav-duo",
      ["f", "d"],
      [{ from: "h", to: "left_arrow" }, { from: "l", to: "right_arrow" }],
      { thresholdMs: 180, sticky: false, escape: ["escape"] },
    ),
    {
      description: "Leader nav",
      layer: "r-mode",
      mappings: [
        { from: "o", to: seqOpenApp("Arc") },
        { from: "p", to: seqPasteText("/prompts:review-push") },
      ],
    },
    {
      description: "Double tap",
      mappings: [
        {
          from: doubleTap("q", { modifiers: "left_command" }),
          to: { key: "q", modifiers: "left_command" },
          double_tap_delay_ms: 200,
        },
      ],
    },
  ],
} satisfies Config
```

## Validation

Run:

```bash
cd ~/code/kar
cargo test --quiet
```

Important test coverage includes:

- condition translation (app/device/input/keyboard/variable-unless)
- simlayer hold/simultaneous behavior
- simlayer leader and hold delay behavior
- double-tap lowering
- import JSON loader path
