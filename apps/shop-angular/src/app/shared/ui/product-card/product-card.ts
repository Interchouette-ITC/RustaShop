import { Component, input } from '@angular/core';
import { RouterLink } from '@angular/router';

import type { ProductResponse } from '@rustashop/shop-api';

/** Compact catalog tile used on the product list. */
@Component({
  selector: 'rs-product-card',
  imports: [RouterLink],
  templateUrl: './product-card.html',
  styleUrl: './product-card.scss',
})
export class ProductCard {
  readonly product = input.required<ProductResponse>();
}
