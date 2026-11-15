//! Lusk Diapo : images de l'interface en français, pour relire la traduction.
//!
//! `LUSK_FR_SHOTS=<dossier> cargo test -p deckcraft-ui-egui --lib i18n_shots` écrit une image par
//! onglet du ruban, par page de Fichier et par volet dans ce dossier. Sans la variable, ne fait
//! rien.

use deckcraft_engine::Session;
use serde_json::json;

use crate::{Services, SlideApp};

fn shoot(dir: &std::path::Path, name: &str, setup: impl FnOnce(&mut SlideApp)) {
    crate::i18n::set_french_on_this_thread(Some(true));
    let mut app = SlideApp::new(Session::new(), Services::default());
    let _ = deckcraft_engine::sample::open_sample(&mut app.session);
    // Comme l'application de bureau : la page Enregistrer sous montre OneDrive et Google Drive.
    app.services.pick_save_in = Some(Box::new(|_: &str, _: &std::path::Path| None));
    setup(&mut app);
    let mut harness = egui_kittest::Harness::builder()
        .with_size(egui::vec2(1500.0, 900.0))
        .with_pixels_per_point(1.0)
        .with_accessibility_check(false)
        .build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
    harness.run_steps(6);
    if let Ok(img) = harness.render() {
        let _ = img.save(dir.join(format!("{name}.png")));
    }
    // Les textes lus par les lecteurs d'écran, pour repérer ce qui reste en anglais.
    use egui_kittest::kittest::NodeT;
    let mut texts: Vec<String> = harness
        .root()
        .children_recursive()
        .filter_map(|n| {
            let a = n.accesskit_node();
            let l = a.label().unwrap_or_default();
            let v = a.value().unwrap_or_default();
            (!l.is_empty() || !v.is_empty()).then(|| if v.is_empty() { l } else { format!("{l} = {v}") })
        })
        .collect();
    texts.dedup();
    let _ = std::fs::write(dir.join(format!("{name}.txt")), texts.join("\n"));
    crate::i18n::set_french_on_this_thread(None);
}

#[test]
fn i18n_shots() {
    let Some(dir) = std::env::var_os("LUSK_FR_SHOTS") else { return };
    let dir = std::path::PathBuf::from(dir);
    let only = std::env::var("LUSK_FR_ONLY").unwrap_or_default();
    let want = |n: &str| only.is_empty() || only.split(',').any(|o| n.contains(o));
    for tab in ["Home", "Insert", "Draw", "Design", "Transitions", "Animations", "Slide Show", "Record", "Review", "View"] {
        let name = format!("ruban-{}", tab.to_lowercase().replace(' ', "-"));
        if want(&name) {
            shoot(&dir, &name, |app| app.ui.tab = tab.into());
        }
    }
    // Les onglets contextuels : une forme, puis un tableau sélectionnés.
    if want("ruban-forme") {
        shoot(&dir, "ruban-forme", |app| {
            let r = app.session.execute("shape.insert", &json!({"preset": "rect", "rect": [80.0, 80.0, 100.0, 60.0]}));
            if let Some(id) = r.ok().and_then(|v| v.get("id").and_then(serde_json::Value::as_u64)) {
                let _ = app.session.execute("edit.select", &json!({"ids": [id]}));
            }
            app.ui.tab = "Shape Format".into();
        });
    }
    if want("volet-forme") {
        shoot(&dir, "volet-forme", |app| {
            let r = app.session.execute("shape.insert", &json!({"preset": "rect", "rect": [80.0, 80.0, 100.0, 60.0]}));
            if let Some(id) = r.ok().and_then(|v| v.get("id").and_then(serde_json::Value::as_u64)) {
                let _ = app.session.execute("edit.select", &json!({"ids": [id]}));
            }
            app.ui.pane = Some("format".into());
        });
    }
    if want("volet-idees-vide") {
        shoot(&dir, "volet-idees-vide", |app| {
            let _ = app.session.execute("file.new", &json!({}));
            app.ui.pane = Some("designer".into());
        });
    }
    if want("idee-appliquee") {
        shoot(&dir, "idee-appliquee", |app| {
            let _ = app.session.execute("file.new", &json!({}));
            let _ = app.run("design.applyIdea", json!({"id": "lusk:cartes"}));
        });
    }
    if want("palette") {
        shoot(&dir, "palette", |app| app.palette = Some(("police".into(), 0)));
    }
    for page in ["home", "new", "open", "saveAs", "print", "export", "about", "updates"] {
        let name = format!("fichier-{page}");
        if want(&name) {
            shoot(&dir, &name, |app| {
                let _ = app.run("app.backstage", json!({"page": page}));
            });
        }
    }
    for id in [
        "table",
        "slideSize",
        "sizePosition",
        "headerFooter",
        "hyperlink",
        "setupShow",
        "zoom",
        "altText",
        "outline",
        "reuseSlides",
        "paragraph",
        "tabs",
        "font",
        "about",
        "preferences",
        "autocorrect",
        "gridOptions",
        "columns",
        "export",
        "symbol",
        "photoAlbum",
        "replaceFonts",
        "equation",
        "dateTime",
        "spelling",
        "pasteSpecial",
        "find",
        "close",
        "accessibility",
        "language",
        "customShow",
        "smartart",
        "bullets",
        "action",
        "themeColors",
        "themeFonts",
    ] {
        let name = format!("dialogue-{id}");
        if want(&name) {
            shoot(&dir, &name, |app| app.dialog = Some(crate::dialogs::Dialog::new(id)));
        }
    }
    for pane in ["format", "background", "animation", "selection", "comments", "designer", "thesaurus"] {
        let name = format!("volet-{pane}");
        if want(&name) {
            shoot(&dir, &name, |app| app.ui.pane = Some(pane.into()));
        }
    }
}
