//! SVG pictures (a logo, an icon) drawn to pixels with resvg, at the size they are shown, like
//! metafiles ([`crate::metafile`]). Their text uses the computer's fonts.

use std::sync::OnceLock;

/// Largest side of a rasterized SVG, in pixels.
const MAX_SIDE: u32 = 4096;

/// Whether `bytes` look like an SVG document (or gzip-compressed SVGZ).
pub fn is_svg(bytes: &[u8]) -> bool {
    if bytes.starts_with(&[0x1F, 0x8B]) {
        return false;
    }
    let head = &bytes[..bytes.len().min(1024)];
    head.windows(4).any(|w| w == b"<svg")
}

/// Options for an SVG with text: the computer's fonts, loaded once (it takes a moment), only
/// when a picture needs them.
fn with_fonts() -> &'static resvg::usvg::Options<'static> {
    static OPTS: OnceLock<resvg::usvg::Options<'static>> = OnceLock::new();
    OPTS.get_or_init(|| {
        let mut o = resvg::usvg::Options::default();
        o.fontdb_mut().load_system_fonts();
        o
    })
}

fn tree(bytes: &[u8]) -> Option<resvg::usvg::Tree> {
    if bytes.windows(5).any(|w| w == b"<text") {
        resvg::usvg::Tree::from_data(bytes, with_fonts()).ok()
    } else {
        resvg::usvg::Tree::from_data(bytes, &resvg::usvg::Options::default()).ok()
    }
}

/// The SVG's own size in 96-dpi pixels (its `width` and `height`, or its view box).
pub fn size(bytes: &[u8]) -> Option<(f64, f64)> {
    let t = tree(bytes)?;
    let s = t.size();
    (s.width() > 0.0 && s.height() > 0.0).then(|| (s.width() as f64, s.height() as f64))
}

/// The SVG drawn with its longer side `side` px, as straight (not premultiplied) RGBA.
pub fn rasterize(bytes: &[u8], side: u32) -> Option<image::RgbaImage> {
    let t = tree(bytes)?;
    let s = t.size();
    let (pw, ph) = (s.width() as f64, s.height() as f64);
    if pw <= 0.0 || ph <= 0.0 {
        return None;
    }
    let side = side.clamp(16, MAX_SIDE) as f64;
    let k = side / pw.max(ph);
    let w = (pw * k).round().clamp(1.0, MAX_SIDE as f64) as u32;
    let h = (ph * k).round().clamp(1.0, MAX_SIDE as f64) as u32;
    let mut pm = resvg::tiny_skia::Pixmap::new(w, h)?;
    resvg::render(&t, resvg::tiny_skia::Transform::from_scale(w as f32 / pw as f32, h as f32 / ph as f32), &mut pm.as_mut());
    let mut img = image::RgbaImage::new(w, h);
    for (d, p) in img.pixels_mut().zip(pm.pixels()) {
        let c = p.demultiply();
        *d = image::Rgba([c.red(), c.green(), c.blue(), c.alpha()]);
    }
    Some(img)
}
