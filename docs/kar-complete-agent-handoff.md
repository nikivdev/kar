# kar Complete Agent Handoff

This is the single handoff document for agents (including `~/repos/block/goose`) to safely modify keyboard layers in this setup.

Scope:
- compiler/runtime repo: `~/code/kar`
- live config file: `/Users/nikiv/config/i/kar/config.ts`
- live helper types for that config: `/Users/nikiv/config/i/kar/types/index.ts`
- seq bridge/logging stack used by many mappings: `~/code/seq`

## 1) System Architecture

Flow of execution:
1. `kar` executes TypeScript config (`deno` first, then `bun`) via `src/runtime.rs`.
2. JSON output is parsed into `UserConfig` (`src/config.rs`).
3. `UserConfig` is compiled to Karabiner manipulators/rules (`src/config.rs`).
4. Result is written into profile `kar` in `~/.config/karabiner/karabiner.json` (`src/karabiner.rs`).
5. Karabiner-Elements executes those manipulators at keypress time.

CLI:
- `kar` or `kar build`
- `kar watch`
- `kar --dry-run`
- `kar -c /path/to/config.ts`

Flow tasks (`~/code/kar/flow.toml`):
- `f setup`
- `f dev` (watch)
- `f build` (dry-run smoke)
- `f signal-gate`, `f signal-fix-plan`
- `f run -- ...`
- `f deploy`

## 2) Latency Model (What To Prefer)

Use this order whenever editing mappings:

1. Native key events (`to: "x"`, key+modifiers arrays)
- Lowest overhead.
- No shell/process/bridge hops.

2. `socket_command` to local seqd
- Very low overhead.
- Good for app actions (`OPEN_APP`, `OPEN_APP_TOGGLE`) when routed directly.

3. `send_user_command`
- Low overhead and structured payloads.
- Best for seq actions and AI/action metadata.

4. `shell`
- Highest overhead and most failure-prone.
- Use only when no structured/native path exists.

Important optimization currently in compiler:
- `send_user_command` payloads with `type: "paste_text"` or `"enter_text"` compile to native key events when text is short ASCII.
- Non-ASCII or long text (>96 chars) falls back to `send_user_command` for reliability.

## 3) What Was Added Recently (Port + Seq-first Features)

Implemented in `~/code/kar`:
- rich condition coverage:
  - app: `app`, `apps`, `app_unless`, `apps_unless`
  - variable: `variable`, `variable_unless`
  - device: `device`, `devices`, `device_unless`, `devices_unless`
  - device exists: `device_exists`, `devices_exists`, `device_exists_unless`, `devices_exists_unless`
  - input source: `input_source`, `input_sources`, `input_source_unless`, `input_sources_unless`
  - keyboard type: `keyboard_type`, `keyboard_types`, `keyboard_type_unless`, `keyboard_types_unless`
- simlayer extensions:
  - `mode: "simultaneous" | "hold"`
  - `modifiers`, `optional`, `condition`, `alone`
  - `delay_ms` for hold layers
  - leader mode: `leader: true` or `leader: { sticky, escape }`
- mapping extensions:
  - `to_after_key_up`
  - `to_delayed: { invoked, canceled }`
  - per-mapping `parameters`
  - mapping-level `condition`
- double tap:
  - `from: { double_tap: ... }`
  - `double_tap_delay_ms`
  - `to_if_single_tap`
- imports:
  - `imports: [importJson(...), importProfile(...)]`
- seq-first helpers in TS API:
  - `seqOpenApp`, `seqOpenAppToggle`
  - `sendUserCommand`, `seqPasteText`, `seqEnterText`, `typeSequence`
- utility helpers:
  - `doubleTap`, `duoLayer`, `withMapper`, `withCondition`, `toSetVar`
- telemetry metadata injection for `send_user_command` object payloads:
  - `_kar_signal.rule_id`, `_kar_signal.mapping_id`, optional `_kar_signal.signal`

