// Karabiner key codes
export type KeyCode =
  // Letters
  | "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m"
  | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"
  // Numbers
  | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
  // Function keys
  | "f1" | "f2" | "f3" | "f4" | "f5" | "f6" | "f7" | "f8" | "f9" | "f10" | "f11" | "f12"
  // Modifiers
  | "left_command" | "right_command" | "left_control" | "right_control"
  | "left_option" | "right_option" | "left_shift" | "right_shift"
  | "caps_lock" | "fn"
  // Navigation
  | "return_or_enter" | "escape" | "delete_or_backspace" | "delete_forward"
  | "tab" | "spacebar"
  | "up_arrow" | "down_arrow" | "left_arrow" | "right_arrow"
  | "page_up" | "page_down" | "home" | "end"
  // Punctuation
  | "hyphen" | "equal_sign" | "open_bracket" | "close_bracket" | "backslash"
  | "non_us_pound" | "semicolon" | "quote" | "grave_accent_and_tilde"
  | "comma" | "period" | "slash" | "non_us_backslash"
  // Keypad
  | "keypad_num_lock" | "keypad_slash" | "keypad_asterisk" | "keypad_hyphen"
  | "keypad_plus" | "keypad_enter" | "keypad_period"
  | "keypad_0" | "keypad_1" | "keypad_2" | "keypad_3" | "keypad_4"
  | "keypad_5" | "keypad_6" | "keypad_7" | "keypad_8" | "keypad_9"
  // Media
  | "vk_consumer_brightness_down" | "vk_consumer_brightness_up"
  | "vk_consumer_play" | "vk_consumer_next" | "vk_consumer_previous"
  | "mute" | "volume_down" | "volume_up" | "volume_increment" | "volume_decrement"
  | "display_brightness_increment" | "display_brightness_decrement"
  | "illumination_increment" | "illumination_decrement"
  // Special
  | "print_screen" | "scroll_lock" | "pause" | "insert"
  | "application" | "help" | "power" | "execute" | "menu" | "select" | "stop" | "again" | "undo"

export type Modifier =
  | "command" | "control" | "option" | "shift"
  | "left_command" | "right_command"
  | "left_control" | "right_control"
  | "left_option" | "right_option"
  | "left_shift" | "right_shift"
  | "fn" | "caps_lock"

// Profile timing settings
export interface ProfileSettings {
  /** Timeout for to_if_alone in ms (default: 80) */
  alone?: number
  /** Threshold for simultaneous key detection in ms (default: 200) */
  sim?: number
}

// Simlayer definition
export interface Simlayer {
  /** The key that activates this simlayer */
  key: KeyCode
  /** Optional custom threshold in ms */
  threshold?: number
}

// From key specification
export type FromKey =
  | KeyCode
  | { key: KeyCode; modifiers?: Modifier | Modifier[]; optional?: Modifier[] }
  | KeyCode[] // Simultaneous keys

// Mouse key specification
export interface MouseKey {
  x?: number
  y?: number
  vertical_wheel?: number
  horizontal_wheel?: number
  speed_multiplier?: number
}

// Pointing button (mouse click)
export type PointingButton = "button1" | "button2" | "button3"

// To key specification
export type ToKey =
  | KeyCode
  | { key: KeyCode; modifiers?: Modifier | Modifier[] }
  | { shell: string }
  | { mouse_key: MouseKey }
  | { pointing_button: PointingButton }
  | ToKey[] // Multiple actions

// Condition for when a rule applies
export type Condition =
  | { app: string }
  | { variable: string; value: number | boolean | string }

// A single key mapping
export interface Mapping {
  from: FromKey
  to: ToKey
  /** Action when key is released quickly (tap) */
  to_if_alone?: ToKey
  /** Action when key is held down */
  to_if_held?: ToKey
}

// A rule containing multiple mappings
export interface Rule {
  description: string
  /** Simlayer name to use for these mappings */
  layer?: string
  /** Condition for when this rule applies */
  condition?: Condition
  mappings: Mapping[]
}

// Simple modification (key remapping without conditions)
export interface SimpleModification {
  from: KeyCode
  to: KeyCode
}

// Main config structure
export interface Config {
  profile?: ProfileSettings
  simlayers?: Record<string, Simlayer>
  /** Simple key remappings (e.g., caps_lock -> escape) */
  simple?: SimpleModification[]
  rules: Rule[]
}

// Helper functions for building shell commands
export function shell(command: string): { shell: string } {
  return { shell: command }
}

export function km(macroName: string): { shell: string } {
  return shell(`osascript -e 'tell application "Keyboard Maestro Engine" to do script "${macroName}"'`)
}

export function open(path: string): { shell: string } {
  return shell(`open "${path}"`)
}

export function zed(path: string): { shell: string } {
  // Expand ~ to $HOME for shell
  const expandedPath = path.startsWith("~/") ? `$HOME${path.slice(1)}` : path
  return shell(`open -a /Applications/Zed.app "${expandedPath}"`)
}

export function openUrl(url: string): { shell: string } {
  return shell(`open "${url}"`)
}

export function alfred(workflow: string, trigger: string, arg?: string): { shell: string } {
  const argPart = arg ? ` with argument "${arg}"` : ""
  return shell(`osascript -e 'tell application id "com.runningwithcrayons.Alfred" to run trigger "${trigger}" in workflow "${workflow}"${argPart}'`)
}

export function raycast(extension: string): { shell: string } {
  return shell(`open -g "raycast://extensions/${extension}"`)
}
