-- BF2042 Weapon Stats Database Schema
-- This file creates the complete database schema without SQLx validation

-- Categories table
CREATE TABLE IF NOT EXISTS categories (
    category_id SERIAL PRIMARY KEY,
    category_name VARCHAR(50) NOT NULL UNIQUE
);

-- Weapons table
CREATE TABLE IF NOT EXISTS weapons (
    weapon_id SERIAL PRIMARY KEY,
    weapon_name VARCHAR(100) NOT NULL UNIQUE,
    category_id INTEGER NOT NULL REFERENCES categories(category_id)
);

-- Barrels table
CREATE TABLE IF NOT EXISTS barrels (
    barrel_id SERIAL PRIMARY KEY,
    barrel_name VARCHAR(100) NOT NULL UNIQUE
);

-- Ammo types table
CREATE TABLE IF NOT EXISTS ammo_types (
    ammo_id SERIAL PRIMARY KEY,
    ammo_type_name VARCHAR(100) NOT NULL UNIQUE
);

-- Weapon ammo compatibility and stats
CREATE TABLE IF NOT EXISTS weapon_ammo_stats (
    weapon_id INTEGER NOT NULL REFERENCES weapons(weapon_id),
    ammo_id INTEGER NOT NULL REFERENCES ammo_types(ammo_id),
    magazine_size SMALLINT NOT NULL,
    empty_reload_time DECIMAL(4,2),
    tactical_reload_time DECIMAL(4,2),
    headshot_multiplier DECIMAL(3,1) NOT NULL,
    pellet_count SMALLINT DEFAULT 1,
    PRIMARY KEY (weapon_id, ammo_id)
);

-- Configurations table (weapon + barrel + ammo combinations)
CREATE TABLE IF NOT EXISTS configurations (
    config_id SERIAL PRIMARY KEY,
    weapon_id INTEGER NOT NULL REFERENCES weapons(weapon_id),
    barrel_id INTEGER NOT NULL REFERENCES barrels(barrel_id),
    ammo_id INTEGER NOT NULL REFERENCES ammo_types(ammo_id),
    velocity SMALLINT NOT NULL,
    rpm_single SMALLINT,
    rpm_burst SMALLINT,
    rpm_auto SMALLINT,
    UNIQUE(weapon_id, barrel_id, ammo_id)
);

-- Damage dropoff data for each configuration
CREATE TABLE IF NOT EXISTS config_dropoffs (
    config_id INTEGER NOT NULL REFERENCES configurations(config_id),
    range SMALLINT NOT NULL,
    damage DECIMAL(5,1) NOT NULL,
    PRIMARY KEY (config_id, range)
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_weapons_category ON weapons(category_id);
CREATE INDEX IF NOT EXISTS idx_configurations_weapon ON configurations(weapon_id);
CREATE INDEX IF NOT EXISTS idx_config_dropoffs_config ON config_dropoffs(config_id);
CREATE INDEX IF NOT EXISTS idx_config_dropoffs_range ON config_dropoffs(range);
CREATE INDEX IF NOT EXISTS idx_weapon_ammo_stats_weapon ON weapon_ammo_stats(weapon_id);
