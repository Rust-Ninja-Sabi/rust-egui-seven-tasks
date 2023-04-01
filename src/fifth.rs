#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::Theme;

use egui::{Color32, Button};

use egui_extras::{TableBuilder,Column};

use std::collections::HashMap;

use rnglib::{RNG, Language};

struct Person {
    id: i32,
    name: String,
    surname: String,
}

pub struct Task5App {
    prefix:String,
    detail_name: String,
    detail_surname: String,
    max_key:i32,
    selected:i32,
    db:HashMap<i32,Person>
}

impl Default for Task5App {
    fn default() -> Self {
        let mut db = HashMap::new();
        let rng = RNG::try_from(&Language::Elven).unwrap();

        let mut max_key = 0;
        let mut first = true;
        let mut detail_name = "".to_string();
        let mut detail_surname = "".to_string();

        for i in 0..25 {
            let first_name = rng.generate_name();
            let last_name = rng.generate_name();

            if first {
                first = false;
                detail_name = last_name.clone();
                detail_surname = first_name.clone();
            };

            db.insert(i,Person{
                id:max_key,
                name:last_name,
                surname:first_name
            });

            max_key +=1;
        };

        Self {
            prefix: "".to_string(),
            detail_name: detail_name,
            detail_surname: detail_surname,
            max_key: max_key,
            selected: 0,
            db:db
        }
    }
}

impl Task5App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

}

impl eframe::App for Task5App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                ui.label("Filter prefix:");
                ui.text_edit_singleline(&mut self.prefix);
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    self.crud_create(&self.detail_surname.clone(),
                                     &self.detail_name.clone());
                }
                if ui.button("Update").clicked() {
                    self.crud_update(self.selected,
                                     &self.detail_surname.clone(),
                                     &self.detail_name.clone())
                }
                if ui.button("Delete").clicked() {
                    self.crud_delete(self.selected);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Name");
                    });
                })
                .body(|mut body| {
                    for (i,p) in self.db.iter().filter(|&(k, p)| (self.prefix).len() as i32 == 0 ||
                        format!("{} {} ({})", &p.name, &p.surname, p.id).starts_with(&self.prefix)   ) {

                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                let mut button = Button::new(format!("{} {} ({})", &p.name, &p.surname, p.id));
                                if p.id == self.selected {
                                    button = button.fill(Color32::LIGHT_BLUE)
                                }
                                if ui.add(button).clicked() {
                                    self.selected = p.id;
                                    self.detail_surname = p.surname.clone();
                                    self.detail_name = p.name.clone();
                                };
                            });
                        });
                    }
                });
        });

        egui::SidePanel::right("right_panel").min_width(240.0).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut self.detail_name);
                });
                ui.horizontal(|ui| {
                    ui.label("Surname");
                    ui.text_edit_singleline(&mut self.detail_surname);
                });
            });
        });
    }
}

impl Task5App {

    fn crud_create(&mut self, surname: &String, name: &String)->(){
        self.max_key +=1;

        self.db.insert(self.max_key, Person{
            id: self.max_key,
            name:name.clone(),
            surname:surname.clone()
        });
    }

    fn crud_update(&mut self, id:i32,  surname: &String, name: &String)->() {

        self.db.insert(id, Person{
            id: id,
            name: name.clone(),
            surname: surname.clone()
        });
    }

    fn crud_delete(&mut self, id:i32)->(){
        self.db.remove(&id);
        let person = self.db.values().last().unwrap();
        self.selected = person.id;
        self.detail_surname = person.surname.clone();
        self.detail_name = person.name.clone();
    }

}


// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {

    let native_options = eframe::NativeOptions{
        default_theme: Theme::Light,
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "7 tasks 5",
        native_options,
        Box::new(|cc| Box::new( Task5App::new(cc))),
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
        Box::new(|cc| Box::new(eframe_template::Task5App::new(cc))),
    )
        .expect("failed to start eframe");
}

