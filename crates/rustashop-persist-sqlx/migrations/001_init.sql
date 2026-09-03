-- MVP commerce schema (PostgreSQL).
-- Money: BIGINT minor units + CHAR(3) currency (no floats).
-- Cart and order lines snapshot labels and unit price at mutation time
-- (Sylius-style immutability lesson from PHP reference trees).

CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE category (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES category (id) ON DELETE SET NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT category_slug_parent_unique UNIQUE (parent_id, slug)
);

CREATE UNIQUE INDEX category_root_slug_idx ON category (slug) WHERE parent_id IS NULL;

CREATE TABLE product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category_id UUID REFERENCES category (id) ON DELETE SET NULL,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX product_category_id_idx ON product (category_id);

CREATE TABLE product_variant (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES product (id) ON DELETE CASCADE,
    sku TEXT NOT NULL UNIQUE,
    name TEXT,
    price_minor BIGINT NOT NULL CHECK (price_minor >= 0),
    currency CHAR(3) NOT NULL,
    stock_quantity INTEGER NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX product_variant_product_id_idx ON product_variant (product_id);

CREATE TABLE customer (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE cart (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_id UUID REFERENCES customer (id) ON DELETE SET NULL,
    token TEXT NOT NULL UNIQUE,
    currency CHAR(3) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX cart_customer_id_idx ON cart (customer_id);

CREATE TABLE cart_line (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cart_id UUID NOT NULL REFERENCES cart (id) ON DELETE CASCADE,
    variant_id UUID NOT NULL REFERENCES product_variant (id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price_minor BIGINT NOT NULL CHECK (unit_price_minor >= 0),
    currency CHAR(3) NOT NULL,
    product_name TEXT NOT NULL,
    variant_sku TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT cart_line_cart_variant_unique UNIQUE (cart_id, variant_id)
);

CREATE TABLE "order" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    number TEXT NOT NULL UNIQUE,
    customer_id UUID REFERENCES customer (id) ON DELETE SET NULL,
    cart_id UUID REFERENCES cart (id) ON DELETE SET NULL,
    state TEXT NOT NULL DEFAULT 'draft',
    currency CHAR(3) NOT NULL,
    items_total_minor BIGINT NOT NULL DEFAULT 0 CHECK (items_total_minor >= 0),
    total_minor BIGINT NOT NULL DEFAULT 0 CHECK (total_minor >= 0),
    idempotency_key TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    placed_at TIMESTAMPTZ
);

CREATE INDEX order_customer_id_idx ON "order" (customer_id);
CREATE INDEX order_state_idx ON "order" (state);

CREATE TABLE order_line (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES "order" (id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variant (id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price_minor BIGINT NOT NULL CHECK (unit_price_minor >= 0),
    line_total_minor BIGINT NOT NULL CHECK (line_total_minor >= 0),
    currency CHAR(3) NOT NULL,
    product_name TEXT NOT NULL,
    variant_sku TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX order_line_order_id_idx ON order_line (order_id);
