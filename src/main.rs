use eframe::egui;

const INIT_WINDOW_SIZE: egui::Vec2 = egui::vec2(640.0, 480.0);

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: (Some(INIT_WINDOW_SIZE)),
        ..Default::default()
    };

    eframe::run_native("practice egui", options, Box::new(|_| { Box::new(PracticeApp::new())}))
}

struct PracticeApp {
    message: String,
}

impl PracticeApp {
    fn new() -> Self { PracticeApp { message: "Hello World".to_string() }}
}

impl eframe::App for PracticeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.message.as_str());
        });
    }
}
