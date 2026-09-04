import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';

import { ApiClient } from './api-client';
import type {
  AddCartLineRequest,
  CartResponse,
  CreateCartRequest,
  UpdateCartLineRequest,
} from './models';

/** Cart CRUD routes. */
@Injectable({ providedIn: 'root' })
export class CartApi {
  private readonly client = inject(ApiClient);

  createCart(body: CreateCartRequest = {}): Observable<CartResponse> {
    return this.client.http.post<CartResponse>(this.client.url('/v1/carts'), body);
  }

  getCart(id: string): Observable<CartResponse> {
    return this.client.http.get<CartResponse>(this.client.url(`/v1/carts/${id}`));
  }

  addLine(cartId: string, body: AddCartLineRequest): Observable<CartResponse> {
    return this.client.http.post<CartResponse>(this.client.url(`/v1/carts/${cartId}/lines`), body);
  }

  updateLine(
    cartId: string,
    lineId: string,
    body: UpdateCartLineRequest,
  ): Observable<CartResponse> {
    return this.client.http.patch<CartResponse>(
      this.client.url(`/v1/carts/${cartId}/lines/${lineId}`),
      body,
    );
  }

  deleteLine(cartId: string, lineId: string): Observable<CartResponse> {
    return this.client.http.delete<CartResponse>(
      this.client.url(`/v1/carts/${cartId}/lines/${lineId}`),
    );
  }
}
