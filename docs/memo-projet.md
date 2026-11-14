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

## À faire (dans l'ordre du cahier des charges)

3. Français, suite : correction automatique et contenu des nouvelles présentations (masques)
   en français, séparateur décimal.
4. Dictionnaires français, breton, anglais et synonymes (de Lusk Texte).
5. Pages de Fichier (Accueil avec le renard, Ouvrir / Enregistrer sous avec OneDrive et Google
   Drive, Mises à jour, Compte) et onglet Aide.
6. Relevé réseau et page Confidentialité.
7. Installateurs Windows, macOS, Linux et workflow « Publication » ; version 1.0.0.
