import { Component, OnInit, inject, signal } from '@angular/core';
import { ActivatedRoute, RouterLink } from '@angular/router';
import { HttpErrorResponse } from '@angular/common/http';
import { FormsModule } from '@angular/forms';

import { ProductDetailResponse } from '../../../api';
import { RustashopApi } from '../../../api';
import { CartStore } from '../../../core/cart/cart.store';
import { MoneyPipe } from '../../../shared/pipes/money.pipe';

@Component({
  selector: 'rs-product-detail',
  imports: [RouterLink, FormsModule, MoneyPipe],
  templateUrl: './product-detail.html',
  styleUrl: './product-detail.scss',
})
export class ProductDetailPage implements OnInit {
  private readonly api = inject(RustashopApi);
  private readonly route = inject(ActivatedRoute);
  private readonly cartStore = inject(CartStore);

  protected readonly product = signal<ProductDetailResponse | null>(null);
  protected readonly loading = signal(true);
  protected readonly error = signal<string | null>(null);
  protected readonly notice = signal<string | null>(null);
  protected readonly selectedVariantId = signal<string | null>(null);
  protected readonly quantity = signal(1);
  protected readonly adding = signal(false);

  ngOnInit(): void {
    const id = this.route.snapshot.paramMap.get('id');
    if (!id) {
      this.error.set('Missing product id.');
      this.loading.set(false);
      return;
    }
    this.api.getProduct(id).subscribe({
      next: (body) => {
        this.product.set(body);
        const first = body.variants[0];
        this.selectedVariantId.set(first?.id ?? null);
        this.loading.set(false);
      },
      error: (err: unknown) => {
        this.error.set(formatHttpError(err));
        this.loading.set(false);
      },
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
      this.error.set(formatHttpError(err));
    } finally {
      this.adding.set(false);
    }
  }
}

function formatHttpError(err: unknown): string {
  if (err instanceof HttpErrorResponse) {
    if (err.status === 404) {
      return 'Product not found.';
    }
    return `Request failed (${err.status}).`;
  }
  return 'Request failed.';
}
