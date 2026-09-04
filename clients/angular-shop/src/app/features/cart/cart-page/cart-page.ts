import { Component, OnInit, inject, signal } from '@angular/core';
import { Router, RouterLink } from '@angular/router';
import { FormsModule } from '@angular/forms';
import { firstValueFrom } from 'rxjs';

import { CartStore } from '../../../core/cart/cart.store';
import { RustashopApi } from '../../../api';
import { MoneyPipe } from '../../../shared/pipes/money.pipe';

@Component({
  selector: 'rs-cart-page',
  imports: [RouterLink, FormsModule, MoneyPipe],
  templateUrl: './cart-page.html',
  styleUrl: './cart-page.scss',
})
export class CartPage implements OnInit {
  private readonly cartStore = inject(CartStore);
  private readonly api = inject(RustashopApi);
  private readonly router = inject(Router);

  protected readonly cart = this.cartStore.cart;
  protected readonly busy = this.cartStore.busy;
  protected readonly error = signal<string | null>(null);
  protected readonly checkingOut = signal(false);

  ngOnInit(): void {
    void this.cartStore.ensureCart().catch((err: unknown) => {
      this.error.set(messageOf(err));
    });
  }

  protected async setQuantity(lineId: string, quantity: number): Promise<void> {
    const qty = Math.max(1, Math.floor(quantity));
    this.error.set(null);
    try {
      await this.cartStore.updateLine(lineId, qty);
    } catch (err) {
      this.error.set(messageOf(err));
    }
  }

  protected async remove(lineId: string): Promise<void> {
    this.error.set(null);
    try {
      await this.cartStore.removeLine(lineId);
    } catch (err) {
      this.error.set(messageOf(err));
    }
  }

  protected async checkout(): Promise<void> {
    const cart = this.cart();
    if (!cart || cart.lines.length === 0) {
      return;
    }
    this.checkingOut.set(true);
    this.error.set(null);
    try {
      const key = crypto.randomUUID();
      const order = await firstValueFrom(this.api.checkout({ cart_id: cart.id }, key));
      this.cartStore.clearSession();
      await this.router.navigate(['/checkout', order.id], {
        state: { order },
      });
    } catch (err) {
      this.error.set(messageOf(err));
    } finally {
      this.checkingOut.set(false);
    }
  }
}

function messageOf(err: unknown): string {
  if (err && typeof err === 'object' && 'message' in err) {
    return String((err as { message: unknown }).message);
  }
  return 'Cart action failed.';
}
