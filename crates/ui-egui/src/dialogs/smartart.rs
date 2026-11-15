//! Insertion › SmartArt : choisir un graphique (des vignettes dessinées) et taper ses éléments,
//! avec un aperçu qui suit ce qu'on tape.

use egui::{Color32, CornerRadius, Painter, Pos2, Rect, Stroke, StrokeKind, Ui, pos2, vec2};
use serde_json::json;

use super::{Dialog, SlideApp, buttons, kit};
use crate::icons::Icon;
use crate::theme::{self, Tokens};

/// The graphics offered: key, name, what it shows.
const KINDS: &[(&str, &str, &str)] = &[
    ("list", "Basic List", "Shows items of equal weight, one after the other."),
    ("process", "Basic Process", "Shows steps that follow each other, left to right."),
    ("cycle", "Basic Cycle", "Shows steps that repeat in a loop."),
    ("hierarchy", "Hierarchy", "Shows who or what comes under whom: the first line on top."),
    ("pyramid", "Pyramid", "Shows levels that build on each other, the first at the top."),
    ("matrix", "Matrix", "Shows four parts of a whole, in a 2 × 2 grid."),
];

/// The boxes of a `kind` graphic of `n` items laid out in `r`; for a hierarchy, the first box
/// is the top one.
fn boxes(kind: &str, r: Rect, n: usize) -> Vec<Rect> {
    let n = n.max(1);
    let gap = (r.width().min(r.height()) * 0.06).max(2.0);
    match kind {
        "process" => {
            let w = (r.width() - gap * 2.0 * (n as f32 - 1.0)) / n as f32;
            let h = (r.height() * 0.55).min(w * 0.8);
            (0..n).map(|i| Rect::from_min_size(pos2(r.left() + i as f32 * (w + gap * 2.0), r.center().y - h / 2.0), vec2(w, h))).collect()
        }
        "cycle" => {
            let rad = r.width().min(r.height()) * 0.36;
            let side = (rad * 1.1).min(r.height() * 0.32);
            (0..n)
                .map(|i| {
                    let a = -std::f32::consts::FRAC_PI_2 + i as f32 * std::f32::consts::TAU / n as f32;
                    Rect::from_center_size(r.center() + vec2(a.cos(), a.sin()) * rad, vec2(side * 1.25, side * 0.8))
                })
                .collect()
        }
        "hierarchy" => {
            let rest = n.saturating_sub(1).max(1);
            let h = r.height() * 0.3;
            let w = ((r.width() - gap * (rest as f32 - 1.0)) / rest as f32).min(r.width() * 0.4);
            let mut out = vec![Rect::from_center_size(pos2(r.center().x, r.top() + h / 2.0), vec2(w.max(r.width() * 0.3), h))];
            let total = w * rest as f32 + gap * (rest as f32 - 1.0);
            for i in 0..n.saturating_sub(1) {
                out.push(Rect::from_min_size(pos2(r.center().x - total / 2.0 + i as f32 * (w + gap), r.bottom() - h), vec2(w, h)));
            }
            out
        }
        "pyramid" => {
            let h = (r.height() - gap * (n as f32 - 1.0)) / n as f32;
            (0..n)
                .map(|i| {
                    let w = r.width() * (0.35 + 0.65 * (i as f32 + 1.0) / n as f32);
                    Rect::from_min_size(pos2(r.center().x - w / 2.0, r.top() + i as f32 * (h + gap)), vec2(w, h))
                })
                .collect()
        }
        "matrix" => {
            let (w, h) = ((r.width() - gap) / 2.0, (r.height() - gap) / 2.0);
            (0..4)
                .map(|i| Rect::from_min_size(pos2(r.left() + (i % 2) as f32 * (w + gap), r.top() + (i / 2) as f32 * (h + gap)), vec2(w, h)))
                .collect()
        }
        _ => {
            let h = (r.height() - gap * (n as f32 - 1.0)) / n as f32;
            (0..n).map(|i| Rect::from_min_size(pos2(r.left(), r.top() + i as f32 * (h + gap)), vec2(r.width(), h))).collect()
        }
    }
}

fn arrow(p: &Painter, a: Pos2, b: Pos2, ink: Color32, w: f32) {
    p.line_segment([a, b], Stroke::new(w, ink));
    let d = (b - a).normalized();
    let n = vec2(-d.y, d.x);
    let s = w * 2.6 + 2.0;
    p.add(egui::Shape::convex_polygon(vec![b, b - d * s + n * s * 0.6, b - d * s - n * s * 0.6], ink, Stroke::NONE));
}

