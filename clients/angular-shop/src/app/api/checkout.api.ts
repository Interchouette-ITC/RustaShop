import { Injectable, inject } from '@angular/core';
import { HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

import { ApiClient } from './api-client';
import type { CheckoutRequest, OrderResponse } from './models';

/** Checkout route. */
@Injectable({ providedIn: 'root' })
export class CheckoutApi {
  private readonly client = inject(ApiClient);

  placeOrder(body: CheckoutRequest, idempotencyKey?: string): Observable<OrderResponse> {
    let headers = new HttpHeaders();
    if (idempotencyKey) {
      headers = headers.set('Idempotency-Key', idempotencyKey);
    }
    return this.client.http.post<OrderResponse>(this.client.url('/v1/checkout'), body, {
      headers,
    });
  }
}
