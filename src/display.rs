#[cfg(feature = "std")]
use std::fmt::Display;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const BIT_MASKS: [u8; 8] = [
    0b1000_0000,
    0b0100_0000,
    0b0010_0000,
    0b0001_0000,
    0b0000_1000,
    0b0000_0100,
    0b0000_0010,
    0b0000_0001,
];

pub(crate) struct DisplayBuffer {
    /// Display is 64x32 pixels
    /// A pixel is either on or off,
    /// meaning we can store 8 pixels in 1 byte
    buffer: [u8; 256],
}

impl DisplayBuffer {
    pub fn new() -> Self {
        Self { buffer: [0; 256] }
    }

    fn pos_to_index(x: u8, y: u8) -> usize {
        y as usize * DISPLAY_WIDTH / 8 + x as usize / 8
    }

    /// Flip the value of the pixel at the given x and y positions.
    /// If the pixel is turned off in the process, this function will return true.
    pub(crate) fn flip_pixel(&mut self, x: u8, y: u8) -> bool {
        let index = Self::pos_to_index(x, y);
        let sub_index = (x % 8) as usize;
        let pixel_byte = &mut self.buffer[index];
        let is_turned_off = *pixel_byte & BIT_MASKS[sub_index] != 0;
        *pixel_byte ^= BIT_MASKS[sub_index];
        is_turned_off
    }

    pub fn is_pixel_on(&self, x: u8, y: u8) -> bool {
        let index = Self::pos_to_index(x, y);
        let sub_index = (x % 8) as usize;
        let pixel_byte = self.buffer[index];
        pixel_byte & BIT_MASKS[sub_index] != 0
    }

    pub(crate) fn clear(&mut self) {
        self.buffer.fill(0);
    }
}

#[cfg(feature = "std")]
impl Display for DisplayBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..DISPLAY_HEIGHT as u8 {
            for col in 0..DISPLAY_WIDTH as u8 {
                let symbol = if self.is_pixel_on(col, row) {
                    '◼'
                } else {
                    '◻'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_draw_sprite_row() {
        let mut display = DisplayBuffer::new();
        for x in 0..8 {
            assert!(!display.is_pixel_on(x, 0));
            assert!(!display.flip_pixel(x, 0));
            assert!(display.is_pixel_on(x, 0));
        }
    }

    #[test]
    fn can_clear_sprite_row() {
        let mut display = DisplayBuffer::new();
        for x in 0..8 {
            assert!(!display.is_pixel_on(x, 0));
            assert!(!display.flip_pixel(x, 0));
            assert!(display.is_pixel_on(x, 0));
            assert!(display.flip_pixel(x, 0));
            assert!(!display.is_pixel_on(x, 0));
        }
    }
}
