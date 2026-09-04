import { Component, OnInit, inject, signal } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';

import { CartStore } from '../../core/cart/cart.store';

@Component({
  selector: 'rs-shop-shell',
  imports: [RouterOutlet, RouterLink, RouterLinkActive],
  templateUrl: './shop-shell.html',
  styleUrl: './shop-shell.scss',
})
export class ShopShell implements OnInit {
  private readonly cartStore = inject(CartStore);

  protected readonly lineCount = this.cartStore.lineCount;
  protected readonly brand = signal('RustaShop');

  ngOnInit(): void {
    void this.cartStore.ensureCart().catch(() => {
      // Empty cart session until the API is up.
    });
  }
}
