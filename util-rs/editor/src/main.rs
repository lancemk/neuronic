// use eframe::egui::containers;
use eframe::{egui, epi};

#[derive(Default)]
struct MyEguiApp {}

impl epi::App for MyEguiApp {
    
    fn name(&self) -> &str {
        "My egui App"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });

        egui::ScrollArea::
    }
}

fn main() {
    let app = MyEguiApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
