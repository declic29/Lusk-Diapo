//! Lusk Diapo : les pièces communes des fenêtres (titres de section à icône, cartes, boutons à
//! icône, tuiles de choix, aides). Elles donnent aux fenêtres flottantes (Album photo, SmartArt,
//! Lien, Commentaire, En-tête et pied de page, Date et heure, Équation, Vérificateur
//! d'accessibilité) la même allure soignée.

use egui::{Align2, Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, pos2, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// A section title: an accent icon, then the title in bold.
pub(crate) fn section(ui: &mut Ui, icon: Icon, title: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
        icons::paint(ui.painter(), r, icon, t.accent, false);
        ui.add_space(2.0);
        ui.label(egui::RichText::new(crate::i18n::tr(title)).font(theme::bold(13.5)).color(t.text));
    });
    ui.add_space(2.0);
}

/// A rounded card with a light border that groups related controls.
pub(crate) fn card<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.card)
        .stroke(Stroke::new(1.0, t.card_border))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add(ui)
        })
        .inner
}

fn paint_frame(ui: &Ui, rect: Rect, resp: &Response, selected: bool, enabled: bool, rest: Option<Color32>) {
    let t = Tokens::get(ui.ctx());
    let p = ui.painter();
    let fill = if !enabled {
        Color32::TRANSPARENT
    } else if selected {
        t.selected
    } else if resp.is_pointer_button_down_on() {
        t.pressed
    } else if resp.hovered() {
        t.hover
    } else {
        rest.unwrap_or(t.control)
    };
    p.rect_filled(rect, CornerRadius::same(6), fill);
    let stroke = if selected { Stroke::new(1.5, t.accent) } else { Stroke::new(1.0, t.border) };
    p.rect_stroke(rect, CornerRadius::same(6), stroke, StrokeKind::Inside);
    if resp.has_focus() {
        p.rect_stroke(rect.expand(2.0), CornerRadius::same(8), Stroke::new(1.5, t.text), StrokeKind::Outside);
    }
}

/// A button with an icon and its label (both translated), 30 px high.
pub(crate) fn icon_button(ui: &mut Ui, icon: Icon, label: &str, enabled: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let label = crate::i18n::tr(label);
    let galley = ui.painter().layout_no_wrap(label.to_string(), theme::font(13.0), t.text);
    let size = vec2(galley.size().x + 42.0, 30.0);
    let sense = if enabled { Sense::click() } else { Sense::hover() };
    let (rect, resp) = ui.allocate_exact_size(size, sense);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label));
    if ui.is_rect_visible(rect) {
        paint_frame(ui, rect, &resp, false, enabled, None);
        let ink = if enabled { t.text } else { t.text_faint };
        let icon_ink = if enabled { t.accent } else { t.text_faint };
        let ir = Rect::from_center_size(pos2(rect.left() + 18.0, rect.center().y), vec2(16.0, 16.0));
        icons::paint(ui.painter(), ir, icon, icon_ink, !enabled);
        ui.painter().text(pos2(rect.left() + 32.0, rect.center().y), Align2::LEFT_CENTER, label, theme::font(13.0), ink);
    }
    resp
}

/// The main button of a window: accent fill, an icon and its label (translated), 30 px high.
pub(crate) fn primary_button(ui: &mut Ui, icon: Icon, label: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let label = crate::i18n::tr(label);
    let font = theme::bold(13.0);
    let galley = ui.painter().layout_no_wrap(label.to_string(), font.clone(), t.accent_text);
    let size = vec2((galley.size().x + 50.0).max(110.0), 30.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        let fill = if resp.is_pointer_button_down_on() {
            t.accent.gamma_multiply(0.8)
        } else if resp.hovered() {
            t.accent.gamma_multiply(0.9)
        } else {
            t.accent
        };
        p.rect_filled(rect, CornerRadius::same(6), fill);
        if resp.has_focus() {
            p.rect_stroke(rect.expand(2.0), CornerRadius::same(8), Stroke::new(1.5, t.text), StrokeKind::Outside);
        }
        let content_w = 16.0 + 8.0 + galley.size().x;
        let x0 = rect.center().x - content_w / 2.0;
        let ir = Rect::from_center_size(pos2(x0 + 8.0, rect.center().y), vec2(16.0, 16.0));
        icons::paint(p, ir, icon, t.accent_text, false);
        p.text(pos2(x0 + 24.0, rect.center().y), Align2::LEFT_CENTER, label, font, t.accent_text);
    }
    resp
}

