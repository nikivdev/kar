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

export interface LeaderMode {
  sticky?: boolean
  escape?: KeyCode[]
}

// Simlayer definition
export interface Simlayer {
  /** The key that activates this simlayer */
  key: KeyCode
  /** Mandatory modifiers for the layer key (e.g. left_control + w) */
  modifiers?: Modifier | Modifier[]
  /** Optional modifiers accepted with the layer key */
  optional?: Modifier[]
  /** Optional custom threshold in ms */
  threshold?: number
  /** Optional custom to_if_alone timeout in ms for hold mode */
  alone?: number
  /** Optional activation mode (default: simultaneous) */
  mode?: "hold" | "simultaneous"
  /** Optional condition that gates this layer */
  condition?: Condition
  /** Optional hold delay in ms before layer activates */
  delay_ms?: number
  /** Optional leader mode for hold layers */
  leader?: boolean | LeaderMode
  /** Optional metadata/documentation field */
  note?: string
}

// From key specification
export type FromKey =
  | KeyCode
  | { key: KeyCode; modifiers?: Modifier | Modifier[]; optional?: Modifier[] }
  | { double_tap: KeyCode; modifiers?: Modifier | Modifier[]; optional?: Modifier[] }
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

export interface SocketCommand {
  endpoint: string
  command: string
}

export interface SendUserCommand {
  payload: unknown
  endpoint?: string
}

export interface DeviceIdentifier {
  vendor_id?: number
  product_id?: number
  location_id?: number
  is_keyboard?: boolean
  is_pointing_device?: boolean
  is_game_pad?: boolean
  is_consumer?: boolean
}

export interface InputSource {
  language?: string
  input_source_id?: string
  input_mode_id?: string
}

export interface SetVariable {
  name: string
  value: number | boolean | string
}

export interface DelayedAction {
  invoked: ToKey
  canceled: ToKey
}

export interface MappingParameters {
  simultaneous_threshold_ms?: number
  to_if_alone_timeout_ms?: number
  to_if_held_down_threshold_ms?: number
  to_delayed_action_delay_ms?: number
}

export type ImportSource =
  | { import_json: string }
  | { import_profile: string; karabiner_json?: string }

// To key specification
export type ToKey =
  | KeyCode
  | { key: KeyCode; modifiers?: Modifier | Modifier[] }
  | { shell: string }
  | { socket_command: SocketCommand }
  | { set_variable: SetVariable }
  | { send_user_command: SendUserCommand }
  | { mouse_key: MouseKey }
  | { pointing_button: PointingButton }
  | ToKey[] // Multiple actions

// Condition for when a rule applies
export type Condition =
  | { app: string }
  | { apps: string[] }
  | { app_unless: string }
  | { apps_unless: string[] }
  | { variable: string; value: number | boolean | string }
  | { variable_unless: string; value: number | boolean | string }
  | { device: DeviceIdentifier }
  | { devices: DeviceIdentifier[] }
  | { device_unless: DeviceIdentifier }
  | { devices_unless: DeviceIdentifier[] }
  | { device_exists: DeviceIdentifier }
  | { devices_exists: DeviceIdentifier[] }
  | { device_exists_unless: DeviceIdentifier }
  | { devices_exists_unless: DeviceIdentifier[] }
  | { input_source: InputSource }
  | { input_sources: InputSource[] }
  | { input_source_unless: InputSource }
  | { input_sources_unless: InputSource[] }
  | { keyboard_type: string }
  | { keyboard_types: string[] }
  | { keyboard_type_unless: string }
  | { keyboard_types_unless: string[] }

export type SignalCriticality = "low" | "med" | "high"

export interface MappingSignal {
  /** Optional normalized intent label for training data attribution */
  intent?: string
  /** Optional free-form tags for downstream filtering */
  tags?: string[]
  /** Optional priority/impact indicator */
  criticality?: SignalCriticality
}

// A single key mapping
export interface Mapping {
  /** Stable mapping id for telemetry join/dedupe */
  id?: string
  from: FromKey
  to: ToKey
  /** Action when key is released quickly (tap) */
  to_if_alone?: ToKey
  /** Action when key is held down */
  to_if_held?: ToKey
  /** Action after key-up */
  to_after_key_up?: ToKey
  /** Delayed action semantics */
  to_delayed?: DelayedAction
  /** Per-mapping manipulator parameters */
  parameters?: MappingParameters
  /** Mapping-local condition (merged with rule/layer conditions) */
  condition?: Condition
  /** For from.double_tap: action when single-tap wins */
  to_if_single_tap?: ToKey
  /** For from.double_tap: delay window in ms */
  double_tap_delay_ms?: number
  /** Optional structured signal metadata (schema-only; no runtime cost in kar) */
  signal?: MappingSignal
  /** Optional metadata/documentation field */
  note?: string
}

// A rule containing multiple mappings
export interface Rule {
  /** Stable rule id for telemetry join/dedupe */
  id?: string
  description: string
  /** Simlayer name to use for these mappings */
  layer?: string
  /** Condition for when this rule applies */
  condition?: Condition
  /** Optional metadata/documentation field */
  note?: string
  mappings: Mapping[]
}

// Simple modification (key remapping without conditions)
export interface SimpleModification {
  from: KeyCode
  to: KeyCode
  /** Optional metadata/documentation field */
  note?: string
}

// Main config structure
export interface Config {
  profile?: ProfileSettings
  simlayers?: Record<string, Simlayer>
  /** Simple key remappings (e.g., caps_lock -> escape) */
  simple?: SimpleModification[]
  /** Import rules from JSON files or existing profiles */
  imports?: ImportSource[]
  rules: Rule[]
}

