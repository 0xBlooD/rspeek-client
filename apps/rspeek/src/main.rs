//! RSpeek desktop application entry point.

use eframe::egui;
use rspeek_ui::RSpeekUi;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct DesktopApp {
    ui: RSpeekUi,
}

impl DesktopApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ui: RSpeekUi::new(),
        }
    }
}

impl eframe::App for DesktopApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui.update(ui);
    }
}

fn main() -> eframe::Result {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rspeek=info,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting RSpeek Desktop Client");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([640.0, 480.0])
            .with_title("RSpeek"),
        ..Default::default()
    };

    eframe::run_native(
        "RSpeek",
        options,
        Box::new(|cc| Ok(Box::new(DesktopApp::new(cc)))),
    )
}
