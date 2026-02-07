# Karabiner Troubleshooting (Kar)

## Symptom: layers stop working or keys get eaten

**Root cause we hit:** simlayers were implemented as **hold** rules while the config defines
**every letter** as a simlayer key. Hold rules fire on key down and can consume the key,
which means:

- `s` never reaches the `s-mode` mappings (variable never set when you expect),
- regular typing gets blocked,
- even system shortcuts like `Cmd+Tab` fail if `left_command` is a simlayer key.

**Fix:** use **simultaneous chord** simlayers (Goku-style), not hold.

## Expected behavior (stable)

- Only explicit `from: ["a","b"]` entries are true simultaneous shortcuts.
- Simlayers are **chords**: press layer key + target key within threshold.
- No `simlayer hold: ...` rules should exist in `karabiner.json`.

## Quick checks

1. **Profile selected**
   - Run `kar` and confirm it selects `kar` in `karabiner.json`.
   - Check:
     ```
     jq '.profiles[] | {name, selected}' ~/.config/karabiner/karabiner.json
     ```

2. **No hold simlayers**
   - Ensure there are no `simlayer hold` rules:
     ```
     rg -n "simlayer hold" ~/.config/karabiner/karabiner.json
     ```
   - Output should be empty.

3. **Simlayers are chords**
   - Example: `s + l` should be a simultaneous mapping in `skey (essential)`.
   - Verify:
     ```
     jq '.profiles[] | select(.name=="kar") | .complex_modifications.rules[]
         | select(.description=="skey (essential)")
         | .manipulators[] | select(.from.simultaneous!=null)
         | .from.simultaneous' ~/.config/karabiner/karabiner.json
     ```

## Tuning

If chord timing feels too tight, raise the per-layer threshold:

```ts
// ~/.config/kar/config.ts
"s-mode": { key: "s", threshold: 700 }
```

Then run:
```
kar
```

## Known bad pattern (avoid)

- Any implementation that sets a simlayer variable on key down (hold-based),
  **when that key is also part of normal typing**, will cause stuck or eaten keys.

## Recovery

If a bad config locks input:

1. Switch back to `default` profile in Karabiner-Elements UI.
2. Restore from backup if needed:
   ```
   cp ~/.config/karabiner/karabiner.json.bak.* ~/.config/karabiner/karabiner.json
   ```
3. Run `kar` again after fixing config.
