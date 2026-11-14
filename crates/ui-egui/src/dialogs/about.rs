use super::{Color32, Rect, Sense, SlideApp, Tokens, Ui, pos2, theme, vec2};

/// Room the About credits need for their table; the About tab keeps its compact size.
pub(super) const CREDITS_SIZE: egui::Vec2 = vec2(820.0, 380.0);

/// The Contributors / Models body in a block of its own fixed size, below the tab row. Sizing the
/// dialog's `ui` instead (`set_min_size` / `set_max_height`) squeezed the tab row and the first
/// credits row together, so the text was garbled and the tabs stopped answering (#843).
pub(super) fn credits_block(ui: &mut Ui, models: bool) -> Rect {
    ui.allocate_ui(CREDITS_SIZE, |ui| {
        ui.set_min_size(CREDITS_SIZE);
        if models { crate::credits::models_ui(ui) } else { crate::credits::contributors_ui(ui) }
    })
    .response
    .rect
}

pub fn about(_app: &mut SlideApp, ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.vertical_centered(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(220.0, 127.0), Sense::hover());
        paint_wordmark(ui.painter(), r);
        ui.label(egui::RichText::new(crate::i18n::tr("Lusk Diapo")).font(theme::bold(22.0)));
        ui.label(egui::RichText::new(crate::i18n::trs(version_line())).color(t.text_dim));
        ui.add_space(6.0);
        ui.label(crate::i18n::tr("Présentations et diaporamas, libre et gratuit, porté par l'association Lusk (loi 1901)."));
        ui.label(crate::i18n::tr("Logiciel libre, sans lien avec Microsoft."));
        ui.add_space(6.0);
        ui.hyperlink_to(crate::i18n::tr("lusk.bzh"), deckcraft_engine::links::WEBSITE);
        ui.hyperlink_to(crate::i18n::tr("github.com/declic29/Lusk-Diapo"), deckcraft_engine::links::GITHUB);
        ui.add_space(6.0);
        ui.label(egui::RichText::new(crate::i18n::tr("© 2026 Lusk. Licence MIT ou Apache 2.0.")).size(11.0).color(t.text_faint));
        ui.label(
            egui::RichText::new(crate::i18n::tr("Dérivé de DeckCraft, © 2026 ArtCraft Team and the DeckCraft contributors."))
                .size(11.0)
                .color(t.text_faint),
        );
    });
}

/// "Version X.Y.Z" plus, for release builds, the short commit and build date. The version comes
/// from `[workspace.package] version` (the single source of truth, `cargo xtask version`); the
/// release workflow sets `DECKCRAFT_BUILD_SHA` / `DECKCRAFT_BUILD_DATE` at build time.
pub fn version_line() -> String {
    let mut s = format!("Version {}", env!("CARGO_PKG_VERSION"));
    let sha = option_env!("DECKCRAFT_BUILD_SHA").map(|s| s.get(..9).unwrap_or(s)).filter(|s| !s.is_empty());
    let date = option_env!("DECKCRAFT_BUILD_DATE").filter(|s| !s.is_empty());
    match (sha, date) {
        (Some(sha), Some(date)) => s.push_str(&format!(" ({sha}, {date})")),
        (Some(x), None) | (None, Some(x)) => s.push_str(&format!(" ({x})")),
        (None, None) => {}
    }
    s
}

/// Lusk Diapo's mark: the three stacked cards with the « D » of the Lusk Diapo logo
/// (`assets/app-icon/hicolor/256x256/apps/bzh.lusk.diapo.png`), square in the middle of `r`.
pub fn paint_logo(p: &egui::Painter, r: Rect) {
    let side = r.width().min(r.height());
    paint_png(
        p,
        "lusk_diapo_icon",
        include_bytes!("../../../../assets/app-icon/hicolor/256x256/apps/bzh.lusk.diapo.png"),
        Rect::from_center_size(r.center(), vec2(side, side)),
    );
}

/// The whole Lusk Diapo logo, cards and « Diapo » (`assets/app-icon/lusk-diapo-logo.png`), fitted
/// in the middle of `r` with its proportions kept.
pub fn paint_wordmark(p: &egui::Painter, r: Rect) {
    paint_png(p, "lusk_diapo_logo", include_bytes!("../../../../assets/app-icon/lusk-diapo-logo.png"), r);
}

/// Draws an embedded PNG fitted in `r` (proportions kept). The texture is decoded once per name.
fn paint_png(p: &egui::Painter, name: &'static str, png: &'static [u8], r: Rect) {
    let ctx = p.ctx();
    let id = egui::Id::unique(("lusk_png", name));
    let tex = ctx.data(|d| d.get_temp::<egui::TextureHandle>(id)).unwrap_or_else(|| {
        let img =
            image::load_from_memory(png).map(|i| i.to_rgba8()).unwrap_or_else(|_| image::RgbaImage::from_pixel(1, 1, image::Rgba([0, 0, 0, 0])));
        let tex = ctx.load_texture(
            name,
            egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw()),
            egui::TextureOptions::LINEAR,
        );
        ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
        tex
    });
    let [w, h] = tex.size().map(|v| v.max(1) as f32);
    let scale = (r.width() / w).min(r.height() / h);
    let dest = Rect::from_center_size(r.center(), vec2(w * scale, h * scale));
    p.image(tex.id(), dest, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
}

#[cfg(test)]
mod about_credits_tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::{CREDITS_SIZE, credits_block};
    use crate::dialogs::Dialog;
    use crate::{Services, SlideApp};

    fn tab_of(h: &Harness<'_, SlideApp>) -> String {
        let d = h.state().dialog.as_ref().expect("the About dialog is open");
        d.fields.get("tab").cloned().unwrap_or_else(|| "about".into())
    }

    /// The reported symptom (#843): in the real dialog the tabs stopped answering once Contributors
    /// or Models was open. Click through Contributors, Models and back to About.
    #[test]
    fn about_dialog_tabs_keep_responding() {
        let mut app = SlideApp::new(deckcraft_engine::Session::with_new(), Services::default());
        app.dialog = Some(Dialog::new("about"));
        let mut h = Harness::builder().with_size(egui::vec2(1440.0, 900.0)).with_step_dt(1.0 / 60.0).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        assert_eq!(tab_of(&h), "about");
        for label in ["Contributors", "Models", "About", "Models", "Contributors"] {
            h.get_by_label(label).click();
            h.run_steps(3);
            assert_eq!(tab_of(&h), label.to_lowercase(), "clicking {label} switches the tab");
        }
    }

    #[test]
    fn credits_get_their_own_block_below_the_tab_row() {
        for models in [false, true] {
            let ctx = egui::Context::default();
            let mut tabs = egui::Rect::NOTHING;
            let mut block = egui::Rect::NOTHING;
            let mut whole = egui::Rect::NOTHING;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                tabs = ui.label(crate::i18n::tr("About")).rect;
                block = credits_block(ui, models);
                whole = ui.min_rect();
            });
            out.textures_delta.clear();
            assert_eq!(block.size(), CREDITS_SIZE, "models={models}");
            assert!(block.min.y >= tabs.max.y, "the credits start below the tab row (models={models})");
            assert!(whole.max.y >= block.max.y, "the dialog grows to hold the block (models={models})");
            assert!(whole.height() > CREDITS_SIZE.y, "the tab row is not squeezed into the credits height (models={models})");
        }
    }
}
