//! Weapon-related data structures

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Weapon category
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

/// Basic weapon information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Weapon {
    pub id: i32,
    pub name: String,
    pub category_id: i32,
    pub base_damage: f64,
    pub fire_rate: f64,
    pub velocity: f64,
    pub range: f64,
}

/// Weapon configuration with barrel and ammo
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WeaponConfig {
    pub id: i32,
    pub weapon_id: i32,
    pub barrel_id: Option<i32>,
    pub ammo_id: Option<i32>,
    pub effective_damage: f64,
    pub effective_range: f64,
}

/// Damage dropoff at specific ranges
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DamageDropoff {
    pub id: i32,
    pub config_id: i32,
    pub range_meters: f64,
    pub damage_multiplier: f64,
}
