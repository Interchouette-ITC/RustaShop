import { APP_BASE_HREF } from '@angular/common';
import {
  ApplicationConfig,
  ErrorHandler,
  provideBrowserGlobalErrorListeners,
  provideZonelessChangeDetection,
} from '@angular/core';
import { provideHttpClient } from '@angular/common/http';
import { provideRouter } from '@angular/router';

import { RsErrorHandler } from './core/logging/rs-error-handler';
import { routes } from './app.routes';

/** Prefer the live `<base href>` (build flag or container rewrite). */
function baseHrefFromDocument(): string {
  const href = document.querySelector('base')?.getAttribute('href')?.trim();
  if (!href) {
    return '/';
  }
  if (href === '/') {
    return '/';
  }
  return href.endsWith('/') ? href : `${href}/`;
}

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideZonelessChangeDetection(),
    provideHttpClient(),
    provideRouter(routes),
    { provide: APP_BASE_HREF, useFactory: baseHrefFromDocument },
    { provide: ErrorHandler, useClass: RsErrorHandler },
  ],
};
