-- Demo catalog seed (idempotent). Safe to re-run: never drops tables.
-- Does NOT wipe the database. To destroy all data:
--   CONFIRM=YES make db-reset
--   make db-seed

INSERT INTO category (id, slug, name) VALUES
    ('11111111-1111-1111-1111-111111111111', 'apparel', 'Apparel')
ON CONFLICT (id) DO UPDATE
SET
    slug = EXCLUDED.slug,
    name = EXCLUDED.name;

-- Insert only when neither the canonical id nor the slug is already taken.
INSERT INTO product (id, category_id, slug, name, description, enabled)
SELECT
    v.id,
    v.category_id,
    v.slug,
    v.name,
    v.description,
    v.enabled
FROM (
    VALUES
        (
            '22222222-2222-2222-2222-222222222221'::uuid,
            '11111111-1111-1111-1111-111111111111'::uuid,
            'hoodie',
            'Hoodie',
            'Cotton hoodie',
            TRUE
        ),
        (
            '22222222-2222-2222-2222-222222222222'::uuid,
            '11111111-1111-1111-1111-111111111111'::uuid,
            'mug',
            'Mug',
            'Ceramic mug',
            TRUE
        ),
        (
            '22222222-2222-2222-2222-222222222223'::uuid,
            '11111111-1111-1111-1111-111111111111'::uuid,
            'tote',
            'Tote',
            'Canvas tote',
            TRUE
        )
) AS v(id, category_id, slug, name, description, enabled)
WHERE NOT EXISTS (SELECT 1 FROM product p WHERE p.id = v.id)
  AND NOT EXISTS (SELECT 1 FROM product p WHERE p.slug = v.slug);

-- Refresh labels for rows that already match by slug (keeps existing ids).
UPDATE product AS p
SET
    name = v.name,
    description = v.description,
    enabled = v.enabled,
    category_id = v.category_id
FROM (
    VALUES
        (
            'hoodie',
            'Hoodie',
            'Cotton hoodie',
            TRUE,
            '11111111-1111-1111-1111-111111111111'::uuid
        ),
        (
            'mug',
            'Mug',
            'Ceramic mug',
            TRUE,
            '11111111-1111-1111-1111-111111111111'::uuid
        ),
        (
            'tote',
            'Tote',
            'Canvas tote',
            TRUE,
            '11111111-1111-1111-1111-111111111111'::uuid
        )
) AS v(slug, name, description, enabled, category_id)
WHERE p.slug = v.slug;

INSERT INTO product_variant (id, product_id, sku, name, price_minor, currency, stock_quantity)
SELECT v.id, p.id, v.sku, v.name, v.price_minor, v.currency, v.stock_quantity
FROM (
    VALUES
        (
            '33333333-3333-3333-3333-333333333331'::uuid,
            'hoodie',
            'HOODIE-M',
            'Medium',
            4500::bigint,
            'EUR',
            8
        ),
        (
            '33333333-3333-3333-3333-333333333332'::uuid,
            'mug',
            'MUG-STD',
            'Standard',
            1200::bigint,
            'EUR',
            20
        ),
        (
            '33333333-3333-3333-3333-333333333333'::uuid,
            'tote',
            'TOTE-STD',
            'Standard',
            1800::bigint,
            'EUR',
            12
        )
) AS v(id, product_slug, sku, name, price_minor, currency, stock_quantity)
INNER JOIN product p ON p.slug = v.product_slug
WHERE NOT EXISTS (SELECT 1 FROM product_variant pv WHERE pv.sku = v.sku)
  AND NOT EXISTS (SELECT 1 FROM product_variant pv WHERE pv.id = v.id);
