//! Test utilities and mocks for the streaming API

use crate::models::{
    BestConfigInCategory, DamageAtRange, Weapon, WeaponAmmoStatsWithNames, WeaponConfigWithDropoffs,
};
use crate::Result;
use futures::Stream;
use std::pin::Pin;

/// Mock data for testing
pub struct MockData;

impl MockData {
    pub fn sample_weapons() -> Vec<Weapon> {
        vec![
            Weapon {
                weapon_id: 1,
                weapon_name: "AK-24".to_string(),
                category_id: 1,
            },
            Weapon {
                weapon_id: 2,
                weapon_name: "M5A3".to_string(),
                category_id: 1,
            },
            Weapon {
                weapon_id: 3,
                weapon_name: "SWS-10".to_string(),
                category_id: 2,
            },
        ]
    }

    pub fn sample_configs() -> Vec<WeaponConfigWithDropoffs> {
        vec![
            WeaponConfigWithDropoffs {
                config_id: 1,
                weapon_name: "AK-24".to_string(),
                barrel_name: "Standard Issue".to_string(),
                ammo_type_name: "Standard".to_string(),
                velocity: 715,
                rpm_single: Some(600),
                rpm_burst: Some(850),
                rpm_auto: Some(600),
                range: 50,
                damage: rust_decimal::Decimal::new(30, 0),
            },
            WeaponConfigWithDropoffs {
                config_id: 1,
                weapon_name: "AK-24".to_string(),
                barrel_name: "Standard Issue".to_string(),
                ammo_type_name: "Standard".to_string(),
                velocity: 715,
                rpm_single: Some(600),
                rpm_burst: Some(850),
                rpm_auto: Some(600),
                range: 100,
                damage: rust_decimal::Decimal::new(25, 0),
            },
        ]
    }
}

/// Mock streaming client for testing
pub struct MockStatsClient {
    weapons: Vec<Weapon>,
    configs: Vec<WeaponConfigWithDropoffs>,
}

impl MockStatsClient {
    pub fn new() -> Self {
        Self {
            weapons: MockData::sample_weapons(),
            configs: MockData::sample_configs(),
        }
    }

    pub fn weapons_by_category(&self, _category: &str) -> impl Stream<Item = Result<Weapon>> + '_ {
        futures::stream::iter(self.weapons.clone().into_iter().map(Ok))
    }

    pub fn weapon_configs(
        &self,
        _weapon_name: &str,
    ) -> impl Stream<Item = Result<WeaponConfigWithDropoffs>> + '_ {
        futures::stream::iter(self.configs.clone().into_iter().map(Ok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;

    #[tokio::test]
    async fn test_mock_weapons_stream() {
        let client = MockStatsClient::new();
        let weapons: Vec<Weapon> = client
            .weapons_by_category("assault_rifle")
            .try_collect()
            .await
            .unwrap();

        assert_eq!(weapons.len(), 3);
        assert_eq!(weapons[0].weapon_name, "AK-24");
    }

    #[tokio::test]
    async fn test_mock_configs_stream() {
        let client = MockStatsClient::new();
        let configs: Vec<WeaponConfigWithDropoffs> =
            client.weapon_configs("AK-24").try_collect().await.unwrap();

        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].weapon_name, "AK-24");
    }

    #[tokio::test]
    async fn test_stream_early_termination() {
        let client = MockStatsClient::new();
        let mut stream = client.weapons_by_category("assault_rifle");

        let mut count = 0;
        while let Some(_weapon) = stream.try_next().await.unwrap() {
            count += 1;
            if count >= 2 {
                break; // Test early termination
            }
        }

        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_stream_filtering() {
        let client = MockStatsClient::new();
        let ak_weapons: Vec<Weapon> = client
            .weapons_by_category("assault_rifle")
            .try_filter(|weapon| futures::future::ready(weapon.weapon_name.contains("AK")))
            .try_collect()
            .await
            .unwrap();

        assert_eq!(ak_weapons.len(), 1);
        assert_eq!(ak_weapons[0].weapon_name, "AK-24");
    }
}
