#[derive(Debug, PartialEq)]
pub enum Pixel {
    BlackWhite(bool),
    Grayscale(u8),
    IndexedColor(u8),
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
}

#[derive(Debug)]
pub struct PixelMap {
    pixels: Vec<Vec<Pixel>>,
    row_len: usize,
    col_len: usize,
}

impl PixelMap {
    pub fn build(pixels: Vec<Vec<Pixel>>) -> Result<PixelMap, &'static str> {
        let row_len = pixels.len();
        let col_len = pixels.first().unwrap().len();
        if pixels.iter().all(|row| row.len() == col_len) {
            Ok(PixelMap {
                pixels,
                row_len,
                col_len,
            })
        } else {
            Err("Pixel map cannot be jagged; all rows must have the same number of columns")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pixel_map_black_pixels() {
        let pixels = vec![
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)],
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)],
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)],
        ];
        let map = PixelMap::build(pixels).unwrap();

        assert_eq!(map.row_len, 3);
        assert_eq!(map.col_len, 2);
        assert_eq!(
            map.pixels[0],
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)]
        );
        assert_eq!(
            map.pixels[1],
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)]
        );
        assert_eq!(
            map.pixels[2],
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)]
        );
    }

    #[test]
    #[should_panic(expected = "rows must have the same number of columns")]
    fn try_create_pixel_map_jagged_rows() {
        let pixels = vec![
            vec![Pixel::RGB(0, 0, 0), Pixel::RGB(0, 0, 0)],
            vec![Pixel::RGB(0, 0, 0)],
            vec![
                Pixel::RGB(0, 0, 0),
                Pixel::RGB(0, 0, 0),
                Pixel::RGB(0, 0, 0),
            ],
        ];
        PixelMap::build(pixels).unwrap();
    }
}
