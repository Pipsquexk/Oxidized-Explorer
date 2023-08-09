extern crate chrono;
use chrono::Local;
use chrono::DateTime;
use std::io::{self, stdout, Write};
use std::time::{SystemTime, Duration};
use eframe::egui;

mod utils;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut search_str: String = String::default();

    eframe::run_simple_native("Oxidized Explorer", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|hui| {
                hui.button("Copy");
                hui.button("Delete");
                let search_label = hui.label("Search: ");
                hui.text_edit_singleline(&mut search_str).labelled_by(search_label.id);
            });
            for file in utils::get_current_directory().unwrap() {
                ui.horizontal(|lui| {
                    let curFile = file.unwrap();
                    lui.label(curFile.file_name().to_str().unwrap());
                    let mod_time: DateTime<Local> = curFile.metadata().unwrap().modified().unwrap().into();
                    lui.label(mod_time.format("%d/%m/%Y %T").to_string());
                });
            }
            
        });
    })
}
