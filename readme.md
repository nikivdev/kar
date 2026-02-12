# kar

> Manage [Karabiner](https://github.com/pqrs-org/Karabiner-Elements) config in TypeScript

## Dev 

With [flow](https://github.com/nikivdev/flow), run `f setup`, then `f` will search through list of tasks.

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
```

## Contributing

[Use AI](https://nikiv.dev/how-i-code) & [flow](https://github.com/nikivdev/flow). All meaningful issues and PRs will be merged in. Thank you.

[![Discord](https://go.nikiv.dev/badge-discord)](https://go.nikiv.dev/discord) [![X](https://go.nikiv.dev/badge-x)](https://x.com/nikivdev) [![nikiv.dev](https://go.nikiv.dev/badge-nikiv)](https://nikiv.dev)
