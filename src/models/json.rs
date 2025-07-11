//! JSON data structures for parsing weapons.json

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root structure of weapons.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponsData {
    pub categories: Vec<CategoryData>,
}

/// Category with weapons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryData {
    pub name: String,
    pub weapons: Vec<WeaponData>,
}

/// Weapon with stats and ammo configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponData {
    pub name: String,
    pub stats: Vec<WeaponStatData>,
    #[serde(rename = "ammoStats")]
    pub ammo_stats: HashMap<String, AmmoStatData>,
}

/// Individual weapon configuration (barrel + ammo combination)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStatData {
    #[serde(rename = "barrelType")]
    pub barrel_type: String,
    pub dropoffs: Vec<DamageDropoffData>,
    pub velocity: i16,
    #[serde(rename = "rpmSingle")]
    pub rpm_single: Option<i16>,
    #[serde(rename = "rpmBurst")]
    pub rpm_burst: Option<i16>,
    #[serde(rename = "rpmAuto")]
    pub rpm_auto: Option<i16>,
    #[serde(rename = "ammoType")]
    pub ammo_type: String,
}

/// Damage dropoff at specific range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageDropoffData {
    pub damage: f64,
    pub range: i16,
}

/// Ammo-specific statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmoStatData {
    #[serde(rename = "magSize")]
    pub mag_size: i16,
    #[serde(rename = "headshotMultiplier")]
    pub headshot_multiplier: f64,
    #[serde(rename = "emptyReload")]
    pub empty_reload: f64,
    #[serde(rename = "tacticalReload")]
    pub tactical_reload: f64,
    #[serde(rename = "pelletCount")]
    pub pellet_count: Option<i16>,
}
