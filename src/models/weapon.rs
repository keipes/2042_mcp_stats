//! Weapon-related data structures

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Custom decimal type for precise damage calculations
/// Represents DECIMAL(5,1) - 4 digits before decimal, 1 after (e.g., 9999.9)
pub type Damage = rust_decimal::Decimal;

/// Custom decimal type for reload times
/// Represents DECIMAL(4,2) - 2 digits before decimal, 2 after (e.g., 99.99)
pub type ReloadTime = rust_decimal::Decimal;

/// Custom decimal type for headshot multiplier
/// Represents DECIMAL(3,1) - 2 digits before decimal, 1 after (e.g., 99.9)
pub type HeadshotMultiplier = rust_decimal::Decimal;

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
    pub empty_reload_time: Option<ReloadTime>,
    pub tactical_reload_time: Option<ReloadTime>,
    pub headshot_multiplier: HeadshotMultiplier,
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
    pub damage: Damage,
}

/// Combined weapon configuration with dropoffs for streaming
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeaponConfigWithDropoffs {
    pub config_id: i32,
    pub weapon_name: String,
    pub barrel_name: String,
    pub ammo_type_name: String,
    pub velocity: i16,
    pub rpm_single: Option<i16>,
    pub rpm_burst: Option<i16>,
    pub rpm_auto: Option<i16>,
    pub range: i16,
    pub damage: Damage,
}

/// Weapon ammo stats with names for streaming
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeaponAmmoStatsWithNames {
    pub weapon_name: String,
    pub ammo_type_name: String,
    pub magazine_size: i16,
    pub empty_reload_time: Option<ReloadTime>,
    pub tactical_reload_time: Option<ReloadTime>,
    pub headshot_multiplier: HeadshotMultiplier,
    pub pellet_count: Option<i16>,
}

/// Complex query result for damage at range
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DamageAtRange {
    pub weapon_name: String,
    pub barrel_name: String,
    pub ammo_type_name: String,
    pub effective_range: i16,
    pub damage: Damage,
    pub velocity: i16,
    pub rpm_single: Option<i16>,
    pub rpm_burst: Option<i16>,
    pub rpm_auto: Option<i16>,
}

/// Best configuration in category result
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BestConfigInCategory {
    pub weapon_name: String,
    pub barrel_name: String,
    pub ammo_type_name: String,
    pub effective_range: i16,
    pub damage: Damage,
    pub velocity: i16,
    pub rpm_single: Option<i16>,
    pub rpm_burst: Option<i16>,
    pub rpm_auto: Option<i16>,
    pub magazine_size: i16,
    pub empty_reload_time: Option<ReloadTime>,
    pub tactical_reload_time: Option<ReloadTime>,
    pub headshot_multiplier: HeadshotMultiplier,
}

/// Complete weapon details including all configurations and stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponDetails {
    pub weapon: Weapon,
    pub configurations: Vec<WeaponConfigWithDropoffs>,
    pub ammo_stats: Vec<WeaponAmmoStatsWithNames>,
}

/// Database validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub table_counts: std::collections::HashMap<String, i64>,
}
