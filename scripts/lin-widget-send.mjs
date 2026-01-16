#!/usr/bin/env node
import fs from "node:fs";
import os from "node:os";
import path from "node:path";

const inboxPath =
  process.env.LIN_INTENT_INBOX_PATH ||
  path.join(
    os.homedir(),
    "Library",
    "Application Support",
    "Lin",
    "intent-inbox.jsonl",
  );

const logPath = path.join(
  os.homedir(),
  "Library",
  "Logs",
  "Lin",
  "widget.log",
);

function log(message) {
  try {
    const dir = path.dirname(logPath);
    fs.mkdirSync(dir, { recursive: true });
    const timestamp = new Date().toISOString();
    fs.appendFileSync(logPath, `[${timestamp}] ${message}\n`, "utf8");
  } catch {
    // best-effort logging only
  }
}

function ensureInboxFile() {
  const dir = path.dirname(inboxPath);
  fs.mkdirSync(dir, { recursive: true });
  if (!fs.existsSync(inboxPath)) {
    fs.writeFileSync(inboxPath, "");
  }
}

function toStringValue(value) {
  if (value === null || value === undefined) return undefined;
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  return undefined;
}

function buildEntry(payload) {
  const now = Date.now();
  const title = toStringValue(payload?.title) || "Widget";
  const message = toStringValue(payload?.message) || toStringValue(payload?.body) || toStringValue(payload?.text);
  const action = payload?.action || null;
  const actionType = toStringValue(action?.type || payload?.actionType || payload?.action);
  const actionLabel = toStringValue(action?.label || payload?.actionTitle);
  const actionValue = toStringValue(action?.value || payload?.value);
  const ttlMsRaw = process.env.LIN_WIDGET_TTL_MS;
  const ttlMs = ttlMsRaw ? Number(ttlMsRaw) : 0;

  return {
    id: payload?.id || `widget-${now}`,
    kind: payload?.kind || "widget",
    title,
    message,
    createdAt: payload?.createdAt || now,
    expiresAt:
      payload?.expiresAt ||
      (ttlMs > 0 ? now + ttlMs : now + 60 * 1000),
    action: actionType,
    actionTitle: actionLabel,
    value: actionValue,
  };
}

function writeEntry(entry) {
  ensureInboxFile();
  fs.appendFileSync(inboxPath, `${JSON.stringify(entry)}\n`, "utf8");
  log(`write entry kind=${entry.kind} id=${entry.id} title=${entry.title}`);
}

async function readStdin() {
  return await new Promise((resolve) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (chunk) => {
      data += chunk;
    });
    process.stdin.on("end", () => resolve(data));
  });
}

async function main() {
  const raw = (await readStdin()).trim();
  if (!raw) {
    log("no input provided");
    console.error("[lin-widget] no input provided");
    process.exit(1);
  }

  log(`received payload bytes=${raw.length}`);
  const lines = raw.split(/\r?\n/).filter(Boolean);
  log(`parsed lines count=${lines.length}`);
  for (const line of lines) {
    let payload = null;
    try {
      payload = JSON.parse(line);
    } catch {
      log(`line not json, using raw message: ${line.slice(0, 120)}`);
      payload = { title: "Widget", message: line };
    }
    const entry = buildEntry(payload);
    writeEntry(entry);
  }
}

main().catch((err) => {
  log(`fatal error: ${err?.message || err}`);
  console.error(`[lin-widget] fatal: ${err?.message || err}`);
  process.exit(1);
});
