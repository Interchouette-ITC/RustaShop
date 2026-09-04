import { Injectable, inject } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

import type { components } from './schema';

/** OpenAPI `HealthResponse`. */
export type HealthResponse = components['schemas']['HealthResponse'];

/** OpenAPI `ProductListResponse`. */
export type ProductListResponse = components['schemas']['ProductListResponse'];

/**
 * Thin HTTP client for the RustaShop Commerce API.
 *
 * Browser calls go to `/api/...` (dev proxy → Actix on `:8080`).
 * Override with `window.__RUSTASHOP_API_BASE__` when serving a built app.
 */
@Injectable({ providedIn: 'root' })
export class RustashopApi {
  private readonly http = inject(HttpClient);

  private baseUrl(): string {
    const fromWindow = (globalThis as { __RUSTASHOP_API_BASE__?: string }).__RUSTASHOP_API_BASE__;
    return (fromWindow ?? '/api').replace(/\/$/, '');
  }

  /** `GET /healthz` */
  healthz(): Observable<HealthResponse> {
    return this.http.get<HealthResponse>(`${this.baseUrl()}/healthz`);
  }

  /** `GET /v1/products` */
  listProducts(): Observable<ProductListResponse> {
    return this.http.get<ProductListResponse>(`${this.baseUrl()}/v1/products`);
  }
}