/// Draw a `kind` graphic in `r`: boxes filled with `fill` (none for a thumbnail outline),
/// connectors in `ink`, and `labels` when given.
fn paint_graphic(p: &Painter, r: Rect, kind: &str, labels: &[String], ink: Color32, fill: Option<Color32>, text: Color32) {
    let n = if kind == "matrix" {
        4
    } else if labels.is_empty() {
        3
    } else {
        labels.len().min(8)
    };
    let rects = boxes(kind, r, n);
    // Connectors first, under the boxes.
    let line_w = if fill.is_some() { 2.0 } else { 1.3 };
    match kind {
        "process" => {
            for w in rects.windows(2) {
                arrow(p, pos2(w[0].right() + 1.0, w[0].center().y), pos2(w[1].left() - 1.0, w[1].center().y), ink, line_w);
            }
        }
        "cycle" if rects.len() > 1 => {
            let c = r.center();
            let rad = r.width().min(r.height()) * 0.36;
            for i in 0..rects.len() {
                let a0 = -std::f32::consts::FRAC_PI_2 + (i as f32 + 0.3) * std::f32::consts::TAU / rects.len() as f32;
                let a1 = -std::f32::consts::FRAC_PI_2 + (i as f32 + 0.7) * std::f32::consts::TAU / rects.len() as f32;
                arrow(p, c + vec2(a0.cos(), a0.sin()) * rad, c + vec2(a1.cos(), a1.sin()) * rad, ink, line_w);
            }
        }
        "hierarchy" if rects.len() > 1 => {
            let top = rects[0];
            let mid = (top.bottom() + rects[1].top()) / 2.0;
            p.line_segment([top.center_bottom(), pos2(top.center().x, mid)], Stroke::new(line_w, ink));
            let (l, rr) = (rects[1].center().x, rects[rects.len() - 1].center().x);
            p.line_segment([pos2(l, mid), pos2(rr, mid)], Stroke::new(line_w, ink));
            for b in &rects[1..] {
                p.line_segment([pos2(b.center().x, mid), b.center_top()], Stroke::new(line_w, ink));
            }
        }
        _ => {}
    }
    let round = if fill.is_some() { 6 } else { 3 };
    for (i, b) in rects.iter().enumerate() {
        if kind == "pyramid" {
            // A trapezoid band of the pyramid.
            let k = r.width() * 0.65 / n as f32 / 2.0;
            let pts = vec![pos2(b.left() + k, b.top()), pos2(b.right() - k, b.top()), pos2(b.right(), b.bottom()), pos2(b.left(), b.bottom())];
            let _ = match fill {
                Some(f) => p.add(egui::Shape::convex_polygon(pts, shade(f, i, n), Stroke::NONE)),
                None => p.add(egui::Shape::closed_line(pts, Stroke::new(1.3, ink))),
            };
        } else {
            match fill {
                Some(f) => {
                    p.rect_filled(*b, CornerRadius::same(round), shade(f, i, n));
                }
                None => {
                    p.rect_stroke(*b, CornerRadius::same(round), Stroke::new(1.3, ink), StrokeKind::Inside);
                }
            }
        }
        if let Some(label) = labels.get(i)
            && fill.is_some()
        {
            let font = theme::font((b.height() * 0.32).clamp(9.0, 13.0));
            let galley = p.layout(label.clone(), font, text, b.width() - 6.0);
            let pos = b.center() - galley.size() / 2.0;
            p.with_clip_rect(b.shrink(2.0)).galley(pos, galley, text);
        }
    }
}

/// `base` a little lighter for each later box, so the boxes read apart.
fn shade(base: Color32, i: usize, n: usize) -> Color32 {
    let k = if n > 1 { i as f32 / (n as f32 - 1.0) * 0.35 } else { 0.0 };
    let mix = |c: u8| (c as f32 + (255.0 - c as f32) * k) as u8;
    Color32::from_rgb(mix(base.r()), mix(base.g()), mix(base.b()))
}

/// Insert › SmartArt. Returns true when the dialog should close.
pub(super) fn smartart_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(600.0);
    let kind = d.get("kind", "process");
    let mut text = d.get("text", crate::i18n::tr("Plan\nBuild\nLaunch"));
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(300.0);
            kit::section(ui, Icon::SmartArt, "Choose a graphic");
            egui::Grid::new("smartart-kinds").spacing(vec2(8.0, 8.0)).show(ui, |ui| {
                for (i, (k, name, _)) in KINDS.iter().enumerate() {
                    let resp = kit::tile(ui, vec2(94.0, 92.0), name, kind == *k, |p, r, ink| paint_graphic(p, r, k, &[], ink, None, ink));
                    if resp.clicked() {
                        d.fields.insert("kind".into(), (*k).into());
                    }
                    if i % 3 == 2 {
                        ui.end_row();
                    }
                }
            });
        });
        ui.add_space(12.0);
        ui.vertical(|ui| {
            kit::section(ui, Icon::List, "Items, one per line");
            ui.add(egui::TextEdit::multiline(&mut text).hint_text(crate::i18n::tr("SmartArt items")).desired_rows(6).desired_width(f32::INFINITY));
        });
    });
    d.fields.insert("text".into(), text.clone());
    let items: Vec<String> = text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect();
    ui.add_space(4.0);
    kit::section(ui, Icon::Preview, "Preview");
    kit::card(ui, |ui| {
        let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 150.0), egui::Sense::hover());
        let shown: Vec<String> = if items.is_empty() { vec!["…".into()] } else { items.clone() };
        paint_graphic(ui.painter(), r.shrink2(vec2(40.0, 8.0)), &kind, &shown, t.text_dim, Some(t.accent), t.accent_text);
        if let Some((_, _, about)) = KINDS.iter().find(|(k, ..)| *k == kind) {
            ui.add_space(4.0);
            ui.label(egui::RichText::new(crate::i18n::tr(about)).size(12.0).color(t.text_dim));
        }
        if kind == "matrix" && items.len() > 4 {
            ui.label(egui::RichText::new(crate::i18n::tr("A matrix shows the first four items.")).size(11.5).color(t.text_faint));
        }
    });
    let (ok, cancel) = buttons(ui, "Insert");
    if ok {
        let _ = app.run("insert.smartArt", json!({"kind": kind, "items": items}));
        app.ui.pane = Some("smartart".into());
    }
    ok || cancel
}
