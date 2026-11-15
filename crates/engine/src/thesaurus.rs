//! Review › Thesaurus: synonyms looked up offline (so it works in the browser build too). French
//! (in the French interface first), from the myThes French thesaurus of Dicollecte (`data/thesaurus_fr.txt`, LGPL), then a
//! small built-in English word list (`data/thesaurus.txt`).

use std::collections::HashMap;
use std::sync::OnceLock;

const DATA: &str = include_str!("../data/thesaurus.txt");
const DATA_FR: &str = include_str!("../data/thesaurus_fr.txt");

/// One sense of a word: its headword, its part of speech (French list: « nom », « verbe »…;
/// empty in the English one) and synonyms.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Sense {
    pub word: String,
    pub pos: String,
    pub synonyms: Vec<String>,
}

fn english_senses(head: &str) -> Vec<Sense> {
    DATA.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split_once('|'))
        .filter(|(h, _)| h.trim() == head)
        .map(|(h, s)| Sense {
            word: h.trim().to_string(),
            pos: String::new(),
            synonyms: s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
        })
        .collect()
}

/// The French list by headword (lower case): its lines `mot|nature|synonymes`, read once.
fn french_index() -> &'static HashMap<String, Vec<&'static str>> {
    static INDEX: OnceLock<HashMap<String, Vec<&'static str>>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut m: HashMap<String, Vec<&'static str>> = HashMap::new();
        for l in DATA_FR.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            if let Some((h, _)) = l.split_once('|') {
                m.entry(h.to_lowercase()).or_default().push(l);
            }
        }
        m
    })
}

fn french_senses(head: &str) -> Vec<Sense> {
    french_index()
        .get(head)
        .into_iter()
        .flatten()
        .filter_map(|l| {
            let mut parts = l.splitn(3, '|');
            let (word, pos, syn) = (parts.next()?, parts.next()?, parts.next()?);
            Some(Sense {
                word: word.to_string(),
                pos: pos.to_string(),
                synonyms: syn.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
            })
        })
        .collect()
}

/// Forms of a French word to try after the word itself: without an elided article (« l'idée »
/// → « idée »), singular (« projets », « chevaux »), masculine (« grande », « heureuse ») and the
/// infinitive of a few common verb endings (« présentons » → « présenter »).
fn french_forms(w: &str) -> Vec<String> {
    let mut out = Vec::new();
    let w = ["l'", "l’", "d'", "d’", "qu'", "qu’", "j'", "j’", "n'", "n’", "s'", "s’", "c'", "c’", "m'", "m’", "t'", "t’"]
        .iter()
        .find_map(|p| w.strip_prefix(p))
        .unwrap_or(w);
    out.push(w.to_string());
    let mut bases = vec![w.to_string()];
    if let Some(s) = w.strip_suffix("aux") {
        bases.push(format!("{s}al"));
    }
    for suf in ["s", "x"] {
        if let Some(s) = w.strip_suffix(suf) {
            bases.push(s.to_string());
        }
    }
    for b in bases.clone() {
        for (from, to) in [("euse", "eux"), ("trice", "teur"), ("ive", "if"), ("ienne", "ien"), ("onne", "on"), ("ère", "er"), ("e", "")] {
            if let Some(s) = b.strip_suffix(from)
                && s.chars().count() >= 2
            {
                bases.push(format!("{s}{to}"));
            }
        }
    }
    out.extend(bases);
    for (suf, inf) in [
        ("ons", "er"),
        ("ez", "er"),
        ("ent", "er"),
        ("ait", "er"),
        ("ais", "er"),
        ("aient", "er"),
        ("é", "er"),
        ("ée", "er"),
        ("és", "er"),
        ("ées", "er"),
        ("ant", "er"),
        ("era", "er"),
        ("issons", "ir"),
        ("issez", "ir"),
        ("issent", "ir"),
        ("it", "ir"),
        ("i", "ir"),
        ("is", "ir"),
    ] {
        if let Some(s) = w.strip_suffix(suf)
            && s.chars().count() >= 2
        {
            out.push(format!("{s}{inf}"));
        }
    }
    out
}

