// Simple example demonstrating all kar features
// Copy to ~/.config/kar/config.ts to use
import type { Config } from "../../types/index.ts"
import { km, shell, zed, open, raycast, alfred } from "../../types/index.ts"

export default {
  // Timing settings
  profile: {
    alone: 80, // to_if_alone timeout (ms)
    sim: 200, // simultaneous key threshold (ms)
  },

  // Simple remappings (no conditions)
  simple: [{ from: "caps_lock", to: "escape" }],

  // Simlayers: default is simultaneous chord mode (layer key + target key)
  simlayers: {
    "s-mode": { key: "s", threshold: 250 },
    "d-mode": { key: "d", threshold: 250 },
    // Layer key with mandatory modifier + app-scoped condition:
    // "w-mode": { key: "w", modifiers: "left_control", condition: { app: "^dev\\.zed\\.Zed$" } },
    // Hold-mode example (recommended for non-typing keys):
    // "escape-mode": { key: "escape", mode: "hold", alone: 120 },
  },

  rules: [
    // ===================
    // Simlayer: press s + another key
    // ===================
    {
      description: "s-mode (navigation)",
      layer: "s-mode",
      mappings: [
        // Simple key remap: s+h = left arrow
        { from: "h", to: "left_arrow" },
        { from: "j", to: "down_arrow" },
        { from: "k", to: "up_arrow" },
        { from: "l", to: "right_arrow" },

        // Key with modifier: s+a = cmd+c (copy)
        { from: "a", to: { key: "c", modifiers: "left_command" } },

        // Multiple modifiers: s+b = cmd+opt+left (word back)
        { from: "b", to: { key: "left_arrow", modifiers: ["left_command", "left_option"] } },

        // Sequence of keys: s+w = select word
        {
          from: "w",
          to: [
            { key: "left_arrow", modifiers: "left_option" },
            { key: "right_arrow", modifiers: ["left_option", "left_shift"] },
          ],
        },
      ],
    },

    // ===================
    // Simlayer: mouse and media
    // ===================
    {
      description: "d-mode (mouse/media)",
      layer: "d-mode",
      mappings: [
        // Mouse clicks
        { from: "v", to: { pointing_button: "button1" } }, // Left click
        { from: "b", to: { pointing_button: "button3" } }, // Middle click
        { from: "c", to: { pointing_button: "button2" } }, // Right click

        // Mouse scroll
        { from: "j", to: { mouse_key: { vertical_wheel: 64 } } }, // Scroll down
        { from: "k", to: { mouse_key: { vertical_wheel: -64 } } }, // Scroll up

        // Media keys
        { from: "h", to: "vk_consumer_previous" },
        { from: "l", to: "vk_consumer_next" },
        { from: "spacebar", to: "vk_consumer_play" },
        { from: "n", to: "volume_decrement" },
        { from: "m", to: "volume_increment" },
        { from: "tab", to: "mute" },

        // Brightness
        { from: "q", to: "display_brightness_decrement" },
        { from: "w", to: "display_brightness_increment" },
      ],
    },

    // ===================
    // Simultaneous keys (no layer needed)
    // ===================
    {
      description: "simultaneous keys",
      mappings: [
        // Press j+k together = escape
        { from: ["j", "k"], to: "escape" },

        // Press f+j together = cmd+space
        { from: ["f", "j"], to: { key: "spacebar", modifiers: "left_command" } },
      ],
    },

    // ===================
    // Shell commands
    // ===================
    {
      description: "app shortcuts",
      layer: "d-mode",
      mappings: [
        // Open app
        { from: "s", to: open("/Applications/Safari.app") },

        // Open in Zed editor
        { from: "z", to: zed("~/.config/kar/config.ts") },

        // Custom shell command
        { from: "t", to: shell("open -a Terminal") },

        // Keyboard Maestro macro
        { from: "1", to: km("My Macro") },

        // Raycast extension
        { from: "r", to: raycast("raycast/clipboard-history/clipboard-history") },

        // Alfred workflow
        { from: "a", to: alfred("com.example.workflow", "trigger") },
      ],
    },

    // ===================
    // App-specific rules
    // ===================
    {
      description: "Safari shortcuts",
      condition: { app: "^com\\.apple\\.Safari$" },
      mappings: [
        // Only active in Safari: cmd+shift+r = hard refresh
        {
          from: { key: "r", modifiers: ["left_command", "left_shift"] },
          to: { key: "r", modifiers: ["left_command", "left_option"] },
        },
      ],
    },

    // ===================
    // Key swap (global)
    // ===================
    {
      description: "swap colon and semicolon",
      mappings: [
        // ; produces : and shift+; produces ;
        {
          from: { key: "semicolon", modifiers: [] },
          to: { key: "semicolon", modifiers: "left_shift" },
        },
        { from: { key: "semicolon", modifiers: "left_shift" }, to: "semicolon" },
      ],
    },
  ],
} satisfies Config
