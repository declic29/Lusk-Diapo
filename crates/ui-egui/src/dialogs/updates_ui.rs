//! One background check at start-up (and on demand from Help) for a newer release, and the modal
//! prompt that offers to update and restart. The flow itself is `craft_update::updater::Updater`.

use craft_update::updater::{Action, Status, Updater};
use craft_update::{CheckResultDialog, CheckResultResponse};

use crate::SlideApp;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) struct Runtime {
    updater: Updater,
    /// The desktop host turns the check at start on; tests and the web build leave it off.
    enabled: bool,
    check_requested: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self { updater: Updater::new(&crate::UPDATE_APP, VERSION).with_translator(translate), enabled: false, check_requested: false }
    }
}

/// Lusk Diapo : les fenêtres de mise à jour (la proposition au démarrage et le résultat d'une
/// recherche) dans la langue de l'interface. Leurs textes anglais sont les clés de la table, leurs
/// `{valeurs}` remplies après la traduction.
fn translate(text: &str, args: &[(&str, &str)]) -> String {
    craft_update::translate::fill(crate::i18n::tr(text), args)
}

pub(crate) fn enable(app: &mut SlideApp) {
    app.updates_rt.enabled = true;
}

/// Help > Check for Updates…: runs a check and reports the outcome either way.
pub(crate) fn request_check(app: &mut SlideApp) {
    if !matches!(app.updates_rt.updater.status(), Status::Checking) {
        app.updates_rt.check_requested = true;
    }
}

/// Starts the helper of an update that is ready, so that it installs once the process has exited.
/// Later calls do nothing.
pub(crate) fn start_pending_update(app: &mut SlideApp) {
    app.updates_rt.updater.start_pending_update();
}

pub(crate) fn tick(app: &mut SlideApp, ctx: &egui::Context) {
    let rt = &mut app.updates_rt;
    if std::mem::take(&mut rt.check_requested) {
        rt.updater.check_now(ctx);
    } else if rt.enabled && app.ui.check_updates_at_start {
        rt.updater.check_at_start(ctx);
    }
    rt.updater.tick(ctx);
}

pub(crate) fn show(app: &mut SlideApp, ctx: &egui::Context) {
    // An installation starts, and a ready update asks the app to close, only while nothing is unsaved.
    let clean = !app.session.documents().iter().any(|d| d.is_dirty());
    match app.updates_rt.updater.show_if(ctx, clean) {
        Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
        // The app's close path asks about unsaved work; `on_exit` starts the restart helper.
        Action::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        Action::None => {}
    }
    // The result window reports a requested check, and at the first start after an update what
    // that update came to (installed, or why it failed).
    let rt = &mut app.updates_rt;
    if !matches!(rt.updater.status(), Status::Idle) {
        let response = CheckResultDialog::new(&crate::UPDATE_APP, VERSION).with_translator(translate).show(ctx, rt.updater.status());
        answer(rt, response);
    }
}

/// Fichier › Mises à jour : la version installée, la recherche et son résultat, le réglage de la
/// recherche au démarrage (repris de Lusk Tab).
pub(crate) fn page(app: &mut SlideApp, ui: &mut egui::Ui) {
    let t = crate::theme::Tokens::get(ui.ctx());
    let text = |ui: &mut egui::Ui, s: &str| {
        ui.add(egui::Label::new(egui::RichText::new(s).font(crate::theme::font(13.0)).color(t.text)).wrap());
    };
    ui.label(egui::RichText::new(format!("Lusk Diapo {VERSION}")).font(crate::theme::bold(16.0)).color(t.text));
    text(ui, "Version installée sur cet ordinateur.");
    ui.add_space(20.0);
    let status = app.updates_rt.updater.status().clone();
    let checking = matches!(status, Status::Checking);
    ui.horizontal(|ui| {
        if ui.add_enabled(!checking, egui::Button::new("Rechercher des mises à jour").min_size(egui::vec2(0.0, 30.0))).clicked() {
            request_check(app);
        }
        if checking {
            ui.add(egui::Spinner::new());
            ui.label("Recherche en cours…");
        }
    });
    let outcome = match &status {
        Status::UpToDate => Some(format!("Vous avez la dernière version de Lusk Diapo ({VERSION}).")),
        Status::Failed(e) if e == "no releases yet" => Some("Aucune version n'a encore été publiée sur GitHub.".to_string()),
        Status::Failed(e) => Some(format!("La recherche n'a pas abouti. Vérifiez la connexion à Internet, puis réessayez.\n(Détail : {e})")),
        Status::Installing => Some("Une nouvelle version est en cours de téléchargement et d'installation.".to_string()),
        Status::Ready => Some("Une nouvelle version est prête : elle s'installera à la fermeture de Lusk Diapo.".to_string()),
        Status::Managed => Some("Les mises à jour de Lusk Diapo sont gérées par un autre programme sur cet ordinateur.".to_string()),
        Status::Updated(v) => Some(format!("Lusk Diapo a été mis à jour en version {v}.")),
        Status::InstallFailed(e) => Some(format!("La dernière mise à jour n'a pas pu être installée : {e}")),
        _ => None,
    };
    if let Some(outcome) = outcome {
        ui.add_space(8.0);
        text(ui, &outcome);
    }
    if matches!(status, Status::Ready) && ui.button("Redémarrer pour mettre à jour").clicked() {
        app.updates_rt.updater.restart_to_update();
    }
    ui.add_space(20.0);
    let mut at_start = app.ui.check_updates_at_start;
    if ui.checkbox(&mut at_start, "Rechercher automatiquement les mises à jour au démarrage").changed() {
        app.ui.check_updates_at_start = at_start;
    }
    ui.add_space(20.0);
    text(
        ui,
        "Les nouvelles versions sont publiées par l'association Lusk. Quand une version plus récente existe, Lusk Diapo \
         propose de la télécharger et vérifie son empreinte avant de l'installer. L'installation attend que toutes vos \
         présentations soient enregistrées.",
    );
    ui.add_space(8.0);
    text(ui, "Seul le numéro de la dernière version est demandé : aucune présentation ni aucune donnée personnelle n'est envoyée.");
}

