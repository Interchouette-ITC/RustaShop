import { APP_BASE_HREF } from '@angular/common';
import {
  ApplicationConfig,
  provideBrowserGlobalErrorListeners,
  provideZonelessChangeDetection,
} from '@angular/core';
import { provideHttpClient } from '@angular/common/http';
import { provideRouter } from '@angular/router';

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
  ],
};
