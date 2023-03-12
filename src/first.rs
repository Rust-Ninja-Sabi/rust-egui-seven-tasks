#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::Vec2;
use eframe::Theme;

pub struct Task1App {
    value: i32,
}

impl Default for Task1App {
    fn default() -> Self {
        Self {
            value: 0
        }
    }
}

impl Task1App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Task1App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{}", self.value));
                if ui.button("Count").clicked() {
                    self.value += 1;
                }
            })
        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let native_options = eframe::NativeOptions{
        default_theme: Theme::Light,
        initial_window_size: Option::from(
            Vec2::new(40.0, 10.0)
        ),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "7 tasks 1",
        native_options,
        Box::new(|cc| Box::new( Task1App::new(cc))),
    );
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
        Box::new(|cc| Box::new(eframe_template::Task1App::new(cc))),
    )
        .expect("failed to start eframe");
}

