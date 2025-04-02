// Import required standard library modules and external crates
use std::{env::args, fs::File, io::{BufRead, BufReader}, path::{Path, PathBuf}};
use anyhow::{Result, bail}; // For error handling
use image::{GrayImage, Luma}; // For grayscale image creation
use show_image::{create_window, ImageInfo, ImageView, WindowProxy}; // For displaying images
use walkdir::WalkDir; // For recursively searching files

/// Reads a single `.asc` elevation file and converts it to a grayscale image
fn read_asc_to_grayscale(path: &Path) -> Result<GrayImage> {
    // Open the ASCII file for reading
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Read metadata from the header (first 6 lines)
    let ncols: usize = lines.next().unwrap()?.split_whitespace().last().unwrap().parse()?;
    let nrows: usize = lines.next().unwrap()?.split_whitespace().last().unwrap().parse()?;
    lines.next(); // Skip xllcorner (not used)
    lines.next(); // Skip yllcorner (not used)
    lines.next(); // Skip cellsize (not used)
    let nodata_value: f32 = lines.next().unwrap()?.split_whitespace().last().unwrap().parse()?;

    // Read the elevation grid data into a 2D vector
    let mut data: Vec<Vec<f32>> = Vec::with_capacity(nrows);
    for line in lines {
        let row: Vec<f32> = line?
            .split_whitespace()
            .map(|v| v.parse::<f32>().unwrap()) // Convert each string to float
            .collect();
        data.push(row);
    }

    // Determine the minimum and maximum values (excluding nodata) for normalization
    let min = data.iter().flatten()
        .filter(|&&v| v != nodata_value)
        .cloned()
        .fold(f32::INFINITY, f32::min);
    
    let max = data.iter().flatten()
        .filter(|&&v| v != nodata_value)
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max);

    // Create an empty grayscale image of the same size
    let mut img = GrayImage::new(ncols as u32, nrows as u32);

    // Fill the image pixel-by-pixel by mapping elevation values to 0–255 grayscale
    for (y, row) in data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let gray = if val == nodata_value {
                0u8 // If it's a nodata value, make the pixel black
            } else {
                ((val - min) / (max - min) * 255.0) as u8 // Normalize to 0–255
            };
            img.put_pixel(x as u32, y as u32, Luma([gray]));
        }
    }

    Ok(img) // Return the grayscale image
}

/// Recursively search a directory for `.asc` files and return their paths
fn find_asc_files(root: &Path) -> Vec<PathBuf> {
    WalkDir::new(root) // Walk through the directory and its subdirectories
        .into_iter()
        .filter_map(Result::ok) // Ignore errors during walk
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "asc")) // Keep only .asc files
        .map(|e| e.path().to_path_buf()) // Get the full path
        .collect()
}

/// Main function — reads all `.asc` files, converts them to images, saves, and displays them
#[show_image::main] // Enables image windows to stay open
fn main() -> Result<()> {
    // Get folder path from CLI argument (default: "0925_6225")
    let root = args().nth(1).unwrap_or_else(|| "0925_6225".to_string());
    let asc_files = find_asc_files(Path::new(&root));

    // If no files found, stop with an error
    if asc_files.is_empty() {
        bail!("No .asc files found in '{}'", root);
    }

    println!("Found {} ASC files. Processing...", asc_files.len());

    // Store window handles to prevent them from closing immediately
    let mut windows: Vec<WindowProxy> = Vec::new();

    // Process each ASC file
    for (i, path) in asc_files.iter().enumerate() {
        // Convert ASC to grayscale image
        let image = read_asc_to_grayscale(path)?;

        // Create a human-readable label like "1_filename"
        let label = format!("{}_{}", i + 1, path.file_stem().unwrap().to_string_lossy());

        // Save the grayscale image to disk as PNG
        let output_name = format!("{label}.png");
        image.save(&output_name)?;
        println!("Saved: {output_name}");

        // Create a window and display the image
        let window = create_window(&label, Default::default())?;
        let view = ImageView::new(ImageInfo::mono8(image.width(), image.height()), image.as_raw());
        window.set_image("preview", view)?;

        // Store the window handle to keep it open later
        windows.push(window);
    }

    println!("All images displayed. Close each window to exit.");

    // Wait until each window is manually closed by the user
    for window in windows {
        window.wait_until_destroyed()?;
    }

    Ok(())
}


// to run: cargo run -- 0925_6225
