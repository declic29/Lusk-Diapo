//! Lusk Diapo : les fenêtres Commentaire, Lien, En-tête et pied de page, Équation et
//! Vérificateur d'accessibilité, mises en page avec des icônes ([`super::kit`]).

use egui::{Align2, Color32, CornerRadius, Rect, Sense, Stroke, StrokeKind, Ui, pos2, vec2};
use serde_json::{Value, json};

use super::date_time::{date_rows, default_language};
use super::equation::pretty_equation;
use super::hyperlink::{hyperlink_command, hyperlink_seed_of, hyperlink_should_apply};
use super::{Dialog, SlideApp, buttons, check, kit};
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// A round badge with the author's initials, as on the comment cards.
fn avatar(ui: &mut Ui, initials: &str, size: f32) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    ui.painter().circle_filled(r.center(), size / 2.0, t.accent);
    ui.painter().text(r.center(), Align2::CENTER_CENTER, initials, theme::bold(size * 0.42), Color32::WHITE);
}

fn initials_of(name: &str) -> String {
    let s: String = name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect();
    if s.is_empty() { "?".into() } else { s.to_uppercase() }
}

/// Insertion › Commentaire and Révision › Nouveau commentaire.
pub(super) fn comment_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(420.0);
    let author = app.session.prefs.author.clone();
    let slide = app.session.active().map(|s| s.selection.slide + 1).unwrap_or(1);
    let on_shape = app.session.active().is_some_and(|s| !s.selection.shapes.is_empty());
    ui.horizontal(|ui| {
        avatar(ui, &initials_of(&author), 34.0);
        ui.vertical(|ui| {
            let name = if author.trim().is_empty() { crate::i18n::trs("You") } else { author.clone() };
            ui.label(egui::RichText::new(name).font(theme::bold(13.5)));
            let place =
                if on_shape { crate::i18n::trf("On the selected object, slide {0}", &[&slide]) } else { crate::i18n::trf("On slide {0}", &[&slide]) };
            ui.label(egui::RichText::new(place).size(11.5).color(t.text_dim));
        });
    });
    ui.add_space(6.0);
    let mut v = d.get("text", "");
    let edit = ui.add(egui::TextEdit::multiline(&mut v).hint_text(crate::i18n::tr("Comment")).desired_rows(5).desired_width(f32::INFINITY));
    if !d.fields.contains_key("focused") {
        edit.request_focus();
        d.fields.insert("focused".into(), "1".into());
    }
    d.fields.insert("text".into(), v.clone());
    ui.add_space(4.0);
    kit::hint(ui, "Everyone who opens the presentation can read the comment. Find it again in the Comments pane.");
    let (ok, cancel) = buttons(ui, "Post");
    if ok && !v.trim().is_empty() {
        let _ = app.run("comment.add", json!({"text": v}));
        app.ui.pane = Some("comments".into());
    }
    (ok && !v.trim().is_empty()) || cancel
}

