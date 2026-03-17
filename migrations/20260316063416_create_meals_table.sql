-- Add migration script here


-- 1. Create meal_type enum;

CREATE TYPE meal_type_enum AS ENUM ('breakfast', 'lunch', 'dinner', 'snack');


-- 2. Create meals table;

CREATE TABLE IF NOT EXISTS meals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    -- meal details;
    name VARCHAR(255) NOT NULL, -- name of the food;
    picture TEXT,

    -- meal type;
    meal_type meal_type_enum NOT NULL,
    
    -- AI generated values;
    fats FLOAT4 NOT NULL CHECK (fats >= 0),
    carbs FLOAT4 NOT NULL CHECK (carbs >= 0),
    fiber FLOAT4 NOT NULL CHECK (fiber >= 0),
    protein FLOAT4 NOT NULL CHECK (protein >= 0),
    calories FLOAT4 NOT NULL CHECK (calories >= 0),
    confidence FLOAT4 NOT NULL CHECK (confidence >= 0 AND confidence <= 1),

    -- timestamps;
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- 3. Attach update_updated_at_column fn to meals table;

CREATE OR REPLACE TRIGGER set_meals_updated_at
BEFORE UPDATE ON meals
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- 4. Indexes;

CREATE INDEX idx_meals_user_id ON meals(user_id);
CREATE INDEX idx_meals_created_at ON meals(created_at);

CREATE INDEX idx_meals_name ON meals(name);
CREATE INDEX idx_meals_meal_type ON meals(meal_type);

CREATE INDEX idx_meals_user_id_meal_type ON meals(user_id, meal_type);