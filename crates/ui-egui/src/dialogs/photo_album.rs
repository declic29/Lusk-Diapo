use super::{Dialog, SlideApp, Tokens, Ui, Value, buttons, check, json, kit};
use crate::icons::Icon;

fn album_items(d: &Dialog) -> Option<Vec<Value>> {
    let raw = d.fields.get("items").map(String::as_str).unwrap_or("[]");
    serde_json::from_str::<Value>(raw).ok().and_then(|v| v.as_array().cloned())
}

fn store_album_items(d: &mut Dialog, items: &[Value]) {
    let raw = serde_json::to_string(items).unwrap_or_else(|_| "[]".into());
    d.fields.insert("items".into(), raw);
}

fn album_item_label(v: &Value) -> String {
    if let Some(text) = v.get("text").and_then(Value::as_str) {
        let text = text.trim();
        if text.is_empty() { crate::i18n::trs("Text Box") } else { crate::i18n::trf("Text: {0}", &[&text]) }
    } else {
        v.get("name").and_then(Value::as_str).filter(|s| !s.is_empty()).unwrap_or("Picture").to_string()
    }
}

fn pick_album_picture(app: &mut SlideApp) -> Option<Value> {
    let path = if let Some(pick) = app.services.pick_open.as_mut() {
        pick("picture")
    } else {
        app.set_status("Pass pictures to insert.photoAlbum");
        None
    };
    let path = path?;
    let loaded = if let Some(read) = app.services.read.as_ref() {
        match read(&path) {
            Ok(bytes) => bytes,
            Err(e) => {
                app.set_status(e);
                return None;
            }
        }
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match std::fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) => {
                    app.set_status(format!("{path}: {e}"));
                    return None;
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            app.set_status("The web build inserts album pictures from data passed to insert.photoAlbum");
            return None;
        }
    };
    if loaded.is_empty() {
        app.set_status("That file is empty");
        return None;
    }
    if loaded.len() > 32 * 1024 * 1024 {
        app.set_status("That picture is larger than 32 MB");
        return None;
    }
    let name = std::path::Path::new(&path).file_name().and_then(std::ffi::OsStr::to_str).filter(|s| !s.is_empty()).unwrap_or("picture").to_string();
    Some(json!({"name": name, "data": deckcraft_engine::cmd::base64_encode(&loaded)}))
}

/// Draw an accessible album combo box, preserving its current/default selection and storing a
/// newly chosen key; the caller supplies the visible label.
fn album_choice(ui: &mut Ui, d: &mut Dialog, id: &str, name: &str, key: &str, default: &str, choices: &[(&str, &str)]) {
    let cur = d.get(key, default);
    let label = choices.iter().find(|(k, _)| *k == cur).map(|(_, l)| *l).unwrap_or(default);
    let opened = craft_ui::ComboBox::from_id_salt(id).selected_text(crate::i18n::trs(label)).width(200.0).show_ui(ui, |ui| {
        for (k, l) in choices {
            if ui.selectable_label(cur == *k, crate::i18n::trs(*l)).clicked() {
                d.fields.insert(key.into(), (*k).to_string());
            }
        }
    });
    let access = crate::i18n::trs(name);
    let shown = crate::i18n::trs(label);
    opened.response.widget_info(move || {
        let mut info = egui::WidgetInfo::labeled(egui::Role::ComboBox, true, access.clone());
        info.current_text_value = Some(shown.clone());
        info
    });
}

const ALBUM_THEMES: &[&str] = &["Lusk Diapo", "Harbor", "Ember", "Meadow", "Nocturne", "Paper", "Coral Reef", "Slate"];

fn current_theme(app: &SlideApp) -> String {
    app.session
        .active()
        .and_then(|d| d.doc.masters.first())
        .map(|m| m.theme.name.clone())
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| "Lusk Diapo".into())
}

fn album_json_id(v: &Value) -> Option<u64> {
    v.as_u64().or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))
}

