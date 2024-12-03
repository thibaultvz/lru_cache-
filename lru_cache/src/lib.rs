//! # Cache LRU
//!
//! Ce module implémente un cache **Least Recently Used (LRU)** générique en Rust, avec la possibilité de persister les données dans un fichier.
//!
//! ## Fonctionnalités principales
//! - **Stockage des données les plus récemment utilisées** : Lorsqu'un nouvel élément est inséré et que la capacité est atteinte, l'élément le moins récemment utilisé est supprimé.
//! - **Généricité** : Les clés et valeurs peuvent être de tout type qui implémente les traits nécessaires (par exemple, `Clone`, `Hash`, etc.).
//! - **Persistance** : Le cache peut être sauvegardé dans un fichier et rechargé ultérieurement.
//!
//! ## Exemple rapide
//! ```rust
//! use lru_cache::cache::{Cache, CacheTrait};
//!
//! let mut cache = Cache::new(3);
//! cache.put("A", "value_a".to_string());
//! cache.put("B", "value_b".to_string());
//! assert_eq!(cache.get(&"A"), Some(&"value_a".to_string()));
//! ```
//!
//! ## Modules
//! - [`cache`] : Implémente le cache LRU en mémoire.
//! - [`persistent`] : Ajoute des fonctionnalités de persistance au cache LRU.

pub mod cache;
pub mod persistent;
