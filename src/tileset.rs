use crate::prelude::*;

pub struct Tileset {
    pub image_width: u32,
    pub image_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub padding: (u32, u32),
    pub offset: (u32, u32),

    pub tile_width: u32,
    pub tile_height: u32,
}

impl Tileset {
    pub fn new(
        image_width: u32,
        image_height: u32,
        columns: u32,
        rows: u32,
        padding: (u32, u32),
        offset: (u32, u32),
    ) -> Result<Self> {
        let (tile_width, tile_height) = Self::calculate_tile_dimensions(
            image_width,
            image_height,
            columns,
            rows,
            padding,
            offset,
        )?;

        Ok(Self {
            image_width,
            image_height,
            columns,
            rows,
            padding,
            offset,
            tile_width,
            tile_height,
        })
    }

    pub fn get_rect(&self, index_x: u32, index_y: u32) -> ((u32, u32), (u32, u32)) {
        let padding_x = self.padding.0 * index_x;
        let padding_y = self.padding.1 * index_y;
        let x = self.offset.0 + padding_x + index_x * self.tile_width;
        let y = self.offset.1 + padding_y + index_y * self.tile_height;
        // ((top, left), (width, height))
        ((x, y), (self.tile_width, self.tile_height))
    }

    fn calculate_tile_dimensions(
        image_width: u32,
        image_height: u32,
        columns: u32,
        rows: u32,
        padding: (u32, u32),
        offset: (u32, u32),
    ) -> Result<(u32, u32)> {
        let total_offset_x = (offset.0 * 2) as f32;
        let total_offset_y = (offset.1 * 2) as f32;

        let total_padding_x = ((columns - 1) * padding.0) as f32;
        let total_padding_y = ((rows - 1) * padding.1) as f32;

        let actual_image_width = image_width as f32 - total_offset_x - total_padding_x;
        let actual_image_height = image_height as f32 - total_offset_y - total_padding_y;

        let tile_width = actual_image_width / columns as f32;
        let tile_height = actual_image_height / rows as f32;

        if tile_width != tile_width.round() || tile_height != tile_height.round() {
            Err(MyError::InvalidTileSize(tile_width, tile_height))
        } else {
            Ok((tile_width as u32, tile_height as u32))
        }
    }
}
