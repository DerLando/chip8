pub(crate) struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub const fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn press(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn release(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }
}