/// The senses for `word`, trying simple English endings when the word itself isn't listed
/// ("goals" → "goal", "improved" → "improve", "growing" → "grow"). Synonyms take the word's
/// capitalisation ("Goal" → "Aim", "GOAL" → "AIM").
pub fn lookup(word: &str) -> Vec<Sense> {
    let w: String = word.trim().trim_matches(|c: char| !c.is_alphanumeric() && c != '-').chars().take(64).collect();
    let lower = w.to_lowercase();
    if lower.is_empty() {
        return vec![];
    }
    let mut tries = vec![lower.clone()];
    for (suffix, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ied", "y"), ("ed", "e"), ("ed", ""), ("ing", "e"), ("ing", "")] {
        if let Some(stem) = lower.strip_suffix(suffix)
            && stem.chars().count() >= 2
        {
            tries.push(format!("{stem}{add}"));
        }
    }
    let french = french_forms(&lower).iter().map(|t| french_senses(t)).find(|v| !v.is_empty()).unwrap_or_default();
    let english = tries.iter().map(|t| english_senses(t)).find(|v| !v.is_empty()).unwrap_or_default();
    // The interface's language first (French when new presentations are in French).
    let found: Vec<Sense> =
        if crate::french_documents() { french.into_iter().chain(english).collect() } else { english.into_iter().chain(french).collect() };
    if found.is_empty() {
        return vec![];
    }
    let upper = w.chars().count() > 1 && w.chars().all(|c| !c.is_lowercase());
    let title = !upper && w.chars().next().is_some_and(char::is_uppercase);
    found
        .into_iter()
        .map(|s| Sense {
            word: s.word,
            pos: s.pos,
            synonyms: s
                .synonyms
                .into_iter()
                .map(|x| {
                    if upper {
                        x.to_uppercase()
                    } else if title {
                        let mut c = x.chars();
                        c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
                    } else {
                        x
                    }
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn senses_endings_and_case() {
        let g = lookup("goal");
        assert_eq!(english_senses("goal").len(), 1);
        assert!(g[0].synonyms.contains(&"objective".to_string()));
        assert_eq!(english_senses("change").len(), 2, "verb and noun senses");
        assert_eq!(lookup("Goals")[0].synonyms[0], "Aim");
        assert_eq!(lookup("GOAL")[0].synonyms[0], "AIM");
        assert_eq!(lookup("improved")[0].word, "improve");
        assert_eq!(lookup("growing")[0].word, "grow");
        assert_eq!(lookup("strategies")[0].word, "strategy");
        assert_eq!(lookup("“goal,”")[0].word, "goal", "punctuation around the word");
        assert!(lookup("").is_empty() && lookup("zzxq").is_empty() && lookup("   ").is_empty());
        assert!(lookup(&"a".repeat(10_000)).is_empty());
    }

    /// French words are found too (the French thesaurus), with their part of speech, from
    /// their plural, feminine, elided or conjugated forms.
    #[test]
    fn french_words_have_synonyms() {
        let m = lookup("maison");
        assert!(!m.is_empty() && m.iter().any(|s| s.synonyms.iter().any(|x| x == "foyer")), "{m:?}");
        assert!(m.iter().all(|s| !s.pos.is_empty()));
        assert_eq!(lookup("projets")[0].word, "projet");
        assert_eq!(lookup("l'idée")[0].word, "idée");
        assert_eq!(lookup("heureuses")[0].word, "heureuse");
        assert_eq!(lookup("chevaux")[0].word, "cheval");
        assert_eq!(lookup("présentons")[0].word, "présenter");
        assert_eq!(lookup("Maison")[0].synonyms[0].chars().next().map(char::is_uppercase), Some(true));
        // English words still come from the English list.
        assert!(lookup("goal").iter().any(|s| s.synonyms.contains(&"objective".to_string())));
    }

    #[test]
    fn data_is_well_formed() {
        for l in DATA.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let (h, s) = l.split_once('|').unwrap_or_else(|| panic!("no `|`: {l}"));
            assert_eq!(h, h.trim().to_lowercase(), "{l}");
            let syn: Vec<&str> = s.split(',').map(str::trim).collect();
            assert!(syn.len() >= 2 && syn.iter().all(|x| !x.is_empty() && *x != h), "{l}");
            let mut d = syn.clone();
            d.sort();
            d.dedup();
            assert_eq!(d.len(), syn.len(), "duplicate synonym: {l}");
        }
    }

    #[test]
    fn commands_look_up_and_replace_the_word_at_the_caret() {
        use serde_json::json;
        let mut s = crate::Session::with_new();
        let id = s.execute("insert.textBox", &json!({"rect": [50, 50, 400, 40]})).unwrap()["id"].as_u64().unwrap();
        s.execute("text.edit", &json!({"id": id})).unwrap();
        s.execute("text.insert", &json!({"text": "Our Goal is growth."})).unwrap();
        s.execute("text.edit", &json!({"id": id, "at": [0, 6]})).unwrap();
        let r = s.execute("review.thesaurus", &json!({})).unwrap();
        assert_eq!(r["word"], "Goal");
        assert_eq!(r["senses"][0]["synonyms"][0], "Aim");
        s.execute("review.replaceWord", &json!({"with": "Aim"})).unwrap();
        let text = |s: &crate::Session| {
            s.doc().unwrap().shapes().iter().find(|x| x.id.0 as u64 == id).and_then(|x| x.text.as_ref()).map(|t| t.paragraphs[0].text()).unwrap()
        };
        assert_eq!(text(&s), "Our Aim is growth.");
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(text(&s), "Our Goal is growth.");
        assert_eq!(s.execute("review.thesaurus", &json!({"word": "growth."})).unwrap()["senses"][0]["word"], "growth");
        assert!(s.execute("review.replaceWord", &json!({"with": ""})).is_err());
    }
}
