use std::error::Error;

use crate::pixels::PixelMap;

#[derive(Debug)]
struct PortableBitMap {
    height: usize,
    width: usize,
    pixels: PixelMap,
}

enum FileType {
    PBM,
    PGM,
    PPM,
}

impl PortableBitMap {
    pub fn new(pixels: PixelMap) -> PortableBitMap {
        PortableBitMap {
            height: pixels.row_len(),
            width: pixels.col_len(),
            pixels,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn write_to_file(&self, file_type: FileType, file_path: &str) -> Result<(), &'static str> {
        Err("temporary")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pbm_empty_pixel_map() {
        let pbm = PortableBitMap::new(PixelMap::default());

        assert_eq!(pbm.width, 0);
        assert_eq!(pbm.height, 0);
    }

    #[test]
    fn write_to_file_blackwhite() {
        let pbm = PortableBitMap::new(PixelMap::default());
        let file_path = "empty_img.pbm";

        pbm.write_to_file(FileType::PBM, file_path).unwrap();
    }
}
