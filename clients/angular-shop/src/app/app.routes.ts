import { Routes } from '@angular/router';

import { ShopShell } from './shared/layout/shop-shell';

export const routes: Routes = [
  {
    path: '',
    component: ShopShell,
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./features/catalog/product-list/product-list').then((m) => m.ProductListPage),
      },
      {
        path: 'products/:id',
        loadComponent: () =>
          import('./features/catalog/product-detail/product-detail').then(
            (m) => m.ProductDetailPage,
          ),
      },
      {
        path: 'cart',
        loadComponent: () => import('./features/cart/cart-page/cart-page').then((m) => m.CartPage),
      },
      {
        path: 'checkout/:orderId',
        loadComponent: () =>
          import('./features/checkout/checkout-page/checkout-page').then((m) => m.CheckoutPage),
      },
      { path: '**', redirectTo: '' },
    ],
  },
];
