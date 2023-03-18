#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::epaint::Vec2;
use eframe::Theme;

use egui_extras::DatePickerButton;

use chrono::{NaiveDate, Local};

pub struct Task3App {
    flight: Flight,
    from: NaiveDate,
    to: NaiveDate
}

impl Default for Task3App {
    fn default() -> Self {
        Self {
            flight: Flight::OneWay,
            from: Local::now().date_naive(),
            to: Local::now().date_naive()
        }
    }
}

impl Task3App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

#[derive(Debug,PartialEq)]
enum Flight{
    OneWay,
    Return
}

impl eframe::App for Task3App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical(|ui| {
                egui::ComboBox::from_id_source("flight")
                    .selected_text(format!("{:?}", self.flight))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.flight, Flight::OneWay, "one-way flight");
                        ui.selectable_value(&mut self.flight, Flight::Return, "return flight");
                    });
                let response = ui.add(DatePickerButton::new(&mut self.from));
                if response.changed() {
                    println!("The date changed!");
                };
                ui.scope(|ui|{
                    ui.visuals_mut().warn_fg_color = egui::Color32::RED;
                    let response = ui.add_enabled(self.flight==Flight::Return ,
                                                  DatePickerButton::new(&mut self.to));

                    if response.changed() {
                        println!("The date changed!");
                    };
                });

                if ui.add_enabled(self.flight==Flight::OneWay || (self.to - self.from).num_days()>=0,
                                  egui::Button::new("Book")).clicked() {
                }
            });

        });
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let native_options = eframe::NativeOptions{
        default_theme: Theme::Light,
        initial_window_size: Option::from(
            Vec2::new(200.0, 40.0)
        ),
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "7 tasks 3",
        native_options,
        Box::new(|cc| Box::new( Task3App::new(cc))),
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
        Box::new(|cc| Box::new(eframe_template::Task3App::new(cc))),
    )
        .expect("failed to start eframe");
}

