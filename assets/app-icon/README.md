# Icône de Lusk Diapo

L'icône reprend le logo de Lusk Diapo fourni par l'association Lusk : trois cartes empilées (rose,
violette, turquoise) et le « D » en dégradé jaune-rouge. Le logo complet (`lusk-diapo-logo.png`,
fond transparent) sert à l'À propos et à l'écran d'accueil ; les trois cartes avec le « D »
(`lusk-diapo-icone.png`) sont centrées sur un carré transparent et toutes les tailles en sont
tirées.

Pour changer d'icône : remplacer `lusk-diapo-icone.png`, lancer `packaging/icons.sh`, puis mettre à
jour les lignes de `ATTRIBUTION.md` si la provenance change.

| Fichier | Usage |
|---|---|
| `lusk-diapo-logo.png` | Logo complet (cartes et « Diapo ») |
| `lusk-diapo-icone.png` | Cartes et « D », image source de l'icône |
| `deckcraft-1024.png` | Icône centrée sur un carré de 1024 px |
| `deckcraft.icns` | Icône de l'application macOS (grille d'Apple, marge transparente) |
| `deckcraft.ico` | Icône du programme et de l'installateur Windows (16 à 256 px) |
| `hicolor/<taille>/apps/bzh.lusk.diapo.png` | Thème d'icônes Linux, 16 à 512 px |

L'identifiant de l'application est `bzh.lusk.diapo` : noms des icônes Linux, identifiant Wayland et
`packaging/linux/bzh.lusk.diapo.desktop`. Les fichiers gardent le nom interne `deckcraft`, comme le
programme lui-même.
