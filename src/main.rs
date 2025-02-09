#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // not yet documented

use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::CentralPanel;
use eframe::egui::Context;
use eframe::egui::RichText;
use eframe::egui::ScrollArea;
use eframe::egui::TopBottomPanel;
use eframe::egui::ViewportBuilder;
use eframe::NativeOptions;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::time::Duration;
use totp_rs::Algorithm;
use totp_rs::Secret;
use totp_rs::TOTP;

#[derive(Serialize, Deserialize, Debug)]
struct Account {
    id: i32,
    account_name: String,
    username: String,
    secret: String,
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_simple_native("Authenticator", options, move |ctx, _frame| {
        ctx.request_repaint_after(Duration::from_secs(30));
        show_header(ctx);
        show_content(ctx);
    })
}

fn gen_totp(acc: &Account) -> String {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Raw(acc.secret.as_bytes().to_vec())
            .to_bytes()
            .unwrap(),
    )
    .unwrap();
    totp.generate_current().unwrap()
}

// fn update_accounts(ctx: &Context) {
//     let accounts: Vec<Account> = vec![Account {
//         id: 1,
//         account_name: String::from("Lorem Ipsum"),
//         username: String::from("loremia"),
//         secret: String::from("OVKGWTRTM5ZVETSHKN2FMODEIM3XQVKS"),
//     }];
//     let data = serde_json::to_string(&accounts).unwrap();
//     fs::write("accounts.json", data).expect("error writing accounts file:");
// }

fn show_header(ctx: &Context) {
    TopBottomPanel::top("heading").show(ctx, |ui| {
        ui.columns(2, |col| {
            // col[0].label(RichText::new("Authenticator").size(18.0));
            // if col[1]
            //     .add_sized([10.0, 10.0], Button::new(RichText::new("Add").size(18.0)))
            //     .clicked()
            // {
            //     update_accounts(ctx);
            // };
            col[1].centered_and_justified(|ui| {
                let _ = ui.button(RichText::new("Add").size(18.0));
            });
        });
    });
}

fn show_content(ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        let contents = fs::read_to_string("accounts.json").expect("error reading account file:");
        let accounts: Vec<Account> = serde_json::from_str(&contents).unwrap();
        ScrollArea::vertical()
            .auto_shrink(false)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                for acc in accounts.iter() {
                    ui.columns(1, |col| {
                        col[0].label(RichText::new(acc.account_name.to_string()).size(18.0));
                        col[0].label(RichText::new(acc.username.to_string()).size(14.0));
                        col[0].label(RichText::new(gen_totp(acc)).size(30.0));
                    });
                    ui.separator();
                }
            });
    });
}
