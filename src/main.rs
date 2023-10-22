use eframe::egui;
use egui::{Visuals, Style};
use egui_dock::{DockState, DockArea};

const INIT_WINDOW_SIZE: egui::Vec2 = egui::vec2(640.0, 480.0);

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: (Some(INIT_WINDOW_SIZE)),
        ..Default::default()
    };

    eframe::run_native("practice egui", options, Box::new(|cc| {
        let style = Style {visuals: Visuals::dark(), ..Style::default() };
        cc.egui_ctx.set_style(style);
        Box::new(PracticeApp::new())
    }))
}

struct PracticeApp {
    tree: DockState<String>
}

impl PracticeApp {
    fn new() -> Self { 
        let tree = DockState::new((0..10).into_iter().map(|n| format!("tab{}", n)).collect());
        PracticeApp { tree }
    }
}

impl eframe::App for PracticeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .show(ctx, &mut TabViewer {});
    }
}

struct TabViewer {}

impl egui_dock::TabViewer for TabViewer {
    type Tab = String;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        (&*tab).into()
    }
    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.label(tab.as_str());
    }
}
