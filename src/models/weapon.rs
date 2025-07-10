//! Weapon-related data structures

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Weapon category (categories.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}

/// Basic weapon information (weapons.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Weapon {
    pub weapon_id: i32,
    pub weapon_name: String,
    pub category_id: i32,
}

/// Barrel information (barrels.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Barrel {
    pub barrel_id: i32,
    pub barrel_name: String,
}

/// Ammo type information (ammo_types.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AmmoType {
    pub ammo_id: i32,
    pub ammo_type_name: String,
}

/// Weapon ammo-specific stats (weapon_ammo_stats.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeaponAmmoStats {
    pub weapon_id: i32,
    pub ammo_id: i32,
    pub magazine_size: i16,
    pub empty_reload_time: f64,
    pub tactical_reload_time: f64,
    pub headshot_multiplier: f64,
    pub pellet_count: Option<i16>,
}

/// Weapon configuration with barrel and ammo (configurations.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Configuration {
    pub config_id: i32,
    pub weapon_id: i32,
    pub barrel_id: i32,
    pub ammo_id: i32,
    pub velocity: i16,
    pub rpm_single: Option<i16>,
    pub rpm_burst: Option<i16>,
    pub rpm_auto: Option<i16>,
}

/// Damage dropoff at specific ranges (config_dropoffs.csv)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ConfigDropoff {
    pub config_id: i32,
    pub range: i16,
    pub damage: f64,
}
