//! Database manager for schema and data operations

use std::env;

use sled::Tree;

use crate::models::{Category, DatabaseConfig, ValidationReport};
use crate::{Result, StatsError};

pub struct DatabaseManager;

impl DatabaseManager {
    pub async fn new(_config: &DatabaseConfig) -> Result<Self> {
        // TODO: implement
        let db_root = env::var("DB_ROOT").unwrap_or_else(|_| "db".to_string());
        let db = sled::open(db_root).unwrap();
        // let tree = db.open_tree("main").unwrap();
        // tree.insert(key, value);
        Ok(DatabaseManager)
    }

    pub fn insert_category(&self, _category: &Category) -> Result<()> {
        // TODO: implement
        Ok(())
    }

    pub fn get_category(&self, _category_id: i32) -> Result<Option<Category>> {
        // TODO: implement
        Ok(None)
    }

    pub async fn test_connection(&self) -> Result<()> {
        // TODO: implement
        Ok(())
    }

    pub async fn validate_data(&self) -> Result<ValidationReport> {
        // TODO: implement
        Ok(ValidationReport {
            is_valid: true,
            issues: Vec::new(),
            table_counts: std::collections::HashMap::new(),
        })
    }
}

// range.weapon.ammo.barrel.damage
// # Range 0
// ## Damage 30
// ### WeaponConfig1
// ### WeaponConfig2
// ## Ammo 2

// R1D30 Config1
// Name.Barrel.Ammo

// RPM700 Name.Barrel.Ammo
// R1D30 Config2
// R1D30 Config3
