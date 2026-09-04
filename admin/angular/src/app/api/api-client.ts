import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';

import { environment } from '../../environments/environment';

/** Shared HTTP base URL for Commerce API clients. */
@Injectable({ providedIn: 'root' })
export class ApiClient {
  readonly http = inject(HttpClient);
  private readonly baseUrl = environment.apiBaseUrl.replace(/\/$/, '');
  private readonly adminPrefix = environment.adminApiPrefix.replace(/^\/+|\/+$/g, '');

  /** Absolute URL for a path starting with `/`. */
  url(path: string): string {
    return `${this.baseUrl}${path}`;
  }

  /** Operator API path under `/v1/{adminApiPrefix}/…` (must match API env). */
  adminUrl(resource: string): string {
    const path = resource.replace(/^\/+/, '');
    return this.url(`/v1/${this.adminPrefix}/${path}`);
  }
}
