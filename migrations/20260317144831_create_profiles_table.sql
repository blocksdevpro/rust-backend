-- Add migration script here


-- 1. Create enums;

CREATE TYPE profile_activity_enum AS ENUM ('sedentary', 'light', 'moderate', 'active', 'vigorous');
CREATE TYPE profile_gender_enum AS ENUM ('male', 'female');
CREATE TYPE profile_goal_enum AS ENUM ('lose', 'gain', 'maintain');


-- 2. Create profiles table;

CREATE TABLE IF NOT EXISTS profiles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    
    -- numbers;
    age INT NOT NULL CHECK (age >= 1 AND age <= 100 ),
    height INT NOT NULL CHECK (height >= 50 AND height <= 220),
    weight INT NOT NULL CHECK (weight >= 10 AND weight <= 150),

    -- enums;
    goal profile_goal_enum NOT NULL DEFAULT 'maintain',
    gender profile_gender_enum NOT NULL DEFAULT 'male',
    activity profile_activity_enum NOT NULL DEFAULT 'sedentary',

    -- targets;
    target_fats FLOAT4 NOT NULL CHECK (target_fats >= 0),
    target_fiber FLOAT4 NOT NULL CHECK (target_fiber >= 0),
    target_carbs FLOAT4 NOT NULL CHECK (target_carbs >= 0),
    target_protein FLOAT4 NOT NULL CHECK (target_protein >= 0),
    target_calories FLOAT4 NOT NULL CHECK (target_calories >= 0),


    -- timestamps;
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

-- 3. Attach update_updated_at_column fn to profiles table;

CREATE OR REPLACE TRIGGER set_profiles_updated_at
BEFORE UPDATE ON profiles
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- 4. Indexes;

CREATE INDEX idx_profiles_user_id ON profiles(user_id);
CREATE INDEX idx_profiles_created_at ON profiles(created_at);