/// Copy a command's dialog params into fields once. A fresh album has no `items`.
fn seed_photo_album(app: &SlideApp, d: &mut Dialog) {
    if d.fields.contains_key("seeded") {
        return;
    }
    d.fields.insert("seeded".into(), "1".into());
    let p = d.params.clone();
    if let Some(items) = p.get("items").and_then(Value::as_array) {
        store_album_items(d, items);
        d.fields.insert("editing".into(), "1".into());
    }
    for key in ["layout", "frame", "theme"] {
        if let Some(v) = p.get(key).and_then(Value::as_str).filter(|s| !s.is_empty()) {
            d.fields.insert(key.into(), v.to_string());
        }
    }
    for key in ["captions", "grayscale"] {
        if let Some(v) = p.get(key).and_then(Value::as_bool) {
            d.fields.insert(key.into(), if v { "1".into() } else { "0".into() });
        }
    }
    if let Some(id) = p.get("album").and_then(album_json_id).filter(|n| *n != 0) {
        d.fields.insert("album".into(), id.to_string());
    }
    let theme_was = d.fields.get("theme").cloned().filter(|s| !s.is_empty()).unwrap_or_else(|| current_theme(app));
    if !d.fields.contains_key("theme") {
        d.fields.insert("theme".into(), theme_was.clone());
    }
    d.fields.entry("themeWas".into()).or_insert(theme_was);
}

fn selected_album_picture(d: &Dialog, items: &[Value]) -> bool {
    let i = d.fields.get("sel").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    items.get(i).is_some_and(|item| item.get("data").is_some() || item.get("path").is_some())
}

fn with_selected_picture(d: &mut Dialog, f: impl FnOnce(&mut serde_json::Map<String, Value>)) {
    let Some(mut list) = album_items(d) else { return };
    let i = d.fields.get("sel").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let Some(item) = list.get_mut(i) else { return };
    if item.get("data").is_none() && item.get("path").is_none() {
        return;
    }
    let Some(obj) = item.as_object_mut() else { return };
    f(obj);
    store_album_items(d, &list);
}

/// A small picture of an album item for the list and the preview, made once per picture.
fn thumbnail(ui: &Ui, item: &Value) -> Option<egui::TextureHandle> {
    let data = item.get("data")?.as_str()?;
    let key = egui::Id::unique(("album-thumb", data.len(), &data[..data.len().min(96)], &data[data.len().saturating_sub(96)..]));
    if let Some(tex) = ui.data(|d| d.get_temp::<egui::TextureHandle>(key)) {
        return Some(tex);
    }
    let bytes = deckcraft_engine::cmd::base64_decode(data)?;
    let img = deckcraft_render::decode_image(&bytes)?;
    let color = egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw());
    let tex = crate::textures::upload(ui.ctx(), "album-thumb", crate::textures::shrink_to(color, 320));
    ui.data_mut(|d| d.insert_temp(key, tex.clone()));
    Some(tex)
}

/// A sketch of the album's slides: `layout` pictures (with a title bar when it has one) in
/// `frame` shape.
fn layout_sketch(ui: &mut Ui, layout: &str, frame: &str) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(egui::vec2(150.0, 86.0), egui::Sense::hover());
    let p = ui.painter();
    p.rect_filled(r, egui::CornerRadius::same(4), t.canvas);
    p.rect_stroke(r, egui::CornerRadius::same(4), egui::Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let mut area = r.shrink(8.0);
    if layout.ends_with("title") {
        p.rect_filled(egui::Rect::from_min_size(area.min, egui::vec2(area.width() * 0.6, 7.0)), egui::CornerRadius::same(2), t.text_faint);
        area.min.y += 13.0;
    }
    let n = match layout.trim_end_matches("title") {
        "2" => 2,
        "4" => 4,
        _ => 1,
    };
    let (cols, rows) = if n == 4 { (2, 2) } else { (n, 1) };
    let gap = 5.0;
    let (w, h) = ((area.width() - gap * (cols as f32 - 1.0)) / cols as f32, (area.height() - gap * (rows as f32 - 1.0)) / rows as f32);
    let full = layout == "fit";
    for i in 0..n {
        let cell = egui::Rect::from_min_size(area.min + egui::vec2((i % cols) as f32 * (w + gap), (i / cols) as f32 * (h + gap)), egui::vec2(w, h));
        let pic = if full { r.shrink(1.0) } else { egui::Rect::from_center_size(cell.center(), egui::vec2(w.min(h * 1.4), h)) };
        let round = if frame == "rounded" { 6 } else { 1 };
        p.rect_filled(pic, egui::CornerRadius::same(round), t.accent.gamma_multiply(0.55));
        if frame == "simple" {
            p.rect_stroke(pic, egui::CornerRadius::same(round), egui::Stroke::new(2.5, egui::Color32::WHITE), egui::StrokeKind::Inside);
        } else if frame != "none" {
            p.rect_stroke(pic, egui::CornerRadius::same(round), egui::Stroke::new(1.0, t.text_dim), egui::StrokeKind::Inside);
        }
    }
}

