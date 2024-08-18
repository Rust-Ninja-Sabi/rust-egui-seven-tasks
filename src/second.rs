#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::Vec2;
use eframe::Theme;

struct Task2App {
    c_value: String,
    f_value: String
}

impl Default for Task2App {
    fn default() -> Self {
        Self {
            c_value: "0.0".to_string(),
            f_value: Task2App::to_fahrenheit(&"0.0".to_string())
        }
    }
}

impl Task2App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn to_celsius(f_value:&String)->String {
        let mut result = "-".to_string();
        if let Ok(v) = f_value.parse::<f32>() {
            result = ((v - 32.0) * (5.0/9.0)).to_string();
        };
        result
    }
    pub fn to_fahrenheit(c_value:&String)->String {
        let mut result = "-".to_string();
        if let Ok(v) = c_value.parse::<f32>() {
            result = (v * (9.0/5.0) + 32.0).to_string();
        };
        result
    }
}

impl eframe::App for Task2App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let reponse_c = ui.text_edit_singleline( &mut self.c_value);
                    if reponse_c.changed() {
                        self.f_value = Task2App::to_fahrenheit(&self.c_value)
                    }
                    ui.label(" Celsius = ");
                });
                ui.horizontal(|ui| {
                    let response_f = ui.text_edit_singleline( &mut self.f_value);
                    if response_f.changed() {
                        self.c_value = Task2App::to_celsius(&self.f_value)
                    }
                    ui.label("Fahrenheit");
                })
            })

        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions{
        default_theme: Theme::Light,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([100.0, 10.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Temperature Conv",
        native_options,
        Box::new(|cc| {
            Ok(Box::<Task2App>::default())
        }),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(eframe_template::Task2App::new(cc))),
    )
        .expect("failed to start eframe");
}

