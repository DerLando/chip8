const TICKS_PER_SECOND: u8 = 60;
const MS_PER_TICK: u8 = (1000u16 / TICKS_PER_SECOND as u16) as u8;

/// A basic timer abstractions. Since I don't want to use threads
/// to have a simpler model for WASM, the timer rather has to be
/// polled using it's [`Timer::tick()`] function.
#[cfg(feature = "std")]
pub(crate) struct Timer {
    last_tick: std::time::Instant,
}

#[cfg(feature = "std")]
impl Timer {
    pub fn new() -> Self {
        Self {
            last_tick: std::time::Instant::now(),
        }
    }
    /// Tick the timer and return the amount of steps
    /// it took to get back in sync. The timer will store the [`Instant`]
    /// this function got called on and calculate the number of steps
    /// from the difference towards the last invocation to the tick function
    pub fn tick(&mut self) -> u8 {
        let elapsed = self.last_tick.elapsed().as_millis();
        let steps = elapsed * TICKS_PER_SECOND as u128 / 1000;
        self.last_tick = std::time::Instant::now();

        steps as u8
    }
}

#[cfg(all(not(feature = "std"), feature = "js"))]
pub(crate) struct Timer {
    last_tick: f64,
}

#[cfg(all(not(feature = "std"), feature = "js"))]
impl Timer {
    pub fn new() -> Self {
        Self {
            last_tick: js_sys::Date::now(),
        }
    }

    pub fn tick(&mut self) -> u8 {
        let now = js_sys::Date::now();
        let elapsed = now - self.last_tick;

        let steps = elapsed as u128 * TICKS_PER_SECOND as u128 / 1000;
        self.last_tick = now;

        steps as u8
    }
}
