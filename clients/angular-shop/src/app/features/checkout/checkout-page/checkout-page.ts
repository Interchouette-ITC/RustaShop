import { Component, inject, signal } from '@angular/core';
import { Router, RouterLink } from '@angular/router';

import type { OrderResponse } from '@rustashop/shop-api';
import { MoneyPipe } from '@rustashop/shop-shared';

@Component({
  selector: 'rs-checkout-page',
  imports: [RouterLink, MoneyPipe],
  templateUrl: './checkout-page.html',
  styleUrl: './checkout-page.scss',
})
export class CheckoutPage {
  private readonly router = inject(Router);

  protected readonly order = signal<OrderResponse | null>(readOrder(this.router));
  protected readonly missing = signal(this.order() == null);
}

function readOrder(router: Router): OrderResponse | null {
  const fromNav = router.getCurrentNavigation()?.extras.state?.['order'] as
    OrderResponse | undefined;
  if (fromNav) {
    return fromNav;
  }
  const fromHistory = history.state?.['order'] as OrderResponse | undefined;
  return fromHistory ?? null;
}
