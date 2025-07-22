use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::{Arc, RwLock},
};

use bimap::BiMap;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize)]
struct WeaponsData {
    categories: Vec<CategoryData>,
}

#[derive(Deserialize)]
struct CategoryData {
    name: String,
    weapons: Vec<WeaponData>,
}

#[derive(Deserialize, Clone)]
struct AmmoStat {
    mag_size: i16,
    tactical_reload: Option<Decimal>,
    empty_reload: Option<Decimal>,
    headshot_multiplier: Decimal,
    pellet_count: Option<i16>,
}

#[derive(Deserialize, Clone)]
struct WeaponData {
    name: String,
    stats: Vec<WeaponStats>,
    ammo_stats: Option<std::collections::HashMap<String, AmmoStat>>,
}

#[derive(Deserialize, Clone)]
struct WeaponStats {
    #[serde(rename = "barrelType")]
    barrel_type: String,
    #[serde(rename = "ammoType")]
    ammo_type: String,
    dropoffs: Vec<DamageRange>,
    #[serde(rename = "rpmSingle")]
    rpm_single: Option<i16>,
    #[serde(rename = "rpmBurst")]
    rpm_burst: Option<i16>,
    #[serde(rename = "rpmAuto")]
    rpm_auto: Option<i16>,
    velocity: Option<i16>,
}

#[derive(Deserialize, Clone)]
struct DamageRange {
    damage: Decimal,
    range: i16,
}

fn load_data() -> WeaponsData {
    let weapons_json = include_str!("../weapons.json");
    let weapons_data: WeaponsData = from_str(weapons_json).expect("Failed to parse weapons.json");
    for category in &weapons_data.categories {
        println!("Category: {}", category.name);
    }
    weapons_data
}

#[derive(Debug, Clone)]
struct WeaponConfig {
    weapon_name: String,
    barrel_type: String,
    ammo_type: String,
    config_stats: ConfigStats,
}
#[derive(Debug, Clone)]
struct ConfigStats {
    rpm_single: Option<i16>,
    rpm_burst: Option<i16>,
    rpm_auto: Option<i16>,
    velocity: Option<i16>,
}
struct RangeData {
    config: WeaponConfig,
    damage: Decimal,
    range: i16,
}
// impl Serialize for RangeData {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.ser
//         let mut state = serializer.serialize_struct("RangeData", 3)?;
//         state.serialize_field("weapon_name", &self.config.weapon_name)?;
//         state.serialize_field("barrel_type", &self.config.barrel_type)?;
//         state.serialize_field("ammo_type", &self.config.ammo_type)?;
//         state.serialize_field("damage", &self.damage)?;
//         state.end()
//     }
// }

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
struct WeaponConfigKey {
    weapon_name: String,
    barrel_type: String,
    ammo_type: String,
}

impl WeaponConfigKey {
    fn from_config(config: &WeaponConfig) -> Self {
        WeaponConfigKey {
            weapon_name: config.weapon_name.clone(),
            barrel_type: config.barrel_type.clone(),
            ammo_type: config.ammo_type.clone(),
        }
    }
}

struct RangeDamage {
    range: i16,
    damage: Decimal,
}
struct GatheredStats {
    ranges_with_damage_data: Vec<i16>,
    // configs: Vec<WeaponConfig>,
    config_damage_by_range: HashMap<WeaponConfigKey, Vec<RangeDamage>>,
    weapon_configs: HashMap<String, Vec<WeaponConfig>>,
    categories: HashMap<String, Vec<WeaponData>>,
}

