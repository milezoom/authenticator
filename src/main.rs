#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 600.0)),
        ..Default::default()
    };

    eframe::run_simple_native("Authenticator", options, move |ctx, _frame| {
        egui::TopBottomPanel::top("heading").show(ctx, |ui| {
            ui.columns(2, |col| {
                col[0].heading("Authenticator");
                col[1].add_sized([10.0, 10.0], egui::Button::new("Add"));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new("Hello world!"));
            ui.label("some long text here, okay");
            if ui.button("click me").clicked() {}
        });
    })
}
