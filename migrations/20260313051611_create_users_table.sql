-- Add migration script here


-- 1. Create extension "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 2. Create a reusable function for updated_at;
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 3. Create users table;
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    google_id VARCHAR(255) UNIQUE NOT NULL,

    name VARCHAR(255),
    email VARCHAR(255) UNIQUE NOT NULL,
    picture TEXT,

    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT NULL
);

-- 4. Attach update_updated_at_column fn to users table;

CREATE OR REPLACE TRIGGER set_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- -- 5. Index on google_id
CREATE INDEX IF NOT EXISTS idx_users_name ON users(name);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);