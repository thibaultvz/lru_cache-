//! # Module Persistent
//!
//! Ce module permet de sauvegarder un cache LRU dans un fichier et de le recharger ultérieurement.
//!
//! ## Fonctionnalités
//! - Sauvegarde des paires clé-valeur dans un fichier texte (format : `clé=valeur`).
//! - Chargement d’un cache à partir d’un fichier existant.
//!
//! ## Exemple
//! ```rust
//! use lru_cache::persistent::PersistentCacheTrait;
//! use lru_cache::cache::{Cache, CacheTrait};
//!
//! let path = "my_cache.txt";
//! let mut cache = Cache::new_persistent(3, path);
//! cache.put("A".to_string(), "value_a".to_string());
//! cache.save(path);
//!
//! let mut new_cache = Cache::new_persistent(3, path);
//! assert_eq!(new_cache.get(&"A".to_string()), Some(&"value_a".to_string()));
//! ```

use super::cache::{Cache, CacheTrait};
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{Read, Write};

/// Trait pour ajouter la persistance au cache
pub trait PersistentCacheTrait<K, V> {
    /// Sauvegarde le cache dans un fichier
    fn save(&self, path: &str);

    /// Charge un cache depuis un fichier
    fn new_persistent(capacity: usize, path: &str) -> Cache<K, V>;
}

impl<K, V> PersistentCacheTrait<K, V> for Cache<K, V>
where
    K: Clone + Eq + Hash + ToString + std::str::FromStr,
    V: ToString + std::str::FromStr,
{
    fn save(&self, path: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap_or_else(|e| panic!("Erreur lors de l'ouverture du fichier : {}", e));

        for key in self.get_keys() {
            if let Some(value) = self.get_map().get(key) {
                writeln!(file, "{}={}", key.to_string(), value.to_string())
                    .unwrap_or_else(|e| panic!("Erreur lors de l'écriture : {}", e));
            }
        }
    }

    fn new_persistent(capacity: usize, path: &str) -> Cache<K, V> {
        let mut cache = Cache::new(capacity);
        if let Ok(mut file) = File::open(path) {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap_or_else(|e| {
                panic!("Erreur lors de la lecture du fichier : {}", e);
            });
            for line in content.lines() {
                if let Some((k, v)) = line.split_once('=') {
                    if let (Ok(key), Ok(value)) = (k.parse::<K>(), v.parse::<V>()) {
                        cache.put(key, value);
                    }
                }
            }
        }
        cache
    }
}
