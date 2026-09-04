import type { components } from './schema';

/** OpenAPI schema aliases used by the shop HTTP clients. */
export type HealthResponse = components['schemas']['HealthResponse'];
export type ProductListResponse = components['schemas']['ProductListResponse'];
export type ProductDetailResponse = components['schemas']['ProductDetailResponse'];
export type ProductResponse = components['schemas']['ProductResponse'];
export type ProductVariantResponse = components['schemas']['ProductVariantResponse'];
export type CartResponse = components['schemas']['CartResponse'];
export type CartLineResponse = components['schemas']['CartLineResponse'];
export type MoneyResponse = components['schemas']['MoneyResponse'];
export type CreateCartRequest = components['schemas']['CreateCartRequest'];
export type AddCartLineRequest = components['schemas']['AddCartLineRequest'];
export type UpdateCartLineRequest = components['schemas']['UpdateCartLineRequest'];
export type CheckoutRequest = components['schemas']['CheckoutRequest'];
export type OrderResponse = components['schemas']['OrderResponse'];
export type OrderLineResponse = components['schemas']['OrderLineResponse'];
