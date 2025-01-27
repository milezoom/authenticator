#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::Button;
use eframe::egui::CentralPanel;
use eframe::egui::ScrollArea;
use eframe::egui::TopBottomPanel;
use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_simple_native("Authenticator", options, move |ctx, _frame| {
        TopBottomPanel::top("heading").show(ctx, |ui| {
            ui.columns(2, |col| {
                col[0].heading("Authenticator");
                col[1].add_sized([10.0, 10.0], Button::new("Add"));
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink(false)
                .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    for n in 0..100 {
                        ui.label(format!("Hello World #{}", n));
                    }
                });
        });
    })
}