impl GatheredStats {
    fn gather(weapons_data: &WeaponsData) -> Self {
        let mut ranges_with_damage_data: Vec<i16> = Vec::new();
        let mut config_damage_by_range: HashMap<WeaponConfigKey, Vec<RangeDamage>> = HashMap::new();
        let mut weapon_configs: HashMap<String, Vec<WeaponConfig>> = HashMap::new();
        let mut categories: HashMap<String, Vec<WeaponData>> = HashMap::new();
        for category in &weapons_data.categories {
            let mut weapon_list: Vec<WeaponData> = Vec::new();
            for weapon in &category.weapons {
                let mut configs: Vec<WeaponConfig> = Vec::new();
                for stat in &weapon.stats {
                    for dropoff in &stat.dropoffs {
                        if !ranges_with_damage_data.contains(&dropoff.range) {
                            ranges_with_damage_data.push(dropoff.range);
                        }
                        let key = WeaponConfigKey::from_config(&WeaponConfig {
                            weapon_name: weapon.name.clone(),
                            barrel_type: stat.barrel_type.clone(),
                            ammo_type: stat.ammo_type.clone(),
                            config_stats: ConfigStats {
                                rpm_single: stat.rpm_single,
                                rpm_burst: stat.rpm_burst,
                                rpm_auto: stat.rpm_auto,
                                velocity: stat.velocity,
                            },
                        });
                        let range_damage = RangeDamage {
                            range: dropoff.range,
                            damage: dropoff.damage,
                        };
                        config_damage_by_range
                            .entry(key.clone())
                            .or_default()
                            .push(range_damage);
                        configs.push(WeaponConfig {
                            weapon_name: weapon.name.clone(),
                            barrel_type: stat.barrel_type.clone(),
                            ammo_type: stat.ammo_type.clone(),
                            config_stats: ConfigStats {
                                rpm_single: stat.rpm_single,
                                rpm_burst: stat.rpm_burst,
                                rpm_auto: stat.rpm_auto,
                                velocity: stat.velocity,
                            },
                        });
                    }
                }
                weapon_configs.insert(weapon.name.clone(), configs);
                weapon_list.push(weapon.clone());
            }
            categories.insert(category.name.clone(), weapon_list);
        }
        return GatheredStats {
            ranges_with_damage_data,
            config_damage_by_range,
            weapon_configs,
            categories,
        };
    }

    fn config_damage_at_range(&self, key: WeaponConfigKey, range: i16) -> Decimal {
        let mut range_damage = Decimal::from(0);
        if let Some(damages) = self.config_damage_by_range.get(&key) {
            for RangeDamage { range: r, damage } in damages {
                if (r <= &range) {
                    range_damage = *damage;
                }
            }
        }
        range_damage
    }
}

impl ToString for RangeData {
    fn to_string(&self) -> String {
        format!(
            "{} - {} - {}: Damage {:.2}",
            self.config.weapon_name, self.config.barrel_type, self.config.ammo_type, self.damage
        )
    }
}

// 1.Range=0 2.BTK=4 3.TTK=300 4.Config=AK47 4.Barrel=Short 4.Ammo=Standard
// 1.Range=1 2.BTK=4 3.TTK=300 4.Config=AK47 4.Barrel=Short 4.Ammo=Standard
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct ByRangeSortable {
    range: i16,
    damage: Decimal,
    config_key: WeaponConfigKey,
}

// R1 Name Ammo Barrel
fn by_range(gathered_stats: &GatheredStats) -> String {
    let mut sorted_output_map: BTreeMap<ByRangeSortable, String> = BTreeMap::new();
    let ranges = gathered_stats.ranges_with_damage_data.clone();
    let categories = gathered_stats
        .categories
        .iter()
        .map(|(name, value)| (name, value))
        .collect::<Vec<_>>();
    let header_config_key = WeaponConfigKey {
        weapon_name: "!".into(),
        barrel_type: "!".into(),
        ammo_type: "!".into(),
    };
    eprintln!("\n\nGathered Stats:\n{:?}\n", "gathered_stats");
    let ranges_with_damage_data = gathered_stats.ranges_with_damage_data.clone();
    for range in &ranges {
        sorted_output_map.insert(
            ByRangeSortable {
                range: *range,
                damage: Decimal::from(99_999),
                config_key: header_config_key.clone(),
            },
            format!("1 Range={}\n", range),
        );
    }
    let name_condenser = NameCondenser::new();
    eprint!("Condensed Names:\n{}\n", name_condenser.to_reference());
    for range in &ranges_with_damage_data {
        let mut unique_damages: HashSet<Decimal> = HashSet::<Decimal>::new();

        for (category, weapons) in categories.clone() {
            println!("Category: {}", category);
            // if category != "Assault Rifles" {
            //     continue; // Skip categories other than Assault Rifles
            // }
            for weapon in weapons {
                let mut matching_configs: HashMap<Decimal, Vec<WeaponConfig>> = HashMap::new();
                println!("  Weapon: {}", weapon.name);
                let weapon_configs = gathered_stats.weapon_configs.get(&weapon.name);
                if let Some(weapon_configs) = weapon_configs {
                    for config in weapon_configs {
                        let key = WeaponConfigKey::from_config(config);
                        let merged_key = WeaponConfigKey {
                            weapon_name: config.weapon_name.clone(),
                            barrel_type: "".into(),
                            ammo_type: "".into(),
                        };
                        let damage = gathered_stats.config_damage_at_range(key.clone(), *range);
                        unique_damages.insert(damage);
                        matching_configs
                            .entry(damage)
                            .or_default()
                            .push(config.clone());
                    }
                }
                for (damage, configs) in matching_configs {
                    // condense barrel and ammo names while assigning to hashsets
                    let barrels_set: HashSet<String> = configs
                        .iter()
                        .map(|config| name_condenser.condense_barrel(&config.barrel_type))
                        .collect();
                    let barrels: String = barrels_set.into_iter().collect::<Vec<_>>().join("");
                    let ammos_set: HashSet<String> = configs
                        .iter()
                        .map(|config| name_condenser.condense_ammo(&config.ammo_type))
                        .collect();
                    let ammos: String = ammos_set.into_iter().collect::<Vec<_>>().join(" ");

                    let weapon_name = configs
                        .first()
                        .map_or("Unknown Weapon".to_string(), |c| c.weapon_name.clone());
                    let to_output: String =
                        format!("N={} B={} A={}\n", weapon_name, barrels, ammos);
                    sorted_output_map.insert(
                        ByRangeSortable {
                            range: *range,
                            damage: -damage,
                            config_key: WeaponConfigKey {
                                weapon_name: weapon_name.clone(),
                                barrel_type: barrels,
                                ammo_type: ammos,
                            },
                        },
                        to_output,
                    );
                }
            }
        }
        eprint!("Range {}: ", range);
        for damage in unique_damages {
            sorted_output_map.insert(
                ByRangeSortable {
                    range: *range,
                    damage: -damage,
                    config_key: header_config_key.clone(),
                },
                format!("2 Damage={}\n", damage),
            );
        }
    }

    let mut output = String::new();
    let reference_output = name_condenser.to_reference();
    output.push_str(reference_output.as_str());
    for (_, output_string) in sorted_output_map {
        output.push_str(&output_string);
    }

    output
}