/// Photo Album. Returns true when the dialog should close.
pub(super) fn photo_album_dialog(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog) -> bool {
    seed_photo_album(app, d);
    let t = Tokens::get(ui.ctx());
    ui.set_width(640.0);
    let items = album_items(d).unwrap_or_default();
    let mut sel = d.get("sel", "0").parse::<usize>().unwrap_or(0);
    // The list's buttons, above the two columns.
    ui.horizontal(|ui| {
        if kit::icon_button(ui, Icon::Picture, "Add Pictures…", true).clicked()
            && let Some(item) = pick_album_picture(app)
        {
            match album_items(d) {
                Some(mut list) if list.len() < 48 => {
                    list.push(item);
                    let next = list.len().saturating_sub(1);
                    store_album_items(d, &list);
                    d.fields.insert("sel".into(), next.to_string());
                }
                Some(_) => app.set_status("A photo album holds at most 48 pictures or text boxes"),
                None => app.set_status("The picture list is invalid"),
            }
        }
        if kit::icon_button(ui, Icon::TextBox, "New Text Box", true).clicked() {
            let text = d.get("boxText", crate::i18n::tr("Text Box"));
            let text = text.trim();
            if text.is_empty() {
                app.set_status("Type the text for the new text box");
            } else if text.chars().count() > 400 {
                app.set_status("Text is longer than 400 characters");
            } else if let Some(mut list) = album_items(d) {
                if list.len() >= 48 {
                    app.set_status("A photo album holds at most 48 pictures or text boxes");
                } else {
                    list.push(json!({"text": text}));
                    let next = list.len().saturating_sub(1);
                    store_album_items(d, &list);
                    d.fields.insert("sel".into(), next.to_string());
                }
            }
        }
        if kit::icon_button(ui, Icon::DeleteSlide, "Remove", !items.is_empty()).clicked()
            && let Some(mut list) = album_items(d)
            && let Some(i) = d.get("sel", "0").parse::<usize>().ok()
            && i < list.len()
        {
            list.remove(i);
            store_album_items(d, &list);
            let next = i.min(list.len().saturating_sub(1));
            d.fields.insert("sel".into(), next.to_string());
        }
    });
    ui.add_space(6.0);
    let items = album_items(d).unwrap_or_default();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(300.0);
            kit::section(ui, Icon::Image, "Pictures in album");
            kit::card(ui, |ui| {
                // Fixed height so adding pictures does not push Create past the window.
                egui::ScrollArea::vertical().max_height(190.0).min_scrolled_height(190.0).auto_shrink([false, false]).id_salt("album_list").show(
                    ui,
                    |ui| {
                        if items.is_empty() {
                            ui.add_space(50.0);
                            ui.vertical_centered(|ui| {
                                let (r, _) = ui.allocate_exact_size(egui::vec2(34.0, 34.0), egui::Sense::hover());
                                crate::icons::paint(ui.painter(), r, Icon::Image, t.text_faint, false);
                                ui.label(egui::RichText::new(crate::i18n::tr("No pictures yet")).color(t.text_dim));
                                ui.label(egui::RichText::new(crate::i18n::tr("Add Pictures… to start")).size(11.5).color(t.text_faint));
                            });
                        }
                        for (i, item) in items.iter().enumerate() {
                            let icon = if item.get("text").is_some() { Icon::TextBox } else { Icon::Image };
                            if kit::list_row(ui, icon, &album_item_label(item), sel == i).clicked() {
                                sel = i;
                            }
                        }
                    },
                );
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let l = kit::form_label(ui, "Text box:", 90.0);
                let mut v = d.get("boxText", crate::i18n::tr("Text Box"));
                ui.add(egui::TextEdit::singleline(&mut v).desired_width(f32::INFINITY)).labelled_by(l.id);
                d.fields.insert("boxText".into(), v);
            });
        });
        ui.add_space(12.0);
        ui.vertical(|ui| {
            kit::section(ui, Icon::Preview, "Preview");
            kit::card(ui, |ui| {
                let (r, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 150.0), egui::Sense::hover());
                match items.get(sel) {
                    Some(item) if item.get("text").is_some() => {
                        ui.painter().rect_filled(r.shrink(14.0), egui::CornerRadius::same(4), t.canvas);
                        let text = item.get("text").and_then(Value::as_str).unwrap_or("");
                        let galley = ui.painter().layout(text.to_string(), crate::theme::font(14.0), t.text, r.width() - 40.0);
                        ui.painter().galley(r.center() - galley.size() / 2.0, galley, t.text);
                    }
                    Some(item) => {
                        if let Some(tex) = thumbnail(ui, item) {
                            let rot = item.get("rot").and_then(Value::as_f64).unwrap_or(0.0);
                            let turned = ((rot / 90.0).round() as i64).rem_euclid(2) == 1;
                            let [w, h] = tex.size().map(|v| v as f32);
                            let (sw, sh) = if turned { (h, w) } else { (w, h) };
                            let k = (r.width() / sw).min(r.height() / sh);
                            let size = egui::vec2(w * k, h * k);
                            let gray = item.get("grayscale").and_then(Value::as_bool).unwrap_or(false)
                                || d.fields.get("grayscale").is_some_and(|v| v == "1");
                            let tint = if gray { egui::Color32::from_gray(200) } else { egui::Color32::WHITE };
                            // The picture turned like it will be on the slide.
                            let mut mesh = egui::Mesh::with_texture(tex.id());
                            let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                            mesh.add_rect_with_uv(egui::Rect::from_center_size(r.center(), size), uv, tint);
                            mesh.rotate(egui::emath::Rot2::from_angle((rot as f32).to_radians()), r.center());
                            ui.painter().add(egui::Shape::mesh(mesh));
                        } else {
                            crate::icons::paint(
                                ui.painter(),
                                egui::Rect::from_center_size(r.center(), egui::vec2(40.0, 40.0)),
                                Icon::Image,
                                t.text_faint,
                                false,
                            );
                        }
                    }
                    None => {
                        crate::icons::paint(
                            ui.painter(),
                            egui::Rect::from_center_size(r.center(), egui::vec2(40.0, 40.0)),
                            Icon::Image,
                            t.text_faint,
                            false,
                        );
                    }
                }
            });
            let picture = selected_album_picture(d, &items);
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if kit::icon_only(ui, Icon::RotateLeft, "Rotate Left", picture).clicked() {
                    with_selected_picture(d, |obj| {
                        let cur = obj.get("rot").and_then(Value::as_f64).unwrap_or(0.0);
                        obj.insert("rot".into(), json!(((cur - 90.0) % 360.0 + 360.0) % 360.0));
                    });
                }
                if kit::icon_only(ui, Icon::RotateRight, "Rotate Right", picture).clicked() {
                    with_selected_picture(d, |obj| {
                        let cur = obj.get("rot").and_then(Value::as_f64).unwrap_or(0.0);
                        obj.insert("rot".into(), json!(((cur + 90.0) % 360.0 + 360.0) % 360.0));
                    });
                }
            });
            ui.horizontal(|ui| {
                kit::form_label(ui, "Brightness", 80.0);
                if kit::icon_only(ui, Icon::Minus, "Less Brightness", picture).clicked() {
                    nudge_picture(d, "brightness", -0.1);
                }
                if kit::icon_only(ui, Icon::Plus, "More Brightness", picture).clicked() {
                    nudge_picture(d, "brightness", 0.1);
                }
            });
            ui.horizontal(|ui| {
                kit::form_label(ui, "Contrast", 80.0);
                if kit::icon_only(ui, Icon::Minus, "Less Contrast", picture).clicked() {
                    nudge_picture(d, "contrast", -0.1);
                }
                if kit::icon_only(ui, Icon::Plus, "More Contrast", picture).clicked() {
                    nudge_picture(d, "contrast", 0.1);
                }
            });
            let items = album_items(d).unwrap_or_default();
            ui.label(egui::RichText::new(album_picture_status(&items, sel)).size(11.5).color(t.text_dim));
        });
    });
    d.fields.insert("sel".into(), sel.to_string());
    ui.add_space(4.0);
    kit::section(ui, Icon::Layout, "Album layout");
    kit::card(ui, |ui| {
        ui.horizontal_top(|ui| {
            egui::Grid::new("album-layout").num_columns(2).spacing(egui::vec2(10.0, 8.0)).show(ui, |ui| {
                kit::form_label(ui, "Picture layout", 140.0);
                album_choice(
                    ui,
                    d,
                    "album_layout",
                    "Picture layout",
                    "layout",
                    "1",
                    &[
                        ("fit", "Fit to slide"),
                        ("1", "1 picture"),
                        ("2", "2 pictures"),
                        ("4", "4 pictures"),
                        ("1title", "1 picture with title"),
                        ("2title", "2 pictures with title"),
                        ("4title", "4 pictures with title"),
                    ],
                );
                ui.end_row();
                kit::form_label(ui, "Frame shape", 140.0);
                album_choice(
                    ui,
                    d,
                    "album_frame",
                    "Frame shape",
                    "frame",
                    "rect",
                    &[("none", "None"), ("rect", "Rectangle"), ("rounded", "Rounded"), ("simple", "Simple, white")],
                );
                ui.end_row();
                kit::form_label(ui, "Album theme", 140.0);
                album_choice(ui, d, "album_theme", "Album theme", "theme", "Lusk Diapo", &ALBUM_THEMES.iter().map(|n| (*n, *n)).collect::<Vec<_>>());
                ui.end_row();
            });
            ui.add_space(12.0);
            let (layout, frame) = (d.get("layout", "1"), d.get("frame", "rect"));
            layout_sketch(ui, &layout, &frame);
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            check(ui, d, "Captions under pictures", "captions", false);
            ui.add_space(16.0);
            check(ui, d, "ALL pictures black and white", "grayscale", false);
        });
    });
    let captions = d.fields.get("captions").is_some_and(|v| v == "1");
    let grayscale = d.fields.get("grayscale").is_some_and(|v| v == "1");
    let editing = d.fields.get("editing").is_some_and(|v| v == "1");
    let (ok, cancel) = buttons(ui, if editing { "Update" } else { "Create" });
    if ok {
        let Some(list) = album_items(d) else {
            app.set_status("The picture list is invalid");
            return false;
        };
        if list.is_empty() {
            app.set_status("Add a picture or a text box first");
            return false;
        }
        let layout = d.get("layout", "1");
        let frame = d.get("frame", "rect");
        let theme = d.get("theme", "Lusk Diapo");
        let theme_was = d.fields.get("themeWas").cloned().unwrap_or_default();
        let mut payload = json!({"pictures": list, "layout": layout, "frame": frame, "captions": captions, "grayscale": grayscale});
        if theme != theme_was
            && let Some(obj) = payload.as_object_mut()
        {
            obj.insert("theme".into(), json!(theme));
        }
        if editing {
            if let Some(album) = d.fields.get("album").filter(|s| !s.is_empty() && s.as_str() != "0")
                && let Ok(id) = album.parse::<u64>()
                && let Some(obj) = payload.as_object_mut()
            {
                obj.insert("album".into(), json!(id));
            }
            return app.run("insert.photoAlbumEdit", payload).is_ok();
        }
        return app.run("insert.photoAlbum", payload).is_ok();
    }
    cancel
}

fn album_picture_status(items: &[Value], sel: usize) -> String {
    let Some(item) = items.get(sel) else {
        return crate::i18n::trs("Select a picture to rotate it or change brightness and contrast");
    };
    if item.get("data").is_none() && item.get("path").is_none() {
        return "Text boxes have no picture options".into();
    }
    let rot = item.get("rot").and_then(Value::as_f64).unwrap_or(0.0);
    let brightness = item.get("brightness").and_then(Value::as_f64).unwrap_or(0.0);
    let contrast = item.get("contrast").and_then(Value::as_f64).unwrap_or(0.0);
    crate::i18n::trf("Rotation {0}°, brightness {1}, contrast {2}", &[&format!("{rot:.0}"), &format!("{brightness:.1}"), &format!("{contrast:.1}")])
}

fn nudge_picture(d: &mut Dialog, key: &str, delta: f64) {
    with_selected_picture(d, |obj| {
        let cur = obj.get(key).and_then(Value::as_f64).unwrap_or(0.0);
        let next = (((cur + delta) * 10.0).round() / 10.0).clamp(-1.0, 1.0);
        obj.insert(key.into(), json!(next));
    });
}
