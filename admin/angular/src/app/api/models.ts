/** Money JSON from the Commerce API. */
export interface MoneyDto {
  amount_minor: number;
  currency: string;
}

/** Order JSON from admin list/PATCH. */
export interface OrderDto {
  id: string;
  number: string;
  cart_id: string | null;
  state: string;
  payment_status: string;
  currency: string;
  items_total: MoneyDto;
  total: MoneyDto;
}

/** Admin order list body. */

export interface OrderListDto {
  items: OrderDto[];
}

/** Product JSON from admin list. */
export interface ProductDto {
  id: string;
  category_id: string | null;
  slug: string;
  name: string;
  description: string | null;
  enabled: boolean;
}

/** Admin product list body. */

export interface ProductListDto {
  items: ProductDto[];
}

/** Allowed fulfillment statuses for PATCH. */
export const ORDER_STATUSES = ['placed', 'paid', 'shipped', 'cancelled'] as const;

export type OrderStatus = (typeof ORDER_STATUSES)[number];
