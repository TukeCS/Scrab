mod ui;
mod file_search;

use ui::MyApp;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "File Finder",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}
