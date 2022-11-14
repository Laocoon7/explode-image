use crate::prelude::*;

pub struct ExplodeArgs {
    input_path: PathBuf,
    output_path: PathBuf,
    format: ImageFormat,
    format_extension: String,
    columns: u32,
    rows: u32,
    padding: (u32, u32),
    offset: (u32, u32),
}

impl ExplodeArgs {
    pub fn new(input_path: &PathBuf, output_path: &PathBuf, format: ImageFormat, format_extension: &str, columns: u32, rows: u32, padding: Option<(u32, u32)>, offset: Option<(u32, u32)>) -> Self {
        let padding = match padding {
            Some(t) => t,
            None => (0u32, 0u32),
        };

        let offset = match offset {
            Some(t) => t,
            None => (0u32, 0u32),
        };

        Self {
            input_path: input_path.clone(),
            output_path: output_path.clone(),
            format,
            format_extension: format_extension.to_string(),
            columns,
            rows,
            padding,
            offset,
        }
    }
}

pub fn explode_image(
    args: ExplodeArgs,
) -> Result<()> {
    // Open the input image
    let mut input_image = ImageReader::open(&args.input_path)?.decode()?;

    // Get image's width/height
    let image_width = input_image.width();
    let image_height = input_image.height();

    // build a tileset out of the image based on the dimensions
    let tileset = Tileset::new(image_width, image_height, args.columns, args.rows, args.padding, args.offset)?;

    // Create the output dir if necessary
    std::fs::create_dir_all(&args.output_path)?;

    // Tell the user we are working
    println!(
        "Exploding image: `{:?}` into `{:?}` with (tile_width: {}, tile_height: {}), (columns: {}, rows: {}) with (padding_x: {}, padding_y: {}), and (offset_x: {}, offset_y: {}) as {:?}",
        &args.input_path,
        &args.output_path,
        tileset.tile_width,
        tileset.tile_height,
        args.columns,
        args.rows,
        args.padding.0,
        args.padding.1,
        args.offset.0,
        args.offset.1,
        &args.format_extension,
    );

    // For each tile
    for y in 0..tileset.rows {
        for x in 0..tileset.columns {
            // Get the index
            let index = y * tileset.columns + x;

            // Get the tile's bounding rect
            let ((x, y), (width, height)) = tileset.get_rect(x, y);

            // Tell the user which file we are exporting
            println!(
                "Exporting: {}.png",
                index,
            );

            // Cut the image out
            let subimg = imageops::crop(&mut input_image, x, y, width, height);

            // Save the image
            if let Err(e) = subimg.to_image().save_with_format(
                &args.output_path.join(format!("{}.{}", index, &args.format_extension)),
                args.format,
            ) {
                println!("{}", e);
            }
        }
    }

    Ok(())
}
