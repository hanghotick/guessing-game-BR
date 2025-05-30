use eframe::{egui, epi};
use rand::Rng;

struct GuessingGameApp {
    secret_number: u32,
    guess: String,
    message: String,
    color: egui::Color32,
    selected_color: String,
    dark_mode: bool,
    show_wizard: bool,
}

impl Default for GuessingGameApp {
    fn default() -> Self {
        Self {
            secret_number: rand::thread_rng().gen_range(1..=100),
            guess: String::new(),
            message: "Welcome to the Guessing Game!".to_owned(),
            color: egui::Color32::WHITE,
            selected_color: "white".to_owned(),
            dark_mode: true,
            show_wizard: false,
        }
    }
}

impl epi::App for GuessingGameApp {
    fn name(&self) -> &str {
        "Guessing Game GUI"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎲 Guessing Game");
            ui.horizontal(|ui| {
                ui.label("Theme:");
                if ui.button("🌙 Dark").clicked() {
                    self.dark_mode = true;
                }
                if ui.button("☀️ Light").clicked() {
                    self.dark_mode = false;
                }
            });
            ui.separator();
            ui.label("Choose your color:");
            for (name, color) in [
                ("red", egui::Color32::RED),
                ("green", egui::Color32::GREEN),
                ("yellow", egui::Color32::YELLOW),
                ("blue", egui::Color32::BLUE),
                ("magenta", egui::Color32::from_rgb(255,0,255)),
                ("cyan", egui::Color32::from_rgb(0,255,255)),
                ("white", egui::Color32::WHITE),
            ] {
                if ui.add(egui::Button::new(name).fill(color)).clicked() {
                    self.color = color;
                    self.selected_color = name.to_owned();
                }
            }
            ui.separator();
            ui.colored_label(self.color, &self.message);
            ui.horizontal(|ui| {
                ui.label("Your guess (1-100):");
                let response = ui.text_edit_singleline(&mut self.guess);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.process_guess();
                }
                if ui.button("Guess!").clicked() {
                    self.process_guess();
                }
            });
            if ui.button("Reset Game").clicked() {
                self.secret_number = rand::thread_rng().gen_range(1..=100);
                self.message = "Game reset! New number chosen.".to_owned();
                self.guess.clear();
                self.show_wizard = false;
            }
            // Easter egg: hidden wizard button
            if ui.button(" 0").clicked() {
                self.show_wizard = true;
            }
            if self.show_wizard {
                ui.label("🧙‍♂️ You found the wizard! Magic is in the air...");
            }
        });
    }
}

impl GuessingGameApp {
    fn process_guess(&mut self) {
        let guess_num = match self.guess.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= 100 => num,
            _ => {
                self.message = "⚠️ Please enter a valid number between 1 and 100!".to_owned();
                return;
            }
        };
        if guess_num < self.secret_number {
            self.message = "Too small! 📉".to_owned();
        } else if guess_num > self.secret_number {
            self.message = "Too big! 📈".to_owned();
        } else {
            self.message = "You win! 🎉".to_owned();
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guessing Game GUI",
        options,
        Box::new(|_cc| Box::new(GuessingGameApp::default())),
    );
}
