use walkdir::WalkDir;
use std::ffi::OsStr;
use show_image::{ImageView, ImageInfo, create_window};
use std::path::{self, PathBuf};
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // assuming that the data is in the root folder where the cargo.lock is
    let data_path = "0925_6225";
    let filename = "LITTO3D_FRA_0925_6224_MNT_20150529_LAMB93_RGF93_IGN69.asc";

    // match find_file(filename, data_path) {
    //     Some(path) => {
    //         println!("file found!");
    //         read_asc_file(&path);
    //     },
    //     None => {
    //         println!("specified file: ' {filename} ' not found..");
    //         return;
    //     },
    // }

    let path_to_file = find_file(filename, data_path)
        .with_context(|| format!("Failed to open ' {filename} '"))?;

    read_asc_file(&path_to_file)?;

    Ok(())
}

fn find_file(filename: &str, data_path: &str) -> Option<PathBuf> {
    for entry in WalkDir::new(data_path) {
        let entry = entry.unwrap();
        if entry.file_name() == OsStr::new(filename) {
            return Some(entry.path().to_path_buf());
        }
    }
    None
}

fn read_asc_file(filepath: &PathBuf) -> io::Result<()> {
    let file = File::open(filepath)?; // Open the file
    let reader = BufReader::new(file); // Wrap in BufReader for efficient reading

    for line in reader.lines() {
        let line = line?; // Handle potential IO errors
        println!("{}", line); // Print each line
    }
    Ok(())
}
