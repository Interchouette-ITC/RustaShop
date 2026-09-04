import { Component, OnInit, inject, signal } from '@angular/core';
import { Router, RouterLink } from '@angular/router';
import { FormsModule } from '@angular/forms';

import { CartStore, CheckoutService, formatApiError } from '@rustashop/shop-core';
import { MoneyPipe } from '@rustashop/shop-shared';
import { template as cartPageTpl, styles as cartPageStyles } from '@generated/cart_page.ng';

@Component({
  selector: 'rs-cart-page',
  imports: [RouterLink, FormsModule, MoneyPipe],
  template: cartPageTpl,
  styles: cartPageStyles,
})
export class CartPage implements OnInit {
  private readonly cartStore = inject(CartStore);
  private readonly checkout = inject(CheckoutService);
  private readonly router = inject(Router);

  protected readonly cart = this.cartStore.cart;
  protected readonly busy = this.cartStore.busy;
  protected readonly checkingOut = this.checkout.busy;
  protected readonly error = signal<string | null>(null);

  ngOnInit(): void {
    void this.cartStore.ensureCart().catch((err: unknown) => {
      this.error.set(formatApiError(err, 'Cart action failed.'));
    });
  }

  protected async setQuantity(lineId: string, quantity: number): Promise<void> {
    const qty = Math.max(1, Math.floor(quantity));
    this.error.set(null);
    try {
      await this.cartStore.updateLine(lineId, qty);
    } catch (err) {
      this.error.set(formatApiError(err, 'Cart action failed.'));
    }
  }

  protected async remove(lineId: string): Promise<void> {
    this.error.set(null);
    try {
      await this.cartStore.removeLine(lineId);
    } catch (err) {
      this.error.set(formatApiError(err, 'Cart action failed.'));
    }
  }

  protected async placeOrder(): Promise<void> {
    this.error.set(null);
    try {
      const order = await this.checkout.placeOrder();
      await this.router.navigate(['/checkout', order.id], {
        state: { order },
      });
    } catch (err) {
      this.error.set(this.checkout.error() ?? formatApiError(err, 'Checkout failed.'));
    }
  }
}