/// Acts on the result window's answer.
fn answer(rt: &mut Runtime, response: CheckResultResponse) {
    match response {
        CheckResultResponse::Close => rt.updater.dismiss_status(),
        CheckResultResponse::CheckAgain => rt.check_requested = true,
        // The next frame's `show_if` asks the window to close once nothing is unsaved.
        CheckResultResponse::RestartToUpdate => rt.updater.restart_to_update(),
        CheckResultResponse::None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_offered_release_opens_the_prompt() {
        let mut app = SlideApp::new(deckcraft_engine::Session::new(), crate::Services::default());
        let ctx = egui::Context::default();
        assert!(!app.updates_rt.updater.prompt_open());
        app.updates_rt.updater.offer(craft_update::Release { tag: "v99.0.0".into(), ..Default::default() });
        ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx())).textures_delta.clear();
        assert!(app.updates_rt.updater.prompt_open());
    }

    /// An app whose updater keeps its outcome records under `root`, reads no launcher choice and
    /// asks no server, with the check at start turned on.
    fn app_with_records(root: &std::path::Path) -> SlideApp {
        let mut app = SlideApp::new(deckcraft_engine::Session::new(), crate::Services::default());
        app.updates_rt.updater = Updater::new(&crate::UPDATE_APP, VERSION)
            .with_translator(translate)
            .with_paths(craft_update::Paths::under(root))
            .with_pins_path(root.join("pins.json"))
            .with_source(std::sync::Arc::new(|| Ok(None)));
        app.updates_rt.enabled = true;
        app
    }

    fn records_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("deckcraft-updates-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    /// The first start after an update reports it; OK dismisses the report.
    #[test]
    fn the_start_after_an_update_reports_it_until_dismissed() {
        let root = records_dir("installed");
        std::fs::create_dir_all(root.join("data")).unwrap();
        std::fs::write(root.join("data").join("self-update-done.txt"), VERSION).unwrap();
        let mut app = app_with_records(&root);
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            tick(&mut app, ui.ctx());
            show(&mut app, ui.ctx());
        })
        .textures_delta
        .clear();
        assert!(matches!(app.updates_rt.updater.status(), Status::Updated(_)), "{:?}", app.updates_rt.updater.status());
        answer(&mut app.updates_rt, CheckResultResponse::Close);
        assert_eq!(app.updates_rt.updater.status(), &Status::Idle);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// The first start after a failed update says why; Check Again asks for a new check.
    #[test]
    fn the_start_after_a_failed_update_reports_why_and_offers_a_new_check() {
        let root = records_dir("failed");
        std::fs::create_dir_all(root.join("data").join("logs")).unwrap();
        std::fs::write(root.join("data").join("logs").join("self-update-error.txt"), "the disk is full").unwrap();
        let mut app = app_with_records(&root);
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            tick(&mut app, ui.ctx());
            show(&mut app, ui.ctx());
        })
        .textures_delta
        .clear();
        assert_eq!(app.updates_rt.updater.status(), &Status::InstallFailed("the disk is full".into()));
        answer(&mut app.updates_rt, CheckResultResponse::CheckAgain);
        assert!(app.updates_rt.check_requested);
        let _ = std::fs::remove_dir_all(&root);
    }
}
