import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import { rsConsoleWrite } from './app/core/logging/rs-console';
import { environment } from './environments/environment';

rsConsoleWrite({
  ns: 'rs:boot',
  topic: 'shop-angular',
  kv: {
    production: environment.production,
    apiBaseUrl: environment.apiBaseUrl,
  },
});

bootstrapApplication(App, appConfig).catch((err: unknown) => {
  rsConsoleWrite({
    ns: 'rs:error',
    topic: 'bootstrap',
    level: 'error',
    kv: { err: err instanceof Error ? err.message : String(err) },
  });
  console.error(err);
});
