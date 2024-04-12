#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use goldberg::goldberg_stmts;
use std::io;
use tsar_client::{Client, Data};

fn main() -> Result<(), eframe::Error> {
    goldberg_stmts! {{
        let api = Client::new("58816206-b24c-41d4-a594-8500746a78ee", "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAELlyGTmNEv3AarudyshJUUA9ig1pOfSl5qWX8g/hkPiieeKlWvv9o4IZmWI4cCrcR0fteVEcUhBvu5GAr/ITBqA==");

        let data = api.authenticate_user().unwrap();

        eframe::run_native(
            "TSAR Crack Test #1",
            eframe::NativeOptions::default(),
            Box::new(|cc| {
                Box::new(App {
                    username: data.user.username.unwrap(),
                    id: data.user.id,
                    hwid: data.hwid,
                })
            }),
        )
    }}
}

struct App {
    id: String,
    username: String,
    hwid: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        goldberg_stmts! {{
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("TSAR Crack Test #1");

                ui.separator();

                ui.label(format!("Hello, {}", self.username));
                ui.label(format!("Your user ID is {}", self.id));
                ui.label(format!("Your user HWID is {}", self.hwid));
            });
        }}
    }
}