## 4) Schema Surface (Compiler)

Primary source: `~/code/kar/types/index.ts` + `~/code/kar/src/config.rs`.

`Config`:
- `profile?: { alone?: number; sim?: number }`
- `simlayers?: Record<string, Simlayer>`
- `simple?: SimpleModification[]`
- `imports?: ImportSource[]`
- `rules: Rule[]`

`Simlayer`:
- `key`
- `modifiers?`
- `optional?`
- `threshold?`
- `alone?`
- `mode?` (`simultaneous` default, `hold`)
- `condition?`
- `delay_ms?`
- `leader?`
- `note?`

`Mapping`:
- `from`, `to`
- `to_if_alone?`, `to_if_held?`
- `to_after_key_up?`
- `to_delayed?`
- `parameters?`
- `condition?`
- `to_if_single_tap?`
- `double_tap_delay_ms?`
- `id?`, `signal?`, `note?`

`FromKey` supports:
- simple key
- `{ key, modifiers?, optional? }`
- `{ double_tap, modifiers?, optional? }`
- simultaneous array (`["j", "k"]`)

`ToKey` supports:
- key, key+modifiers
- `shell`
- `socket_command`
- `set_variable`
- `send_user_command`
- mouse key
- pointing button
- array of actions

## 5) Live Config Reality (`/Users/nikiv/config/i/kar/config.ts`)

The live config currently imports from `./types/index.ts` in the same directory:
- `/Users/nikiv/config/i/kar/types/index.ts`

That helper file includes local convenience functions used heavily in your config:
- `openApp`, `openAppToggle`, `openUrl`, `openUrlInApp`, `zed`
- `seq`, `seqSocket`
- `paste`, `enter`, `keystroke`
- `km`, `raycast`, `alfred`, `linWidget`
- `seqAgentFromClipboard`, `seqScreenshotOpen`, etc.

So, for Goose edits to your current live config, this file is the behavior contract to follow.

## 6) Layer Edit Protocol For Goose (Strict)

Goal: safely modify a full layer in `/Users/nikiv/config/i/kar/config.ts`.

1. Identify exact target rule block
- Find rule by `description` and `layer` in `rules` array.
- Do not guess by nearby comments only.

2. Edit only intended surface
- Prefer modifying `mappings` inside that one rule.
- Keep unrelated rules untouched.
- Preserve existing comments and order unless user requested reordering.

3. Prefer low-latency action forms
- First choice: native key events.
- Then structured helpers (`openApp`, `seqSocket`, `paste`, `enter`, `sendUserCommand`).
- Avoid introducing new shell commands when a helper already exists.

4. Preserve trigger semantics
- Keep `from` key and layer key semantics exact.
- If adding simultaneous chords, explicitly use array `from: ["x", "y"]`.
- If adding double-tap behavior, use explicit fields and delay.

5. Avoid conflicts
- Search same layer for duplicate `from` keys.
- Search global rules for the same high-risk trigger if it would create ambiguity.

6. Validate compile and apply
- `kar -c /Users/nikiv/config/i/kar/config.ts --dry-run`
- `kar -c /Users/nikiv/config/i/kar/config.ts`

7. Verify runtime behavior
- Test target layer hotkeys in an app where layer should be active.
- If seq action involved, inspect bridge logs:
  - `~/code/seq/cli/cpp/out/logs/kar_uc_bridge.stderr.log`

8. Report back with exact delta
- list changed mappings (`from` -> `to`) and why
- include any fallback behavior introduced

## 7) Preferred Patterns For New Mappings

Use these templates:

Native remap:
```ts
{ from: "h", to: "left_arrow" }
```

Native chord:
```ts
{ from: "j", to: { key: "j", modifiers: ["left_command", "left_shift"] } }
```

Seq app action (structured):
```ts
{ from: "o", to: openApp("Arc") }
```

