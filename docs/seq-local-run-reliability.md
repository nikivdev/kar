# seq local-run reliability (Kar + seq)

## Problem
Certain Kar mappings that use `seq(...)` (local `seq run`) would stop working intermittently even though the key chord triggered.

A concrete failure was `space+o` (open x.com in Arc): the mapping compiled correctly, but action execution silently failed.

## Root cause
`/Users/nikiv/code/seq/cli/cpp/out/bin/seq` was getting killed at launch with exit code `137` due to invalid code-signature state.

This manifested as:
- `seq perf` returning exit `137` (or no output)
- `seq run "..."` failing unexpectedly from Kar shell-command path

## Verification commands

```bash
codesign --verify --verbose=4 /Users/nikiv/code/seq/cli/cpp/out/bin/seq
/Users/nikiv/code/seq/cli/cpp/out/bin/seq perf
/Users/nikiv/code/seq/cli/cpp/out/bin/seq run "open x.com front page in Arc"
```

Healthy behavior:
- codesign verify passes
- `seq perf` returns JSON
- `seq run ...` returns `OK`

## Fixes implemented

### 1) seq runtime safety fallback
In `seq` build script (`cli/cpp/run.sh`) a post-publish guard now:
- verifies signatures of final artifacts (`seq`, `libseqmem.dylib`, `libseqch.dylib`)
- re-signs invalid ones automatically

This prevents shipping broken local artifacts that Kar relies on.

### 2) sequence eager-path safety
In `seq` (`cli/cpp/src/actions.mm`), eager keystroke path now:
- uses PID fast-path when available
- falls back to `wait_frontmost + app_settle` when PID is unavailable

This keeps latency low while avoiding key-delivery races.

### 3) Kar mapping strategy
- Keep Arc frontmost-sensitive sequences on `seq(...)` with explicit waits.
- For `j+k` Safari new-tab specifically, prefer `seqSocket("open Safari new tab")` over local `seq(...)`.

## Recurring issue: `j+k` opens Safari but does not create tab

### Symptom
- `j+k` is detected.
- Safari activates, but `cmd+t` is dropped.
- In some cases, when already in Safari, nothing happens.

### Why this recurs
`seq(...)` runs local `seq run` from Karabiner shell-command context. This can drift in reliability depending on runtime signing/permission state and launch context, even when the macro itself is valid.

`seqSocket(...)` goes through the user-command receiver path and has been consistently more stable for this specific keystroke-heavy chord.

### Known-good mapping

```ts
{
  from: ["j", "k"],
  parameters: { simultaneous_threshold_ms: 40 },
  to: seqSocket("open Safari new tab"),
}
```

Use the same for `["k","j"]`.

## Why this matters
When a mapping compiles but still “does nothing,” threshold tuning often is not the issue. Validate local `seq` executable health first.
