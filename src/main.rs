use eframe::egui;
use goldberg::goldberg_stmts;
use tsar_sdk::{AuthParams, Client, ClientParams};

fn main() -> Result<(), eframe::Error> {
    goldberg_stmts! {{
        let options = ClientParams {
            app_id: "f911842b-5b3d-4c59-b5d1-4adb8f71557b".to_string(),
            client_key: "MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEvJrwPvdeDUcV8Qr02tzgFrp+8qfCV/vG1HcQJYYV8u5vYUfGABMAYT0qOQltXEX9DTcB2fzLfwQnl7yiAaNruQ==".to_string(),
        };

        let client = Client::create(options).expect("Authentication failed.");
        
        let mut user_result = client.authenticate(AuthParams::default());
        while user_result.is_err() {
            println!("Attempting to check if user is valid...");
            std::thread::sleep(std::time::Duration::from_secs(3));

            user_result = client.authenticate(AuthParams {
                open_browser: false,
            });
        }

        let user = user_result.unwrap();
        println!("User authorized. {:#?}", user);

        eframe::run_native(
            "TSAR Rust Demo",
            eframe::NativeOptions::default(),
            Box::new(|_| {
                Box::new(App {
                    username: user.name.unwrap_or_else(|| "Unknown".to_string()),
                    id: user.id,
                    hwid: Client::get_hwid().unwrap_or_else(|_| "Unknown HWID".to_string()),
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
