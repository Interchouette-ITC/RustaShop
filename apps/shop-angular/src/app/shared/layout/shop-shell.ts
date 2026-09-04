import { Component, OnInit, inject, signal } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';

import { CartStore } from '@rustashop/shop-core';
import { rsConsoleWrite } from '../../core/logging/rs-console';

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
    rsConsoleWrite({ ns: 'rs:boot', topic: 'shell' });
    void this.cartStore.ensureCart().catch((err: unknown) => {
      rsConsoleWrite({
        ns: 'rs:cart',
        topic: 'ensureCart',
        level: 'warn',
        kv: { err: err instanceof Error ? err.message : String(err) },
      });
    });
  }
}
