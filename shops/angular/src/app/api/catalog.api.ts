import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';

import { ApiClient } from './api-client';
import type { ProductDetailResponse, ProductListResponse } from './models';

/** Catalog product routes. */
@Injectable({ providedIn: 'root' })
export class CatalogApi {
  private readonly client = inject(ApiClient);

  listProducts(): Observable<ProductListResponse> {
    return this.client.http.get<ProductListResponse>(this.client.url('/v1/products'));
  }

  getProduct(id: string): Observable<ProductDetailResponse> {
    return this.client.http.get<ProductDetailResponse>(this.client.url(`/v1/products/${id}`));
  }
}
