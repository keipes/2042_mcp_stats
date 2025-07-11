//! Stats client for querying weapon data

use tracing::{info, debug};
use crate::Result;
use crate::models::{DatabaseConfig, Weapon, WeaponConfigWithDropoffs, WeaponAmmoStatsWithNames, DamageAtRange, BestConfigInCategory, WeaponDetails};
use crate::database::DatabaseManager;

/// Client for querying weapon statistics
pub struct StatsClient {
    db_manager: DatabaseManager,
}

impl StatsClient {
    /// Create a new stats client
    pub async fn new() -> Result<Self> {
        let config = DatabaseConfig::from_env()?;
        let db_manager = DatabaseManager::new(&config).await?;
        
        // Test the connection
        db_manager.test_connection().await?;
        
        info!("StatsClient initialized successfully");
        Ok(Self { db_manager })
    }

    /// Create a new stats client with custom configuration
    pub async fn with_config(config: &DatabaseConfig) -> Result<Self> {
        let db_manager = DatabaseManager::new(config).await?;
        db_manager.test_connection().await?;
        
        info!("StatsClient initialized with custom config");
        Ok(Self { db_manager })
    }

    /// Get weapons by category (simple version for now)
    pub async fn weapons_by_category(&self, category_name: &str) -> Result<Vec<Weapon>> {
        debug!("Querying weapons by category: {}", category_name);
        
        let rows = sqlx::query!(
            r#"
            SELECT w.weapon_id, w.weapon_name, w.category_id
            FROM weapons w
            JOIN categories c ON w.category_id = c.category_id
            WHERE c.category_name = $1
            ORDER BY w.weapon_name
            "#,
            category_name
        )
        .fetch_all(self.db_manager.pool())
        .await?;
        
        let weapons = rows.into_iter().map(|row| Weapon {
            weapon_id: row.weapon_id,
            weapon_name: row.weapon_name,
            category_id: row.category_id,
        }).collect();
        
        Ok(weapons)
    }

    /// Get weapon configurations with damage dropoffs
    pub async fn weapon_configs(&self, weapon_name: &str) -> Result<Vec<WeaponConfigWithDropoffs>> {
        debug!("Querying weapon configurations for: {}", weapon_name);
        
        let rows = sqlx::query!(
            r#"
            SELECT
                c.config_id,
                w.weapon_name,
                b.barrel_name,
                a.ammo_type_name,
                c.velocity,
                c.rpm_single,
                c.rpm_burst,
                c.rpm_auto,
                cd.range,
                cd.damage
            FROM weapons w
            JOIN configurations c ON w.weapon_id = c.weapon_id
            JOIN config_dropoffs cd ON c.config_id = cd.config_id
            JOIN barrels b ON c.barrel_id = b.barrel_id
            JOIN ammo_types a ON c.ammo_id = a.ammo_id
            WHERE w.weapon_name = $1
            ORDER BY b.barrel_name, a.ammo_type_name, cd.range
            "#,
            weapon_name
        )
        .fetch_all(self.db_manager.pool())
        .await?;
        
        let configs = rows.into_iter().map(|row| WeaponConfigWithDropoffs {
            config_id: row.config_id,
            weapon_name: row.weapon_name,
            barrel_name: row.barrel_name,
            ammo_type_name: row.ammo_type_name,
            velocity: row.velocity,
            rpm_single: row.rpm_single,
            rpm_burst: row.rpm_burst,
            rpm_auto: row.rpm_auto,
            range: row.range,
            damage: row.damage,
        }).collect();
        
        Ok(configs)
    }

