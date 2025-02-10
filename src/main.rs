#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // not yet documented

use arboard::Clipboard;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::CentralPanel;
use eframe::egui::Context;
use eframe::egui::RichText;
use eframe::egui::ScrollArea;
use eframe::egui::TopBottomPanel;
use eframe::egui::ViewportBuilder;
use eframe::run_native;
use eframe::App;
use eframe::Frame;
use eframe::NativeOptions;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use totp_rs::Algorithm;
use totp_rs::Secret;
use totp_rs::TOTP;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Account {
    id: i32,
    account_name: String,
    username: String,
    secret: String,
}

fn gen_totp(acc: &Account) -> String {
    let secret = Secret::Encoded(acc.secret.to_string());
    let totp = TOTP::new_unchecked(Algorithm::SHA1, 6, 1, 30, secret.to_bytes().unwrap());
    totp.generate_current().unwrap()
}

enum Page {
    PageList,
    PageEdit,
}

struct AuthenticatorApp {
    page: Page,
    selected_id: i32,
    selected_account_name: String,
    selected_username: String,
    selected_secret: String,
}

impl Default for AuthenticatorApp {
    fn default() -> Self {
        Self {
            page: Page::PageList,
            selected_id: 0,
            selected_account_name: String::from(""),
            selected_username: String::from(""),
            selected_secret: String::from(""),
        }
    }
}

impl App for AuthenticatorApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        match self.page {
            Page::PageList => show_page_list(self, ctx),
            Page::PageEdit => show_page_edit(self, ctx),
        }
    }
}

fn show_page_list(app: &mut AuthenticatorApp, ctx: &Context) {
    ctx.request_repaint_after(Duration::from_secs(10));

    TopBottomPanel::top("heading").show(ctx, |ui| {
        ui.columns(2, |col| {
            col[1].centered_and_justified(|ui| {
                let btn_add = ui.button(RichText::new("Add").size(18.0));
                if btn_add.clicked() {
                    app.page = Page::PageEdit;
                    ctx.request_repaint();
                }
            });
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::vertical()
            .auto_shrink(false)
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .show(ui, |ui| {
                for acc in read_file_account().iter() {
                    if acc.secret == "" {
                        continue;
                    }
                    ui.columns(2, |col| {
                        let totp_code = gen_totp(acc);
                        col[0].vertical(|ui| {
                            ui.label(RichText::new(acc.account_name.to_string()).size(18.0));
                            ui.label(RichText::new(acc.username.to_string()).size(14.0));
                            ui.label(RichText::new(totp_code.clone()).size(30.0));
                        });
                        col[1].vertical_centered_justified(|ui| {
                            let btn_edit = ui.button(RichText::new("Edit").size(18.0));
                            if btn_edit.clicked() {
                                app.selected_id = acc.id;
                                app.selected_account_name = acc.account_name.clone();
                                app.selected_username = acc.username.clone();
                                app.selected_secret = acc.secret.clone();
                                app.page = Page::PageEdit;
                                ctx.request_repaint();
                            }
                            let btn_copy = ui.button(RichText::new("Copy").size(18.0));
                            if btn_copy.clicked() {
                                Clipboard::new()
                                    .unwrap()
                                    .set_text(totp_code.clone())
                                    .unwrap();
                            }
                        });
                    });
                    ui.separator();
                }
            });
    });
}

fn show_page_edit(app: &mut AuthenticatorApp, ctx: &Context) {
    TopBottomPanel::top("heading").show(ctx, |ui| {
        ui.columns(2, |col| {
            col[1].centered_and_justified(|ui| {
                let btn_save = ui.button(RichText::new("Save").size(18.0));
                if btn_save.clicked() {
                    update_accounts(app);
                    app.page = Page::PageList;
                    ctx.request_repaint();
                }
            });
        });
    });

    CentralPanel::default().show(ctx, |ui| {
        let accname_label = ui.label(RichText::new("Account Name").size(18.0));
        ui.text_edit_singleline(&mut app.selected_account_name)
            .labelled_by(accname_label.id);
        ui.add_space(10.0);
        let username_label = ui.label(RichText::new("Username").size(18.0));
        ui.text_edit_singleline(&mut app.selected_username)
            .labelled_by(username_label.id);
        ui.add_space(10.0);
        let secret_label = ui.label(RichText::new("Secret").size(18.0));
        ui.text_edit_singleline(&mut app.selected_secret)
            .labelled_by(secret_label.id);
    });
}

fn read_file_account() -> Vec<Account> {
    check_and_create_file();
    let contents = fs::read_to_string("accounts.json").expect("error reading account file:");
    serde_json::from_str(&contents).unwrap_or_default()
}

fn write_file_account(accounts: Vec<Account>) {
    let data = serde_json::to_string(&accounts).unwrap();
    check_and_create_file();
    fs::write("accounts.json", data).expect("error writing accounts file:");
}

fn check_and_create_file() {
    let is_exists = Path::new("accounts.json").exists();
    if is_exists == false {
        let _ = File::create("accounts.json");
    }
}

fn update_accounts(app: &mut AuthenticatorApp) {
    if app.selected_secret == "" {
        return;
    }

    let mut accounts: Vec<Account> = read_file_account();
    let mut id = accounts.len() as i32 + 1;
    if app.selected_id > 0 {
        id = app.selected_id;
        let mut new_accounts: Vec<Account> = Vec::new();
        for acc in accounts.iter() {
            if id != acc.id {
                new_accounts.push(acc.clone());
            }
        }
        accounts = new_accounts;
    }
    accounts.push(Account {
        id: id,
        account_name: app.selected_account_name.clone(),
        username: app.selected_username.clone(),
        secret: app.selected_secret.clone(),
    });
    app.selected_account_name = String::from("");
    app.selected_username = String::from("");
    app.selected_secret = String::from("");
    accounts.sort_by(|a, b| a.id.cmp(&b.id));
    write_file_account(accounts);
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_resizable(false),
        ..Default::default()
    };

    run_native(
        "Authenticator",
        options,
        Box::new(|_| Ok(Box::new(AuthenticatorApp::default()))),
    )
}
