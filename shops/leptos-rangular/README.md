# Leptos + rangular shop (track B)

```bash
make run-api          # :8080
make shop-leptos-rangular  # :4181, proxies /api → API
```

Routes: `/` catalog, `/products/:id` add-to-cart, `/cart`.
Cart id key `rs.cartId` matches the Angular shop (same browser = shared cart).
Shared card markup: `templates/default/product_card`.
