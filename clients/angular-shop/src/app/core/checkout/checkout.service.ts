import { Injectable, inject, signal } from '@angular/core';
import { firstValueFrom } from 'rxjs';

import { OrderResponse, RustashopApi } from '../../api';
import { CartStore } from '../cart/cart.store';
import { formatApiError } from '../http/api-error';

/**
 * Places an order from the current cart and clears the local cart session.
 */
@Injectable({ providedIn: 'root' })
export class CheckoutService {
  private readonly api = inject(RustashopApi);
  private readonly cartStore = inject(CartStore);

  private readonly busySignal = signal(false);
  private readonly errorSignal = signal<string | null>(null);
  private readonly lastOrderSignal = signal<OrderResponse | null>(null);

  readonly busy = this.busySignal.asReadonly();
  readonly error = this.errorSignal.asReadonly();
  readonly lastOrder = this.lastOrderSignal.asReadonly();

  /** Checkout the open cart; clears local cart id on success. */
  async placeOrder(): Promise<OrderResponse> {
    const cart = this.cartStore.cart() ?? (await this.cartStore.ensureCart());
    if (cart.lines.length === 0) {
      const empty = new Error('Cart is empty.');
      this.errorSignal.set(empty.message);
      throw empty;
    }

    this.busySignal.set(true);
    this.errorSignal.set(null);
    try {
      const order = await firstValueFrom(
        this.api.checkout({ cart_id: cart.id }, crypto.randomUUID()),
      );
      this.lastOrderSignal.set(order);
      this.cartStore.clearSession();
      return order;
    } catch (err) {
      this.errorSignal.set(formatApiError(err, 'Checkout failed.'));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }

  clearError(): void {
    this.errorSignal.set(null);
  }
}
