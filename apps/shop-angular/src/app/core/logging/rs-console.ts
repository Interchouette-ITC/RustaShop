/**
 * Styled browser console lines for `[rs:*]` namespaces.
 * Fluorescent tag per namespace; wall-clock + nav-relative ms on every line.
 */

/** Console namespaces that write `%c[rs:…]` lines. */
export type RsConsoleNamespace =
  | 'rs:boot'
  | 'rs:api'
  | 'rs:catalog'
  | 'rs:cart'
  | 'rs:checkout'
  | 'rs:error';

/** Tag colors - one hue per namespace (rust / shop palette). */
const NS_TAG_COLOR: Record<RsConsoleNamespace, string> = {
  'rs:boot': '#00ff9c',
  'rs:api': '#00e5ff',
  'rs:catalog': '#ffea00',
  'rs:cart': '#ff9100',
  'rs:checkout': '#b388ff',
  'rs:error': '#ff1744',
};

const STYLE_LABEL = 'color:#fff7ed;font-weight:600';
const STYLE_CLOCK = 'color:#fdba74;font-weight:600';
const STYLE_MS = 'color:#fbbf24;font-weight:700';
const STYLE_KV = 'color:inherit;font-weight:normal;background:transparent';

/** `%c` CSS for an `[rs:…]` tag pill. */
export function rsLogTagStyle(ns: RsConsoleNamespace): string {
  const fg = NS_TAG_COLOR[ns];
  return `color:${fg};font-weight:700;background:#1c1917;padding:0.1em 0.35em;border-radius:0.2em`;
}

function formatKvValue(value: unknown): string {
  if (value === null || value === undefined) {
    return String(value);
  }
  if (typeof value === 'string' || typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  return JSON.stringify(value);
}

function formatKv(kv?: Record<string, unknown>): string {
  if (!kv || Object.keys(kv).length === 0) {
    return '';
  }
  return Object.entries(kv)
    .map(([k, v]) => `${k}=${formatKvValue(v)}`)
    .join(' ');
}

function nowMs(): number {
  if (typeof performance === 'undefined') {
    return 0;
  }
  return performance.now();
}

function pad2(n: number, width = 2): string {
  return String(Math.trunc(n)).padStart(width, '0');
}

/** Local wall-clock stamp (`YYYY-MM-DD HH:mm:ss.SSS`). */
export function rsClockStamp(date: Date = new Date()): string {
  return (
    `${date.getFullYear()}-${pad2(date.getMonth() + 1)}-${pad2(date.getDate())} ` +
    `${pad2(date.getHours())}:${pad2(date.getMinutes())}:${pad2(date.getSeconds())}.` +
    `${pad2(date.getMilliseconds(), 3)}`
  );
}

export interface RsConsoleWriteOptions {
  ns: RsConsoleNamespace;
  topic: string;
  ms?: number;
  kv?: Record<string, unknown>;
  level?: 'log' | 'warn' | 'info' | 'error';
}

/**
 * Writes one styled `[rs:ns] topic stamp N.Nms key=value…` line.
 */
export function rsConsoleWrite(options: RsConsoleWriteOptions): void {
  const { ns, topic, kv, level = 'log' } = options;
  const ms = options.ms ?? nowMs();
  const clock = rsClockStamp();
  const kvStr = formatKv(kv);
  const suffix = kvStr.length > 0 ? ` ${kvStr}` : '';
  console[level](
    `%c[${ns}]%c ${topic} %c${clock}%c %c${ms.toFixed(1)}ms%c${suffix}`,
    rsLogTagStyle(ns),
    STYLE_LABEL,
    STYLE_CLOCK,
    STYLE_KV,
    STYLE_MS,
    STYLE_KV,
  );
}
