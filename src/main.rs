mod cli;
mod error;
mod explode_image;
mod tileset;

mod prelude {
    pub use std::ffi::OsString;
    pub use std::path::{Path, PathBuf};

    pub use clap::Parser;
    pub use image::*;
    pub use image::io::Reader as ImageReader;
    pub use thiserror::Error;

    pub use super::cli::*;
    pub use super::error::*;
    pub use super::explode_image::*;
    pub use super::tileset::*;
}
use prelude::*;

fn main() {
    // Parse command line
    let args = Cli::parse();

    // Get ImageFormat - Default: ImageFormat::Png
    let format_extension;
    let format = match &args.image_ext {
        Some(s) => match ImageFormat::from_extension(s) {
            Some(f) => {
                format_extension = *(f.extensions_str().first().unwrap());
                f
            },
            None => {
                let f = ImageFormat::Png;
                format_extension = *(f.extensions_str().first().unwrap());
                println!("Unknown extension format: `{:?}` using `{}`", &args.image_ext, &format_extension);
                ImageFormat::Png
            },
        },
        None => {
            let f = ImageFormat::Png;
            format_extension = *(f.extensions_str().first().unwrap());
            println!("Using default image format: `{}`", &format_extension);
            ImageFormat::Png
        },
    };

    // Get just the input file name
    match &args.image_input.file_stem() {
        Some(input_name) => {
            // Get output folder - Default: InputFile.parent()
            let output_folder = match &args.image_output_folder {
                // Append input file name as a folder to output folder
                Some(folder) => folder.join(input_name),
                None => match args.image_input.parent() {
                    Some(p) => p.to_path_buf(),
                    None => PathBuf::new(),
                },
            };

            // Build args
            let explode_args = ExplodeArgs::new(&args.image_input, &output_folder, format, format_extension, args.columns, args.rows, args.padding, args.offset);

            // Explode Image
            if let Err(e) = explode_image(explode_args) {
                println!("{}", e);
            }
        }
        None => println!("Cannot get input image file."),
    }
}
