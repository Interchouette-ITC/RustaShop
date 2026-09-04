import { Component, OnInit, inject, signal } from '@angular/core';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { FormsModule } from '@angular/forms';

import { CartStore, CatalogStore, formatApiError } from '@rustashop/shop-core';
import { MoneyPipe } from '@rustashop/shop-shared';
import { template as productDetailTpl, styles as productDetailStyles } from '@generated/product_detail.ng';

@Component({
  selector: 'rs-product-detail',
  imports: [RouterLink, FormsModule, MoneyPipe],
  template: productDetailTpl,
  styles: productDetailStyles,
})
export class ProductDetailPage implements OnInit {
  private readonly catalog = inject(CatalogStore);
  private readonly cartStore = inject(CartStore);
  private readonly route = inject(ActivatedRoute);

  protected readonly product = this.catalog.product;
  protected readonly loading = this.catalog.loadingDetail;
  protected readonly error = signal<string | null>(null);
  protected readonly notice = signal<string | null>(null);
  protected readonly selectedVariantId = signal<string | null>(null);
  protected readonly quantity = signal(1);
  protected readonly adding = signal(false);

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');
    if (!id) {
      this.error.set('Missing product id.');
      return;
    }
    void this.catalog
      .loadProduct(id)
      .then((body) => {
        this.error.set(null);
        this.selectedVariantId.set(body.variants[0]?.id ?? null);
      })
      .catch((err: unknown) => {
        this.error.set(this.catalog.error() ?? formatApiError(err));
      });
  }

  protected async addToCart(): Promise<void> {
    const variantId = this.selectedVariantId();
    if (!variantId) {
      return;
    }
    this.adding.set(true);
    this.notice.set(null);
    this.error.set(null);
    try {
      await this.cartStore.addLine(variantId, this.quantity());
      this.notice.set('Added to cart.');
    } catch (err) {
      this.error.set(formatApiError(err));
    } finally {
      this.adding.set(false);
    }
  }
}
