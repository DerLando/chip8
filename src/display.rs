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

    pub(crate) fn clear(&mut self) {
        self.buffer.fill(0);
    }
}
