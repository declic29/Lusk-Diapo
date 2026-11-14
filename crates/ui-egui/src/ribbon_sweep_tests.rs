//! Lusk Diapo : vérifie que chaque bouton du ruban fait quelque chose.
//!
//! `LUSK_SWEEP=<fichier> cargo test -p deckcraft-ui-egui --lib ribbon_sweep -- --nocapture` clique
//! tour à tour chaque bouton de chaque onglet, dans une présentation d'exemple neuve où une forme
//! est sélectionnée, et écrit dans le fichier ceux qui ne changent rien (ni la présentation, ni la
//! sélection, ni l'affichage, ni un menu, une boîte de dialogue ou un volet) et ceux qui plantent.
//! Sans la variable, ne fait rien.

use deckcraft_engine::Session;
use egui_kittest::kittest::NodeT;
use serde_json::json;

use crate::{Services, SlideApp};

const TABS: [&str; 10] = ["Home", "Insert", "Draw", "Design", "Transitions", "Animations", "Slide Show", "Record", "Review", "View"];

fn app(tab: &str, select: &str) -> SlideApp {
    let mut app = SlideApp::new(Session::new(), Services::default());
    let _ = deckcraft_engine::sample::open_sample(&mut app.session);
    match select {
        "shape" => {
            let r = app.session.execute("shape.insert", &json!({"preset": "rect", "rect": [80.0, 80.0, 100.0, 60.0]}));
            if let Some(id) = r.ok().and_then(|v| v.get("id").and_then(serde_json::Value::as_u64)) {
                let _ = app.session.execute("edit.select", &json!({"ids": [id]}));
            }
        }
        "table" => {
            let _ = app.session.execute("insert.table", &json!({"rows": 3, "cols": 3}));
        }
        "picture" => {
            let png = deckcraft_engine::cmd::base64_encode(include_bytes!("../../../assets/app-icon/hicolor/64x64/apps/bzh.lusk.diapo.png"));
            let _ = app.session.execute("insert.picture", &json!({"data": png}));
        }
        _ => {}
    }
    app.ui.tab = tab.into();
    app
}

/// Everything a click can change, in one comparable line.
fn signature(app: &mut SlideApp, nodes: usize, ctx: &egui::Context) -> String {
    let doc = app.session.active().map(|d| (d.revision, format!("{:?}", d.selection), d.doc.slides.len()));
    format!(
        "{doc:?}|{:?}|{:?}|{}|{}|{:?}|{}|{}|{}|{}|{}|{}|{}|{}",
        app.dialog.as_ref().map(|d| d.id.clone()),
        app.ui.pane,
        app.backstage.is_some(),
        app.ui.tab,
        app.palette.is_some(),
        app.show.is_some(),
        app.status.as_ref().map(|s| s.0.clone()).unwrap_or_default(),
        nodes,
        ctx.any_popup_open(),
        serde_json::to_string(&app.ui).unwrap_or_default().len(),
        format!("{:?}", app.session.tool).len(),
        app.session.painter.is_some(),
        app.session.clipboard.shapes.len() + app.session.clipboard.slides.len(),
    )
}

fn harness(app: SlideApp) -> egui_kittest::Harness<'static, SlideApp> {
    egui_kittest::Harness::builder().with_size(egui::vec2(2600.0, 900.0)).with_accessibility_check(false).build_ui_state(
        |ui, app: &mut SlideApp| {
            app.logic(ui.ctx());
            app.ui(ui);
        },
        app,
    )
}

/// The ribbon's buttons: what the tab shows between the tab row and the slides.
fn ribbon_buttons(h: &egui_kittest::Harness<'static, SlideApp>) -> Vec<(String, egui::Pos2)> {
    let mut out = Vec::new();
    for n in h.root().children_recursive() {
        let a = n.accesskit_node();
        let role = a.role();
        if !matches!(role, egui::accesskit::Role::Button | egui::accesskit::Role::CheckBox | egui::accesskit::Role::ComboBox) {
            continue;
        }
        let Some(b) = a.raw_bounds() else { continue };
        let c = egui::pos2(((b.x0 + b.x1) / 2.0) as f32, ((b.y0 + b.y1) / 2.0) as f32);
        if c.y < 62.0 || c.y > 160.0 || c.x > 2590.0 || a.is_disabled() {
            continue;
        }
        let label = a.label().unwrap_or_default();
        if label.is_empty() {
            continue;
        }
        out.push((label, c));
    }
    out
}

#[test]
fn ribbon_sweep() {
    let Some(out) = std::env::var_os("LUSK_SWEEP") else { return };
    let mut report = String::new();
    let mut total = 0;
    let contexts: Vec<(&str, &str)> = TABS
        .iter()
        .map(|t| (*t, "shape"))
        .chain([("Shape Format", "shape"), ("Table Design", "table"), ("Layout", "table"), ("Picture Format", "picture")])
        .collect();
    for (tab, select) in contexts {
        let mut h = harness(app(tab, select));
        h.run_steps(4);
        let buttons = ribbon_buttons(&h);
        for (label, pos) in buttons {
            total += 1;
            let mut h = harness(app(tab, select));
            h.run_steps(4);
            let nodes = h.root().children_recursive().count();
            let ctx = h.ctx.clone();
            let before = signature(h.state_mut(), nodes, &ctx);
            let clicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                h.hover_at(pos);
                h.run_steps(1);
                h.event(egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: true, modifiers: egui::Modifiers::NONE });
                h.run_steps(1);
                h.event(egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: false, modifiers: egui::Modifiers::NONE });
                h.run_steps(4);
                let nodes = h.root().children_recursive().count();
                let ctx = h.ctx.clone();
                signature(h.state_mut(), nodes, &ctx)
            }));
            match clicked {
                Err(_) => report.push_str(&format!("PLANTAGE\t{tab}\t{label}\n")),
                Ok(after) if after == before => report.push_str(&format!("SANS EFFET\t{tab}\t{label}\n")),
                Ok(_) => {}
            }
        }
    }
    report.push_str(&format!("# {total} boutons essayés\n"));
    std::fs::write(out, report).unwrap();
}
