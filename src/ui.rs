use eframe::egui;

pub struct MyApp {
    pub file_name: String,
    pub search_dir: String,
    pub results: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        MyApp {
            file_name: String::new(),
            search_dir: get_default_search_dir(),
            results: Vec::new(),
        }
    }
}

// Determine the default search directory based on the operating system
fn get_default_search_dir() -> String {
    #[cfg(target_os = "windows")]
    {
        "C:\\".to_string() // Default to C:\ on Windows
    }

    #[cfg(not(target_os = "windows"))]
    {
        "/".to_string() // Default to root on Unix-like systems
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Finder");

            ui.horizontal(|ui| {
                ui.label("Search Directory: ");
                ui.text_edit_singleline(&mut self.search_dir);
            });

            ui.horizontal(|ui| {
                ui.label("File Name: ");
                ui.text_edit_singleline(&mut self.file_name);
            });

            if ui.button("Search").clicked() {
                self.results = crate::file_search::search_file(&self.search_dir, &self.file_name)
                    .into_iter()
                    .map(|path| path.to_string_lossy().to_string())
                    .collect();
            }

            ui.separator();
            ui.label("Results:");

            egui::ScrollArea::vertical().show(ui, |ui| {
                for result in &self.results {
                    ui.label(result);
                }
            });
        });
    }
}

