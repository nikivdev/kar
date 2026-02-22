# kar

> Manage [Karabiner](https://github.com/pqrs-org/Karabiner-Elements) config in TypeScript

## Dev 

With [flow](https://github.com/nikivdev/flow), run `f setup`, then `f` will search through list of tasks.

## Signal Quality Gate

Run before training-data export or RL prep:

```bash
cd ~/code/kar
f signal-gate
f signal-fix-plan
```

This runs config scan + threshold checks and fails fast if telemetry quality drops.

## Install

With [flow](https://github.com/nikivdev/flow), run: `f deploy` (this will put `kar` in your path).

## Usage

```bash
kar              # Build and apply config to 'kar' profile
kar watch        # Watch config and rebuild on changes
kar --dry-run    # Print generated JSON without writing
kar -c other.ts  # Use different config file
kar init         # Create example config
```

## Examples

- [examples/simple/config.ts](examples/simple/config.ts) - all features explained
- [examples/complex/config.ts](examples/complex/config.ts) - comprehensive real-world config

Author's karabiner config can be seen [here](https://github.com/nikivdev/snaps/blob/main/kar/config.ts). [This PR](https://github.com/pqrs-org/Karabiner-Elements/pull/4396) to Karabiner has more useful context about this config. It uses [seq](https://github.com/nikivdev/seq) heavily. Can read [this](https://github.com/nikivdev/seq/blob/main/docs/karabiner-setup.md) to try set it up but it is unstable.

## Helper Functions

```typescript
// Keyboard Maestro macro
km("macro name")

// Shell command
shell("echo hello")

// Open file/app in Zed
zed("~/.config/kar/config.ts")

// Open URL or path
open("raycast://extensions/...")

// Alfred workflow trigger
alfred("workflow_id", "trigger_name", "optional_arg")

// Raycast extension
raycast("extensions/raycast/...")
```

## Modifiers

Single: `"left_command"`, `"left_shift"`, `"left_option"`, `"left_control"`

Multiple: `["left_command", "left_shift"]`

## Simlayers

Default behavior is **simultaneous chord mode** (Goku-style): press layer key + target key.

```typescript
simlayers: {
  "s-mode": { key: "s", threshold: 250 }, // default mode: "simultaneous"
}
```

Optional **hold mode** is available per layer:

```typescript
simlayers: {
  "caps-mode": { key: "escape", mode: "hold", alone: 120 },
}
```

Goku-style layer key modifiers and layer-level conditions are supported:

```typescript
simlayers: {
  "w-mode": {
    key: "w",
    modifiers: "left_control",
    condition: { app: "^dev\\.zed\\.Zed$" },
  },
}
```

Use hold mode only for non-typing keys (or tightly scoped conditions).  
For letter keys in normal typing, prefer simultaneous mode to avoid eaten keys.

Optional hold-layer delay prevents accidental activations during fast typing:

```typescript
simlayers: {
  "r-mode": { key: "r", mode: "hold", delay_ms: 140 },
}
```

Optional leader mode (for hold layers) keeps the layer active after key release
until escape:

```typescript
simlayers: {
  "r-mode": { key: "r", mode: "hold", leader: true },
  // custom:
  // "r-mode": { key: "r", mode: "hold", leader: { sticky: true, escape: ["spacebar"] } },
}
```

## Key Mapping Examples

```typescript
// Simple key
{ from: "h", to: "left_arrow" }

// Key with modifier
{ from: "a", to: { key: "c", modifiers: "left_command" } }

// Multiple modifiers
{ from: "b", to: { key: "left_arrow", modifiers: ["left_command", "left_option"] } }

// Shell command
{ from: "o", to: shell("open -a Safari") }

// Keyboard Maestro
{ from: "m", to: km("My Macro") }

// Mouse scroll
{ from: "j", to: { mouse_key: { vertical_wheel: 60 } } }

// Multiple actions (sequence)
{ from: "w", to: [
  { key: "left_arrow", modifiers: "left_option" },
  { key: "right_arrow", modifiers: ["left_option", "left_shift"] }
]}

// Optional note for documentation
{ from: "o", note: "Open X front page in Arc", to: [open("Arc"), { key: "1", modifiers: "left_control" }] }

// Optional stable ids + signal metadata (schema-only, no runtime latency impact in kar)
{
  id: "map.open.arc.home",
  from: "o",
  to: open("https://arc.net"),
  signal: { intent: "open_arc_home", tags: ["browser", "nav"], criticality: "low" },
}

// Seq low-latency bindings (prefer socket/send_user_command over shell):
{ from: "o", to: seqOpenApp("Arc") }                  // socket_command -> seqd
{ from: "p", to: seqOpenAppToggle("Arc") }            // socket_command -> seqd
{ from: "y", to: seqPasteText("/prompts:review-push") } // native key events fast-path for short ASCII
{ from: "u", to: seqEnterText("what to run next?") }  // same + enter
```

## Condition Coverage

Rule/layer conditions currently support:

- app scopes: `app`, `apps`, `app_unless`, `apps_unless`
- variable scopes: `variable`, `variable_unless`
- device scopes: `device`, `devices`, `device_unless`, `devices_unless`
- device existence: `device_exists`, `devices_exists`, `device_exists_unless`, `devices_exists_unless`
- input source: `input_source`, `input_sources`, `input_source_unless`, `input_sources_unless`
- keyboard type: `keyboard_type`, `keyboard_types`, `keyboard_type_unless`, `keyboard_types_unless`

Mapping-level `condition` is also supported and merged with rule/layer conditions.

## Double Tap

Use `from: { double_tap: ... }` on a mapping:

```typescript
{
  description: "double tap quit guard",
  mappings: [
    {
      from: { double_tap: "q", modifiers: "left_command" },
      to: { key: "q", modifiers: "left_command" },
      double_tap_delay_ms: 200,
      // optional single-tap override:
      // to_if_single_tap: { key: "q", modifiers: "left_command" },
    },
  ],
}
```

This compiles into the standard Karabiner delayed-action variable pattern.

## Imports

Import existing rule sets from JSON files or other Karabiner profiles:

```typescript
imports: [
  importJson("~/.config/karabiner/assets/complex_modifications/vi_mode.json"),
  importProfile("legacy-profile"),
  // explicit config path:
  // importProfile("legacy-profile", "~/.config/karabiner/karabiner.json"),
]
```

## Utility Helpers

Helpers for DRY config composition:

```typescript
const nav = withMapper(["h", "j", "k", "l"] as const, (k) => ({ from: k, to: "left_arrow" }))
const appOnly = withCondition({ app: "^com\\.apple\\.Terminal$" }, nav)
```

## Contributing

[Use AI](https://nikiv.dev/how-i-code) & [flow](https://github.com/nikivdev/flow). All meaningful issues and PRs will be merged in. Thank you.

[![Discord](https://go.nikiv.dev/badge-discord)](https://go.nikiv.dev/discord) [![X](https://go.nikiv.dev/badge-x)](https://x.com/nikivdev) [![nikiv.dev](https://go.nikiv.dev/badge-nikiv)](https://nikiv.dev)
