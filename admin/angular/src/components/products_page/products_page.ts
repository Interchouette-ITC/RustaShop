import { Component, effect, inject, signal } from '@angular/core';

import { AdminProductsApi, type ProductDto } from '@rustashop/admin-api';
import { AdminTokenStore, formatApiError } from '@rustashop/admin-core';
import {
  template as productsPageTpl,
  styles as productsPageStyles,
} from '@generated/products_page.ng';

@Component({
  selector: 'rs-products-page',
  template: productsPageTpl,
  styles: productsPageStyles,
})
export class ProductsPage {
  private readonly api = inject(AdminProductsApi);
  private readonly tokens = inject(AdminTokenStore);

  protected readonly products = signal<ProductDto[]>([]);
  protected readonly busy = signal(false);
  protected readonly error = signal<string | null>(null);
  protected readonly hasToken = this.tokens.hasToken;

  constructor() {
    effect(() => {
      if (this.tokens.hasToken()) {
        void this.reload();
      } else {
        this.products.set([]);
        this.error.set(null);
      }
    });
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
      this.products.set(page.items);
    } catch (err) {
      this.error.set(formatApiError(err, 'Failed to load products.'));
      this.products.set([]);
    } finally {
      this.busy.set(false);
    }
  }
}
