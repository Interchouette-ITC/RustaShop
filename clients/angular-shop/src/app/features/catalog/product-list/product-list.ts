import { Component, OnInit, inject, signal } from '@angular/core';
import { RouterLink } from '@angular/router';
import { HttpErrorResponse } from '@angular/common/http';

import { ProductResponse, RustashopApi } from '../../../api';

@Component({
  selector: 'rs-product-list',
  imports: [RouterLink],
  templateUrl: './product-list.html',
  styleUrl: './product-list.scss',
})
export class ProductListPage implements OnInit {
  private readonly api = inject(RustashopApi);

  protected readonly products = signal<ProductResponse[]>([]);
  protected readonly loading = signal(true);
  protected readonly error = signal<string | null>(null);

  ngOnInit(): void {
    this.api.listProducts().subscribe({
      next: (body) => {
        this.products.set(body.items);
        this.loading.set(false);
      },
      error: (err: unknown) => {
        this.error.set(formatHttpError(err));
        this.loading.set(false);
      },
    });
  }
}

function formatHttpError(err: unknown): string {
  if (err instanceof HttpErrorResponse) {
    return `Could not load catalog (${err.status}). Is the API running?`;
  }
  return 'Could not load catalog.';
}
