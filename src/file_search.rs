use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use path_clean::PathClean;

pub fn search_file(start_path: &str, file_name: &str) -> Vec<PathBuf> {
    let cleaned_path = Path::new(start_path).clean(); // Normalize the path
    if !cleaned_path.exists() {
        eprintln!("The directory does not exist: {}", cleaned_path.display());
        return vec![];
    }

    WalkDir::new(&cleaned_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file() && entry.file_name() == file_name)
        .map(|entry| entry.into_path())
        .collect()
}

