#!/usr/bin/env python3
"""Scan Kar TypeScript configs for keyboard-signal telemetry readiness.

This script executes config.ts files (same model as kar runtime) and summarizes:
- mapping/action shape
- metadata coverage
- observability friendliness (send_user_command vs shell)
"""

from __future__ import annotations

import argparse
import json
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass
class Runtime:
    name: str
    cmd_prefix: list[str]


def detect_runtime() -> Runtime:
    if shutil.which("deno"):
        return Runtime(name="deno", cmd_prefix=["deno", "eval", "--ext=ts"])
    if shutil.which("bun"):
        return Runtime(name="bun", cmd_prefix=["bun", "eval"])
    raise RuntimeError("No TS runtime found (deno or bun)")


def load_config(runtime: Runtime, path: Path) -> dict[str, Any]:
    wrapper = f'import config from "file://{path}"; console.log(JSON.stringify(config));'
    proc = subprocess.run(
        [*runtime.cmd_prefix, wrapper],
        text=True,
        capture_output=True,
        timeout=45,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or f"runtime_failed:{runtime.name}")
    try:
        payload = json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"invalid_json_output:{exc}") from exc
    if not isinstance(payload, dict):
        raise RuntimeError("config_json_not_object")
    return payload


def _count_to_action(action: Any, counts: dict[str, int]) -> None:
    if isinstance(action, list):
        counts["to_array"] += 1
        for item in action:
            _count_to_action(item, counts)
        return
    if isinstance(action, str):
        counts["to_key_simple"] += 1
        return
    if not isinstance(action, dict):
        counts["to_unknown"] += 1
        return

    if "send_user_command" in action:
        counts["to_send_user_command"] += 1
    elif "socket_command" in action:
        counts["to_socket_command"] += 1
    elif "shell" in action:
        counts["to_shell"] += 1
    elif "mouse_key" in action:
        counts["to_mouse_key"] += 1
    elif "pointing_button" in action:
        counts["to_pointing_button"] += 1
    elif "key" in action:
        counts["to_key_with_modifiers"] += 1
    else:
        counts["to_unknown"] += 1


def analyze_config(payload: dict[str, Any]) -> dict[str, Any]:
    rules = payload.get("rules") if isinstance(payload.get("rules"), list) else []
    simlayers = payload.get("simlayers") if isinstance(payload.get("simlayers"), dict) else {}
    simple_mods = payload.get("simple") if isinstance(payload.get("simple"), list) else []

    counts: dict[str, int] = {
        "rules": len(rules),
        "simlayers": len(simlayers),
        "simple_modifications": len(simple_mods),
        "mappings": 0,
        "rules_with_id": 0,
        "mappings_with_note": 0,
        "mappings_with_id": 0,
        "mappings_with_signal": 0,
        "mappings_with_signal_intent": 0,
        "mappings_with_signal_tags": 0,
        "mappings_with_signal_criticality": 0,
        "rules_with_note": 0,
        "rules_with_layer": 0,
        "rules_with_condition": 0,
        "from_simple": 0,
        "from_with_modifiers": 0,
        "from_simultaneous": 0,
        "to_send_user_command": 0,
        "to_socket_command": 0,
        "to_shell": 0,
        "to_key_simple": 0,
        "to_key_with_modifiers": 0,
        "to_mouse_key": 0,
        "to_pointing_button": 0,
        "to_array": 0,
        "to_unknown": 0,
    }

    for rule in rules:
        if not isinstance(rule, dict):
            continue
        if rule.get("id"):
            counts["rules_with_id"] += 1
        if rule.get("note"):
            counts["rules_with_note"] += 1
        if rule.get("layer"):
            counts["rules_with_layer"] += 1
        if rule.get("condition"):
            counts["rules_with_condition"] += 1

        mappings = rule.get("mappings") if isinstance(rule.get("mappings"), list) else []
        for mapping in mappings:
            if not isinstance(mapping, dict):
                continue
            counts["mappings"] += 1
            if mapping.get("id"):
                counts["mappings_with_id"] += 1
            if mapping.get("note"):
                counts["mappings_with_note"] += 1
            signal = mapping.get("signal")
            if isinstance(signal, dict):
                counts["mappings_with_signal"] += 1
                if signal.get("intent"):
                    counts["mappings_with_signal_intent"] += 1
                tags = signal.get("tags")
                if isinstance(tags, list) and len(tags) > 0:
                    counts["mappings_with_signal_tags"] += 1
                if signal.get("criticality"):
                    counts["mappings_with_signal_criticality"] += 1

            frm = mapping.get("from")
            if isinstance(frm, str):
                counts["from_simple"] += 1
            elif isinstance(frm, list):
                counts["from_simultaneous"] += 1
            elif isinstance(frm, dict):
                counts["from_with_modifiers"] += 1

            _count_to_action(mapping.get("to"), counts)
            if "to_if_alone" in mapping:
                _count_to_action(mapping.get("to_if_alone"), counts)
            if "to_if_held" in mapping:
                _count_to_action(mapping.get("to_if_held"), counts)

    mappings = max(1, counts["mappings"])
    ratios = {
        "rule_id_coverage": round(counts["rules_with_id"] / max(1, counts["rules"]), 4),
        "mapping_id_coverage": round(counts["mappings_with_id"] / mappings, 4),
        "mapping_signal_coverage": round(counts["mappings_with_signal"] / mappings, 4),
        "mapping_signal_intent_coverage": round(counts["mappings_with_signal_intent"] / mappings, 4),
        "mapping_note_coverage": round(counts["mappings_with_note"] / mappings, 4),
        "observable_action_share": round(
            (counts["to_send_user_command"] + counts["to_socket_command"])
            / max(
                1,
                counts["to_send_user_command"]
                + counts["to_socket_command"]
                + counts["to_shell"]
                + counts["to_key_simple"]
                + counts["to_key_with_modifiers"]
                + counts["to_mouse_key"]
                + counts["to_pointing_button"]
            ),
            4,
        ),
        "shell_action_share": round(
            counts["to_shell"]
            / max(
                1,
                counts["to_send_user_command"]
                + counts["to_socket_command"]
                + counts["to_shell"]
                + counts["to_key_simple"]
                + counts["to_key_with_modifiers"]
                + counts["to_mouse_key"]
                + counts["to_pointing_button"]
            ),
            4,
        ),
    }

    return {"counts": counts, "ratios": ratios}