const FORMAT_PREAMBLE: &str = r#"Output format:
Hierarchy Level
1 Range
2 Damage

"#;
// tests
#[cfg(test)]
mod tests {
    use rkyv::string;

    use super::*;

    #[test]
    fn test_load_data() {
        eprintln!("\n\nLoading data...\n");
        let data = load_data();
        let stats = GatheredStats::gather(&data);
        let output = by_range(&stats);
        //output bytes
        print!("{}", output);
        println!("{} bytes", output.len());
        // write output to file
        let mut my_output = String::new();
        my_output.push_str(FORMAT_PREAMBLE);
        my_output.push_str(&output);
        std::fs::write("output.txt", my_output).expect("Unable to write file");
    }

    #[test]
    fn test_string_expand() {
        let original = "Elastic Bumper Cars".to_string();
        assert_eq!(string_expand(original.clone(), 1), "E");
        assert_eq!(string_expand(original.clone(), 2), "EB");
        assert_eq!(string_expand(original.clone(), 3), "EBC");
        assert_eq!(string_expand(original.clone(), 4), "ElBC");
        assert_eq!(string_expand(original.clone(), 5), "ElBuC");
        assert_eq!(string_expand(original.clone(), 6), "ElBuCa");
        assert_eq!(string_expand(original.clone(), 7), "ElaBuCa");
        let type4 = "Type 4".to_string();
        assert_eq!(string_expand(type4.clone(), 2), "T4");
        assert_eq!(string_expand(type4.clone(), 3), "Ty4");
        let apba = "Armor Piercing (Burst/Auto)".to_string();
        assert_eq!(string_expand(apba.clone(), 4), "APBA");
        let buckshot = "#00 Buckshot".to_string();
        assert_eq!(string_expand(buckshot.clone(), 2), "0B");
        let close_combat_extended = "Close Combat Extended".to_string();
        assert_eq!(string_expand(close_combat_extended.clone(), 4), "ClCE");
        // NVKS=NVK-SHH
        let nvks = "NVK-SHH".to_string();
        assert_eq!(string_expand(nvks.clone(), 3), "NVK");
        assert_eq!(string_expand(nvks.clone(), 6), "NVKSHH");
    }
}
struct NameCondenser {
    ammo_condensed: Arc<RwLock<BiMap<String, String>>>,
    barrel_condensed: Arc<RwLock<BiMap<String, String>>>,
}

