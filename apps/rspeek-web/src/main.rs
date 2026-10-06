//! RSpeek Web client entry point (WASM).

use eframe::egui;
use rspeek_ui::RSpeekUi;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
struct WebApp {
    ui: RSpeekUi,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl WebApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            ui: RSpeekUi::new(),
        }
    }
}

impl eframe::App for WebApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.ui.update(ui);
    }
}

fn main() {
    console_error_panic_hook::set_once();

    #[cfg(target_arch = "wasm32")]
    {
        use eframe::wasm_bindgen::JsCast as _;

        let document = web_sys::window()
            .expect("no global `window` exists")
            .document()
            .expect("should have a document on window");

        let canvas = document
            .get_element_by_id("rspeek_canvas")
            .expect("failed to find canvas element with id 'rspeek_canvas'")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("element is not a canvas");

        let web_options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async {
            eframe::WebRunner::new()
                .start(
                    canvas,
                    web_options,
                    Box::new(|cc| Ok(Box::new(WebApp::new(cc)))),
                )
                .await
                .expect("failed to start eframe WebRunner");
        });
    }
}
