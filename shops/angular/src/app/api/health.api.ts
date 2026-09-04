import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';

import { ApiClient } from './api-client';
import type { HealthResponse } from './models';

/** `GET /healthz` */
@Injectable({ providedIn: 'root' })
export class HealthApi {
  private readonly client = inject(ApiClient);

  healthz(): Observable<HealthResponse> {
    return this.client.http.get<HealthResponse>(this.client.url('/healthz'));
  }
}
