//! Lusk Diapo en français : la table des textes de l'interface, de l'anglais d'origine (DeckCraft)
//! vers le français.
//!
//! Les textes du code restent en anglais ; chaque endroit qui affiche un texte de l'interface le
//! passe à [`tr`], qui rend sa traduction quand l'interface est en français ([`set_french`],
//! réglé au démarrage de l'application de bureau) et le texte d'origine sinon (les tests, qui
//! cherchent les boutons par leur nom anglais).
//!
//! La table est le fichier `i18n/fr.tsv`, une ligne par texte : `anglais<TAB>français`. `\n` y
//! note un retour à la ligne (les boutons du ruban sur deux lignes), `\t` une tabulation. Un texte
//! absent de la table s'affiche en anglais : pour corriger ou ajouter une traduction, il suffit de
//! modifier ce fichier puis de recompiler. Les termes suivent ceux de PowerPoint en français
//! (Accueil, Insertion, Création, Diaporama, Révision, Affichage…).

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

static FRENCH: AtomicBool = AtomicBool::new(false);

/// Met l'interface en français (`true`) ou en anglais.
pub fn set_french(on: bool) {
    FRENCH.store(on, Ordering::Relaxed);
    // Les invites des espaces réservés vides (« Cliquez pour ajouter un titre ») suivent.
    deckcraft_model::set_french_prompts(on);
}

thread_local! {
    /// Pour les tests : la langue de ce fil seulement, sans toucher aux autres tests.
    static THIS_THREAD: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

/// Met ce fil seul en français (`Some(true)`), en anglais, ou le rend au réglage général (`None`).
pub fn set_french_on_this_thread(on: Option<bool>) {
    THIS_THREAD.with(|c| c.set(on));
}

/// L'interface est-elle en français ?
pub fn french() -> bool {
    THIS_THREAD.with(std::cell::Cell::get).unwrap_or_else(|| FRENCH.load(Ordering::Relaxed))
}

/// La table française, lue une fois.
fn table() -> &'static HashMap<String, String> {
    static T: OnceLock<HashMap<String, String>> = OnceLock::new();
    T.get_or_init(|| parse(include_str!("i18n/fr.tsv")))
}

/// Lit une table `anglais<TAB>français` ; les lignes vides et celles qui commencent par `#` sont
/// ignorées.
fn parse(tsv: &str) -> HashMap<String, String> {
    let unescape = |s: &str| s.replace("\\n", "\n").replace("\\t", "\t");
    tsv.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .filter_map(|l| l.split_once('\t'))
        .filter(|(en, fr)| !en.is_empty() && !fr.trim().is_empty())
        .map(|(en, fr)| (unescape(en), unescape(fr.trim_end_matches('\r'))))
        .collect()
}

/// Le texte `s` dans la langue de l'interface : sa traduction française quand l'interface est en
/// français et que la table la connaît, sinon `s` lui-même.
pub fn tr(s: &str) -> &str {
    if !french() {
        return s;
    }
    table().get(s).map_or(s, String::as_str)
}

/// Comme [`tr`], pour un texte construit (un nom venu du moteur, d'une liste…) : une copie.
pub fn trs(s: impl AsRef<str>) -> String {
    tr(s.as_ref()).to_string()
}

/// Un texte avec des valeurs : `{0}`, `{1}`… de la traduction de `s` remplacés par `args`.
pub fn trf(s: &str, args: &[&dyn std::fmt::Display]) -> String {
    let mut out = tr(s).to_string();
    for (i, a) in args.iter().enumerate() {
        out = out.replace(&format!("{{{i}}}"), &a.to_string());
    }
    out
}

/// Nombre de textes de la table (pour les tests).
pub fn len() -> usize {
    table().len()
}

/// La table entière (pour les tests).
pub fn entries() -> impl Iterator<Item = (&'static str, &'static str)> {
    table().iter().map(|(a, b)| (a.as_str(), b.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_reads_escapes_and_skips_comments() {
        let t = parse("# commentaire\nInk to\\nShape\tEncre en\\nforme\n\nPaste\tColler\r\nEmpty\t \n");
        assert_eq!(t.get("Ink to\nShape").map(String::as_str), Some("Encre en\nforme"));
        assert_eq!(t.get("Paste").map(String::as_str), Some("Coller"));
        assert!(!t.contains_key("Empty"));
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn every_line_of_the_french_table_is_well_formed() {
        let src = include_str!("i18n/fr.tsv");
        let mut seen = std::collections::HashSet::new();
        for (n, l) in src.lines().enumerate() {
            if l.trim().is_empty() || l.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = l.split('\t').collect();
            assert_eq!(parts.len(), 2, "ligne {} : « anglais<TAB>français » attendu : {l:?}", n + 1);
            assert!(seen.insert(parts[0]), "ligne {} : « {} » est déjà traduit plus haut", n + 1, parts[0]);
            // Les valeurs `{0}`, `{name}`… de l'anglais se retrouvent dans le français.
            for w in parts[0].split('{').skip(1).filter_map(|s| s.split_once('}')).map(|(k, _)| k) {
                assert!(parts[1].contains(&format!("{{{w}}}")), "ligne {} : {{{w}}} manque dans « {} »", n + 1, parts[1]);
            }
        }
    }

    #[test]
    fn english_until_french_is_chosen() {
        set_french_on_this_thread(Some(false));
        assert_eq!(tr("Paste"), "Paste");
        assert_eq!(trf("Slide {0} of {1}", &[&1, &5]), "Slide 1 of 5");
        set_french_on_this_thread(Some(true));
        assert_eq!(tr("Paste"), "Coller");
        assert_eq!(tr("Ink to\nShape"), "Encre en\nforme");
        assert_eq!(trf("Slide {0} of {1}", &[&1, &5]), "Diapositive 1 sur 5");
        // Un texte absent de la table reste tel quel.
        assert_eq!(tr("Northern Lights"), "Northern Lights");
        set_french_on_this_thread(None);
    }

    /// The ribbon's tabs and the File menu all have their French name.
    #[test]
    fn the_tabs_and_file_pages_are_translated() {
        for en in [
            "File",
            "Home",
            "Insert",
            "Draw",
            "Design",
            "Transitions",
            "Animations",
            "Slide Show",
            "Record",
            "Review",
            "View",
            "New",
            "Open",
            "Info",
            "Save As",
            "Print",
            "Export",
            "About",
            "Options",
        ] {
            assert!(table().contains_key(en), "{en} has no French name");
        }
        assert!(len() > 1500, "the table has {} texts", len());
    }
}
