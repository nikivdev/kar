# kar

> Manage [Karabiner](https://github.com/pqrs-org/Karabiner-Elements) config in TypeScript

## Install

```bash
flow deploy
```

Installs `kar` binary to `~/bin/kar` and types to `~/.config/kar/types/`.

## Usage

```bash
kar              # Build and apply config to 'kar' profile
kar watch        # Watch config and rebuild on changes
kar --dry-run    # Print generated JSON without writing
kar -c other.ts  # Use different config file
kar -p goku      # Target different profile
kar init         # Create example config
```

## Config

Config lives at `~/.config/kar/config.ts`:

```typescript
import type { Config } from "./types/index.ts"
import { km, shell, zed, open, alfred, raycast } from "./types/index.ts"

export default {
  profile: {
    alone: 80,  // to_if_alone timeout (ms)
    sim: 30,    // simultaneous key threshold (ms)
  },

  // Simple key remappings (no conditions)
  simple: [
    { from: "caps_lock", to: "escape" },
  ],

  // Simlayer definitions
  simlayers: {
    "s-mode": { key: "s", threshold: 250 },
    "semicolon-mode": { key: "semicolon", threshold: 250 },
  },

  rules: [
    // Simlayer rule - hold 's' to activate
    {
      description: "s-mode (navigation)",
      layer: "s-mode",
      mappings: [
        { from: "h", to: "left_arrow" },
        { from: "j", to: "down_arrow" },
        { from: "k", to: "up_arrow" },
        { from: "l", to: "right_arrow" },
        { from: "d", to: "delete_or_backspace" },
        { from: "f", to: "return_or_enter" },
        { from: "a", to: { key: "c", modifiers: "left_command" } },  // Copy
        { from: "n", to: { key: "v", modifiers: "left_command" } },  // Paste
      ],
    },

    // Simultaneous keys (no layer)
    {
      description: "simultaneous keys",
      mappings: [
        { from: ["j", "k"], to: km("open Safari new tab") },
        { from: ["k", "l"], to: shell("open -g raycast://...") },
      ],
    },

    // Mouse scroll
    {
      description: "scroll mode",
      layer: "d-mode",
      mappings: [
        { from: "j", to: { mouse_key: { vertical_wheel: 60 } } },
        { from: "k", to: { mouse_key: { vertical_wheel: -60 } } },
      ],
    },
  ],
} satisfies Config
```

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

## Development

```bash
flow dev    # Watch and rebuild kar on source changes
flow build  # Test build with example config
flow run    # Run kar CLI directly
```

## Contributing

Make issues with bugs/features or PRs. Unfinished work will be merged too if the idea is good. Software or docs can always be better.

Thank you. Can see [this](https://nikiv.dev/how-i-code) for how to code fast with AI.

### 🖤

[![Discord](https://go.nikiv.dev/badge-discord)](https://go.nikiv.dev/discord) [![X](https://go.nikiv.dev/badge-x)](https://x.com/nikivdev) [![nikiv.dev](https://go.nikiv.dev/badge-nikiv)](https://nikiv.dev)
