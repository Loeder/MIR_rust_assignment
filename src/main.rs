use walkdir::WalkDir;
use std::ffi::OsStr;
// use show_image::{ImageView, ImageInfo, create_window};
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use std::fs::File;
use anyhow::{Context, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    // assuming that the data is in the root folder where the cargo.lock is
    let data_path = "0925_6225";
    let filename = "LITTO3D_FRA_0925_6224_MNT_20150529_LAMB93_RGF93_IGN69.asc";

    let path_to_file = find_file(filename, data_path)
        .with_context(|| format!("Failed to open ' {filename} '"))?;

    let (metadata, matrix) = read_asc_file(&path_to_file)?;

    for (k, v) in &metadata {
        println!("Key: {k}, Value: {v}");
    }
    println!("{:?}", matrix[0]);

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

fn read_asc_file(filepath: &PathBuf) -> Result<(HashMap<String, f64>, Vec<Vec<f64>>)> {
    let file = File::open(filepath)?; // Open the file
    let reader = BufReader::new(file); // Wrap in BufReader for efficient reading
    
    let mut lines = reader.lines(); // creater iterator for lines in the file
    let mut metadata: HashMap<String, f64> = HashMap::new();
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    
    for _ in 0..6 {
        if let Some(line) = lines.next() {
            let line = line.context("Failed to read metadata line")?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            let key: String = parts[0].to_lowercase();
            let value: f64 = parts[1].parse().context("Failed to transform metadata value to float")?;
            metadata.insert(key, value);
        }
    }

    for line in lines {
        let line = line.context("Failed to read a matrix row")?;
        let row: Vec<f64> = line
            .split_whitespace()// Split by spaces
            .map(|num| num.parse::<f64>().context("Failed to parse matrix value to float")) // Parse as f64
            .collect::<Result<Vec<f64>>>()?; // Convert Result<Vec<Result<f64>>> into Result<Vec<f64>>
        matrix.push(row);
    }

    Ok((metadata, matrix))
}
