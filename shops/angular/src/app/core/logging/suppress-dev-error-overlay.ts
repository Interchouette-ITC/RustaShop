import { rsConsoleWrite } from './rs-console';

const OVERLAY_TAG = 'VITE-ERROR-OVERLAY';

/**
 * Dev-only: strip Vite/Angular full-page error overlays and pipe text to console.
 * Build/HMR errors stay visible in the terminal; the SPA viewport stays usable.
 */
export function installDevErrorOverlayToConsole(): void {
  if (typeof document === 'undefined') {
    return;
  }

  const swallow = (el: Element): void => {
    const text = (el.textContent ?? '').replace(/\s+/g, ' ').trim() || 'dev build error';
    rsConsoleWrite({
      ns: 'rs:error',
      topic: 'dev-overlay',
      level: 'error',
      kv: { message: text.slice(0, 2000) },
    });
    el.remove();
  };

  const scan = (root: ParentNode): void => {
    if (root instanceof Element && root.tagName === OVERLAY_TAG) {
      swallow(root);
      return;
    }
    root.querySelectorAll?.('vite-error-overlay').forEach(swallow);
  };

  scan(document.documentElement);

  const observer = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      for (const node of mutation.addedNodes) {
        if (node instanceof Element || node instanceof DocumentFragment) {
          scan(node);
        }
      }
    }
  });
  observer.observe(document.documentElement, { childList: true, subtree: true });
}
