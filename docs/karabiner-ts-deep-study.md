# `karabiner.ts` Study And `kar` Ports

This note captures what we studied in `~/repos/evan-liu/karabiner.ts` and what
we ported into `~/code/kar` with a zero-latency-first bias for seq workflows.

## Implemented Now

## 1) Condition model expansion

`kar` now supports these `condition` shapes in rule/layer configs:

- `app`, `apps`, `app_unless`, `apps_unless`
- `variable`, `variable_unless`
- `device`, `devices`, `device_unless`, `devices_unless`
- `device_exists`, `devices_exists`, `device_exists_unless`, `devices_exists_unless`
- `input_source`, `input_sources`, `input_source_unless`, `input_sources_unless`
- `keyboard_type`, `keyboard_types`, `keyboard_type_unless`, `keyboard_types_unless`

This mirrors the most useful condition coverage from `karabiner.ts` while
keeping the current declarative config style in `kar`.

## 2) Low-latency seq bindings in TypeScript helpers

Added helper functions in `types/index.ts`:

- `seqOpenApp(app, endpoint?)` -> `socket_command` (`OPEN_APP ...`)
- `seqOpenAppToggle(app, endpoint?)` -> `socket_command` (`OPEN_APP_TOGGLE ...`)
- `sendUserCommand(payload, endpoint?)` -> `send_user_command`
- `seqPasteText(text, endpoint?)` -> `send_user_command` payload `{v:1,type:"paste_text"}`
- `seqEnterText(text, endpoint?)` -> `send_user_command` payload `{v:1,type:"enter_text"}`
- `typeSequence(text, endpoint?)` -> alias to `seqPasteText(...)`

Latency behavior:

- For short ASCII snippets, `kar` compiler expands paste/enter payloads to
  native key events directly (fast path, no bridge hop).
- For long/non-ASCII text, it automatically falls back to `send_user_command`
  for reliability.
- For app switching, `socket_command` avoids shell process spawn.

## 3) Typed `send_user_command` support

`ToKey` typing now includes:

```ts
{ send_user_command: { payload: unknown; endpoint?: string } }
```

so seq command payloads can be authored without `any`.

## Why These First

From `karabiner.ts`, these give the best value/risk ratio for this repo:

- strong condition expressiveness (reliability)
- low-latency seq action wiring (performance)
- no extra runtime daemons/processes (stability)

## What We Did Not Port Yet

Still useful from `karabiner.ts`, but intentionally postponed:

- builder-style API (`rule().manipulators().condition()` etc.)
- `double_tap` abstraction
- delayed/leader/duo-layer abstractions
- import helpers (`importJson`, `importProfile`)
- generalized mapper utilities (`withMapper`, `withCondition`, `withModifier`)

These are good candidates, but each adds API surface and migration complexity.

## Verification

Run:

```bash
cd ~/code/kar
cargo test --quiet
```

Tests include condition translation coverage and simlayer behavior checks.