// Helper functions for building shell commands
export function shell(command: string): { shell: string } {
  return { shell: command }
}

export function socketCommand(endpoint: string, command: string): { socket_command: SocketCommand } {
  return { socket_command: { endpoint, command } }
}

export function seqSocket(macroName: string, endpoint = "/tmp/seqd.sock"): { socket_command: SocketCommand } {
  return socketCommand(endpoint, `RUN ${macroName}`)
}

export function sendUserCommand(payload: unknown, endpoint?: string): { send_user_command: SendUserCommand } {
  return { send_user_command: { payload, endpoint } }
}

function seqPayload(payload: Record<string, unknown>): Record<string, unknown> {
  return payload.v === 1 ? payload : { v: 1, ...payload }
}

// Zero-latency-ish text path:
// - short ASCII snippets compile to native key events in kar compiler (no bridge hop)
// - non-ASCII / long text automatically falls back to seq bridge send_user_command path
export function seqPasteText(text: string, endpoint?: string): { send_user_command: SendUserCommand } {
  return sendUserCommand(seqPayload({ type: "paste_text", text }), endpoint)
}

export function seqEnterText(text: string, endpoint?: string): { send_user_command: SendUserCommand } {
  return sendUserCommand(seqPayload({ type: "enter_text", text }), endpoint)
}

// Explicit "type sequence" helper using the same fast/fallback behavior as seqPasteText.
export function typeSequence(text: string, endpoint?: string): { send_user_command: SendUserCommand } {
  return seqPasteText(text, endpoint)
}

// Direct seqd socket commands (no shell process spawn).
export function seqOpenApp(app: string, endpoint = "/tmp/seqd.sock"): { socket_command: SocketCommand } {
  return socketCommand(endpoint, `OPEN_APP ${app}`)
}

export function seqOpenAppToggle(app: string, endpoint = "/tmp/seqd.sock"): { socket_command: SocketCommand } {
  return socketCommand(endpoint, `OPEN_APP_TOGGLE ${app}`)
}

export function toSetVar(name: string, value: number | boolean | string): { set_variable: SetVariable } {
  return { set_variable: { name, value } }
}

// Declare a double-tap from-key object.
export function doubleTap(
  key: KeyCode,
  options?: { modifiers?: Modifier | Modifier[]; optional?: Modifier[] },
): { double_tap: KeyCode; modifiers?: Modifier | Modifier[]; optional?: Modifier[] } {
  return {
    double_tap: key,
    ...(options?.modifiers ? { modifiers: options.modifiers } : {}),
    ...(options?.optional ? { optional: options.optional } : {}),
  }
}

export function importJson(path: string): ImportSource {
  return { import_json: path }
}

export function importProfile(profile: string, karabinerJson?: string): ImportSource {
  return { import_profile: profile, ...(karabinerJson ? { karabiner_json: karabinerJson } : {}) }
}

export function withMapper<T>(src: readonly T[], mapper: (value: T, index: number) => Mapping): Mapping[] {
  return src.map(mapper)
}

export function withCondition(condition: Condition, mappings: readonly Mapping[]): Mapping[] {
  return mappings.map((m) => ({
    ...m,
    condition: m.condition ?? condition,
  }))
}

// Build a duo-layer rule using existing schema primitives.
// Trigger: simultaneous key pair -> set layer variable
// Layer mappings: gated by variable condition
export function duoLayer(
  layerName: string,
  keys: readonly [KeyCode, KeyCode],
  mappings: readonly Mapping[],
  options?: {
    thresholdMs?: number
    sticky?: boolean
    escape?: KeyCode[]
  },
): Rule {
  const [k1, k2] = keys
  const isSticky = options?.sticky ?? false
  const layerCondition: Condition = { variable: layerName, value: 1 }
  const off = toSetVar(layerName, 0)
  const layeredMappings = withCondition(layerCondition, mappings).map((m) => {
    if (isSticky) return m
    const to = Array.isArray(m.to) ? [...m.to, off] : [m.to, off]
    return { ...m, to }
  })
  const escapeMappings = (options?.escape ?? []).map((key): Mapping => ({
    from: key,
    to: off,
    condition: layerCondition,
  }))
  const trigger: Mapping = {
    from: [k1, k2],
    to: toSetVar(layerName, 1),
    to_after_key_up: off,
    ...(options?.thresholdMs
      ? { parameters: { simultaneous_threshold_ms: options.thresholdMs } }
      : {}),
  }
  return {
    description: `duo-layer ${layerName}`,
    mappings: [trigger, ...layeredMappings, ...escapeMappings],
  }
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

export function linWidget(
  tsPath: string,
  options?: {
    ttlMs?: number
  },
): { shell: string } {
  const expandedPath = tsPath.startsWith("~/") ? `$HOME${tsPath.slice(1)}` : tsPath
  const runnerPath = "$HOME/config/i/kar/scripts/lin-widget-run.sh"
  const logFile = "$HOME/Library/Logs/Lin/widget.log"
  const quotedPath = JSON.stringify(expandedPath)
  const quotedRunner = JSON.stringify(runnerPath)
  const quotedLog = JSON.stringify(logFile)
  const ttlMs = options?.ttlMs ?? 0
  const pipeline = [
    `log_file=${quotedLog}; mkdir -p \"$(dirname \\\"$log_file\\\")\";`,
    ttlMs > 0 ? `export LIN_WIDGET_TTL_MS=${ttlMs};` : "",
    `exec ${quotedRunner} ${quotedPath}`,
  ].filter(Boolean).join(" ")
  return shell(`bash -lc ${JSON.stringify(pipeline)}`)
}