Low-latency prompt text:
```ts
{ from: "l", to: paste("/prompts:review-push") }
```

Prompt + submit:
```ts
{ from: "u", to: enter("what to run next?") }
```

## 8) AI-facing Guardrails

When an AI edits a layer, enforce:
- no destructive key swaps outside requested rule
- no broad app conditions unless requested
- no shell sleeps/timeouts unless no reliable alternative
- no random endpoint changes for seq commands
- no changes to `profile.sim` / `profile.alone` unless explicitly requested

## 9) Observability + Debugging

If mapping appears broken:
1. confirm compiled profile was updated:
- run `kar -c /Users/nikiv/config/i/kar/config.ts`
- ensure output says profile updated

2. inspect Karabiner EventViewer for incoming `from` key sequence.

3. inspect seq bridge log for command delivery:
- `tail -n 120 ~/code/seq/cli/cpp/out/logs/kar_uc_bridge.stderr.log`

4. if text actions fail in one app only:
- verify focused app accepts synthetic key events.
- test fallback `send_user_command` path if needed.

## 10) Known Compatibility Note

`~/code/kar/types/index.ts` and `/Users/nikiv/config/i/kar/types/index.ts` may diverge.

- Compiler (`kar` binary) supports the richer feature set listed above.
- Live config TS type-checking depends on `/Users/nikiv/config/i/kar/types/index.ts`.

If Goose needs a newer schema field that types reject, either:
- sync/update `/Users/nikiv/config/i/kar/types/index.ts`, or
- express the same behavior using currently exported helpers in that file.

## 11) Recommended Change Workflow (Fast + Safe)

For layer changes only:
1. edit `/Users/nikiv/config/i/kar/config.ts`
2. `kar -c /Users/nikiv/config/i/kar/config.ts --dry-run`
3. `kar -c /Users/nikiv/config/i/kar/config.ts`
4. test key path in target app
5. check seq bridge log if action is seq-backed

For compiler/helper changes in `~/code/kar` too:
1. implement in `~/code/kar`
2. `cd ~/code/kar && cargo test --quiet`
3. `f deploy` (or install binary/types)
4. rerun config compile/apply

## 12) Hand-off Prompt Stub For Goose

Use this as a starting instruction to Goose:

```text
Modify exactly one keyboard layer in /Users/nikiv/config/i/kar/config.ts.
Target rule: <description + layer name>.
Do not change unrelated rules.
Prefer native key events or existing helpers (openApp/seqSocket/paste/enter) over shell commands.
After edits run:
1) kar -c /Users/nikiv/config/i/kar/config.ts --dry-run
2) kar -c /Users/nikiv/config/i/kar/config.ts
Then report changed mappings and any potential conflicts.
```

---

If you want this doc to include a concrete section for one specific layer (for example `r-mode` with a full before/after mapping table), provide that exact layer target and desired behavior and append it as an operator playbook.

## 13) Live Layer Inventory (Current `/Users/nikiv/config/i/kar/config.ts`)

Simlayer keys (54 configured):
- `semicolon-mode`, `quote-mode`, `backslash-mode`
- `1-mode`, `2-mode`, `3-mode`, `4-mode`, `5-mode`, `7-mode`, `8-mode`, `9-mode`, `0-mode`
- `hyphen-mode`, `equal-sign-mode`, `tab-mode`
- `q-mode`, `w-mode`, `e-mode`, `r-mode`, `t-mode`, `u-mode`, `y-mode`, `i-mode`, `o-mode`, `p-mode`
- `open-bracket-mode`, `close-bracket-mode`
- `a-mode`, `s-mode`, `d-mode`, `f-mode`, `g-mode`
- `escape-mode`, `tilde-mode`, `z-mode`, `x-mode`, `c-mode`, `v-mode`, `b-mode`, `n-mode`, `m-mode`
- `comma-mode`, `dot-mode`, `slash-mode`
- `left-control-mode`, `left-option-mode`, `left-command-mode`, `right-command-mode`, `spacebar-mode`
- `ts-mode`, `go-mode`, `py-mode`, `swift-mode`, `rust-mode` (currently commented in rules)

