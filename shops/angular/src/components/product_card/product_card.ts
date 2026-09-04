import { Component, Input } from '@angular/core';

import { template as productCardTpl, styles as productCardStyles } from '@generated/product_card.ng';

/** Compact catalog tile. Markup: `@rustashop/template-default`. */
@Component({
  selector: 'rs-product-card',
  template: productCardTpl,
  styles: productCardStyles,
})
export class ProductCard {
  @Input({ required: true }) name!: string;
  @Input({ required: true }) slug!: string;
  @Input() description: string | null | undefined = null;
  @Input({ required: true }) detailHref!: string;
}
