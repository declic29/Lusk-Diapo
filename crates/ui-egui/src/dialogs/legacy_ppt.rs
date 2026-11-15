//! Lusk Diapo : la fenêtre « Ancien format PowerPoint 97-2003 ». Lusk Diapo ne lit pas le
//! format binaire .ppt ; quand on en ouvre un, cette fenêtre l'explique et dit comment le
//! convertir en .pptx avec LibreOffice Impress (gratuit) ou PowerPoint.

use egui::{CornerRadius, Rect, Sense, Ui, vec2};

use super::{Dialog, SlideApp, kit};
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// Where to download LibreOffice, in French.
pub(crate) const LIBREOFFICE_URL: &str = "https://fr.libreoffice.org/download/telecharger-libreoffice/";

/// One way to convert the file: the program, then the steps.
fn way(ui: &mut Ui, t: &Tokens, program: &str, steps: &str) {
    ui.horizontal_top(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(18.0, 20.0), Sense::hover());
        icons::paint(ui.painter(), r, Icon::File, t.accent, false);
        ui.add_space(4.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(crate::i18n::tr(program)).font(theme::bold(13.5)).color(t.text));
            ui.add(egui::Label::new(egui::RichText::new(crate::i18n::tr(steps)).size(12.5).line_height(Some(17.0)).color(t.text_dim)).wrap());
        });
    });
}

/// The window. Returns true when it should close.
pub(super) fn legacy_ppt_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    let t = Tokens::get(ui.ctx());
    ui.set_width(460.0);
    let name = d.get("name", "");
    ui.add_space(10.0);
    ui.horizontal_top(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(46.0, 46.0), Sense::hover());
        ui.painter().rect_filled(r, CornerRadius::same(23), t.accent.gamma_multiply(0.16));
        icons::paint(ui.painter(), Rect::from_center_size(r.center(), vec2(24.0, 24.0)), Icon::Info, t.accent, false);
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.add_space(3.0);
            ui.label(egui::RichText::new(crate::i18n::tr("Old PowerPoint 97-2003 format")).font(theme::bold(17.0)).color(t.text));
            ui.add_space(2.0);
            let text = if name.is_empty() {
                crate::i18n::tr("Lusk Diapo can't open .ppt files. Convert this one to .pptx, then open the copy.").to_string()
            } else {
                crate::i18n::trf("Lusk Diapo can't open « {0} ». Convert it to .pptx, then open the copy.", &[&name])
            };
            ui.add(egui::Label::new(egui::RichText::new(text).size(13.0).line_height(Some(18.0)).color(t.text_dim)).wrap());
        });
    });
    ui.add_space(12.0);
    kit::card(ui, |ui| {
        way(ui, &t, "With LibreOffice Impress (free)", "Open the file, then File › Save As, type « PowerPoint 2007-365 (.pptx) ».");
        ui.add_space(8.0);
        way(ui, &t, "With PowerPoint", "Open the file, then File › Save As, type « PowerPoint Presentation (*.pptx) ».");
    });
    ui.add_space(8.0);
    kit::hint(ui, "The original .ppt file is kept: the conversion makes a new .pptx file next to it.");
    kit::divider(ui);
    let mut close = false;
    ui.horizontal(|ui| {
        if kit::icon_button(ui, Icon::Link, "Download LibreOffice", true).clicked() {
            app.open_url(LIBREOFFICE_URL);
        }
        ui.allocate_ui_with_layout(vec2(ui.available_width(), 30.0), egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if kit::primary_button(ui, Icon::Check, "OK").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                close = true;
            }
        });
    });
    ui.add_space(2.0);
    close
}
