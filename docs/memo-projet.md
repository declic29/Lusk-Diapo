# Lusk Diapo — mémo du projet

À relire au début de chaque session de travail.

- Lusk Diapo : le logiciel de présentation de l'association Lusk, en français. Fork de DeckCraft
  (storytold/deckcraft, MIT OR Apache-2.0), repris le 13 novembre 2026 à la version 0.18.1.
- Association : Lusk, contact@lusk.bzh, https://www.lusk.bzh.
- Code : https://github.com/declic29/Lusk-Diapo, branche `main`. Philippe enregistre et pousse
  depuis son PC ; Claude prépare, vérifie et dépose les fichiers.
- Méthode : le cahier des charges commun « Adapter une application Craft pour Lusk » (Lusk Texte
  et Lusk Tab servent de modèles).
- Couleur : orange Lusk **#F77C32** (au lieu de l'orange DeckCraft #F26B1D / #D95A1A).
- Noms techniques inchangés : programme `deckcraft`, crates `deckcraft-*`.

## Fait

- 13 novembre 2026 : orange Lusk #F77C32 dans l'interface (accent clair et sombre, onglets
  contextuels, repères, icônes, À propos) et dans le thème de présentation intégré.
- 13 novembre 2026 : ménage du dépôt (documents et workflows de la suite Craft, contributeurs,
  marques ArtCraft, captures DeckCraft ; Dependabot mensuel, corrections seulement ; ligne Lusk
  dans LICENSE-MIT). Marque : logo fourni par l'association (`assets/app-icon/lusk-diapo-logo.png`)
  et icône tirée des trois cartes avec le « D » (`lusk-diapo-icone.png`, `packaging/icons.sh`) ;
  identifiant `bzh.lusk.diapo` ; nom Lusk Diapo partout dans l'interface (le thème intégré
  s'appelle aussi « Lusk Diapo ») ; À propos avec le logo, en français ; liens vers lusk.bzh et le
  dépôt ; module de mise à jour réglé sur `declic29/Lusk-Diapo` (fichiers `Lusk-Diapo-…`,
  éditeur Lusk, orange #F77C32) ; README, NOTICE, ATTRIBUTION en français.
- 13 novembre 2026 (soir) : écran d'accueil (`crates/ui-egui/src/splash.rs`, repris de Lusk Tab) :
  fond fourni par l'association (`assets/splash/accueil-lusk-diapo.jpg`), logo en fondu, barre
  de chargement (interface, dictionnaires, présentation), mentions légales, fondu de sortie.
  Pas d'écran d'accueil sur macOS, avec `--no-splash`/`LUSK_NO_SPLASH`, le canal de contrôle,
  `--sample`, `--show` ou `DECKCRAFT_NO_PREFS`. Les textes se modifient en haut de `splash.rs`.
- 14 novembre 2026 : interface en français. Table `crates/ui-egui/src/i18n/fr.tsv` (environ
  2 000 textes, `anglais<TAB>français`, termes de PowerPoint en français) lue par
  `crates/ui-egui/src/i18n.rs` (`tr`, `trs`, `trf` pour les textes avec valeurs `{0}`). Couvre le
  ruban et ses onglets, les pages de Fichier, les volets, les boîtes de dialogue, les menus, la
  barre d'état, les noms des transitions, animations, formes, SmartArt, dispositions, thèmes et
  commandes, les fenêtres de mise à jour et les invites des espaces réservés vides. Au premier
  démarrage en français : centimètres, auteur « Utilisateur », papier A4 pour l'impression.
  `LUSK_LANG=en` remet l'anglais ; les tests restent en anglais. Pour relire :
  `LUSK_FR_SHOTS=<dossier> cargo test -p deckcraft-ui-egui --lib i18n_shots` écrit une image et
  la liste des textes de chaque écran.
- 14 novembre 2026 (après-midi) : tête de renard pour l'À propos de la barre Fichier et page
  Fichier › Mises à jour en bas de la barre (comme Lusk Texte et Lusk Tab) ; palette de commandes
  avec titre, croix, emplacement de chaque commande, sans les commandes réservées aux scripts, et
  un message dans la barre d'état après chaque commande (ou la raison de l'échec) ; huit
  conceptions Lusk dans Idées de conception (Aurore, Océan, Agrume, Ardoise, Prairie, Cartes,
  Minuit, Kraft), proposées même sur une diapositive vide ; correction automatique française
  (ordinaux 1er, 2e…) ; nouvelles présentations « Présentation1 » avec dispositions et masque en
  français ; virgule décimale. Balayage du ruban :
  `LUSK_SWEEP=<fichier> cargo test -p deckcraft-ui-egui --lib ribbon_sweep` clique chaque bouton
  de chaque onglet et note ceux qui ne font rien ou plantent (262 boutons, aucun plantage).
- 14 novembre 2026 (soir) : plus de remerciements (« Credits ») dans Fichier › À propos ni dans la
  boîte À propos : seules les informations sur le logiciel restent (`dialogs/credits.rs` et
  `crates/ui-egui/build.rs` supprimés). Fichier › Enregistrer sous propose OneDrive et Google
  Drive comme Lusk Texte et Lusk Tab (`cloud.rs` repris de Lusk Tab) : la boîte d'enregistrement
  s'ouvre dans le dossier du service ; une tuile grisée avec une aide quand le service n'est pas
  installé.
- 14 novembre 2026 (soir, suite) : plus de page Fichier › Informations (Inspecter la
  présentation, Gérer la présentation, propriétés) : le menu Fichier va d'Ouvrir à Enregistrer.
  Le Vérificateur d'accessibilité reste dans l'onglet Révision.
