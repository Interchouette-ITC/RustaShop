import { Routes } from '@angular/router';

import { AdminShell } from '../components/admin_shell/admin_shell';

export const routes: Routes = [
  {
    path: '',
    component: AdminShell,
    children: [
      {
        path: '',
        loadComponent: () =>
          import('../components/orders_page/orders_page').then((m) => m.OrdersPage),
      },
      { path: '**', redirectTo: '' },
    ],
  },
];
