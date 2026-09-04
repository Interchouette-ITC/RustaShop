import { HttpInterceptorFn } from '@angular/common/http';
import { finalize } from 'rxjs';

import { rsConsoleWrite } from '../logging/rs-console';

/** Logs Commerce API HTTP calls with duration (browser console, colored). */
export const rsApiLogInterceptor: HttpInterceptorFn = (req, next) => {
  const started = performance.now();
  const path = req.url.replace(/^https?:\/\/[^/]+/, '');
  return next(req).pipe(
    finalize(() => {
      rsConsoleWrite({
        ns: 'rs:api',
        topic: req.method,
        ms: performance.now() - started,
        kv: { path },
      });
    }),
  );
};
