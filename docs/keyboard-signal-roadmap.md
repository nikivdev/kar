# Kar Keyboard Signal Roadmap

Last updated: 2026-02-21

## Goal

Improve `~/code/kar` so keyboard-level data is useful for real RL tuning loops (especially next-type prediction and action routing) without adding noticeable typing latency.

## Data Collected Now

### 1) Static config data (from Kar config source)

We can already extract high-value structure from `config.ts` files:
- rule count, layer usage, mapping count
- from/to action types
- note coverage (`note` fields)
- transport type usage (`send_user_command`, `socket_command`, `shell`)

Scanner added:
- `scripts/scan_config_signals.py`
- task: `f signal-scan`

### 2) Runtime data (captured outside kar, mainly in seq)

Current runtime stream used for training prep includes:
- `next_type.key_down`, `next_type.key_up`, `next_type.flags_changed`
- `kar.intent.v1`, `kar.outcome.v1`, `kar.override.v1` (derived)
- `agent.qa.pair`

These are written to `SEQ_CH_MEM_PATH` (default `~/repos/ClickHouse/ClickHouse/user_files/seq_mem.jsonl`).

## Config Scan Snapshot (Current)

From `f signal-scan` over:
- `~/.config/kar/config.ts`
- `/Users/nikiv/config/i/kar/config.ts`
- `examples/config.ts`
- `examples/simple/config.ts`
- `examples/complex/config.ts`

Totals:
- rules: `69`
- simlayers: `64`
- mappings: `946`
- mapping note coverage: `1.8%` (`17/946`)
- `send_user_command`: `346`
- `socket_command`: `0`
- `shell`: `29`
- observable action share (`send_user_command + socket_command`): `28.76%`

For your main config `/Users/nikiv/config/i/kar/config.ts`:
- rules: `50`
- simlayers: `53`
- mappings: `763`
- mapping note coverage: `2.23%`
- `send_user_command`: `346`
- `shell`: `8`

Interpretation:
- Migration to low-latency observable transport is underway (`send_user_command` is heavily used).
- Metadata coverage is still very low (`note` almost absent), limiting semantic labeling for RL.

## Most Helpful Keyboard-Level Data To Add Next

Priority order for training signal value:

1. **Stable mapping identity**
- `mapping_id`, `rule_id`, `layer_id` attached to runtime actions.
- Enables exact attribution: "which mapping produced success/failure/override".

2. **Action semantics tags**
- Tags like: `intent=open_app`, `intent=next_type_accept`, `domain=editor`, `criticality=high`.
- Should come from config (not regex post-processing).

3. **Acceptance/rejection loop for suggestions**
- Capture explicit `accepted`, `dismissed`, `expired`, `latency_ms`.
- This is core supervised signal for next-type models.

4. **Per-burst context envelope**
- app id, bundle id, optional window title hash, project/repo hint.
- Capture at burst boundaries, not every key event (latency-safe).

5. **Correction signal**
- backspace streaks, undo after accept, immediate overwrite after accept.
- High-value negative signal for ranking.

6. **Token/phrase boundary events**
- delimiters, pause boundaries, phrase completion markers.
- Better training units than raw key stream.

## What to Change in `~/code/kar`

### Phase 1 (safe, now)

1. Keep scanning all configs on every iteration.
- Use `f signal-scan` and commit report snapshots when changing config style.

2. Raise metadata coverage quickly.
- Add `note` (or future `signal`) to high-impact mappings first.
- Start with mappings that call `send_user_command` and multi-step arrays.

3. Reduce opaque `shell` actions where possible.
- Prefer `send_user_command` wrappers for observability + latency.

### Phase 2 (small schema extension in kar)

Add optional fields in config schema/types:
- `mapping.id?: string`
- `mapping.signal?: { intent?: string; tags?: string[]; criticality?: "low"|"med"|"high" }`
- `rule.id?: string`

Keep backwards compatibility by making all optional.

Status:
- Implemented in `~/code/kar` as schema-only fields (deserialization + TypeScript types).
- No execution-path behavior changes in kar runtime/manipulator generation.
- This means no additional shell/spawn/network work and no measurable latency impact from this phase alone.

### Phase 3 (runtime wiring)

Propagate `mapping_id` and `signal` metadata through `send_user_command` payloads so seq can ingest them directly as first-class labels.

Status:
- Implemented in config conversion path.
- When `to` contains `send_user_command` with an object payload, kar now injects:
  - `_kar_signal.rule_id`
  - `_kar_signal.mapping_id`
  - `_kar_signal.signal` (when provided in config)
- Non-object payloads are left unchanged for compatibility.
- Runtime typing latency impact is effectively zero:
  - enrichment happens once during config->Karabiner JSON generation, not per keystroke dispatch
  - no new shell/network calls were added in the key path

### Phase 4 (quality gate)

Enforce telemetry-readiness thresholds before training/export runs.

Status:
- Implemented as `scripts/signal_gate.py` + Flow task `f signal-gate`.
- Gate fails fast when any threshold is violated:
  - `observable_action_share >= 0.70`
  - `shell_action_share <= 0.02`
  - `send_user_command_mapping_id_coverage >= 0.50`
  - `send_user_command_mapping_signal_coverage >= 0.50`
  - `send_user_command_mapping_signal_intent_coverage >= 0.50`
- Uses scan output from `kar_signal_scan_v1`; runs scan+gate in one command.

## Latency Guardrails

Non-negotiables:
- no synchronous network calls in key path
- no shell spawn in high-frequency typing path
- batch writes for event ingestion
- all enrichment async and recoverable

## Commands

```bash
cd ~/code/kar
f signal-scan
f signal-scan-report
f signal-gate
```

Raw scanner usage:

```bash
python3 scripts/scan_config_signals.py \
  ~/.config/kar/config.ts \
  /Users/nikiv/config/i/kar/config.ts \
  examples/config.ts examples/simple/config.ts examples/complex/config.ts \
  --output ~/.local/state/kar/signal-scan.json
```

Gate-only usage:

```bash
python3 scripts/signal_gate.py \
  --report ~/.local/state/kar/signal-scan.json
```

## Definition of "Good Enough" Before Next Kar-RL Cycle

- observable action share >= 0.70
- shell action share <= 0.02
- mapping metadata coverage >= 0.50 on high-impact layers
- acceptance/rejection/latency capture fully wired for next-type
