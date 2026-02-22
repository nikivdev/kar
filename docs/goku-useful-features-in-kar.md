# Goku Features Ported To `kar`

This document tracks useful ideas taken from
`~/repos/yqrashawn/GokuRakuJoudo` and implemented in `~/code/kar`.

## Why This Exists

Goku is excellent for expressive Karabiner config ergonomics.  
`kar` keeps the same spirit but with TypeScript authoring and a Rust compiler.

Goal: keep runtime behavior fast and reliable while improving config ergonomics.

## Implemented Features

## 1) Simlayer mode control

`kar` supports:

- `mode: "simultaneous"` (default): chord layer key + target key.
- `mode: "hold"`: press/hold layer key to activate variable, tap to emit original key.

Example:

```ts
simlayers: {
  "caps-mode": { key: "escape", mode: "hold", alone: 120 },
}
```

## 2) Per-layer `alone` timeout

Layer configs can override profile-level `alone` timeout:

```ts
profile: { alone: 80, sim: 200 },
simlayers: {
  "caps-mode": { key: "escape", mode: "hold", alone: 120 },
}
```

## 3) Layer key modifiers (Goku-style)

A simlayer can require mandatory modifiers on its trigger key:

```ts
simlayers: {
  "w-mode": { key: "w", modifiers: "left_control" },
}
```

This enables patterns like `left_control+w` as the layer trigger.

## 4) Layer-level conditions

A simlayer can be gated by a condition:

```ts
simlayers: {
  "w-mode": {
    key: "w",
    modifiers: "left_control",
    condition: { app: "^dev\\.zed\\.Zed$" },
  },
}
```

Rule-level `condition` and simlayer-level `condition` are merged together.

## 5) Expanded condition aliases

`kar` condition typing supports:

- `{ app: string }`
- `{ apps: string[] }`
- `{ app_unless: string }`
- `{ apps_unless: string[] }`
- `{ variable: string; value: ... }`

## Behavior Notes

- In `simultaneous` mode, layer-trigger manipulator honors configured layer modifiers.
- In `hold` mode, trigger manipulator also honors configured layer modifiers.
- If simlayer mandatory modifiers are not set, optional defaults to `"any"` for safety.
- These are compile-time config features; no extra runtime daemon/process overhead.

## Full Example

```ts
import type { Config } from "../types/index.ts"

export default {
  profile: { alone: 80, sim: 200 },

  simlayers: {
    "w-mode": {
      key: "w",
      modifiers: "left_control",
      condition: { app: "^dev\\.zed\\.Zed$" },
    },
    "caps-mode": { key: "escape", mode: "hold", alone: 120 },
  },

  rules: [
    {
      description: "zed only layer",
      layer: "w-mode",
      condition: { variable: "ctx", value: 1 },
      mappings: [{ from: "e", to: "tab" }],
    },
    {
      description: "hold layer",
      layer: "caps-mode",
      mappings: [{ from: "h", to: "left_arrow" }],
    },
  ],
} satisfies Config
```

## Verification

Implemented tests in `src/config.rs` cover:

- hold mode gate behavior
- default simultaneous behavior
- modifier propagation from simlayer to simultaneous trigger
- merged rule + layer conditions

Run:

```bash
cd ~/code/kar
cargo test --quiet
```

## Not Ported Yet (Candidates)

Useful Goku ideas that may be added later:

- named alias dictionaries for devices/input sources/app sets
- richer template expansion for shell/socket payloads
- first-class delayed/tap-dance helpers in TS API

Each should be added only if it improves reliability and keeps config semantics clear.
