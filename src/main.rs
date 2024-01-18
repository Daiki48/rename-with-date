use chrono::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let dir: PathBuf = env::current_dir()?;
    let new_dir: PathBuf = dir.join("renamed");
    fs::create_dir_all(&new_dir)?;

    let date: String = Local::now().format("%Y%m%d%H%M%S").to_string();

    for entry in fs::read_dir(dir)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "exe" {
                    continue;
                }
            }
            let filename: &str = match path.file_name() {
                Some(name) => name.to_str().unwrap(),
                None => continue,
            };
            let new_filename: String = format!("{}_{}", date, filename);
            let new_path: PathBuf = new_dir.join(new_filename);
            fs::copy(&path, &new_path)?;
        }
    }
    Ok(())
}
