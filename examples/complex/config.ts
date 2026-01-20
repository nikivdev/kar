// Complex example config demonstrating all kar features
// Copy to ~/.config/kar/config.ts and customize
import type { Config } from "../../types/index.ts"
import { km, raycast, alfred, shell, zed, open } from "../../types/index.ts"

export default {
  profile: {
    alone: 80, // to_if_alone timeout in ms
    sim: 30, // simultaneous key threshold in ms
  },

  // Remap caps_lock to escape (no conditions)
  simple: [{ from: "caps_lock", to: "escape" }],

  // Define simlayers (hold key to activate layer)
  simlayers: {
    "s-mode": { key: "s", threshold: 250 },
    "d-mode": { key: "d", threshold: 250 },
    "f-mode": { key: "f", threshold: 250 },
    "semicolon-mode": { key: "semicolon", threshold: 250 },
    "spacebar-mode": { key: "spacebar", threshold: 250 },
  },

  rules: [
    // ==========================================
    // S-mode: Navigation and editing
    // ==========================================
    {
      description: "s-mode (navigation)",
      layer: "s-mode",
      mappings: [
        // Vim-style arrow keys
        { from: "h", to: "left_arrow" },
        { from: "j", to: "down_arrow" },
        { from: "k", to: "up_arrow" },
        { from: "l", to: "right_arrow" },

        // Word navigation (option+arrow)
        { from: "n", to: { key: "left_arrow", modifiers: "left_option" } },
        { from: "m", to: { key: "right_arrow", modifiers: "left_option" } },

        // Line start/end (cmd+arrow)
        { from: "b", to: { key: "left_arrow", modifiers: "left_command" } },
        { from: "w", to: { key: "right_arrow", modifiers: "left_command" } },

        // Editing
        { from: "d", to: "delete_or_backspace" },
        { from: "f", to: "return_or_enter" },
        { from: "g", to: "tab" },

        // Delete word backward
        {
          from: "e",
          to: { key: "delete_or_backspace", modifiers: "left_option" },
        },

        // Clipboard
        { from: "a", to: { key: "c", modifiers: "left_command" } }, // Copy
        { from: "v", to: { key: "v", modifiers: "left_command" } }, // Paste

        // Undo/Redo
        { from: "z", to: { key: "z", modifiers: "left_command" } },
        { from: "x", to: { key: "z", modifiers: ["left_command", "left_shift"] } },
      ],
    },

    // ==========================================
    // D-mode: Mouse and media controls
    // ==========================================
    {
      description: "d-mode (mouse/media)",
      layer: "d-mode",
      mappings: [
        // Mouse clicks
        { from: "v", to: { pointing_button: "button1" } }, // Left click
        { from: "b", to: { pointing_button: "button3" } }, // Middle click
        { from: "z", to: { pointing_button: "button2" } }, // Right click

        // Media controls
        { from: "h", to: "vk_consumer_previous" },
        { from: "k", to: "vk_consumer_play" },
        { from: "l", to: "vk_consumer_next" },

        // Volume
        { from: "n", to: "volume_decrement" },
        { from: "m", to: "volume_increment" },
        { from: "tab", to: "mute" },

        // Brightness
        { from: "q", to: "illumination_decrement" },
        { from: "w", to: "illumination_increment" },
        { from: "e", to: "display_brightness_decrement" },
        { from: "r", to: "display_brightness_increment" },
      ],
    },

    // ==========================================
    // F-mode: App launching and shortcuts
    // ==========================================
    {
      description: "f-mode (apps)",
      layer: "f-mode",
      mappings: [
        // Open apps (customize these)
        { from: "a", to: open("/Applications/Safari.app") },
        { from: "s", to: open("/Applications/Slack.app") },
        { from: "d", to: open("/Applications/Discord.app") },
        { from: "t", to: open("/Applications/Terminal.app") },
        { from: "c", to: open("/Applications/Visual Studio Code.app") },

        // Open in Zed editor
        { from: "z", to: zed("~/.config/kar/config.ts") },
        { from: "x", to: zed("~/projects") },

        // Keyboard Maestro macros (if you use KM)
        { from: "1", to: km("Screenshot") },
        { from: "2", to: km("New Note") },

        // Raycast extensions (if you use Raycast)
        { from: "r", to: raycast("raycast/clipboard-history/clipboard-history") },
        { from: "e", to: raycast("raycast/emoji-symbols/search-emoji-symbols") },

        // Alfred workflows (if you use Alfred)
        { from: "g", to: alfred("com.example.workflow", "search") },

        // Custom shell commands
        { from: "p", to: shell("open -a 'System Preferences'") },

        // Modifier combinations
        {
          from: "j",
          to: {
            key: "l",
            modifiers: ["left_command", "left_option", "left_shift"],
          },
        },
      ],
    },

    // ==========================================
    // Semicolon-mode: Shift layer for typing
    // ==========================================
    {
      description: "semicolon-mode (shift)",
      layer: "semicolon-mode",
      mappings: [
        // Hold semicolon + letter = shift+letter (for capital letters)
        { from: "q", to: { key: "q", modifiers: "left_shift" } },
        { from: "w", to: { key: "w", modifiers: "left_shift" } },
        { from: "e", to: { key: "e", modifiers: "left_shift" } },
        { from: "r", to: { key: "r", modifiers: "left_shift" } },
        { from: "t", to: { key: "t", modifiers: "left_shift" } },
        { from: "y", to: { key: "y", modifiers: "left_shift" } },
        { from: "u", to: { key: "u", modifiers: "left_shift" } },
        { from: "i", to: { key: "i", modifiers: "left_shift" } },
        { from: "o", to: { key: "o", modifiers: "left_shift" } },
        { from: "p", to: { key: "p", modifiers: "left_shift" } },
        { from: "a", to: { key: "a", modifiers: "left_shift" } },
        { from: "s", to: { key: "s", modifiers: "left_shift" } },
        { from: "d", to: { key: "d", modifiers: "left_shift" } },
        { from: "f", to: { key: "f", modifiers: "left_shift" } },
        { from: "g", to: { key: "g", modifiers: "left_shift" } },
        { from: "h", to: { key: "h", modifiers: "left_shift" } },
        { from: "j", to: { key: "j", modifiers: "left_shift" } },
        { from: "k", to: { key: "k", modifiers: "left_shift" } },
        { from: "l", to: { key: "l", modifiers: "left_shift" } },
        { from: "z", to: { key: "z", modifiers: "left_shift" } },
        { from: "x", to: { key: "x", modifiers: "left_shift" } },
        { from: "c", to: { key: "c", modifiers: "left_shift" } },
        { from: "v", to: { key: "v", modifiers: "left_shift" } },
        { from: "b", to: { key: "b", modifiers: "left_shift" } },
        { from: "n", to: { key: "n", modifiers: "left_shift" } },
        { from: "m", to: { key: "m", modifiers: "left_shift" } },

        // Numbers for symbols: ;+1 = !, ;+2 = @, etc.
        { from: "1", to: { key: "1", modifiers: "left_shift" } },
        { from: "2", to: { key: "2", modifiers: "left_shift" } },
        { from: "3", to: { key: "3", modifiers: "left_shift" } },
        { from: "4", to: { key: "4", modifiers: "left_shift" } },
        { from: "5", to: { key: "5", modifiers: "left_shift" } },
        { from: "6", to: { key: "6", modifiers: "left_shift" } },
        { from: "7", to: { key: "7", modifiers: "left_shift" } },
        { from: "8", to: { key: "8", modifiers: "left_shift" } },
        { from: "9", to: { key: "9", modifiers: "left_shift" } },
        { from: "0", to: { key: "0", modifiers: "left_shift" } },
      ],
    },

    // ==========================================
    // Spacebar-mode: Window management
    // ==========================================
    {
      description: "spacebar-mode (windows)",
      layer: "spacebar-mode",
      mappings: [
        // Window positioning (requires window manager like Rectangle)
        { from: "h", to: { key: "left_arrow", modifiers: ["left_control", "left_option"] } },
        { from: "l", to: { key: "right_arrow", modifiers: ["left_control", "left_option"] } },
        { from: "k", to: { key: "up_arrow", modifiers: ["left_control", "left_option"] } },
        { from: "j", to: { key: "down_arrow", modifiers: ["left_control", "left_option"] } },

        // Maximize
        { from: "m", to: { key: "return_or_enter", modifiers: ["left_control", "left_option"] } },

        // Switch desktops
        { from: "1", to: { key: "1", modifiers: "left_control" } },
        { from: "2", to: { key: "2", modifiers: "left_control" } },
        { from: "3", to: { key: "3", modifiers: "left_control" } },
      ],
    },

    // ==========================================
    // Simultaneous key presses (no layer)
    // ==========================================
    {
      description: "simultaneous keys",
      mappings: [
        // Press j+k together for escape
        { from: ["j", "k"], to: "escape" },

        // Press d+f together for ctrl+c (interrupt)
        { from: ["d", "f"], to: { key: "c", modifiers: "left_control" } },

        // Press f+j together for cmd+space (Spotlight/Raycast)
        { from: ["f", "j"], to: { key: "spacebar", modifiers: "left_command" } },
      ],
    },

    // ==========================================
    // App-specific rules
    // ==========================================
    {
      description: "Terminal vim mode",
      condition: { app: "^com\\.apple\\.Terminal$" },
      mappings: [
        // In Terminal, use ctrl+[ for escape (vim users)
        { from: "open_bracket", to: "escape" },
      ],
    },

    // ==========================================
    // Global remaps
    // ==========================================
    {
      description: "swap colon and semicolon",
      mappings: [
        // Make ; produce : and shift+; produce ;
        { from: { key: "semicolon", modifiers: [] }, to: { key: "semicolon", modifiers: "left_shift" } },
        { from: { key: "semicolon", modifiers: "left_shift" }, to: "semicolon" },
      ],
    },

    // ==========================================
    // Text expansion (type sequences)
    // ==========================================
    {
      description: "text snippets",
      layer: "f-mode",
      mappings: [
        // Type common strings
        { from: "comma", to: ["c", "o", "n", "s", "t", "spacebar"] },
        { from: "period", to: ["r", "e", "t", "u", "r", "n", "spacebar"] },
      ],
    },
  ],
} satisfies Config
