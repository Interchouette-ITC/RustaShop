import { Component, OnInit, inject } from '@angular/core';

import { CatalogStore } from '@rustashop/shop-core';
import { ProductCard } from '@rustashop/shop-shared';
import { template as productListTpl, styles as productListStyles } from '@generated/product_list.ng';

@Component({
  selector: 'rs-product-list',
  imports: [ProductCard],
  template: productListTpl,
  styles: productListStyles,
})
export class ProductListPage implements OnInit {
  private readonly catalog = inject(CatalogStore);

  protected readonly products = this.catalog.products;
  protected readonly loading = this.catalog.loadingList;
  protected readonly error = this.catalog.error;

  ngOnInit(): void {
    void this.catalog.loadProducts().catch(() => {
      // Error signal already set by CatalogStore.
    });
  }
}
