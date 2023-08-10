extern crate chrono;
use chrono::Local;
use chrono::DateTime;
use std::env;
use std::fs::ReadDir;
use std::path::PathBuf;
use eframe::egui;
use open;
use std::fs;

mod utils;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };


    let mut search_str: String = String::default();
    let mut prev_dirs: [PathBuf; 5] = Default::default();
    let mut new_dir_path: PathBuf = env::current_dir().unwrap();

    eframe::run_simple_native("Oxidized Explorer", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let cur_dir: ReadDir = utils::get_dir_from_file(&new_dir_path).unwrap();
            ui.horizontal_top(|hui| {
                if hui.button("<<<").clicked() {
                    new_dir_path = prev_dirs[0].clone();
                }
                hui.text_edit_singleline(&mut search_str);
                if hui.button("Search").clicked() {
                    println!("Do search stuff!");
                }
            });
            // cur_dir = utils::get_dir_from_file(std::path::PathBuf)
            for file in cur_dir {
                ui.horizontal(|lui| {
                    let cur_file = file.unwrap();
                    if lui.button(cur_file.file_name().to_str().unwrap()).clicked() {
                        println!("{} Was clicked", cur_file.file_name().to_str().unwrap());
                        if cur_file.metadata().unwrap().is_dir() {
                            prev_dirs[0] = new_dir_path.clone();
                            new_dir_path = cur_file.path();
                        }
                        else {
                            let _ = open::that(cur_file.path().to_str().unwrap());
                        }
                    }
                    let mod_time: DateTime<Local> = cur_file.metadata().unwrap().modified().unwrap().into();
                    lui.label(mod_time.format("%d/%m/%Y %T").to_string());
                });
            }
            
        });
    })
}
