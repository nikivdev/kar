#!/usr/bin/env python3
"""Generate a concrete fix plan for failing Kar signal-gate metrics.

Scans config.ts files and reports the highest-impact mappings (especially those
using send_user_command) that are missing stable ids and signal metadata.
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
    proc = subprocess.run([
        *runtime.cmd_prefix,
        wrapper,
    ], text=True, capture_output=True, timeout=45)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or f"runtime_failed:{runtime.name}")
    payload = json.loads(proc.stdout)
    if not isinstance(payload, dict):
        raise RuntimeError("config_json_not_object")
    return payload


def discover_paths(paths: list[str], roots: list[str]) -> list[Path]:
    out: list[Path] = []
    for raw in paths:
        p = Path(raw).expanduser().resolve()
        if p.is_file():
            out.append(p)
    for raw_root in roots:
        root = Path(raw_root).expanduser().resolve()
        if not root.exists():
            continue
        for p in root.rglob("config.ts"):
            if p.is_file():
                out.append(p.resolve())
    uniq: dict[str, Path] = {}
    for p in out:
        uniq[str(p)] = p
    return [uniq[k] for k in sorted(uniq)]


def has_send_user_command(action: Any) -> bool:
    if isinstance(action, list):
        return any(has_send_user_command(item) for item in action)
    if isinstance(action, dict):
        if "send_user_command" in action:
            return True
        return any(has_send_user_command(v) for v in action.values())
    return False


def mapping_from_label(from_value: Any) -> str:
    if isinstance(from_value, str):
        return from_value
    if isinstance(from_value, list):
        return "+".join(str(x) for x in from_value)
    if isinstance(from_value, dict):
        key = from_value.get("key") or from_value.get("key_code")
        if isinstance(key, str):
            return key
    return "<unknown>"


def analyze_config(path: Path, config: dict[str, Any]) -> dict[str, Any]:
    rules = config.get("rules") if isinstance(config.get("rules"), list) else []
    findings: list[dict[str, Any]] = []
    total_send = 0

    for rule_idx, rule in enumerate(rules):
        if not isinstance(rule, dict):
            continue
        rule_id = rule.get("id") if isinstance(rule.get("id"), str) else ""
        rule_desc = str(rule.get("description", ""))
        mappings = rule.get("mappings") if isinstance(rule.get("mappings"), list) else []
        for mapping_idx, mapping in enumerate(mappings):
            if not isinstance(mapping, dict):
                continue
            uses_send = (
                has_send_user_command(mapping.get("to"))
                or has_send_user_command(mapping.get("to_if_alone"))
                or has_send_user_command(mapping.get("to_if_held"))
            )
            if not uses_send:
                continue
            total_send += 1
            mapping_id = mapping.get("id") if isinstance(mapping.get("id"), str) else ""
            signal = mapping.get("signal") if isinstance(mapping.get("signal"), dict) else None
            signal_intent = signal.get("intent") if isinstance(signal, dict) else None
            missing = []
            if not rule_id:
                missing.append("rule.id")
            if not mapping_id:
                missing.append("mapping.id")
            if signal is None:
                missing.append("mapping.signal")
            elif not signal_intent:
                missing.append("mapping.signal.intent")

            if missing:
                findings.append(
                    {
                        "path": str(path),
                        "rule_index": rule_idx,
                        "mapping_index": mapping_idx,
                        "rule_description": rule_desc,
                        "from": mapping_from_label(mapping.get("from")),
                        "missing": missing,
                    }
                )

    return {
        "path": str(path),
        "send_user_command_mappings": total_send,
        "findings": findings,
    }


def print_plan(results: list[dict[str, Any]], limit: int) -> int:
    all_findings: list[dict[str, Any]] = []
    total_send = 0
    for r in results:
        total_send += int(r.get("send_user_command_mappings", 0))
        all_findings.extend(r.get("findings", []))

    print(f"send_user_command mappings scanned: {total_send}")
    print(f"mappings needing metadata: {len(all_findings)}")

    if not all_findings:
        print("No fixes needed. Gate metadata coverage should be healthy.")
        return 0

    print("\nTop fixes (highest impact first):")
    for idx, item in enumerate(all_findings[:limit], start=1):
        missing = ", ".join(item.get("missing", []))
        print(
            f"{idx}. {item.get('path')} rule#{item.get('rule_index')} mapping#{item.get('mapping_index')} "
            f"from={item.get('from')} missing=[{missing}]"
        )

    print("\nTemplate to apply on each mapping:")
    print('{')
    print('  id: "map.<domain>.<action>",')
    print('  signal: { intent: "<intent>", tags: ["<tag1>", "<tag2>"], criticality: "low" },')
    print('  from: "...",')
    print('  to: { send_user_command: { ... } },')
    print('}')

    return 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Generate fix plan for Kar signal metadata coverage")
    parser.add_argument("paths", nargs="*", help="Explicit config.ts paths")
    parser.add_argument("--roots", nargs="*", default=[], help="Roots to scan for config.ts")
    parser.add_argument("--limit", type=int, default=60, help="Max findings to print")
    parser.add_argument(
        "--fail-on-findings",
        action="store_true",
        help="Exit non-zero when missing metadata findings are present.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    paths = discover_paths(args.paths, args.roots)
    if not paths:
        print("No config.ts files found", file=sys.stderr)
        return 2

    runtime = detect_runtime()
    results: list[dict[str, Any]] = []
    for path in paths:
        try:
            config = load_config(runtime, path)
        except Exception as exc:
            print(f"WARN: failed to load {path}: {exc}", file=sys.stderr)
            continue
        results.append(analyze_config(path, config))

    if not results:
        print("No config files could be analyzed", file=sys.stderr)
        return 2

    rc = print_plan(results, limit=max(1, args.limit))
    if rc != 0 and not args.fail_on_findings:
        return 0
    return rc


if __name__ == "__main__":
    raise SystemExit(main())
