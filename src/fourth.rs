#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::Vec2;
use eframe::Theme;

use egui::ProgressBar;

use chrono::{DateTime, Local};

pub struct Task4App {
    start_time: DateTime<Local>,
    max_duration: f32,
}

impl Default for Task4App {
    fn default() -> Self {
        Self {
            start_time: Local::now(),
            max_duration: 10.0
        }
    }
}

impl Task4App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Task4App {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let time_dif = (Local::now() - self.start_time).num_milliseconds() as f32 /1000.0;
            if time_dif > self.max_duration {
                self.start_time = Local::now();
            };
            ui.vertical(|ui| {
                ui.horizontal(|ui|
                    {
                        ui.label("Elapsed time:");
                        ui.add(ProgressBar::new(time_dif/self.max_duration));
                    });
                ui.label(format!("{}",time_dif));
                ui.horizontal(|ui|
                    {
                        ui.label("Duration:");
                        ui.add(egui::Slider::new(&mut self.max_duration, 2.0..=20.0));
                    });

                if ui.button("Reset").clicked() {
                    self.start_time = Local::now();
                }
            });
        });
        egui::Context::request_repaint(ctx);
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {

    let native_options = eframe::NativeOptions{
        default_theme: Theme::Light,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([160.0, 20.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Timer",
        native_options,
        Box::new(|cc| {
            Ok(Box::<Task4App>::default())
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
        Box::new(|cc| Box::new(eframe_template::Task4App::new(cc))),
    )
        .expect("failed to start eframe");
}