- 15 novembre 2026 : relecture de la traduction. Une soixantaine de textes construits avec
  `format!` ou passés directement aux boutons restaient en anglais (volet Commentaires, volet
  Sélection, miniatures, Rechercher, Exporter, album photo, mode Présentateur, minutage,
  messages d'erreur…) : ils passent maintenant par `tr` / `trf`, avec 88 nouvelles lignes dans
  `fr.tsv`. Reste en anglais : « Close window », le nom pour lecteur d'écran de la croix des
  fenêtres, qui vient de la bibliothèque craft-ui.
- 15 novembre 2026, **installateurs en français et workflow « Publication »** (repris de Lusk
  Tab, lui-même repris de Lusk Texte) :
  - `.github/workflows/release.yml` : une étiquette `vX.Y.Z` (égale à la version du Cargo.toml)
    fabrique les installateurs Windows, macOS et Linux et prépare une Release en brouillon, avec
    SHA256SUMS.txt, les copies aux noms stables (`Lusk-Diapo-Windows.msi`, `-Mac.dmg`,
    `-Linux.AppImage`, `-Linux.deb`) des liens du README et les copies au nom technique
    (`deckcraft-<version>-…`) pour la mise à jour. Plus de dépendance au workflow partagé de
    storytold (ni essai chaque nuit) ; `windows-arm64.yml` retiré ;
  - fichiers publiés `Lusk-Diapo-<version>-<système>-<architecture>.<ext>`, reconnus par la mise à
    jour (test `release_files_named_lusk_diapo_are_found`) ;
  - Windows : `packaging/windows/deckcraft.wxs` réécrit sur le modèle de Lusk Tab (installateur en
    français, page d'accueil Lusk `lusk-welcome.wxs`, textes `lusk-fr.wxl`, images à l'icône et au
    renard `images/`, « Pour moi seulement » / « Pour tous les utilisateurs », dossier
    `Programs\Lusk\Lusk Diapo`, raccourci du Bureau coché, « Lancer Lusk Diapo » à la fin, .pptx
    .potx .ppsx dans « Ouvrir avec » sans prendre la place de PowerPoint, .deckcraft ouvert par
    Lusk Diapo). Les raccourcis gardent l'icône de `deckcraft.exe` (#1114). Clé
    `Software\Learning Machines LLC\Lusk Diapo` pour que la mise à jour reconnaisse l'installation.
    Nouvel UpgradeCode `83DB4EEC-1D6E-42BB-AC8B-7C904D735F06` (ne pas le changer) ;
  - macOS : `Lusk Diapo.app` dans `Lusk-Diapo-<version>-macos-universal.dmg`, fond de fenêtre
    Lusk Diapo (`packaging/macos/dmg/background.py`), vérification stricte de la signature dans le
    DMG (#525) ;
  - Linux : AppImage, .deb et .rpm (paquet `lusk-diapo`), .tar.gz ; l'AppImage se met à jour depuis
    `declic29/Lusk-Diapo`.
  - Reste : la capture d'écran de `bzh.lusk.diapo.metainfo.xml.in` est encore celle de DeckCraft ;
    Flatpak et FreeBSD, hors publication, gardent les noms de DeckCraft.
- 15 novembre 2026, **vérification des images, sons et vidéos** (90 fichiers d'essai fabriqués
  avec ffmpeg et ImageMagick, insérés, décodés, rendus, enregistrés en PPTX et .deckcraft puis
  rouverts). Corrigé :
  - les photos prises en portrait avec un téléphone (orientation EXIF) s'affichaient couchées ;
    elles sont redressées partout (affichage, taille à l'insertion, album photo, compression,
    effets artistiques, suppression de l'arrière-plan) : `deckcraft_render::oriented_size` et
    `orient_image` ;
  - les vidéos filmées en portrait (rotation de la piste MP4/MOV) s'affichaient couchées ; elles
    sont redressées (`turn` dans `crates/media/src/video.rs`, taille inversée dans la sonde) ;
  - les images SVG s'affichaient comme une croix : elles sont dessinées avec resvg
    (`crates/render/src/svg.rs`, licence de tiny-skia dans `licenses/tiny-skia/`) ; EMF et WMF
    sont reconnus à l'insertion ; Insertion › Images propose aussi .svg, .emf, .wmf et .jfif ;
  - les vidéos H.264 10 bits, 4:2:2 ou 4:4:4 et l'ALAC dans un .caf restaient noires ou muettes
    sans rien dire : un message l'explique désormais ;
  - les messages d'insertion du moteur (format non pris en charge, vidéo dont seul le son est lu)
    sont traduits (`i18n::tr_message`).
  Restent non lus (avec un message) : vidéos MPEG-4 Part 2/DivX, Motion JPEG, ProRes, VP8,
  H.263 (.3gp), WMV/WMA ; refusés : .avi, .mpg, .ac3, .ico, .avif, .heic.

## À faire (dans l'ordre du cahier des charges)

4. Dictionnaires français, breton, anglais et synonymes (de Lusk Texte).
5. Pages de Fichier (Accueil avec le renard, Ouvrir avec OneDrive et Google Drive, Compte) et onglet Aide.
6. Relevé réseau et page Confidentialité.
7. Version 1.0.0 : passer `version` à 1.0.0 dans le Cargo.toml, pousser l'étiquette `v1.0.0`,
   tester les installateurs du brouillon sur Windows, Mac et Linux, puis publier la Release.
