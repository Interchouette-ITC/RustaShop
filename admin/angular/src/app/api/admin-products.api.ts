import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';

import { ApiClient } from './api-client';
import type { ProductListDto } from './models';

/** Admin product list. */
@Injectable({ providedIn: 'root' })
export class AdminProductsApi {
  private readonly api = inject(ApiClient);

  list(token: string, limit = 100): Promise<ProductListDto> {
    return firstValueFrom(
      this.api.http.get<ProductListDto>(this.api.url('/v1/admin/products'), {
        headers: { Authorization: `Bearer ${token}` },
        params: { limit: String(limit) },
      }),
    );
  }
}
