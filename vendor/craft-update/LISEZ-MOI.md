# craft-update, copie Lusk Diapo

Ce dossier est une copie du module de mise à jour `craft-update` de
[storytold/craft-libs](https://github.com/storytold/craft-libs) (dossier `crates/update`, commit
`63196e66f3033a6623eb629af95a104d26986725`), sous licence MIT ou Apache 2.0 (voir `LICENSE-MIT`
et `LICENSE-APACHE`).

Le `Cargo.toml` racine de Lusk Diapo la met à la place de l'original (`[patch]`).

## Ce qui change par rapport à l'original

Une seule chose, dans `src/self_update.rs` (deux lignes marquées « Lusk Diapo ») : pour trouver
le fichier à installer dans une Release, le module cherche sous **tous les noms de
l'application** (`App::stems()` : l'identifiant `deckcraft` et les noms de `former_ids`, dont
`Lusk-Diapo`), et non plus seulement sous l'identifiant technique `deckcraft`.

Grâce à cela, les Releases ne contiennent que des fichiers `Lusk-Diapo-<version>-…` : plus besoin
des copies `deckcraft-<version>-…`, qui apparaissaient dans la liste des téléchargements.

Le `Cargo.toml` de la copie remplace les réglages du workspace de craft-libs par leurs valeurs, et
n'a pas les dépendances de test.

## Mettre à jour la copie

Pour passer à une version plus récente de craft-libs : recopier `crates/update/src` de ce commit,
refaire la modification des deux lignes, et mettre à jour le commit cité ici et dans le
`Cargo.toml` racine.