/// Insertion › Lien.
pub(super) fn hyperlink_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(560.0);
    let seed = hyperlink_seed_of(app.session.active());
    let slide_default = seed.slide.to_string();
    let mode = d.get("mode", seed.mode);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(118.0);
            ui.label(egui::RichText::new(crate::i18n::tr("Link to:")).size(12.0).color(t.text_dim));
            for (m, icon, label) in
                [("url", Icon::Link, "Web Page or File"), ("slide", Icon::Slide, "This Document"), ("email", Icon::Share, "Email Address")]
            {
                if kit::icon_tile(ui, vec2(112.0, 74.0), icon, label, mode == m).clicked() {
                    d.fields.insert("mode".into(), m.into());
                }
                ui.add_space(4.0);
            }
        });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            match mode.as_str() {
                "slide" => {
                    kit::section(ui, Icon::Slide, "Place in this presentation");
                    let titles: Vec<String> = app
                        .session
                        .active()
                        .map(|s| s.doc.slides.iter().enumerate().map(|(i, sl)| format!("{}. {}", i + 1, sl.title())).collect())
                        .unwrap_or_default();
                    let sel = d.get("slide", &slide_default).parse::<usize>().unwrap_or(0);
                    kit::card(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(190.0).min_scrolled_height(190.0).auto_shrink([false, false]).show(ui, |ui| {
                            for (i, title) in titles.iter().enumerate() {
                                if kit::list_row(ui, Icon::Slide, title, sel == i).clicked() {
                                    d.fields.insert("slide".into(), i.to_string());
                                }
                            }
                        });
                    });
                }
                "email" => {
                    kit::section(ui, Icon::Share, "Email address");
                    text_field(ui, d, "Email address:", "email", &seed.email, "name@example.org");
                    kit::hint(ui, "A click opens a new message to this address in the mail program.");
                }
                _ => {
                    kit::section(ui, Icon::Link, "Web page or file");
                    text_field(ui, d, "Address:", "url", &seed.url, "https://");
                    kit::hint(ui, "Type or paste a web address (https://…) or the path of a file. The link opens during the slide show.");
                }
            }
            ui.add_space(8.0);
            kit::section(ui, Icon::Info, "ScreenTip");
            text_field(ui, d, "ScreenTip:", "tip", &seed.tip, "");
            kit::hint(ui, "Shown when the pointer rests on the link.");
        });
    });
    let (ok, cancel) = buttons(ui, "OK");
    if ok {
        let mode_now = d.get("mode", seed.mode);
        let url = d.get("url", &seed.url);
        let email = d.get("email", &seed.email);
        let slide = d.get("slide", &slide_default).parse::<usize>().unwrap_or(0);
        let tip = d.get("tip", &seed.tip);
        if hyperlink_should_apply(&seed, &mode_now, &url, &email, slide) {
            let _ = app.run("insert.hyperlink", hyperlink_command(&mode_now, &url, &email, slide, &tip, seed.highlight));
        }
    }
    ok || cancel
}

/// A labelled single-line field across the width, bound to `key`.
fn text_field(ui: &mut Ui, d: &mut Dialog, label: &str, key: &str, default: &str, hint: &str) {
    let mut v = d.get(key, default);
    ui.horizontal(|ui| {
        let l = kit::form_label(ui, label, 120.0);
        ui.add(egui::TextEdit::singleline(&mut v).hint_text(hint).desired_width(f32::INFINITY)).labelled_by(l.id);
    });
    d.fields.insert(key.into(), v);
}

/// A sketch of a slide with its footer boxes: the date on the left, the footer text in the
/// middle and the number on the right, filled in when they are on.
fn footer_sketch(ui: &mut Ui, date: bool, footer: bool, number: bool) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(170.0, 96.0), Sense::hover());
    let p = ui.painter();
    p.rect_filled(r, CornerRadius::same(4), t.canvas);
    p.rect_stroke(r, CornerRadius::same(4), Stroke::new(1.0, t.border), StrokeKind::Inside);
    p.rect_filled(Rect::from_min_size(r.min + vec2(24.0, 18.0), vec2(r.width() - 48.0, 10.0)), CornerRadius::same(2), t.text_faint);
    for i in 0..3 {
        p.rect_filled(
            Rect::from_min_size(r.min + vec2(30.0, 38.0 + i as f32 * 9.0), vec2(r.width() - 70.0 - i as f32 * 14.0, 4.0)),
            CornerRadius::same(1),
            t.separator,
        );
    }
    let y = r.bottom() - 14.0;
    for (on, x0, w) in [(date, 8.0, 40.0), (footer, r.width() / 2.0 - 30.0, 60.0), (number, r.width() - 30.0, 22.0)] {
        let b = Rect::from_min_size(pos2(r.left() + x0, y), vec2(w, 7.0));
        if on {
            p.rect_filled(b, CornerRadius::same(2), t.accent);
        } else {
            p.rect_stroke(b, CornerRadius::same(2), Stroke::new(1.0, t.text_faint), StrokeKind::Inside);
        }
    }
}

