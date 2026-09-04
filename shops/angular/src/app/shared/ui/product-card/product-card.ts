import { Component, Input } from '@angular/core';

/**
 * Compact catalog tile. Markup/styles: `templates/default/product_card`
 * (shared with the Leptos shop). Controller stays in this host.
 */
@Component({
  selector: 'rs-product-card',
  templateUrl: '../../../../../../../templates/default/product_card/product_card.html',
  styleUrl: './product-card.scss',
})
export class ProductCard {
  @Input({ required: true }) name!: string;
  @Input({ required: true }) slug!: string;
  @Input() description: string | null | undefined = null;
  @Input({ required: true }) detailHref!: string;
}
