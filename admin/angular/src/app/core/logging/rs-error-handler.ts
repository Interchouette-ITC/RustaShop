import { ErrorHandler, Injectable } from '@angular/core';

import { rsConsoleWrite } from './rs-console';

/**
 * Routes uncaught Angular errors to `[rs:error]` console lines.
 * Does not replace the view with a full-screen dialog.
 */
@Injectable()
export class RsErrorHandler implements ErrorHandler {
  handleError(error: unknown): void {
    const message = error instanceof Error ? error.message : String(error);
    const stack = error instanceof Error ? error.stack : undefined;
    rsConsoleWrite({
      ns: 'rs:error',
      topic: 'uncaught',
      level: 'error',
      kv: {
        message,
        ...(stack ? { stack } : {}),
      },
    });
  }
}
