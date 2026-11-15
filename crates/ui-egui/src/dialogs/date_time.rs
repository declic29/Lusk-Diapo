use super::{DATE_KINDS, Dialog, SlideApp, Tokens, Ui, buttons, check, json, kit};
use crate::icons::Icon;

const DATE_LANGS: &[(&str, &str)] = &[
    ("en-US", "English (United States)"),
    ("en-GB", "English (United Kingdom)"),
    ("fr-FR", "French (France)"),
    ("ja-JP", "Japanese"),
    ("sv-SE", "Swedish"),
];

/// Samples `field_text` can compute for `lang`. Named months exist for English and French, AM/PM
/// for English only; other languages keep the numeric date and the 24-hour times.
pub(super) fn date_rows(lang: &str) -> Vec<(&'static str, String)> {
    let now = deckcraft_text::datetime::DateTime::now_local();
    let mut rows = Vec::new();
    for kind in DATE_KINDS {
        if let Some(now) = now
            && let Some(sample) = deckcraft_text::datetime::field_text(kind, Some(lang), now)
        {
            rows.push((kind, sample));
        }
    }
    if rows.is_empty() {
        rows.push(("datetime1", "datetime1".into()));
    }
    rows
}

/// The language the dates are written in by default: French in the French interface.
pub(super) fn default_language() -> &'static str {
    if crate::i18n::french() { "fr-FR" } else { "en-US" }
}

/// Render language and format controls; OK applies `insert.dateTime`, and the return value says
/// whether the dialog should close.
pub(super) fn date_time_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(560.0);
    let lang = d.get("language", default_language());
    let mut rows = date_rows(&lang);
    // Two formats can write the same text in a language (French has no comma to tell them apart).
    let mut seen = std::collections::HashSet::new();
    rows.retain(|(_, s)| seen.insert(s.clone()));
    if !rows.iter().any(|(k, _)| *k == d.get("format", "datetime1"))
        && let Some((k, _)) = rows.first()
    {
        d.fields.insert("format".into(), (*k).to_string());
    }
    let format = d.get("format", "datetime1");
    let mut update = true;
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(300.0);
            kit::section(ui, Icon::DateTime, "Available formats:");
            kit::card(ui, |ui| {
                egui::ScrollArea::vertical().max_height(300.0).min_scrolled_height(300.0).auto_shrink([false, false]).show(ui, |ui| {
                    for (kind, sample) in &rows {
                        let icon = if matches!(*kind, "datetime8" | "datetime9" | "datetime10" | "datetime11" | "datetime12" | "datetime13") {
                            Icon::Duration
                        } else {
                            Icon::DateTime
                        };
                        if kit::list_row(ui, icon, sample, format == *kind).clicked() {
                            d.fields.insert("format".into(), (*kind).to_string());
                        }
                    }
                });
            });
        });
        ui.add_space(10.0);
        ui.vertical(|ui| {
            kit::section(ui, Icon::Language, "Language");
            let lang_label = DATE_LANGS.iter().find(|(k, _)| *k == lang).map(|(_, l)| *l).unwrap_or("English (United States)");
            let opened = craft_ui::ComboBox::from_id_salt("date-lang").selected_text(crate::i18n::trs(lang_label)).width(200.0).show_ui(ui, |ui| {
                for (k, l) in DATE_LANGS {
                    if ui.selectable_label(lang == *k, crate::i18n::trs(*l)).clicked() {
                        d.fields.insert("language".into(), (*k).to_string());
                    }
                }
            });
            let shown = lang_label.to_string();
            opened.response.widget_info(move || {
                let mut info = egui::WidgetInfo::labeled(egui::Role::ComboBox, true, crate::i18n::tr("Language"));
                info.current_text_value = Some(shown.clone());
                info
            });
            ui.add_space(10.0);
            update = check(ui, d, "Update automatically", "update", true);
            kit::hint(ui, if update { "The date changes to today each time the presentation is opened." } else { "The date stays as it is today." });
            ui.add_space(10.0);
            kit::section(ui, Icon::Preview, "Preview");
            kit::card(ui, |ui| {
                let sample = rows.iter().find(|(k, _)| *k == format).map(|(_, s)| s.clone()).unwrap_or_default();
                ui.label(egui::RichText::new(sample).font(crate::theme::font(17.0)).color(t.text));
            });
            ui.add_space(6.0);
            kit::hint(
                ui,
                "Month and day names are written in English and in French; other languages show the date in figures and the time over 24 hours.",
            );
        });
    });
    let (ok, cancel) = buttons(ui, "OK");
    if ok {
        let lang = d.get("language", default_language());
        let mut format = d.get("format", "datetime1");
        let probe = deckcraft_text::datetime::DateTime::new(2026, 10, 9, 20, 5, 7);
        if deckcraft_text::datetime::field_text(&format, Some(&lang), probe).is_none() {
            format = "datetime1".into();
        }
        return app.run("insert.dateTime", json!({"format": format, "update": update, "language": lang})).is_ok();
    }
    cancel
}
