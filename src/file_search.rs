use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use path_clean::PathClean;

pub fn search_file(start_path: &str, file_name: &str, case_sensitivity: bool) -> Vec<PathBuf> {
    let cleaned_path = Path::new(start_path).clean(); // Normalize the path
    if !cleaned_path.exists() {
        println!("The directory does not exist: {}", cleaned_path.display());
        return vec![];
    }

    WalkDir::new(&cleaned_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().is_file() && {
                let entry_file_name = entry.file_name();
                if case_sensitivity {
                    entry_file_name == file_name
                } else {
                    entry_file_name.to_string_lossy().eq_ignore_ascii_case(file_name)
                }
            }
        })
        .map(|entry| entry.into_path())
        .collect()
}
