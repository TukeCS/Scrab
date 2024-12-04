#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui;
mod file_search;

use ui::MyApp;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Scrab",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
