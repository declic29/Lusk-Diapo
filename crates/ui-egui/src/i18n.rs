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
    // La correction automatique et les nouvelles présentations aussi.
    deckcraft_engine::cmd::text::set_french_autocorrect(on);
    deckcraft_engine::set_french_documents(on);
    deckcraft_engine::geom::set_decimal_comma(on);
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

/// Les messages du moteur qui contiennent des valeurs (un nom de fichier, un format) : le modèle
/// anglais, avec `{0}`, `{1}`… à la place des valeurs, et sa traduction.
const PATTERNS: &[(&str, &str)] = &[
    (
        "{0} was inserted, but its video can't be played ({1} isn't supported yet); only the audio plays",
        "{0} a été insérée, mais son image ne peut pas être lue ({1} n'est pas encore pris en charge) : seul le son sera joué",
    ),
    (
        "{0} was inserted, but it can't be played ({1} isn't supported yet)",
        "{0} a été inséré, mais ne peut pas être lu ({1} n'est pas encore pris en charge)",
    ),
    ("{0} was inserted, but it can't be played ({1})", "{0} a été inséré, mais ne peut pas être lu ({1})"),
    (
        "invalid parameters for `insert.picture`: {0} is not a picture",
        "{0} n'est pas une image que Lusk Diapo sait lire (JPEG, PNG, GIF, WebP, BMP, TIFF, SVG, EMF ou WMF)",
    ),
    (
        "invalid parameters for `insert.video`: {0} is not a supported video ({1})",
        "{0} n'est pas une vidéo que Lusk Diapo sait lire (MP4, MOV, M4V, WebM ou MKV)",
    ),
    ("invalid parameters for `insert.video`: {0} has no video", "{0} ne contient pas d'image vidéo"),
    (
        "invalid parameters for `insert.audio`: {0} is not a supported audio file ({1})",
        "{0} n'est pas un fichier audio que Lusk Diapo sait lire (MP3, M4A, AAC, WAV, FLAC, OGG, Opus, AIFF…)",
    ),
    ("invalid parameters for `insert.audio`: {0} has no audio", "{0} ne contient pas de son"),
    ("{0}: the picture can't be read", "{0} : l'image ne peut pas être lue"),
];

/// Les valeurs de `s` si `s` suit le modèle `template` (`{0}`, `{1}`… dans l'ordre).
fn match_template(template: &str, s: &str) -> Option<Vec<String>> {
    let mut parts = Vec::new();
    let mut rest = template;
    while let Some(i) = rest.find('{') {
        let j = rest[i..].find('}')? + i;
        parts.push(&rest[..i]);
        rest = &rest[j + 1..];
    }
    let mut text = s.strip_prefix(parts.first().copied().unwrap_or(rest))?;
    if parts.is_empty() {
        return text.is_empty().then(Vec::new);
    }
    let mut values = Vec::new();
    for lit in parts.iter().skip(1).copied().chain(std::iter::once(rest)) {
        if lit.is_empty() {
            values.push(text.to_string());
            text = "";
            continue;
        }
        // La dernière occurrence pour la fin du message, la première sinon.
        let k = if std::ptr::eq(lit, rest) { text.rfind(lit)? } else { text.find(lit)? };
        values.push(text[..k].to_string());
        text = &text[k + lit.len()..];
    }
    text.is_empty().then_some(values)
}

/// Un message (du moteur, d'une erreur) dans la langue de l'interface : la table d'abord, puis
/// les modèles avec des valeurs ([`PATTERNS`]).
pub fn tr_message(s: &str) -> String {
    if !french() {
        return s.to_string();
    }
    if let Some(t) = table().get(s) {
        return t.clone();
    }
    for (en, fr) in PATTERNS {
        if let Some(values) = match_template(en, s) {
            let mut out = (*fr).to_string();
            for (i, v) in values.iter().enumerate() {
                out = out.replace(&format!("{{{i}}}"), v);
            }
            return out;
        }
    }
    s.to_string()
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
    fn engine_messages_with_values_are_translated() {
        set_french_on_this_thread(Some(true));
        assert_eq!(
            tr_message("clip.mov was inserted, but its video can't be played (Apple ProRes isn't supported yet); only the audio plays"),
            "clip.mov a été insérée, mais son image ne peut pas être lue (Apple ProRes n'est pas encore pris en charge) : seul le son sera joué"
        );
        assert_eq!(
            tr_message("invalid parameters for `insert.picture`: photo (1).heic is not a picture"),
            "photo (1).heic n'est pas une image que Lusk Diapo sait lire (JPEG, PNG, GIF, WebP, BMP, TIFF, SVG, EMF ou WMF)"
        );
        assert_eq!(tr_message("Paste"), "Coller");
        assert_eq!(tr_message("something else"), "something else");
        set_french_on_this_thread(Some(false));
        assert_eq!(tr_message("Paste"), "Paste");
        set_french_on_this_thread(None);
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
