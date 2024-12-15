use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn search_file(start_path: &str, file_name: &str, case_sensitivity: bool) -> Vec<PathBuf> {
    let path = Path::new(start_path);
    if !path.exists() {
        println!("The directory does not exist: {}", path.display());
        return vec![];
    }

    WalkDir::new(&path)
        .into_iter()
        .flatten()
        .filter(|entry| {
            entry.path().is_file() && {
                let entry_file_name = entry.file_name();
                if case_sensitivity {
                    entry_file_name.to_string_lossy().contains(file_name)
                } else {
                    entry_file_name
                        .to_string_lossy()
                        .to_ascii_lowercase()
                        .contains(&file_name.to_ascii_lowercase())
                }
            }
        })
        .map(|entry| entry.into_path())
        .collect()
}