/// Insertion › En-tête et pied de page.
pub(super) fn header_footer_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(560.0);
    let hf = app.session.active().map(|s| s.doc.header_footer.clone()).unwrap_or_default();
    let mut date = false;
    let mut num_on = false;
    let mut foot = false;
    let mut hide = false;
    let fmt_now = if hf.date_format.is_empty() { "datetime1".to_string() } else { hf.date_format.clone() };
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(350.0);
            kit::section(ui, Icon::HeaderFooter, "Include on slides");
            kit::card(ui, |ui| {
                ui.horizontal(|ui| {
                    paint_icon(ui, Icon::DateTime);
                    date = check(ui, d, "Date and time", "date", hf.date);
                });
                ui.indent("hf-date", |ui| {
                    ui.add_enabled_ui(date, |ui| {
                        // A fixed date is the text as typed; empty means the date updates by itself.
                        let mut fixed = d.get("dateText", &hf.date_text);
                        let automatic = fixed.is_empty();
                        let fmt = d.fields.get("dateFormat").cloned().unwrap_or_else(|| fmt_now.clone());
                        let rows = date_rows(default_language());
                        let label = rows.iter().find(|(k, _)| *k == fmt).map(|(_, s)| s.clone()).unwrap_or_else(|| fmt.clone());
                        ui.add_enabled_ui(automatic, |ui| {
                            ui.horizontal(|ui| {
                                kit::form_label(ui, "Format:", 140.0);
                                craft_ui::ComboBox::from_id_salt("hf-date-format").selected_text(crate::i18n::trs(label)).width(170.0).show_ui(
                                    ui,
                                    |ui| {
                                        for (kind, sample) in &rows {
                                            if ui.selectable_label(fmt == *kind, crate::i18n::trs(sample)).clicked() {
                                                d.fields.insert("dateFormat".into(), (*kind).to_string());
                                            }
                                        }
                                    },
                                );
                            });
                        });
                        ui.horizontal(|ui| {
                            let l = kit::form_label(ui, "Fixed date:", 140.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut fixed).hint_text(crate::i18n::tr("empty = updates by itself")).desired_width(180.0),
                            )
                            .labelled_by(l.id);
                        });
                        d.fields.insert("dateText".into(), fixed);
                    });
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    paint_icon(ui, Icon::SlideNumber);
                    num_on = check(ui, d, "Slide number", "num", hf.slide_number);
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    paint_icon(ui, Icon::HeaderFooter);
                    foot = check(ui, d, "Footer", "footer", hf.footer);
                });
                ui.indent("hf-footer", |ui| {
                    ui.add_enabled_ui(foot, |ui| {
                        let mut ft = d.get("footerText", &hf.footer_text);
                        ui.horizontal(|ui| {
                            let l = kit::form_label(ui, "Footer text:", 140.0);
                            ui.add(egui::TextEdit::singleline(&mut ft).desired_width(180.0)).labelled_by(l.id);
                        });
                        d.fields.insert("footerText".into(), ft);
                    });
                });
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                paint_icon(ui, Icon::HideSlide);
                hide = check(ui, d, "Don't show on title slide", "hide", hf.hide_on_title);
            });
        });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            kit::section(ui, Icon::Preview, "Preview");
            footer_sketch(ui, date, foot, num_on);
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(crate::i18n::tr("Date on the left, footer in the middle, number on the right.")).size(11.0).color(t.text_dim),
            );
        });
    });
    kit::divider(ui);
    let mut apply_all = false;
    let mut apply = false;
    ui.allocate_ui_with_layout(vec2(ui.available_width(), 28.0), egui::Layout::right_to_left(egui::Align::Center), |ui| {
        apply_all = ui
            .add(
                egui::Button::new(egui::RichText::new(crate::i18n::tr("Apply to All")).color(t.accent_text))
                    .fill(t.accent)
                    .min_size(vec2(90.0, 24.0)),
            )
            .clicked();
        apply = ui.add(egui::Button::new(crate::i18n::tr("Apply")).min_size(vec2(80.0, 24.0))).clicked();
        if ui.add(egui::Button::new(crate::i18n::tr("Cancel")).min_size(vec2(72.0, 24.0))).clicked() {
            d.fields.insert("cancel".into(), "1".into());
        }
    });
    if apply || apply_all {
        let mut params = serde_json::Map::new();
        params.insert("date".into(), json!(date));
        params.insert("dateText".into(), json!(d.get("dateText", "")));
        // Only a format the user picked: sending the default would rewrite every slide's
        // existing date field (an imported datetime4, say) to datetime1.
        if d.fields.contains_key("dateFormat") {
            params.insert("dateFormat".into(), json!(d.get("dateFormat", &fmt_now)));
        }
        params.insert("slideNumber".into(), json!(num_on));
        params.insert("footer".into(), json!(foot));
        params.insert("footerText".into(), json!(d.get("footerText", "")));
        params.insert("hideOnTitle".into(), json!(hide));
        params.insert("all".into(), json!(apply_all));
        let _ = app.run("design.headerFooter", Value::Object(params));
    }
    apply || apply_all || d.fields.contains_key("cancel")
}

