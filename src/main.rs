#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Testing!",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
    Ok(())
}

struct MyApp {
    name: String,
    command: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            command: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Your command: ");
                ui.text_edit_singleline(&mut self.command)
                    .labelled_by(name_label.id);
            });
            ui.label(format!("Hello '{}', command {}", self.name, self.command));
        });
    }
}
