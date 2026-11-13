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

## À faire (dans l'ordre du cahier des charges)

2. Marque, suite : écran d'accueil animé (comme Lusk Tab), fond d'écran d'accueil à fournir.
3. Français : catalogue complet, région française, modèles et thèmes, palette de commandes.
4. Dictionnaires français, breton, anglais et synonymes (de Lusk Texte).
5. Pages de Fichier (Accueil avec le renard, Ouvrir / Enregistrer sous avec OneDrive et Google
   Drive, Mises à jour, Compte) et onglet Aide.
6. Relevé réseau et page Confidentialité.
7. Installateurs Windows, macOS, Linux et workflow « Publication » ; version 1.0.0.
