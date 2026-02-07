import type { Config } from "../types/index.ts"
import { km } from "../types/index.ts"

export default {
  profile: {
    alone: 80,
    sim: 200,
  },

  simlayers: {
    "s-mode": { key: "s" },
    "semicolon-mode": { key: "semicolon" },
    "d-mode": { key: "d" },
    "f-mode": { key: "f" },
  },

  rules: [
    // S-mode: Essential navigation and editing
    {
      description: "s-mode (essential)",
      layer: "s-mode",
      mappings: [
        // Vim-style navigation
        { from: "h", to: "left_arrow" },
        { from: "j", to: "down_arrow" },
        { from: "k", to: "up_arrow" },
        { from: "l", to: "right_arrow" },

        // Word navigation
        { from: "b", to: { key: "left_arrow", modifiers: "command" } },
        { from: "m", to: { key: "right_arrow", modifiers: "command" } },

        // Editing
        { from: "d", to: "delete_or_backspace" },
        { from: "f", to: "return_or_enter" },
        { from: "g", to: { key: "tab", modifiers: "command" } }, // Switch window

        // Clipboard
        { from: "a", to: { key: "c", modifiers: "command" } }, // Copy
        { from: "n", to: { key: "v", modifiers: "command" } }, // Paste
        { from: "o", to: { key: "x", modifiers: "command" } }, // Cut

        // Tab/Shift-Tab
        { from: "e", to: "tab" },
        { from: "r", to: { key: "tab", modifiers: "shift" } },

        // Text selection
        { from: "w", to: [
          { key: "left_arrow", modifiers: "option" },
          { key: "right_arrow", modifiers: ["option", "shift"] }
        ]},
      ],
    },

    // Semicolon-mode: Shift layer
    {
      description: "semicolon-mode (shift)",
      layer: "semicolon-mode",
      mappings: [
        { from: "q", to: { key: "q", modifiers: "shift" } },
        { from: "w", to: { key: "w", modifiers: "shift" } },
        { from: "e", to: { key: "e", modifiers: "shift" } },
        { from: "r", to: { key: "r", modifiers: "shift" } },
        { from: "t", to: { key: "t", modifiers: "shift" } },
        { from: "y", to: { key: "y", modifiers: "shift" } },
        { from: "u", to: { key: "u", modifiers: "shift" } },
        { from: "i", to: { key: "i", modifiers: "shift" } },
        { from: "o", to: { key: "o", modifiers: "shift" } },
        { from: "p", to: { key: "p", modifiers: "shift" } },
        { from: "a", to: { key: "a", modifiers: "shift" } },
        { from: "s", to: { key: "s", modifiers: "shift" } },
        { from: "d", to: { key: "d", modifiers: "shift" } },
        { from: "f", to: { key: "f", modifiers: "shift" } },
        { from: "g", to: { key: "g", modifiers: "shift" } },
        { from: "h", to: { key: "h", modifiers: "shift" } },
        { from: "j", to: { key: "j", modifiers: "shift" } },
        { from: "k", to: { key: "k", modifiers: "shift" } },
        { from: "l", to: { key: "l", modifiers: "shift" } },
        { from: "z", to: { key: "z", modifiers: "shift" } },
        { from: "x", to: { key: "x", modifiers: "shift" } },
        { from: "c", to: { key: "c", modifiers: "shift" } },
        { from: "v", to: { key: "v", modifiers: "shift" } },
        { from: "b", to: { key: "b", modifiers: "shift" } },
        { from: "n", to: { key: "n", modifiers: "shift" } },
        { from: "m", to: { key: "m", modifiers: "shift" } },
        // Numbers
        { from: "1", to: { key: "1", modifiers: "shift" } },
        { from: "2", to: { key: "2", modifiers: "shift" } },
        { from: "3", to: { key: "3", modifiers: "shift" } },
        { from: "4", to: { key: "4", modifiers: "shift" } },
        { from: "5", to: { key: "5", modifiers: "shift" } },
        { from: "6", to: { key: "6", modifiers: "shift" } },
        { from: "7", to: { key: "7", modifiers: "shift" } },
        { from: "8", to: { key: "8", modifiers: "shift" } },
        { from: "9", to: { key: "9", modifiers: "shift" } },
      ],
    },

    // Simultaneous key combos (no layer needed)
    {
      description: "simultaneous keys",
      mappings: [
        { from: ["j", "k"], to: km("open Safari new tab") },
        { from: ["k", "n"], to: km("open Comet new tab") },
        { from: ["j", "l"], to: { key: "spacebar", modifiers: ["command", "shift"] } },
        { from: ["k", "l"], to: { key: "spacebar", modifiers: ["command", "option", "control"] } },
        { from: ["j", "semicolon"], to: { key: "9", modifiers: ["command", "option", "shift"] } },
      ],
    },

    // Swap : and ;
    {
      description: "swap : and ;",
      mappings: [
        { from: { key: "semicolon", modifiers: [] }, to: { key: "semicolon", modifiers: "shift" } },
        { from: { key: "semicolon", modifiers: "shift" }, to: "semicolon" },
      ],
    },
  ],
} satisfies Config
