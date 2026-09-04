import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { App } from './app/app';
import { environment } from './environments/environment';

bootstrapApplication(App, appConfig).catch((err: unknown) => {
  console.error('admin bootstrap failed', {
    production: environment.production,
    apiBaseUrl: environment.apiBaseUrl,
    err: err instanceof Error ? err.message : String(err),
  });
});
