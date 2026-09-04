import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';

import { ApiClient } from './api-client';
import type { OrderDto, OrderListDto } from './models';

/** Admin order list and status PATCH. */
@Injectable({ providedIn: 'root' })
export class AdminOrdersApi {
  private readonly api = inject(ApiClient);

  list(token: string, limit = 50): Promise<OrderListDto> {
    return firstValueFrom(
      this.api.http.get<OrderListDto>(this.api.url('/v1/admin/orders'), {
        headers: { Authorization: `Bearer ${token}` },
        params: { limit: String(limit) },
      }),
    );
  }

  patchStatus(token: string, orderId: string, status: string): Promise<OrderDto> {
    return firstValueFrom(
      this.api.http.patch<OrderDto>(this.api.url(`/v1/admin/orders/${orderId}`), { status }, {
        headers: { Authorization: `Bearer ${token}` },
      }),
    );
  }
}
