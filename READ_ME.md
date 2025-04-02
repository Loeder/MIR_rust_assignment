# Villhauer_Strik_Rust_Exam

This repository contains three Rust mini-projects developed as part of an assignment focused on visualizing `.asc` elevation data files in various ways.

## Project Overview

The objective of this exam was to display terrain elevation data using three rendering methods:

1. **Grayscale visualization**
2. **Color gradient (colormap) visualization**
3. **Hillshaded visualization with enhancements**

Each project handles `.asc` (ASCII raster) files that contain gridded elevation data and performs the following key operations:

- Parses elevation data from ASC files
- Normalizes data to display it effectively
- Visualizes the data as an image
- Add a colorbar to display relative height **(Feature)**
- Displays the result in a window using `show_image` **(Feature)**
- Saves the resulting PNG to disk **(Feature)**
- Loops through all `.asc` files in a directory or processes a single file specified by the user **(Feature)**

---

## CLI Arguments for File & Data Selection

The program supports command-line arguments using the `clap` crate. Users can specify:

- **A specific file** to process using the `--filename` (`-f`) flag. 
- **A custom data folder** using the `--datapath` (`-d`) flag. If not specified, it defaults to `0925_6225`.

### Example Usage:

#### **Process all `.asc` files in the default data folder**
```bash
cargo run
```

#### **Process all `.asc` files in a custom data folder**
```bash
cargo run -- --datapath custom_folder
```

#### **Process a specific file**
```bash
cargo run -- --filename example.asc
```

#### **Process a specific file in a custom data folder**
```bash
cargo run -- --filename example.asc --datapath custom_folder
```

If `--filename` is specified, only that file will be processed. If omitted, the program will process all `.asc` files in the specified or default folder.

---

## Folder Structure

```
Villhauer_Strik_Rust_Exam/
├── my_project_greyscale_enhanced/
│   └── Displays elevation data in grayscale
├── my_project_colorgrad_enhanced/
│   └── Displays elevation data using a color gradient
├── my_project_hillshade_enhanced/
│   └── Applies hillshading and color gradient with a vertical colorbar
└── README.md
```

Each subfolder contains its own `Cargo.toml` and `src/main.rs`.

**IMPORTANT:** Make sure each sub-directory `Villhauer_Strik_Rust_Exam/my_project_` contains the data folder titled `0925_6225`! Or specify a custom datafolder with the --datapath flag. The data is currently not included in this repository for storage reasons.

---

## 1. Grayscale Visualization

**Folder:** `my_project_greyscale_enhanced/`

### Description:
- Elevation values are normalized between the minimum and maximum (excluding `nodata`) and mapped to grayscale values between `0` (black) and `255` (white).
- `nodata` pixels are shown in black.
- Uses the `image` crate to generate and save `GrayImage`s.

### Features:
- Loads `.asc` files from a specified folder or processes a single file if provided.
- Saves each image as PNG.
- Displays each image in a separate native window.

---

## 2. Color Gradient Visualization

**Folder:** `my_project_colorgrad_enhanced/`

### Description:
- Similar to the grayscale version, but elevation values are mapped to colors using a `colorgrad` gradient (e.g., `turbo`, `viridis`).
- Makes it easier to visually distinguish elevation ranges.

### Features:
- Same enhancements: loops through files or processes a single file, saves images, opens windows.
- Uses turbo colormap for better visual contrast (can change as you desire).

---

## 3. Hillshaded Relief with Colorbar

**Folder:** `my_project_hillshade_enhanced/`

### Description:
- Combines a color gradient with hillshading to simulate 3D terrain lighting.
- Calculates slope and aspect using Horn’s method.
- Applies a shaded relief effect based on sun azimuth and altitude.
- Adds a vertical colorbar showing elevation range (in meters).

### Enhancements:
- **Colorbar with tick marks**: Placed to the right of the image, scaled, and labeled using `rusttype`.
- **Looping**: Processes all `.asc` files automatically unless a specific file is provided.
- **Display**: Opens a native image window for each result.

---

## Dependencies

All projects rely on the following crates:

- `image`
- `show-image`
- `colorgrad`
- `walkdir`
- `anyhow`
- `clap` (for CLI argument parsing)
- `imageproc` (hillshade only)
- `rusttype` (hillshade only)

Make sure to include these in each project's `Cargo.toml`.

---

## Authors

Sarah Villhauer and Laura Strik  
Marine Intelligent Robotics Program (MIR)  
2025 Rust Exam — Erasmus Mundus  

---

