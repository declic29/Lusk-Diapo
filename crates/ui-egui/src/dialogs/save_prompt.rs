//! Lusk Diapo : la fenêtre « Enregistrer les modifications ? » qui s'ouvre quand on ferme une
//! présentation (ou l'application) sans avoir enregistré. Une pastille avec l'icône Enregistrer,
//! un titre, la présentation concernée dans une carte, puis trois boutons : Ne pas enregistrer à
//! gauche, Annuler et Enregistrer (le bouton principal) à droite.

use egui::{CornerRadius, Rect, Sense, Ui, vec2};

use super::kit;
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// What the person chose.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Choice {
    Save,
    DontSave,
    Cancel,
}

/// How many presentation names the card lists before « and N more ».
const SHOWN: usize = 4;

/// Draw the prompt for the presentations `names` (unsaved) and return the choice, if one was made
/// this frame. Enter means Save.
pub(super) fn save_prompt(ui: &mut Ui, names: &[String]) -> Option<Choice> {
    let t = Tokens::get(ui.ctx());
    ui.set_width(440.0);
    let mut choice = None;
    ui.add_space(10.0);
    ui.horizontal_top(|ui| {
        // The badge: the Save icon on a light accent disc.
        let (r, _) = ui.allocate_exact_size(vec2(46.0, 46.0), Sense::hover());
        ui.painter().rect_filled(r, CornerRadius::same(23), t.accent.gamma_multiply(0.16));
        icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(24.0, 24.0)), Icon::Save, t.accent, false);
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.add_space(3.0);
            ui.label(egui::RichText::new(crate::i18n::tr("Save your changes?")).font(theme::bold(17.0)).color(t.text));
            ui.add_space(2.0);
            let question = if names.len() == 1 {
                crate::i18n::tr("Some changes haven't been saved.").to_string()
            } else {
                crate::i18n::trf("{0} presentations have unsaved changes.", &[&names.len()])
            };
            ui.add(egui::Label::new(egui::RichText::new(question).size(13.0).line_height(Some(18.0)).color(t.text_dim)).wrap());
        });
    });
    ui.add_space(12.0);
    kit::card(ui, |ui| {
        for name in names.iter().take(SHOWN) {
            ui.horizontal(|ui| {
                let (r, _) = ui.allocate_exact_size(vec2(18.0, 20.0), Sense::hover());
                icons::paint(ui.painter(), r, Icon::File, t.accent, false);
                ui.add_space(4.0);
                ui.add(egui::Label::new(egui::RichText::new(name).font(theme::bold(13.5)).color(t.text)).truncate());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(crate::i18n::tr("Not saved")).size(11.5).color(t.text_faint));
                });
            });
        }
        if names.len() > SHOWN {
            ui.label(egui::RichText::new(crate::i18n::trf("and {0} more", &[&(names.len() - SHOWN)])).size(12.0).color(t.text_dim));
        }
    });
    ui.add_space(8.0);
    kit::hint(ui, "Your changes will be lost if you don't save them.");
    kit::divider(ui);
    ui.horizontal(|ui| {
        if kit::icon_button(ui, Icon::Close, "Don't Save", true).clicked() {
            choice = Some(Choice::DontSave);
        }
        ui.allocate_ui_with_layout(vec2(ui.available_width(), 30.0), egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if kit::primary_button(ui, Icon::Save, "Save").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                choice = Some(Choice::Save);
            }
            ui.add_space(4.0);
            let cancel = ui.add(
                egui::Button::new(crate::i18n::tr("Cancel"))
                    .min_size(vec2(84.0, 30.0))
                    .fill(t.control)
                    .stroke(egui::Stroke::new(1.0, t.border))
                    .corner_radius(CornerRadius::same(6)),
            );
            if cancel.clicked() {
                choice = Some(Choice::Cancel);
            }
        });
    });
    ui.add_space(2.0);
    choice
}
