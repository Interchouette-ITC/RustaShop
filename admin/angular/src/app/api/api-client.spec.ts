import { TestBed } from '@angular/core/testing';
import { provideHttpClient } from '@angular/common/http';

import { ApiClient } from './api-client';

describe('ApiClient', () => {
  let client: ApiClient;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [provideHttpClient(), ApiClient],
    });
    client = TestBed.inject(ApiClient);
  });

  it('builds absolute storefront URLs from the API base', () => {
    expect(client.url('/v1/products')).toBe('/api/v1/products');
  });

  it('builds operator URLs under the admin API prefix', () => {
    expect(client.adminUrl('orders')).toBe('/api/v1/admin/orders');
    expect(client.adminUrl('/products')).toBe('/api/v1/admin/products');
  });
});
