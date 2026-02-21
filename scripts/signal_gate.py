#!/usr/bin/env python3
"""Fail-fast gate for Kar signal-readiness metrics."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


DEFAULT_REPORT = "~/.local/state/kar/signal-scan.json"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Enforce telemetry-readiness thresholds on signal-scan output")
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT,
        help="Path to kar_signal_scan_v1 JSON report",
    )
    parser.add_argument("--min-observable-action-share", type=float, default=0.70)
    parser.add_argument("--max-shell-action-share", type=float, default=0.02)
    parser.add_argument("--min-send-user-command-id-coverage", type=float, default=0.50)
    parser.add_argument("--min-send-user-command-signal-coverage", type=float, default=0.50)
    parser.add_argument("--min-send-user-command-intent-coverage", type=float, default=0.50)
    return parser.parse_args()


def read_report(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict):
        raise ValueError("report JSON is not an object")
    return data


def as_float(ratios: dict, key: str) -> float:
    value = ratios.get(key, 0.0)
    if isinstance(value, (int, float)):
        return float(value)
    return 0.0


def check(label: str, value: float, op: str, threshold: float) -> tuple[bool, str]:
    if op == ">=":
        ok = value >= threshold
    elif op == "<=":
        ok = value <= threshold
    else:
        raise ValueError(f"unsupported operator: {op}")
    status = "PASS" if ok else "FAIL"
    line = f"[{status}] {label}: {value:.4f} {op} {threshold:.4f}"
    return ok, line


def main() -> int:
    args = parse_args()
    report_path = Path(args.report).expanduser().resolve()
    if not report_path.exists():
        print(f"ERROR: report not found: {report_path}")
        return 2

    report = read_report(report_path)
    schema = report.get("schema_version")
    if schema != "kar_signal_scan_v1":
        print(f"ERROR: unsupported schema_version: {schema!r}")
        return 2

    summary = report.get("summary") if isinstance(report.get("summary"), dict) else {}
    ratios = summary.get("ratios") if isinstance(summary.get("ratios"), dict) else {}

    checks: list[tuple[bool, str]] = []
    checks.append(
        check(
            "observable_action_share",
            as_float(ratios, "observable_action_share"),
            ">=",
            args.min_observable_action_share,
        )
    )
    checks.append(
        check(
            "shell_action_share",
            as_float(ratios, "shell_action_share"),
            "<=",
            args.max_shell_action_share,
        )
    )
    checks.append(
        check(
            "send_user_command_mapping_id_coverage",
            as_float(ratios, "send_user_command_mapping_id_coverage"),
            ">=",
            args.min_send_user_command_id_coverage,
        )
    )
    checks.append(
        check(
            "send_user_command_mapping_signal_coverage",
            as_float(ratios, "send_user_command_mapping_signal_coverage"),
            ">=",
            args.min_send_user_command_signal_coverage,
        )
    )
    checks.append(
        check(
            "send_user_command_mapping_signal_intent_coverage",
            as_float(ratios, "send_user_command_mapping_signal_intent_coverage"),
            ">=",
            args.min_send_user_command_intent_coverage,
        )
    )

    print(f"Signal gate report: {report_path}")
    for _, line in checks:
        print(line)

    failed = [line for ok, line in checks if not ok]
    if failed:
        print(f"Gate result: FAIL ({len(failed)} check(s) failed)")
        return 1

    print("Gate result: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
