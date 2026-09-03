ALTER TABLE cart
    ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'open';

DO $$
BEGIN
    ALTER TABLE cart
        ADD CONSTRAINT cart_status_check CHECK (status IN ('open', 'checked_out'));
EXCEPTION
    WHEN duplicate_object THEN NULL;
END
$$;