fn paint_icon(ui: &mut Ui, icon: Icon) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    icons::paint(ui.painter(), r, icon, t.accent, false);
}

/// Symbols the Équation window inserts with one click.
const SYMBOLS: &[&str] = &["²", "³", "√", "π", "±", "×", "÷", "≤", "≥", "≠", "≈", "∞", "∑", "∫", "∂", "Δ", "α", "β", "θ", "λ", "μ", "σ", "→", "°"];

/// Insertion › Équation.
pub(super) fn equation_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(500.0);
    kit::section(ui, Icon::Equation, "Type the equation");
    let mut v = d.get("eq", "");
    let edit = ui.add(egui::TextEdit::singleline(&mut v).hint_text(crate::i18n::tr("Equation")).desired_width(f32::INFINITY).font(theme::font(16.0)));
    if !d.fields.contains_key("focused") {
        edit.request_focus();
        d.fields.insert("focused".into(), "1".into());
    }
    kit::hint(ui, "Linear form: a^2+b^2=c^2 or x=(-b+-sqrt(b^2-4ac))/2a. ^ raises, sqrt gives √, pi gives π, <= gives ≤.");
    ui.add_space(4.0);
    kit::section(ui, Icon::Symbol, "Symbols");
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
        for s in SYMBOLS {
            let b = ui.add(egui::Button::new(egui::RichText::new(*s).font(theme::font(15.0))).min_size(vec2(30.0, 28.0)));
            if b.clicked() {
                v.push_str(s);
            }
        }
    });
    d.fields.insert("eq".into(), v.clone());
    ui.add_space(4.0);
    kit::section(ui, Icon::Preview, "Preview");
    kit::card(ui, |ui| {
        let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 56.0), Sense::hover());
        let shown = if v.trim().is_empty() { crate::i18n::trs("The equation appears here") } else { pretty_equation(&v) };
        let ink = if v.trim().is_empty() { t.text_faint } else { t.text };
        let font = egui::FontId::new(24.0, egui::FontFamily::Proportional);
        let galley = ui.painter().layout(shown, font, ink, r.width());
        ui.painter().galley(r.center() - galley.size() / 2.0, galley, ink);
    });
    let (ok, cancel) = buttons(ui, "Insert");
    if ok && !v.is_empty() {
        let pretty = pretty_equation(&v);
        let size = app.session.active().map(|s| s.doc.slide_size).unwrap_or(deckcraft_model::defaults::WIDE);
        let _ = app.run("insert.textBox", json!({"rect": [size.width / 2.0 - 150.0, size.height / 2.0 - 25.0, 300, 50], "text": pretty}));
        let _ = app.run("text.exit", json!({}));
        let _ = app.run("format.font", json!({"family": "Liberation Serif"}));
        let _ = app.run("format.italic", json!({"on": true}));
        let _ = app.run("format.size", json!({"size": 28}));
    }
    ok || cancel
}

/// How to fix an accessibility issue the checker reports.
fn how_to_fix(issue: &str) -> &'static str {
    let l = issue.to_ascii_lowercase();
    if l.contains("alt") {
        "Right-click the object › Edit Alt Text, then describe it in a sentence."
    } else if l.contains("title") {
        "Give the slide a title (a Title layout, or type in its title box); it is read first."
    } else if l.contains("contrast") {
        "Darken the text or lighten its background so it stands out."
    } else if l.contains("table") || l.contains("header") {
        "Mark the table's first row as a header row (Table Design › Header Row)."
    } else if l.contains("link") {
        "Use link text that says where it goes, not the bare address."
    } else {
        "Select the object with Go to and correct it."
    }
}

