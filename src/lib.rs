//! Battlefield 2042 Weapon Statistics Library
//! 
//! This library provides access to weapon statistics and damage calculations
//! for Battlefield 2042, with PostgreSQL backend storage and streaming query support.

pub mod client;
pub mod database;
pub mod models;
pub mod error;

// Re-export main types for easier usage
pub use client::StatsClient;
pub use error::{StatsError, Result};
pub use models::{Category, Weapon, WeaponConfig, DamageDropoff};
