import { Component, OnInit, inject, signal } from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';

import { CartStore } from '@rustashop/shop-core';
import { template as shopShellTpl, styles as shopShellStyles } from '@generated/shop_shell.ng';

import { rsConsoleWrite } from '../../app/core/logging/rs-console';

@Component({
  selector: 'rs-shop-shell',
  imports: [RouterOutlet, RouterLink, RouterLinkActive],
  template: shopShellTpl,
  styles: shopShellStyles,
})
export class ShopShell implements OnInit {
  private readonly cartStore = inject(CartStore);

  protected readonly lineCount = this.cartStore.lineCount;
  protected readonly brand = signal('rustashop');

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
