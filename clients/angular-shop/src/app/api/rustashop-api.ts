import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

import { environment } from '../../environments/environment';
import type { components } from './schema';

/** OpenAPI `HealthResponse`. */
export type HealthResponse = components['schemas']['HealthResponse'];

/** OpenAPI `ProductListResponse`. */
export type ProductListResponse = components['schemas']['ProductListResponse'];

/** OpenAPI `ProductDetailResponse`. */
export type ProductDetailResponse = components['schemas']['ProductDetailResponse'];

/** OpenAPI `ProductResponse`. */
export type ProductResponse = components['schemas']['ProductResponse'];

/** OpenAPI `CartResponse`. */
export type CartResponse = components['schemas']['CartResponse'];

/** OpenAPI `CreateCartRequest`. */
export type CreateCartRequest = components['schemas']['CreateCartRequest'];

/** OpenAPI `AddCartLineRequest`. */
export type AddCartLineRequest = components['schemas']['AddCartLineRequest'];

/** OpenAPI `UpdateCartLineRequest`. */
export type UpdateCartLineRequest = components['schemas']['UpdateCartLineRequest'];

/** OpenAPI `CheckoutRequest`. */
export type CheckoutRequest = components['schemas']['CheckoutRequest'];

/** OpenAPI `OrderResponse`. */
export type OrderResponse = components['schemas']['OrderResponse'];

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

  /** `GET /v1/products/{id}` */
  getProduct(id: string): Observable<ProductDetailResponse> {
    return this.http.get<ProductDetailResponse>(`${this.baseUrl}/v1/products/${id}`);
  }

  /** `POST /v1/carts` */
  createCart(body: CreateCartRequest = {}): Observable<CartResponse> {
    return this.http.post<CartResponse>(`${this.baseUrl}/v1/carts`, body);
  }

  /** `GET /v1/carts/{id}` */
  getCart(id: string): Observable<CartResponse> {
    return this.http.get<CartResponse>(`${this.baseUrl}/v1/carts/${id}`);
  }

  /** `POST /v1/carts/{id}/lines` */
  addCartLine(cartId: string, body: AddCartLineRequest): Observable<CartResponse> {
    return this.http.post<CartResponse>(`${this.baseUrl}/v1/carts/${cartId}/lines`, body);
  }

  /** `PATCH /v1/carts/{id}/lines/{lineId}` */
  updateCartLine(
    cartId: string,
    lineId: string,
    body: UpdateCartLineRequest,
  ): Observable<CartResponse> {
    return this.http.patch<CartResponse>(
      `${this.baseUrl}/v1/carts/${cartId}/lines/${lineId}`,
      body,
    );
  }

  /** `DELETE /v1/carts/{id}/lines/{lineId}` */
  deleteCartLine(cartId: string, lineId: string): Observable<CartResponse> {
    return this.http.delete<CartResponse>(`${this.baseUrl}/v1/carts/${cartId}/lines/${lineId}`);
  }

  /** `POST /v1/checkout` with optional `Idempotency-Key`. */
  checkout(body: CheckoutRequest, idempotencyKey?: string): Observable<OrderResponse> {
    let headers = new HttpHeaders();
    if (idempotencyKey) {
      headers = headers.set('Idempotency-Key', idempotencyKey);
    }
    return this.http.post<OrderResponse>(`${this.baseUrl}/v1/checkout`, body, { headers });
  }
}
