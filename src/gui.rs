use eframe::egui;
use crate::calculate;

pub struct CalculatorApp {
    input: String,
    result: String,
    error: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            result: String::new(),
            error: String::new(),
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Calculator");
            ui.add_space(10.0);

            // Input field with keyboard focus
            let _ = ui.horizontal(|ui| {
                ui.label("Enter calculation:");
                let text_edit = ui.text_edit_singleline(&mut self.input);
                text_edit.request_focus();
                text_edit
            });

            // Check for Enter key press
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.calculate();
            }

            // Buttons for common operations
            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    self.input.push('+');
                }
                if ui.button("-").clicked() {
                    self.input.push('-');
                }
                if ui.button("*").clicked() {
                    self.input.push('*');
                }
                if ui.button("/").clicked() {
                    self.input.push('/');
                }
                if ui.button("Clear").clicked() {
                    self.input.clear();
                    self.result.clear();
                    self.error.clear();
                }
            });

            // Calculate button
            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            // Display results
            if !self.result.is_empty() {
                ui.add_space(10.0);
                ui.label(&self.result);
            }
            if !self.error.is_empty() {
                ui.add_space(10.0);
                ui.label(egui::RichText::new(&self.error).color(egui::Color32::RED));
            }

            // Instructions
            ui.add_space(20.0);
            ui.label("Instructions:");
            ui.label("• Enter numbers and operators (+, -, *, /)");
            ui.label("• Press Enter or click Calculate to compute");
            ui.label("• Spaces are optional (e.g., '5+3' or '5 + 3')");
            ui.label("• Scientific notation is supported (e.g., '1e3 + 2e3')");
        });
    }
}

impl CalculatorApp {
    fn calculate(&mut self) {
        match calculate(&self.input) {
            Ok(result) => {
                self.result = format!("Result: {}", result);
                self.error.clear();
            }
            Err(err) => {
                self.error = format!("Error: {}", err);
                self.result.clear();
            }
        }
    }
} 