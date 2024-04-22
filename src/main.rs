use eframe::egui;
use goldberg::goldberg_stmts;
use tsar_client::{Client, ClientOptions};

fn main() -> Result<(), eframe::Error> {
    goldberg_stmts! {{
        let options = ClientOptions {
            app_id: "58816206-b24c-41d4-a594-8500746a78ee".to_string(),
            client_key: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAELlyGTmNEv3AarudyshJUUA9ig1pOfSl5qWX8g/hkPiieeKlWvv9o4IZmWI4cCrcR0fteVEcUhBvu5GAr/ITBqA==".to_string(),
            debug_print: true
        };

        let client = Client::new(options).expect("Authentication failed.");

        eframe::run_native(
            "TSAR Crack Test",
            eframe::NativeOptions::default(),
            Box::new(|_| {
                Box::new(App {
                    username: client.subscription.user.username.unwrap(),
                    id: client.subscription.user.id,
                    hwid: client.hwid,
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
                ui.heading("TSAR Crack Test");

                ui.separator();

                ui.label(format!("Hello, {}", self.username));
                ui.label(format!("Your user ID is {}", self.id));
                ui.label(format!("Your HWID is {}", self.hwid));
            });
        }}
    }
}