/// Révision › Vérifier l'accessibilité.
pub(super) fn accessibility_dialog(app: &mut SlideApp, ui: &mut Ui, _d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(560.0);
    let v = app.session.execute("review.accessibility", &json!({})).unwrap_or_default();
    let list = v.as_array().cloned().unwrap_or_default();
    // The summary: a green check, or an orange warning with the number of issues.
    kit::card(ui, |ui| {
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(vec2(34.0, 34.0), Sense::hover());
            let good = Color32::from_rgb(0x2E, 0x9E, 0x5B);
            let warn = Color32::from_rgb(0xE0, 0x8A, 0x1E);
            ui.painter().circle_filled(r.center(), 17.0, if list.is_empty() { good } else { warn });
            icons::paint(ui.painter(), r.shrink(8.0), if list.is_empty() { Icon::Check } else { Icon::Warning }, Color32::WHITE, false);
            ui.vertical(|ui| {
                if list.is_empty() {
                    ui.label(egui::RichText::new(crate::i18n::tr("No accessibility issues found")).font(theme::bold(14.0)));
                    ui.label(
                        egui::RichText::new(crate::i18n::tr("People with disabilities should not have difficulty reading this document."))
                            .size(12.0)
                            .color(t.text_dim),
                    );
                } else {
                    let n = list.len();
                    let title = if n == 1 { crate::i18n::trs("1 thing to check") } else { crate::i18n::trf("{0} things to check", &[&n]) };
                    ui.label(egui::RichText::new(title).font(theme::bold(14.0)));
                    ui.label(
                        egui::RichText::new(crate::i18n::tr("Fixing them helps people who use a screen reader or see poorly."))
                            .size(12.0)
                            .color(t.text_dim),
                    );
                }
            });
        });
    });
    if !list.is_empty() {
        ui.add_space(6.0);
        kit::section(ui, Icon::Accessibility, "Issues");
        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            for item in &list {
                let slide = item.get("slide").and_then(Value::as_u64).unwrap_or(0) + 1;
                let issue = item.get("issue").and_then(Value::as_str).unwrap_or("");
                let name = item.get("name").and_then(Value::as_str).unwrap_or("");
                kit::card(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        let (r, _) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::hover());
                        icons::paint(ui.painter(), r, Icon::Warning, Color32::from_rgb(0xE0, 0x8A, 0x1E), false);
                        ui.vertical(|ui| {
                            ui.set_width(ui.available_width() - 110.0);
                            ui.label(egui::RichText::new(crate::i18n::tr(issue)).font(theme::bold(13.0)));
                            let place = if name.is_empty() {
                                crate::i18n::trf("Slide {0}", &[&slide])
                            } else {
                                format!("{} — {name}", crate::i18n::trf("Slide {0}", &[&slide]))
                            };
                            ui.label(egui::RichText::new(place).size(12.0).color(t.text_dim));
                            ui.add(egui::Label::new(egui::RichText::new(crate::i18n::tr(how_to_fix(issue))).size(11.5).color(t.text_dim)).wrap());
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            if kit::icon_button(ui, Icon::ChevronRight, "Go to", true).clicked() {
                                let _ = app.run("slide.go", json!({"index": item.get("slide").cloned().unwrap_or_default()}));
                                if let Some(id) = item.get("shape") {
                                    let _ = app.run("edit.select", json!({"ids": [id]}));
                                }
                            }
                        });
                    });
                });
                ui.add_space(4.0);
            }
        });
    }
    kit::divider(ui);
    let mut close = false;
    ui.allocate_ui_with_layout(vec2(ui.available_width(), 28.0), egui::Layout::right_to_left(egui::Align::Center), |ui| {
        close = ui
            .add(egui::Button::new(egui::RichText::new(crate::i18n::tr("Close")).color(t.accent_text)).fill(t.accent).min_size(vec2(80.0, 24.0)))
            .clicked();
    });
    close || ui.input(|i| i.key_pressed(egui::Key::Enter))
}