Current rule descriptions:
- `colonkey (shift)`, `swap : and ;`, `skey (essential)`, `sim`
- `backkey (sites)`, `1key (repo prompt)`, `2key (move, searches)`, `3key (ports)`, `4key ()`, `5key (swapping languages)`
- `7key (gh workspaces)`, `8key (gh workspaces)`, `9key (gh workspaces)`, `hyphenkey (gh workspaces)`, `equalSignKey (gh workspaces)`
- `tabkey (sites)`, `qkey (cmd + shift)`, `wkey (apps)`, `ekey (cmd)`, `rkey ()`, `tkey ()`, `ukey (sites)`, `ykey (gh workspaces)`
- `ikey (symbols)`, `okey (things)`, `pkey (zed)`, `openBracketKey (gh workspaces)`, `closeBracketKey (gh workspaces)`
- `akey (ctrl)`, `dkey (mouse)`, `fkey (essential)`, `gkey (actions)`, `capskey`, `tilkey ()`, `zkey ()`, `xkey ()`, `ckey ()`, `vkey ()`, `bkey ()`, `nkey ()`, `mkey ()`, `dotkey ()`
- `tsdot`, `godot`, `pydot`, `swiftdot`, `rustdot`
- `leftControlKey (gh workspaces)`, `leftOptionKey (linear)`, `spacekey`, `spacekey (Dia links)`, `slashKey (zed)`

Use this inventory to target edits precisely.

## 14) Helper Catalog (Exact Exported APIs)

Compiler repo helpers (`~/code/kar/types/index.ts`):
- `shell`, `socketCommand`, `seqSocket`, `sendUserCommand`
- `seqPasteText`, `seqEnterText`, `typeSequence`
- `seqOpenApp`, `seqOpenAppToggle`, `toSetVar`
- `doubleTap`, `importJson`, `importProfile`
- `withMapper`, `withCondition`, `duoLayer`
- `km`, `open`, `zed`, `openUrl`, `alfred`, `raycast`, `linWidget`

Live config helpers (`/Users/nikiv/config/i/kar/types/index.ts`):
- `shell`, `socketCommand`, `sendUserCommand`, `seqSocket`
- `paste`, `enter`, `seqSocketFast`
- `kmFast`, `kmShell`, `km`
- `seq`, `keystroke`
- `open`, `openAppFast`, `openApp`, `openAppToggle`, `openAppToggleFast`
- `zed`, `zedFast`, `zedToggle`
- `openUrl`, `openUrlInApp`
- `seqAgentFromClipboard`, `seqScreenshotOpen`
- `alfred`, `raycast`, `linWidget`

When editing `/Users/nikiv/config/i/kar/config.ts`, prefer this local helper catalog first.

## 15) Exactly How Goose Should Modify One Layer

Given a user request like “change `<layer>` to do X”, execute this:

1. Locate target rule:
```bash
rg -n 'description:|layer:' /Users/nikiv/config/i/kar/config.ts
```

2. Edit only the matching rule object in `rules`.

3. Keep mapping style:
- use existing helper style already used in that rule
- preserve comments and notes where they provide operator intent

4. Validation commands:
```bash
kar -c /Users/nikiv/config/i/kar/config.ts --dry-run
kar -c /Users/nikiv/config/i/kar/config.ts
```

5. Runtime checks:
```bash
tail -n 120 ~/code/seq/cli/cpp/out/logs/kar_uc_bridge.stderr.log
```

6. Return a compact report:
- changed `from` keys
- new `to` behavior
- any conflict risk
- commands run + pass/fail

This is the minimum bar for safe autonomous layer editing.
