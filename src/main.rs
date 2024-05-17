use eframe::egui;
use goldberg::goldberg_stmts;
use tsar_client::{Client, ClientOptions};

fn main() -> Result<(), eframe::Error> {
    goldberg_stmts! {{
        let options = ClientOptions {
            app_id: "d45750ee-7c2c-4453-9a90-397007bc1b9d".to_string(),
            client_key: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAElfwlA3IWW8+tI4+T0HNYA0ZmaTQEqBrEbygPgoaAvsK68KRvVXJN/IXOthd3ulxv6LCPqdRewAsG7srEmwAzhA==".to_string(),
            debug_print: true
        };

        let client = Client::new(options).expect("Authentication failed.");

        eframe::run_native(
            "TSAR Rust Demo",
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
                ui.heading("TSAR Rust Demo");

                ui.separator();

                ui.label(format!("Hello, {}", self.username));
                ui.label(format!("Your user ID is {}", self.id));
                ui.label(format!("Your HWID is {}", self.hwid));
            });
        }}
    }
}
