use super::{SlideApp, Tokens, Ui, json, run, theme};

/// Review › Thesaurus: look a word up; clicking a synonym puts it in place of the selected word
/// when editing text, otherwise looks that synonym up in turn.
pub(super) fn thesaurus(app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let mut q = app.thesaurus_query.clone();
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
        crate::icons::paint(ui.painter(), r, crate::icons::Icon::Search, t.text_dim, false);
        let edit = ui.add(egui::TextEdit::singleline(&mut q).hint_text(crate::i18n::tr("Search for a word")).desired_width(f32::INFINITY));
        if edit.changed() {
            app.thesaurus_query = q.clone();
        }
    });
    ui.add_space(8.0);
    let senses = deckcraft_engine::thesaurus::lookup(&q);
    if q.trim().is_empty() {
        crate::dialogs::kit::card(ui, |ui| {
            ui.horizontal_top(|ui| {
                let (r, _) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::hover());
                crate::icons::paint(ui.painter(), r, crate::icons::Icon::Thesaurus, t.accent, false);
                ui.add(
                    egui::Label::new(egui::RichText::new(crate::i18n::tr("Select a word on the slide, or type one above.")).color(t.text_dim)).wrap(),
                );
            });
        });
        return;
    }
    if senses.is_empty() {
        ui.label(egui::RichText::new(crate::i18n::trf("No synonyms for “{0}”.", &[&q.trim()])).color(t.text_dim));
        return;
    }
    let editing = app.session.active().is_some_and(|d| d.selection.text.is_some());
    let tip = crate::i18n::tr(if editing { "Insert this word" } else { "Look up" });
    let mut picked = None;
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for sense in senses {
            crate::dialogs::kit::card(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&sense.word).font(theme::bold(13.5)));
                    if !sense.pos.is_empty() {
                        ui.label(egui::RichText::new(&sense.pos).size(11.5).italics().color(t.text_dim));
                    }
                });
                ui.add_space(2.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(5.0, 5.0);
                    for syn in &sense.synonyms {
                        let b = egui::Button::new(egui::RichText::new(syn).size(12.5)).fill(t.chrome).corner_radius(egui::CornerRadius::same(10));
                        if ui.add(b).on_hover_text(tip).clicked() {
                            picked = Some(syn.clone());
                        }
                    }
                });
            });
            ui.add_space(6.0);
        }
        ui.label(
            egui::RichText::new(crate::i18n::tr(if editing {
                "Click a synonym to put it in place of the word."
            } else {
                "Click a synonym to look it up in turn."
            }))
            .size(11.0)
            .color(t.text_faint),
        );
        ui.label(egui::RichText::new(crate::i18n::tr("French synonyms: Dicollecte thesaurus (LGPL).")).size(10.5).color(t.text_faint));
    });
    if let Some(syn) = picked {
        if editing {
            run(app, "review.replaceWord", json!({"with": syn}));
        }
        app.thesaurus_query = syn;
    }
}
