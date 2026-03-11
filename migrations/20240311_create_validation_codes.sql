-- Create validation_codes table
CREATE TABLE IF NOT EXISTS validation_codes (
    email TEXT PRIMARY KEY,
    code TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
