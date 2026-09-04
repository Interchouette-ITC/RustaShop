import { Injectable, computed, inject, signal } from '@angular/core';
import { firstValueFrom } from 'rxjs';

import { ProductDetailResponse, ProductResponse, RustashopApi } from '../../api';
import { formatApiError } from '../http/api-error';

/**
 * Catalog session state (list + last detail). Pages bind signals; HTTP stays in `RustashopApi`.
 */
@Injectable({ providedIn: 'root' })
export class CatalogStore {
  private readonly api = inject(RustashopApi);

  private readonly productsSignal = signal<ProductResponse[]>([]);
  private readonly productSignal = signal<ProductDetailResponse | null>(null);
  private readonly loadingListSignal = signal(false);
  private readonly loadingDetailSignal = signal(false);
  private readonly errorSignal = signal<string | null>(null);

  readonly products = this.productsSignal.asReadonly();
  readonly product = this.productSignal.asReadonly();
  readonly loadingList = this.loadingListSignal.asReadonly();
  readonly loadingDetail = this.loadingDetailSignal.asReadonly();
  readonly error = this.errorSignal.asReadonly();
  readonly hasProducts = computed(() => this.productsSignal().length > 0);

  /** Loads the product list into `products`. */
  async loadProducts(): Promise<void> {
    this.loadingListSignal.set(true);
    this.errorSignal.set(null);
    try {
      const body = await firstValueFrom(this.api.listProducts());
      this.productsSignal.set(body.items);
    } catch (err) {
      this.productsSignal.set([]);
      this.errorSignal.set(formatApiError(err, 'Could not load catalog.'));
      throw err;
    } finally {
      this.loadingListSignal.set(false);
    }
  }

  /** Loads one product detail (with variants) into `product`. */
  async loadProduct(id: string): Promise<ProductDetailResponse> {
    this.loadingDetailSignal.set(true);
    this.errorSignal.set(null);
    try {
      const body = await firstValueFrom(this.api.getProduct(id));
      this.productSignal.set(body);
      return body;
    } catch (err) {
      this.productSignal.set(null);
      this.errorSignal.set(formatApiError(err, 'Could not load product.'));
      throw err;
    } finally {
      this.loadingDetailSignal.set(false);
    }
  }

  clearError(): void {
    this.errorSignal.set(null);
  }
}
