import { Injectable, computed, inject, signal } from '@angular/core';
import { firstValueFrom } from 'rxjs';

import { CartResponse, RustashopApi } from '../../api';
import { formatApiError } from '../http/api-error';

const CART_ID_KEY = 'rs.cartId';

/**
 * Browser cart session: persists cart id and keeps a live cart snapshot.
 */
@Injectable({ providedIn: 'root' })
export class CartStore {
  private readonly api = inject(RustashopApi);

  private readonly cartSignal = signal<CartResponse | null>(null);
  private readonly busySignal = signal(false);
  private readonly errorSignal = signal<string | null>(null);

  readonly cart = this.cartSignal.asReadonly();
  readonly busy = this.busySignal.asReadonly();
  readonly error = this.errorSignal.asReadonly();
  readonly lineCount = computed(() => {
    const cart = this.cartSignal();
    if (!cart) {
      return 0;
    }
    return cart.lines.reduce((sum, line) => sum + line.quantity, 0);
  });

  /** Loads the stored cart, or creates one when missing. */
  async ensureCart(): Promise<CartResponse> {
    const existingId = readCartId();
    if (existingId) {
      try {
        const cart = await firstValueFrom(this.api.getCart(existingId));
        if (cart.status === 'open') {
          this.cartSignal.set(cart);
          this.errorSignal.set(null);
          return cart;
        }
      } catch {
        clearCartId();
      }
    }
    return this.createCart();
  }

  /** Refreshes the current cart from the API when an id is known. */
  async refresh(): Promise<void> {
    const id = this.cartSignal()?.id ?? readCartId();
    if (!id) {
      this.cartSignal.set(null);
      return;
    }
    this.busySignal.set(true);
    try {
      const cart = await firstValueFrom(this.api.getCart(id));
      this.cartSignal.set(cart);
      writeCartId(cart.id);
      this.errorSignal.set(null);
    } catch (err) {
      this.errorSignal.set(formatError(err));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }

  async addLine(variantId: string, quantity: number): Promise<CartResponse> {
    this.busySignal.set(true);
    this.errorSignal.set(null);
    try {
      const cart = await this.ensureCart();
      const updated = await firstValueFrom(
        this.api.addCartLine(cart.id, { variant_id: variantId, quantity }),
      );
      this.cartSignal.set(updated);
      writeCartId(updated.id);
      return updated;
    } catch (err) {
      this.errorSignal.set(formatError(err));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }

  async updateLine(lineId: string, quantity: number): Promise<CartResponse> {
    const cart = this.cartSignal();
    if (!cart) {
      throw new Error('No cart');
    }
    this.busySignal.set(true);
    this.errorSignal.set(null);
    try {
      const updated = await firstValueFrom(this.api.updateCartLine(cart.id, lineId, { quantity }));
      this.cartSignal.set(updated);
      return updated;
    } catch (err) {
      this.errorSignal.set(formatError(err));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }

  async removeLine(lineId: string): Promise<CartResponse> {
    const cart = this.cartSignal();
    if (!cart) {
      throw new Error('No cart');
    }
    this.busySignal.set(true);
    this.errorSignal.set(null);
    try {
      const updated = await firstValueFrom(this.api.deleteCartLine(cart.id, lineId));
      this.cartSignal.set(updated);
      return updated;
    } catch (err) {
      this.errorSignal.set(formatError(err));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }

  /** Clears the local cart session after a successful checkout. */
  clearSession(): void {
    clearCartId();
    this.cartSignal.set(null);
    this.errorSignal.set(null);
  }

  private async createCart(): Promise<CartResponse> {
    this.busySignal.set(true);
    try {
      const cart = await firstValueFrom(this.api.createCart({ currency: 'EUR' }));
      this.cartSignal.set(cart);
      writeCartId(cart.id);
      this.errorSignal.set(null);
      return cart;
    } catch (err) {
      this.errorSignal.set(formatError(err));
      throw err;
    } finally {
      this.busySignal.set(false);
    }
  }
}

function readCartId(): string | null {
  try {
    return localStorage.getItem(CART_ID_KEY);
  } catch {
    return null;
  }
}

function writeCartId(id: string): void {
  try {
    localStorage.setItem(CART_ID_KEY, id);
  } catch {
    // Private mode / blocked storage: cart still works for the session.
  }
}

function clearCartId(): void {
  try {
    localStorage.removeItem(CART_ID_KEY);
  } catch {
    // ignore
  }
}

function formatError(err: unknown): string {
  return formatApiError(err, 'Cart request failed');
}