/// A square button showing only an icon; `label` (translated) is its tooltip and its name for
/// screen readers.
pub(crate) fn icon_only(ui: &mut Ui, icon: Icon, label: &str, enabled: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let label = crate::i18n::tr(label);
    let sense = if enabled { Sense::click() } else { Sense::hover() };
    let (rect, resp) = ui.allocate_exact_size(vec2(32.0, 30.0), sense);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label));
    if ui.is_rect_visible(rect) {
        paint_frame(ui, rect, &resp, false, enabled, None);
        let ir = Rect::from_center_size(rect.center(), vec2(18.0, 18.0));
        icons::paint(ui.painter(), ir, icon, if enabled { t.text } else { t.text_faint }, !enabled);
    }
    resp.on_hover_text(label)
}

/// A choice tile: a picture drawn by `paint` (in the given rect and ink) above its label. The
/// chosen tile has an accent border.
pub(crate) fn tile(ui: &mut Ui, size: egui::Vec2, label: &str, selected: bool, paint: impl FnOnce(&egui::Painter, Rect, Color32)) -> Response {
    let t = Tokens::get(ui.ctx());
    let label = crate::i18n::tr(label);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, label));
    if ui.is_rect_visible(rect) {
        paint_frame(ui, rect, &resp, selected, true, Some(t.card));
        let ink = if selected { t.accent } else { t.text };
        let font = if selected { theme::bold(11.5) } else { theme::font(11.5) };
        let galley = ui.painter().layout(label.to_string(), font, ink, rect.width() - 8.0);
        let text_h = galley.size().y;
        let art = Rect::from_min_max(rect.min + vec2(10.0, 8.0), pos2(rect.right() - 10.0, rect.bottom() - text_h - 10.0));
        paint(ui.painter(), art, if selected { t.accent } else { t.text_dim });
        let pos = pos2(rect.center().x - galley.size().x / 2.0, rect.bottom() - text_h - 5.0);
        ui.painter().galley(pos, galley, ink);
    }
    resp
}

/// A tile whose picture is one of the app's icons.
pub(crate) fn icon_tile(ui: &mut Ui, size: egui::Vec2, icon: Icon, label: &str, selected: bool) -> Response {
    tile(ui, size, label, selected, |p, r, ink| {
        let side = r.height().min(r.width()).min(30.0);
        icons::paint(p, Rect::from_center_size(r.center(), vec2(side, side)), icon, ink, false);
    })
}

/// A full-width row of a list: an icon, then the label; the chosen row is highlighted.
pub(crate) fn list_row(ui: &mut Ui, icon: Icon, label: &str, selected: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 28.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, selected, label));
    if ui.is_rect_visible(rect) {
        let p = ui.painter();
        if selected {
            p.rect_filled(rect, CornerRadius::same(5), t.selected);
            p.rect_filled(Rect::from_min_size(rect.min + vec2(0.0, 5.0), vec2(3.0, rect.height() - 10.0)), CornerRadius::same(1), t.accent);
        } else if resp.hovered() {
            p.rect_filled(rect, CornerRadius::same(5), t.hover);
        }
        let ink = if selected { t.accent } else { t.text_dim };
        icons::paint(p, Rect::from_center_size(pos2(rect.left() + 18.0, rect.center().y), vec2(16.0, 16.0)), icon, ink, false);
        let galley = p.layout_no_wrap(label.to_string(), theme::font(13.0), t.text);
        let clip = Rect::from_min_max(pos2(rect.left() + 32.0, rect.top()), pos2(rect.right() - 6.0, rect.bottom()));
        p.with_clip_rect(clip).galley(pos2(clip.left(), rect.center().y - galley.size().y / 2.0), galley, t.text);
    }
    resp
}

/// A help line: an info icon and a short explanation in small dimmed text (translated).
pub(crate) fn hint(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
        icons::paint(ui.painter(), r.shrink(1.0), Icon::Info, t.text_faint, false);
        ui.add(egui::Label::new(egui::RichText::new(crate::i18n::tr(text)).size(11.5).color(t.text_dim)).wrap());
    });
}

/// The label column of a form row, `width` px wide, right next to its control.
pub(crate) fn form_label(ui: &mut Ui, text: &str, width: f32) -> Response {
    ui.allocate_ui_with_layout(vec2(width, 24.0), egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.set_min_width(width);
        ui.label(crate::i18n::tr(text))
    })
    .inner
}

/// A thin line between parts of a window.
pub(crate) fn divider(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    let (r, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().line_segment([r.left_center(), r.right_center()], Stroke::new(1.0, t.separator));
    ui.add_space(6.0);
}
