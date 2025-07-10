# Data Types

velocity SMALLINT
rpm_single SMALLINT NULL
rpm_burst SMALLINT NULL
rpm_auto SMALLINT NULL
range SMALLINT
damage DECIMAL(5,1)
magazine_size SMALLINT
empty_reload_time DECIMAL(4,2)
tactical_reload_time DECIMAL(4,2)  
headshot_multiplier DECIMAL(3,1)
pellet_count SMALLINT NULL

# Core Schema

- Primary Key is first field unless otherwise specified.

## categories.csv

category_id,category_name

## weapons.csv

weapon_id,weapon_name,category_id

## barrels.csv

barrel_id,barrel_name

## ammo_types.csv

ammo_id,ammo_type_name

## weapon_ammo_stats.csv

- primary_key(weapon_id,ammo_id)
  weapon_id,ammo_id,magazine_size,empty_reload_time,tactical_reload_time,headshot_multiplier,pellet_count

## configurations.csv

- UNIQUE(weapon_id, barrel_id, ammo_id)

config_id,weapon_id,barrel_id,ammo_id,velocity,rpm_single,rpm_burst,rpm_auto

## config_dropoffs.csv

- UNIQUE(config_id,range)
  config_id,range,damage

# Foreign Keys

weapons.category_id → categories.category_id
configurations.weapon_id → weapons.weapon_id
configurations.barrel_id → barrels.barrel_id
configurations.ammo_id → ammo_types.ammo_id
weapon_ammo_stats.weapon_id → weapons.weapon_id
weapon_ammo_stats.ammo_id → ammo_types.ammo_id
config_dropoffs.config_id → configurations.config_id

# Performance Indexes

## weapons_category

index weapons(category_id)

## configs_weapon

index configurations(weapon_id)

## configs_lookup

index configurations(weapon_id,barrel_id,ammo_id)

## config_dropoffs

index config_dropoffs(config_id)

## weapons_name

index weapons(weapon_name)

## ammo_stats

index weapon_ammo_stats(weapon_id)

## dropoffs_range

index config_dropoffs(range)

## dropoffs_damage

index config_dropoffs(damage DESC)

# Example Queries

## Get all weapons in a specific category

```sql
SELECT weapon_id, weapon_name
FROM weapons
WHERE category_id = ?;
```

## Get Ammo Stats by Weapon Name

```sql

```

## Get Dropoffs by Weapon Name

```sql
SELECT
    c.config_id,
    b.barrel_name,
    a.ammo_type_name,
    cd.range,
    cd.damage,
    c.velocity,
    c.rpm_single,
    c.rpm_burst,
    c.rpm_auto
FROM weapons w
JOIN configurations c ON w.weapon_id = c.weapon_id
JOIN config_dropoffs cd ON c.config_id = cd.config_id
JOIN barrels b ON c.barrel_id = b.barrel_id
JOIN ammo_types a ON c.ammo_id = a.ammo_id
WHERE w.weapon_name = ?
ORDER BY b.barrel_name, a.ammo_type_name, cd.range;
```

## Get Highest Damage Configs in Category at Range

```sql
-- Get damage at effective range (step function behavior)
-- Uses the highest dropoff range that is <= target range
WITH effective_damage AS (
  SELECT
    c.config_id,
    cd.range,
    cd.damage,
    ROW_NUMBER() OVER (
      PARTITION BY c.config_id
      ORDER BY cd.range DESC
    ) as rn
  FROM configurations c
  JOIN config_dropoffs cd ON c.config_id = cd.config_id
  WHERE cd.range <= ?  -- Only ranges at or before target range
)
SELECT
    w.weapon_name,
    b.barrel_name,
    a.ammo_type_name,
    ed.range as effective_range,
    ed.damage,
    c.velocity,
    c.rpm_single,
    c.rpm_burst,
    c.rpm_auto
FROM weapons w
JOIN categories cat ON w.category_id = cat.category_id
JOIN configurations c ON w.weapon_id = c.weapon_id
JOIN effective_damage ed ON c.config_id = ed.config_id AND ed.rn = 1
JOIN barrels b ON c.barrel_id = b.barrel_id
JOIN ammo_types a ON c.ammo_id = a.ammo_id
WHERE cat.category_name = ?
ORDER BY ed.damage DESC
LIMIT 10;
```
