import { Component, inject, OnInit, signal } from '@angular/core';
import { HttpErrorResponse } from '@angular/common/http';

import { ProductListResponse, RustashopApi } from './api';

@Component({
  selector: 'rs-root',
  templateUrl: './app.html',
  styleUrl: './app.scss',
})
export class App implements OnInit {
  private readonly api = inject(RustashopApi);

  protected readonly title = signal('RustaShop');
  protected readonly healthStatus = signal<string | null>(null);
  protected readonly products = signal<ProductListResponse['items']>([]);
  protected readonly error = signal<string | null>(null);

  ngOnInit(): void {
    this.api.healthz().subscribe({
      next: (body) => this.healthStatus.set(body.status),
      error: (err: unknown) => this.error.set(formatHttpError(err)),
    });
    this.api.listProducts().subscribe({
      next: (body) => this.products.set(body.items),
      error: (err: unknown) => this.error.set(formatHttpError(err)),
    });
  }
}

function formatHttpError(err: unknown): string {
  if (err instanceof HttpErrorResponse) {
    return `API ${err.status}: ${err.message}`;
  }
  return 'API request failed';
}
