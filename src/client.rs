//! Stats client for querying weapon data

use crate::database::DatabaseManager;
use crate::models::{
    BestConfigInCategory, DamageAtRange, DatabaseConfig, Weapon, WeaponAmmoStatsWithNames,
    WeaponConfigWithDropoffs,
};
use crate::Result;
use futures::Stream;
use futures::TryStreamExt;
use tracing::{debug, info};

/// Client for querying weapon statistics with streaming API
///
/// All methods return streams for memory-efficient processing of large datasets.
/// Use futures combinators like `try_collect()`, `try_take()`, `try_filter()`
/// to work with the streams.
///
/// # Streaming API Usage Examples
///
/// ```rust
/// use futures::TryStreamExt;
///
/// // Example 1: Process weapons one by one with early termination
/// let mut stream = client.weapons_by_category("assault_rifle");
/// let mut count = 0;
/// while let Some(weapon) = stream.try_next().await? {
///     println!("Processing weapon: {}", weapon.weapon_name);
///     count += 1;
///     if count >= 5 {
///         break; // Stop early, remaining weapons not fetched
///     }
/// }
///
/// // Example 2: Filter and collect specific weapons
/// let filtered_weapons: Vec<Weapon> = client
///     .weapons_by_category("sniper_rifle")
///     .try_filter(|weapon| futures::future::ready(weapon.weapon_name.contains("SWS")))
///     .try_collect()
///     .await?;
///
/// // Example 3: Stream weapon configurations and process in batches
/// let configs: Vec<WeaponConfigWithDropoffs> = client
///     .weapon_configs("AK-24")
///     .try_take(10) // Only take first 10 configurations
///     .try_collect()
///     .await?;
///
/// // Example 4: Use the streaming weapon details API
/// let (weapon, config_stream, ammo_stream) = client
///     .weapon_details("M5A3")
///     .await?;
///
/// // Process configurations as they stream
/// let mut config_stream = std::pin::pin!(config_stream);
/// while let Some(config) = config_stream.try_next().await? {
///     println!("Config: {} - {}", config.barrel_name, config.ammo_type_name);
/// }
///
/// // Collect all results if needed
/// let all_weapons: Vec<Weapon> = client
///     .weapons_by_category("assault_rifle")
///     .try_collect()
///     .await?;
/// ```
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

    /// Get weapons by category
    pub fn weapons_by_category(
        &self,
        category_name: &str,
    ) -> impl Stream<Item = Result<Weapon>> + '_ {
        debug!(
            "Starting streaming query for weapons by category: {}",
            category_name
        );

        sqlx::query_as!(
            Weapon,
            r#"
            SELECT w.weapon_id, w.weapon_name, w.category_id
            FROM weapons w
            JOIN categories c ON w.category_id = c.category_id
            WHERE c.category_name = $1
            ORDER BY w.weapon_name
            "#,
            category_name
        )
        .fetch(self.db_manager.pool())
        .map_err(|e| e.into())
    }

    /// Get weapon configurations with damage dropoffs
    pub fn weapon_configs(
        &self,
        weapon_name: &str,
    ) -> impl Stream<Item = Result<WeaponConfigWithDropoffs>> + '_ {
        debug!(
            "Starting streaming query for weapon configurations: {}",
            weapon_name
        );

        sqlx::query_as!(
            WeaponConfigWithDropoffs,
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
        .fetch(self.db_manager.pool())
        .map_err(|e| e.into())
    }

    /// Get weapon ammo stats
    pub fn weapon_ammo_stats(
        &self,
        weapon_name: &str,
    ) -> impl Stream<Item = Result<WeaponAmmoStatsWithNames>> + '_ {
        debug!(
            "Starting streaming query for weapon ammo stats: {}",
            weapon_name
        );

        sqlx::query_as!(
            WeaponAmmoStatsWithNames,
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
        .fetch(self.db_manager.pool())
        .map_err(|e| e.into())
    }

    /// Get effective damage for weapon configurations at specific range
    pub fn damage_at_range(
        &self,
        weapon_name: &str,
        target_range: i16,
    ) -> impl Stream<Item = Result<DamageAtRange>> + '_ {
        debug!(
            "Starting streaming query for damage at range {} for weapon: {}",
            target_range, weapon_name
        );

        let weapon_name = weapon_name.to_string();
        sqlx::query_as!(
            DamageAtRange,
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
            weapon_name,
            target_range
        )
        .fetch(self.db_manager.pool())
        .map_err(|e| e.into())
    }

    /// Get top performing configurations in a category at specific range
    pub fn best_configs_in_category(
        &self,
        category_name: &str,
        target_range: i16,
        limit: i64,
    ) -> impl Stream<Item = Result<BestConfigInCategory>> + '_ {
        debug!(
            "Starting streaming query for best configs in category {} at range {} (limit: {})",
            category_name, target_range, limit
        );

        let category_name = category_name.to_string();
        sqlx::query_as!(
            BestConfigInCategory,
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
            category_name,
            target_range,
            limit as i64
        )
        .fetch(self.db_manager.pool())
        .map_err(|e| e.into())
    }

    /// Get complete weapon information including all configurations and stats with streaming
    /// This method returns the basic weapon info and streams for configurations and ammo stats
    pub async fn weapon_details(
        &self,
        weapon_name: &str,
    ) -> Result<(
        Weapon,
        impl Stream<Item = Result<WeaponConfigWithDropoffs>> + '_,
        impl Stream<Item = Result<WeaponAmmoStatsWithNames>> + '_,
    )> {
        debug!(
            "Starting streaming query for complete weapon details: {}",
            weapon_name
        );

        // Get basic weapon info first
        let weapon = sqlx::query_as!(
            Weapon,
            r#"
            SELECT w.weapon_id, w.weapon_name, w.category_id
            FROM weapons w
            WHERE w.weapon_name = $1
            "#,
            weapon_name
        )
        .fetch_optional(self.db_manager.pool())
        .await?
        .ok_or_else(|| {
            crate::StatsError::QueryFailed(format!("Weapon '{}' not found", weapon_name))
        })?;

        // Return weapon info and streams for configurations and ammo stats
        let config_stream = self.weapon_configs(weapon_name);
        let ammo_stream = self.weapon_ammo_stats(weapon_name);

        Ok((weapon, config_stream, ammo_stream))
    }

    /// Get a reference to the database manager
    pub fn database_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}
