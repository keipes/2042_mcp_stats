//! Stats client for querying weapon data

use crate::database::DatabaseManager;
use crate::models::{
    BestConfigInCategory, DamageAtRange, DatabaseConfig, Weapon, WeaponAmmoStatsWithNames,
    WeaponConfigWithDropoffs,
};
use crate::Result;
use futures::Stream;
use futures::TryStreamExt;
use tokio::sync::OnceCell;
use tracing::{debug, info};

// Global initialization state to ensure database is only initialized once
static DB_INITIALIZATION: OnceCell<()> = OnceCell::const_new();

/// Ensure database exists
async fn ensure_database_exists(db_manager: &DatabaseManager, database: &str) -> Result<()> {
    // Check if the database exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)")
        .bind(database)
        .fetch_one(db_manager.pool())
        .await?;

    if !exists {
        // Create the database if it does not exist
        sqlx::query(&format!("CREATE DATABASE {}", database))
            .execute(db_manager.pool())
            .await?;
        info!("Database '{}' created successfully", database);
    } else {
        debug!("Database '{}' already exists", database);
    }
    
    Ok(())
}

/// Ensure database is initialized exactly once across all client instances
async fn ensure_database_initialized(db_manager: &DatabaseManager) -> Result<()> {
    DB_INITIALIZATION.get_or_try_init(|| async {
        // Check if weapons table exists, initialize if not
        let table_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM information_schema.tables WHERE table_name = 'weapons')"
        )
        .fetch_one(db_manager.pool())
        .await?;
            
        if !table_exists {
            info!("Database empty, initializing with embedded data");
            db_manager.create_schema().await?;
            db_manager.populate_from_embedded_data().await?;
            info!("Database initialization completed");
        } else {
            debug!("Database already has data");
        }
        Ok::<(), crate::StatsError>(())
    }).await.copied().map_err(Into::into)
}

pub struct StatsClient {
    db_manager: DatabaseManager,
}

impl StatsClient {
    /// Create a new stats client with custom configuration
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        // Administrative connection
        let postgres_db_url = "postgresql://postgres@localhost:5432/postgres";
        let pg_config = DatabaseConfig::new(postgres_db_url.to_string());
        let pg_manager = DatabaseManager::new(&pg_config).await?;
        pg_manager.test_connection().await?;
        let database_name = config.url().split('/').last().unwrap_or("2042_stats");
        ensure_database_exists(&pg_manager, database_name).await?;

        let db_manager = DatabaseManager::new(config).await?;
        db_manager.test_connection().await?;

        // Ensure database is initialized (only happens once globally)
        ensure_database_initialized(&db_manager).await?;

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
