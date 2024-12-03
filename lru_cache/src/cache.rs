//! # Module Cache
//!
//! Ce module fournit une implémentation générique d'un cache **Least Recently Used (LRU)**.
//!
//! ## Fonctionnalités
//! - Ajout de paires clé-valeur avec gestion automatique de la capacité.
//! - Récupération des valeurs associées à une clé.
//! - Mise à jour automatique des priorités selon l'ordre d'utilisation récente.
//!
//! ## Exemple
//! ```rust
//! use lru_cache::cache::{Cache, CacheTrait};
//!
//! let mut cache = Cache::new(3);
//! cache.put("A", "value_a".to_string());
//! assert_eq!(cache.get(&"A"), Some(&"value_a".to_string()));
//! ```

/// Structure représentant un cache LRU générique.
///
/// Le cache conserve uniquement les `n` éléments les plus récemment utilisés.
/// Lorsque la capacité maximale est atteinte, le plus ancien élément est supprimé.

/// Structure pour un cache LRU générique
///
use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<K, V> {
    // Structure existante
    capacity: usize,
    map: HashMap<K, V>,
    keys: Vec<K>, // Ordre d'accès
}

/// Trait pour les opérations de base du cache
pub trait CacheTrait<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&mut self, key: &K) -> Option<&V>;
}

impl<K, V> Cache<K, V>
where
    K: Clone + Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    /// Retourne une référence aux clés du cache
    pub fn get_keys(&self) -> &Vec<K> {
        &self.keys
    }

    /// Retourne une référence à la carte du cache
    pub fn get_map(&self) -> &HashMap<K, V> {
        &self.map
    }
}

impl<K, V> CacheTrait<K, V> for Cache<K, V>
where
    K: Clone + Eq + Hash,
{
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.keys.retain(|k| k != &key);
        } else if self.map.len() >= self.capacity {
            let lru = self.keys.remove(0);
            self.map.remove(&lru);
        }
        self.keys.push(key.clone());
        self.map.insert(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.keys.retain(|k| k != key);
            self.keys.push(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3);
        cache.put("A", "value_a".to_string());
        cache.put("B", "value_b".to_string());
        cache.put("C", "value_c".to_string());
        cache.put("D", "value_d".to_string());
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"D"), Some(&"value_d".to_string()));
        assert_eq!(cache.get(&"B"), Some(&"value_b".to_string()));
        assert_eq!(cache.get(&"C"), Some(&"value_c".to_string()));
        cache.put("A", "value_a".to_string());
        cache.put("X", "value_x".to_string());
        assert_eq!(cache.get(&"B"), None);
    }
}