impl NameCondenser {
    fn new() -> Self {
        let ammo_condensed: Arc<RwLock<BiMap<String, String>>> =
            Arc::new(RwLock::new(BiMap::new()));
        ammo_condensed
            .write()
            .unwrap()
            .insert("Armor Piercing (Single)".to_string(), "APS".to_string());
        let barrel_condensed: Arc<RwLock<BiMap<String, String>>> =
            Arc::new(RwLock::new(BiMap::new()));
        barrel_condensed
            .write()
            .unwrap()
            .insert("6KU".to_string(), "6KU".to_string());

        barrel_condensed
            .write()
            .unwrap()
            .insert("PB".to_string(), "PB".to_string());
        Self {
            ammo_condensed,
            barrel_condensed,
        }
    }

    fn condense_ammo(&self, ammo: &str) -> String {
        get_condensed(self.ammo_condensed.clone(), ammo).unwrap()
        // condense_string(self.ammo_condensed.clone(), ammo, condense_name)
    }

    fn condense_barrel(&self, barrel: &str) -> String {
        get_condensed(self.barrel_condensed.clone(), barrel).unwrap()
    }

    fn barrel_verbose(&self, condensed: &str) -> Option<String> {
        get_verbose(self.barrel_condensed.clone(), condensed)
    }

    fn ammo_verbose(&self, condensed: &str) -> Option<String> {
        get_verbose(self.ammo_condensed.clone(), condensed)
    }

    fn to_reference(&self) -> String {
        let ammo_map = self.ammo_condensed.read().unwrap();
        let barrel_map = self.barrel_condensed.read().unwrap();
        let mut reference = String::new();
        reference.push_str("1 Ammo Condensed Name Key\n");
        for (verbose, condensed) in ammo_map.iter() {
            reference.push_str(&format!("{}={}\n", condensed, verbose));
        }
        reference.push_str("1 Barrel Condensed Name Key\n");
        for (verbose, condensed) in barrel_map.iter() {
            reference.push_str(&format!("{}={}\n", condensed, verbose));
        }
        reference
    }
}

fn get_condensed(bimap: Arc<RwLock<BiMap<String, String>>>, verbose: &str) -> Option<String> {
    let condensed = bimap.read().unwrap().get_by_left(verbose).cloned();
    if condensed.is_none() {
        let mut condensed = condense_name(verbose);
        let mut existing = Some("".to_string());
        while existing.is_some() {
            println!("Condensing: {}", condensed);
            condensed = string_expand(verbose.to_string(), condensed.len() + 1);
            existing = bimap.read().unwrap().get_by_right(&condensed).cloned();
        }
        bimap
            .write()
            .unwrap()
            .insert(verbose.to_string(), condensed.clone());
        Some(condensed)
    } else {
        condensed
    }
}

fn string_expand(original: String, target_length: usize) -> String {
    let original = original
        .replace("(", " ")
        .replace(")", " ")
        .replace("/", " ")
        .replace("#01", "1")
        .replace("#", "")
        .replace("-", "")
        .to_string();
    let words: Vec<&str> = original.split_whitespace().collect();
    let mut result = String::new();

    // Calculate how many full rounds of all words we can fit
    let full_rounds = target_length / words.len();
    let remaining = target_length % words.len();

    // For each word, add characters up to full_rounds + (1 if this word is in remaining)
    for (word_idx, word) in words.iter().enumerate() {
        let chars_from_this_word = full_rounds + if word_idx < remaining { 1 } else { 0 };

        for char_idx in 0..chars_from_this_word {
            if let Some(ch) = word.chars().nth(char_idx) {
                result.push(ch);
            } else {
                break;
            }
        }
    }
    result
}

fn get_verbose(bimap: Arc<RwLock<BiMap<String, String>>>, condensed: &str) -> Option<String> {
    bimap.read().unwrap().get_by_right(condensed).cloned()
}

fn condense_name(name: &str) -> String {
    let replace_strings = [("#0", ""), ("#1", ""), ("#2", "")];
    let name = replace_strings
        .iter()
        .fold(name.to_string(), |acc, (from, to)| acc.replace(from, to));
    let remove_chars = ['#', '(', ')'];
    let name = name
        .chars()
        .filter(|c| !remove_chars.contains(c))
        .collect::<String>();
    let split_chars = ['-', ' '];
    name.split(&split_chars[..])
        .map(|word| {
            word.chars()
                .next()
                .unwrap_or('?')
                .to_uppercase()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}

fn condense_string(
    bimap: Arc<RwLock<BiMap<String, String>>>,
    verbose: &str,
    condenser: fn(&str) -> String,
) -> String {
    if let Some(condensed) = bimap.read().unwrap().get_by_left(verbose) {
        condensed.to_string()
    } else {
        let condensed = condenser(verbose);
        bimap
            .write()
            .unwrap()
            .insert(verbose.to_string(), condensed.clone());
        condensed
    }
}
