import { Component, effect, inject, signal } from '@angular/core';
import { FormsModule } from '@angular/forms';

import {
  AdminOrdersApi,
  ORDER_STATUSES,
  type MoneyDto,
  type OrderDto,
} from '@rustashop/admin-api';
import { AdminTokenStore, formatApiError } from '@rustashop/admin-core';
import { template as ordersPageTpl, styles as ordersPageStyles } from '@generated/orders_page.ng';

@Component({
  selector: 'rs-orders-page',
  imports: [FormsModule],
  template: ordersPageTpl,
  styles: ordersPageStyles,
})
export class OrdersPage {
  private readonly api = inject(AdminOrdersApi);
  private readonly tokens = inject(AdminTokenStore);

  protected readonly statuses = ORDER_STATUSES;
  protected readonly orders = signal<OrderDto[]>([]);
  protected readonly busy = signal(false);
  protected readonly error = signal<string | null>(null);
  protected readonly hasToken = this.tokens.hasToken;

  constructor() {
    effect(() => {
      if (this.tokens.hasToken()) {
        void this.reload();
      } else {
        this.orders.set([]);
        this.error.set(null);
      }
    });
  }

  protected formatMoney(money: MoneyDto): string {
    const major = (money.amount_minor / 100).toFixed(2);
    return `${major} ${money.currency}`;
  }

  protected async reload(): Promise<void> {
    const token = this.tokens.token();
    if (!token) {
      return;
    }
    this.busy.set(true);
    this.error.set(null);
    try {
      const page = await this.api.list(token);
      this.orders.set(page.items);
    } catch (err) {
      this.error.set(formatApiError(err, 'Failed to load orders.'));
      this.orders.set([]);
    } finally {
      this.busy.set(false);
    }
  }

  protected async onStatusChange(order: OrderDto, status: string): Promise<void> {
    if (status === order.state) {
      return;
    }
    const token = this.tokens.token();
    if (!token) {
      return;
    }
    this.busy.set(true);
    this.error.set(null);
    try {
      const updated = await this.api.patchStatus(token, order.id, status);
      this.orders.update((rows) => rows.map((row) => (row.id === updated.id ? updated : row)));
    } catch (err) {
      this.error.set(formatApiError(err, 'Failed to update status.'));
      await this.reload();
    } finally {
      this.busy.set(false);
    }
  }
}
