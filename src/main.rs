#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod ui;
mod file_search;

use ui::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([500.0, 400.0])
        .with_min_inner_size([500.0, 400.0])
        .with_transparent(true), // To have rounded corners we need transparency
        ..Default::default()
    };
    eframe::run_native(
        "Scrab",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
