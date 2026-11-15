<p align="center">
  <img src="assets/app-icon/lusk-diapo-logo.png" width="320" alt="Logo de Lusk Diapo : trois cartes empilées rose, violette et turquoise, et le mot Diapo en dégradé">
</p>

<h1 align="center">Lusk Diapo</h1>

<p align="center">
  <b>Vos présentations vous appartiennent.</b><br>
  Un logiciel de présentation libre et gratuit, porté par l'association Lusk (loi 1901).
</p>

<p align="center">
  <a href="https://www.lusk.bzh"><b>lusk.bzh</b></a> ·
  <a href="https://github.com/declic29/Lusk-Diapo/releases/latest">Télécharger</a> ·
  <a href="https://github.com/declic29/Lusk-Diapo/issues">Signaler un problème</a>
</p>

> Lusk Diapo est en cours de préparation : la première version publique sera la 1.0.0. Les liens
> de téléchargement ci-dessous fonctionneront dès sa publication.

## Lusk Diapo en bref

Lusk Diapo crée et projette des diaporamas, sous Windows, macOS et Linux. Il fonctionne
entièrement sur votre ordinateur : pas de compte, pas de publicité, pas de mesure d'audience, et
vos présentations ne sont jamais envoyées nulle part.

- **Vos fichiers, dans les formats de tout le monde** : PowerPoint (.pptx, .potx, .ppsx), le
  format propre de Lusk Diapo (.deckcraft), plan en texte, Markdown et RTF ; export PDF, images,
  GIF animé et vidéo. Les anciens fichiers PowerPoint 97-2003 (.ppt) s'ouvrent une fois convertis
  en .pptx : Lusk Diapo explique comment le faire avec LibreOffice ou PowerPoint.
- **Un logiciel de présentation complet** : thèmes, dispositions et idées de conception, formes,
  images (y compris SVG, EMF et WMF), tableaux, graphiques, SmartArt, équations, album photo,
  transitions et animations, vidéo et son, commentaires, vérificateur d'accessibilité, mode
  Présentateur avec notes et minuteur.
- **En français d'abord**, avec un dictionnaire des synonymes français (Dicollecte) et les dates
  à la française.

## Télécharger et installer

Téléchargez la dernière version pour votre système :

| Système | Téléchargement |
|---|---|
| Windows (64 bits) | [**Lusk-Diapo-Windows.msi**](https://github.com/declic29/Lusk-Diapo/releases/latest/download/Lusk-Diapo-Windows.msi) |
| macOS (Apple et Intel) | [**Lusk-Diapo-Mac.dmg**](https://github.com/declic29/Lusk-Diapo/releases/latest/download/Lusk-Diapo-Mac.dmg) |
| Linux (toutes distributions) | [**Lusk-Diapo-Linux.AppImage**](https://github.com/declic29/Lusk-Diapo/releases/latest/download/Lusk-Diapo-Linux.AppImage) |
| Linux (Debian, Ubuntu, Mint) | [**Lusk-Diapo-Linux.deb**](https://github.com/declic29/Lusk-Diapo/releases/latest/download/Lusk-Diapo-Linux.deb) |

Les autres formats (version portable pour Windows sans installation, `.rpm` pour Fedora et
openSUSE, `.tar.gz`) et les versions précédentes sont sur la page
[Releases](https://github.com/declic29/Lusk-Diapo/releases). `SHA256SUMS.txt` donne l'empreinte de
chaque fichier, pour vérifier un téléchargement.

Une fois Lusk Diapo installé, **Fichier › Mises à jour** cherche et installe les nouvelles
versions.

**Windows.** L'installeur, en français, propose « Pour moi seulement » (pour votre compte, sans
droits d'administrateur, dans `%LOCALAPPDATA%\Programs\Lusk\Lusk Diapo`) ou « Pour tous les
utilisateurs de cet ordinateur » (dans `C:\Program Files\Lusk\Lusk Diapo`, avec l'accord d'un
administrateur), puis un raccourci sur le Bureau (case cochée au départ).

**macOS.** Ouvrez le `.dmg` et glissez Lusk Diapo dans Applications. La première fois, faites un
clic droit sur l'application puis **Ouvrir**.

**Linux.** Rendez l'AppImage exécutable (`chmod +x`) puis lancez-la, ou installez le paquet de
votre distribution.

### Si Lusk Diapo ne démarre pas

- **Windows, la fenêtre n'apparaît pas** : certains pilotes graphiques posent problème. Dans
  PowerShell, depuis le dossier du programme, essayez un autre moteur d'affichage :
  ```powershell
  $env:WGPU_BACKEND = "vulkan"
  & .\deckcraft.exe
  ```
  `WGPU_BACKEND` accepte aussi `dx12` et `gl`.
- Le **journal** se trouve dans `%APPDATA%\Lusk Diapo\logs\` (Windows),
  `~/Library/Application Support/Lusk Diapo/logs/` (macOS) ou `~/.config/deckcraft/logs/` (Linux).
  Joignez-le à un signalement de problème.

## Compiler Lusk Diapo

Il faut [Rust](https://rustup.rs) (version stable).

```sh
git clone https://github.com/declic29/Lusk-Diapo
cd Lusk-Diapo
cargo run --release -p deckcraft                     # l'application
cargo run --release -p deckcraft -- --sample         # avec la présentation d'exemple
cargo run --release -p deckcraft -- diaporama.pptx   # ouvrir une présentation
```

L'outil en ligne de commande convertit sans ouvrir l'application :

```sh
cargo run --release -p deckcraft-cli -- render --sample --all --scale 1 out/   # les diapositives en images
```

Avant chaque envoi, `cargo xtask ci` vérifie la mise en forme du code, les tests et les
attributions des images et polices.

## Licence et origine

Lusk Diapo est un logiciel libre, sous licence [MIT](LICENSE-MIT) ou [Apache 2.0](LICENSE-APACHE),
au choix. © 2026 Lusk, association loi 1901.

Lusk Diapo est dérivé de DeckCraft, Copyright (c) 2026 ArtCraft Team and the DeckCraft
contributors, publié sous les mêmes licences. Les mentions obligatoires sont dans [NOTICE](NOTICE),
et l'auteur, la source et la licence de chaque image et police sont dans
[ATTRIBUTION.md](ATTRIBUTION.md).

Le logo de Lusk Diapo appartient à l'association Lusk. Aucune icône ni image ne provient de
Microsoft Office ou d'un autre logiciel commercial.