def summarize_many(results: list[dict[str, Any]]) -> dict[str, Any]:
    totals: dict[str, int] = {}
    for entry in results:
        for key, value in entry["analysis"]["counts"].items():
            totals[key] = totals.get(key, 0) + int(value)

    mappings = max(1, totals.get("mappings", 0))
    action_total = max(
        1,
        totals.get("to_send_user_command", 0)
        + totals.get("to_socket_command", 0)
        + totals.get("to_shell", 0)
        + totals.get("to_key_simple", 0)
        + totals.get("to_key_with_modifiers", 0)
        + totals.get("to_mouse_key", 0)
        + totals.get("to_pointing_button", 0),
    )
    ratios = {
        "rule_id_coverage": round(totals.get("rules_with_id", 0) / max(1, totals.get("rules", 0)), 4),
        "mapping_id_coverage": round(totals.get("mappings_with_id", 0) / mappings, 4),
        "mapping_signal_coverage": round(totals.get("mappings_with_signal", 0) / mappings, 4),
        "mapping_signal_intent_coverage": round(
            totals.get("mappings_with_signal_intent", 0) / mappings,
            4,
        ),
        "mapping_note_coverage": round(totals.get("mappings_with_note", 0) / mappings, 4),
        "observable_action_share": round(
            (totals.get("to_send_user_command", 0) + totals.get("to_socket_command", 0)) / action_total,
            4,
        ),
        "shell_action_share": round(totals.get("to_shell", 0) / action_total, 4),
    }
    return {"counts": totals, "ratios": ratios}


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Scan Kar config TS files for keyboard telemetry readiness")
    p.add_argument("paths", nargs="*", help="Explicit config.ts files")
    p.add_argument(
        "--roots",
        nargs="*",
        default=[],
        help="Root directories to search for config.ts files",
    )
    p.add_argument("--output", default="", help="Optional output path for JSON report")
    return p.parse_args()


def discover_paths(args: argparse.Namespace) -> list[Path]:
    out: list[Path] = []
    for raw in args.paths:
        p = Path(raw).expanduser().resolve()
        if p.is_file():
            out.append(p)

    for raw_root in args.roots:
        root = Path(raw_root).expanduser().resolve()
        if not root.exists():
            continue
        for p in root.rglob("config.ts"):
            if p.is_file():
                out.append(p.resolve())

    # deterministic unique order
    uniq: dict[str, Path] = {}
    for p in out:
        uniq[str(p)] = p
    return [uniq[k] for k in sorted(uniq)]


def main() -> int:
    args = parse_args()
    paths = discover_paths(args)
    if not paths:
        print("No config.ts files found.", file=sys.stderr)
        return 1

    runtime = detect_runtime()
    results: list[dict[str, Any]] = []

    for path in paths:
        item: dict[str, Any] = {"path": str(path)}
        try:
            payload = load_config(runtime, path)
            item["ok"] = True
            item["analysis"] = analyze_config(payload)
        except Exception as exc:
            item["ok"] = False
            item["error"] = str(exc)
        results.append(item)

    ok_results = [r for r in results if r.get("ok")]
    summary = summarize_many(ok_results) if ok_results else {"counts": {}, "ratios": {}}

    report = {
        "schema_version": "kar_signal_scan_v1",
        "runtime": runtime.name,
        "files": results,
        "summary": summary,
    }

    blob = json.dumps(report, ensure_ascii=True, indent=2)
    if args.output:
        out_path = Path(args.output).expanduser().resolve()
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(blob + "\n", encoding="utf-8")
        print(f"Wrote: {out_path}")
    else:
        print(blob)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
