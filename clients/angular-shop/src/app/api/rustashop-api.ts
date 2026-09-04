import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

import { environment } from '../../environments/environment';
import type { components } from './schema';

/** OpenAPI `HealthResponse`. */
export type HealthResponse = components['schemas']['HealthResponse'];

/** OpenAPI `ProductListResponse`. */
export type ProductListResponse = components['schemas']['ProductListResponse'];

/**
 * Thin HTTP client for the RustaShop Commerce API.
 *
 * Base URL comes from `environment.apiBaseUrl` (dev: `/api` via proxy).
 */
@Injectable({ providedIn: 'root' })
export class RustashopApi {
  private readonly http = inject(HttpClient);
  private readonly baseUrl = environment.apiBaseUrl.replace(/\/$/, '');

  /** `GET /healthz` */
  healthz(): Observable<HealthResponse> {
    return this.http.get<HealthResponse>(`${this.baseUrl}/healthz`);
  }

  /** `GET /v1/products` */
  listProducts(): Observable<ProductListResponse> {
    return this.http.get<ProductListResponse>(`${this.baseUrl}/v1/products`);
  }
}
