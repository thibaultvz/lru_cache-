//! # Tests d'intégration pour le cache LRU
//!
//! Ces tests vérifient :
//! - La fonctionnalité de base du cache LRU (ajout, récupération, suppression).
//! - La persistance des données dans un fichier.
//! - Le comportement du cache avec un grand nombre d’éléments.

use lru_cache::cache::{Cache, CacheTrait};
use lru_cache::persistent::PersistentCacheTrait;
use std::fs;

#[test]
fn test_lru_scenario() {
    let mut cache = Cache::new(3); // Taille limite de 3

    // Étape 1 : Ajouter A
    assert_eq!(cache.get(&"A".to_string()), None); // "A" n'est pas dans le cache
    cache.put("A".to_string(), "value_a".to_string()); // Ajout de "A"

    // Étape 2 : Ajouter B
    assert_eq!(cache.get(&"B".to_string()), None); // "B" n'est pas dans le cache
    cache.put("B".to_string(), "value_b".to_string()); // Ajout de "B"

    // Étape 3 : Ajouter C
    assert_eq!(cache.get(&"C".to_string()), None); // "C" n'est pas dans le cache
    cache.put("C".to_string(), "value_c".to_string()); // Ajout de "C"

    // Étape 4 : Ajouter D, "A" est éjecté
    cache.put("D".to_string(), "value_d".to_string()); // Ajout de "D"
    assert_eq!(cache.get(&"A".to_string()), None); // "A" a été supprimé
    assert_eq!(cache.get(&"D".to_string()), Some(&"value_d".to_string())); // "D" est présent

    // Étape 5 : Accéder à "B", cela le rend récemment utilisé
    assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string())); // "B" est présent

    // Étape 6 : Vérifier que "A" n'est toujours pas présent
    assert_eq!(cache.get(&"A".to_string()), None); // "A" est toujours manquant

    // Étape 7 : Ajouter "E", "C" est éjecté
    cache.put("E".to_string(), "value_e".to_string()); // Ajout de "E"
    assert_eq!(cache.get(&"C".to_string()), None); // "C" a été supprimé
    assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string())); // "B" est encore là
    assert_eq!(cache.get(&"E".to_string()), Some(&"value_e".to_string())); // "E" est présent
}

#[test]
fn test_lru_with_persistence() {
    //! Teste la capacité du cache à sauvegarder et recharger ses données depuis un fichier.
    //!
    //! Étapes :
    //! 1. Ajoute des éléments au cache.
    //! 2. Sauvegarde les données dans un fichier.
    //! 3. Recharge le cache depuis le fichier et vérifie que les données sont intactes.
    //!
    let path = "test_cache_persistent.txt";

    // Nettoyer le fichier au début
    fs::remove_file(path).unwrap_or_else(|_| {
        println!("Fichier {} introuvable, aucun nettoyage requis.", path);
    });

    // Premier cycle : Sauvegarde initiale
    {
        let mut cache = Cache::new_persistent(3, path);

        // Ajouter les clés A, B, et C
        cache.put("A".to_string(), "value_a".to_string());
        cache.put("B".to_string(), "value_b".to_string());
        cache.put("C".to_string(), "value_c".to_string());
        cache.save(path); // Sauvegarde

        // Vérifiez que le fichier est créé
        assert!(fs::metadata(path).is_ok(), "Le fichier n'a pas été créé.");
    }

    // Deuxième cycle : Modification du cache
    {
        let mut cache = Cache::new_persistent(3, path);

        // Vérifier les clés chargées
        assert_eq!(cache.get(&"A".to_string()), Some(&"value_a".to_string()));
        assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string()));
        assert_eq!(cache.get(&"C".to_string()), Some(&"value_c".to_string()));

        // Ajouter une clé supplémentaire (éjecte "A")
        cache.put("D".to_string(), "value_d".to_string());
        cache.save(path); // Sauvegarde
    }

    // Troisième cycle : Vérification finale
    {
        let mut cache = Cache::new_persistent(3, path);

        // Vérifier les clés après éjection
        assert_eq!(cache.get(&"A".to_string()), None); // "A" a été supprimé
        assert_eq!(cache.get(&"B".to_string()), Some(&"value_b".to_string())); // "B" est présent
        assert_eq!(cache.get(&"C".to_string()), Some(&"value_c".to_string())); // "C" est présent
        assert_eq!(cache.get(&"D".to_string()), Some(&"value_d".to_string())); // "D" est présent
    }
}

#[test]
fn test_lru_large_operations() {
    let mut cache = Cache::new(5); // Taille limite de 5

    // Ajouter 10 éléments et vérifier que les 5 derniers sont conservés
    for i in 1..=10 {
        cache.put(i, i * 10); // Clé=i, Valeur=i*10
    }

    // Vérifier que les 5 premiers éléments ont été supprimés
    for i in 1..=5 {
        assert_eq!(cache.get(&i), None); // Éléments 1 à 5 supprimés
    }

    // Vérifier que les 5 derniers éléments sont présents
    for i in 6..=10 {
        assert_eq!(cache.get(&i), Some(&(i * 10))); // Éléments 6 à 10 présents
    }
}
