// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::Button;
use eframe::egui::CentralPanel;
use eframe::egui::RichText;
use eframe::egui::ScrollArea;
use eframe::egui::TopBottomPanel;
use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;
use std::time::Duration;
use totp_rs::Algorithm;
use totp_rs::Secret;
use totp_rs::TOTP;

// fn main() -> Result<(), eframe::Error> {
//     let options = NativeOptions {
//         viewport: ViewportBuilder::default()
//             .with_inner_size([400.0, 600.0])
//             .with_resizable(false),
//         ..Default::default()
//     };

//     eframe::run_simple_native("Authenticator", options, move |ctx, _frame| {
//         ctx.request_repaint_after(Duration::from_secs(30));
//         TopBottomPanel::top("heading").show(ctx, |ui| {
//             ui.columns(2, |col| {
//                 col[0].label(RichText::new("Authenticator").size(18.0));
//                 col[1].add_sized([10.0, 10.0], Button::new(RichText::new("Add").size(18.0)));
//             });
//         });

//         CentralPanel::default().show(ctx, |ui| {
//             ScrollArea::vertical()
//                 .auto_shrink(false)
//                 .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
//                 .show(ui, |ui| {
//                     for n in 0..25 {
//                         ui.columns(1, |col| {
//                             col[0].label(RichText::new(format!("Item Number #{}", n)).size(16.0));
//                             col[0].label(RichText::new(format!("{}", gen_totp())).size(24.0));
//                         });
//                     }
//                 });
//         });
//     })
// }

// fn gen_totp() -> String {
//     let totp = TOTP::new(
//         Algorithm::SHA1,
//         6,
//         1,
//         30,
//         Secret::Raw("OVKGWTRTM5ZVETSHKN2FMODEIM3XQVKS".as_bytes().to_vec())
//             .to_bytes()
//             .unwrap(),
//     )
//     .unwrap();
//     totp.generate_current().unwrap()
// }

fn main() {
    println!("hello world")
}
