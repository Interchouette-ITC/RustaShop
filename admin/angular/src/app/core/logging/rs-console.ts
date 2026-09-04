/**
 * Styled browser console lines for `[rs:*]` namespaces.
 * Level colors: blue = log/info, orange = warn, red = error (incidents only).
 */

/** Console namespaces that write `%c[rs:…]` lines. */
export type RsConsoleNamespace =
  | 'rs:boot'
  | 'rs:api'
  | 'rs:catalog'
  | 'rs:cart'
  | 'rs:checkout'
  | 'rs:error';

export type RsConsoleLevel = 'log' | 'warn' | 'info' | 'error';

/** Tag accent when level does not override (error always red). */
const NS_TAG_COLOR: Record<RsConsoleNamespace, string> = {
  'rs:boot': '#3b82f6',
  'rs:api': '#60a5fa',
  'rs:catalog': '#3b82f6',
  'rs:cart': '#3b82f6',
  'rs:checkout': '#3b82f6',
  'rs:error': '#ef4444',
};

const LEVEL_FG: Record<RsConsoleLevel, string> = {
  log: '#60a5fa',
  info: '#60a5fa',
  warn: '#fb923c',
  error: '#f87171',
};

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

function nowMs(): number {
  if (typeof performance === 'undefined') {
    return 0;
  }
  return performance.now();
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

/** `%c` CSS for an `[rs:…]` tag pill. */
export function rsLogTagStyle(ns: RsConsoleNamespace, level: RsConsoleLevel = 'log'): string {
  const fg = level === 'error' || ns === 'rs:error' ? LEVEL_FG.error : NS_TAG_COLOR[ns];
  return `color:${fg};font-weight:700;background:#0a0a0a;padding:0.1em 0.35em;border-radius:0.2em`;
}

export interface RsConsoleWriteOptions {
  ns: RsConsoleNamespace;
  topic: string;
  ms?: number;
  kv?: Record<string, unknown>;
  level?: RsConsoleLevel;
}

/**
 * Writes one styled `[rs:ns] topic stamp N.Nms key=value…` line.
 * Use `level: 'error'` only for incidents; warnings orange; routine lines blue.
 */
export function rsConsoleWrite(options: RsConsoleWriteOptions): void {
  const { ns, topic, kv, level = 'log' } = options;
  const ms = options.ms ?? nowMs();
  const clock = rsClockStamp();
  const kvStr = formatKv(kv);
  const suffix = kvStr.length > 0 ? ` ${kvStr}` : '';
  const fg = LEVEL_FG[level];
  const styleLabel = `color:${fg};font-weight:600`;
  const styleClock = 'color:#9ca3af;font-weight:600';
  const styleMs = `color:${fg};font-weight:700`;
  const styleKv = 'color:inherit;font-weight:normal;background:transparent';
  console[level](
    `%c[${ns}]%c ${topic} %c${clock}%c %c${ms.toFixed(1)}ms%c${suffix}`,
    rsLogTagStyle(ns, level),
    styleLabel,
    styleClock,
    styleKv,
    styleMs,
    styleKv,
  );
}