    /// Get weapon ammo stats
    pub async fn weapon_ammo_stats(&self, weapon_name: &str) -> Result<Vec<WeaponAmmoStatsWithNames>> {
        debug!("Querying weapon ammo stats for: {}", weapon_name);
        
        let rows = sqlx::query!(
            r#"
            SELECT
                w.weapon_name,
                a.ammo_type_name,
                was.magazine_size,
                was.empty_reload_time,
                was.tactical_reload_time,
                was.headshot_multiplier,
                was.pellet_count
            FROM weapon_ammo_stats was
            JOIN weapons w ON was.weapon_id = w.weapon_id
            JOIN ammo_types a ON was.ammo_id = a.ammo_id
            WHERE w.weapon_name = $1
            ORDER BY a.ammo_type_name
            "#,
            weapon_name
        )
        .fetch_all(self.db_manager.pool())
        .await?;
        
        let stats = rows.into_iter().map(|row| WeaponAmmoStatsWithNames {
            weapon_name: row.weapon_name,
            ammo_type_name: row.ammo_type_name,
            magazine_size: row.magazine_size,
            empty_reload_time: row.empty_reload_time,
            tactical_reload_time: row.tactical_reload_time,
            headshot_multiplier: row.headshot_multiplier,
            pellet_count: row.pellet_count,
        }).collect();
        
        Ok(stats)
    }

    /// Get effective damage for weapon configurations at specific range
    pub async fn damage_at_range(&self, weapon_name: &str, target_range: i16) -> Result<Vec<DamageAtRange>> {
        debug!("Querying damage at range {} for weapon: {}", target_range, weapon_name);
        
        let damages = sqlx::query_as::<_, DamageAtRange>(
            r#"
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
                JOIN weapons w ON c.weapon_id = w.weapon_id
                WHERE cd.range <= $2 AND w.weapon_name = $1
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
            JOIN configurations c ON w.weapon_id = c.weapon_id
            JOIN effective_damage ed ON c.config_id = ed.config_id AND ed.rn = 1
            JOIN barrels b ON c.barrel_id = b.barrel_id
            JOIN ammo_types a ON c.ammo_id = a.ammo_id
            WHERE w.weapon_name = $1
            ORDER BY ed.damage DESC
            "#,
        )
        .bind(weapon_name)
        .bind(target_range)
        .fetch_all(self.db_manager.pool())
        .await?;
        
        Ok(damages)
    }

    /// Get top performing configurations in a category at specific range
    pub async fn best_configs_in_category(&self, category_name: &str, target_range: i16, limit: i32) -> Result<Vec<BestConfigInCategory>> {
        debug!("Querying best configs in category {} at range {} (limit: {})", category_name, target_range, limit);
        
        let configs = sqlx::query_as::<_, BestConfigInCategory>(
            r#"
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
                WHERE cd.range <= $2
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
                c.rpm_auto,
                was.magazine_size,
                was.empty_reload_time,
                was.tactical_reload_time,
                was.headshot_multiplier
            FROM weapons w
            JOIN categories cat ON w.category_id = cat.category_id
            JOIN configurations c ON w.weapon_id = c.weapon_id
            JOIN effective_damage ed ON c.config_id = ed.config_id AND ed.rn = 1
            JOIN barrels b ON c.barrel_id = b.barrel_id
            JOIN ammo_types a ON c.ammo_id = a.ammo_id
            LEFT JOIN weapon_ammo_stats was ON (w.weapon_id = was.weapon_id AND a.ammo_id = was.ammo_id)
            WHERE cat.category_name = $1
            ORDER BY ed.damage DESC
            LIMIT $3
            "#,
        )
        .bind(category_name)
        .bind(target_range)
        .bind(limit)
        .fetch_all(self.db_manager.pool())
        .await?;
        
        Ok(configs)
    }

    /// Get complete weapon information including all configurations and stats
    pub async fn weapon_details(&self, weapon_name: &str) -> Result<WeaponDetails> {
        debug!("Querying complete weapon details for: {}", weapon_name);
        
        // Get basic weapon info
        let weapon = sqlx::query_as::<_, Weapon>(
            r#"
            SELECT w.weapon_id, w.weapon_name, w.category_id
            FROM weapons w
            WHERE w.weapon_name = $1
            "#,
        )
        .bind(weapon_name)
        .fetch_optional(self.db_manager.pool())
        .await?
        .ok_or_else(|| crate::StatsError::QueryFailed(format!("Weapon '{}' not found", weapon_name)))?;

        // Get configurations with dropoffs
        let configs = self.weapon_configs(weapon_name).await?;
        
        // Get ammo stats
        let ammo_stats = self.weapon_ammo_stats(weapon_name).await?;
        
        Ok(WeaponDetails {
            weapon,
            configurations: configs,
            ammo_stats,
        })
    }

    /// Get a reference to the database manager
    pub fn database_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}
