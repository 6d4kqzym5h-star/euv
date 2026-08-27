use super::*;

/// Implements creation and countdown logic for `Timer`.
impl Timer {
    /// Creates a one-shot timer that fires once after `duration` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - The countdown duration in seconds.
    ///
    /// # Returns
    ///
    /// - `Timer` - The new one-shot timer.
    pub fn create(duration: f64) -> Timer {
        Timer::new(duration.max(0.0), false)
    }

    /// Creates a repeating timer that fires every `duration` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - The interval between firings in seconds.
    ///
    /// # Returns
    ///
    /// - `Timer` - The new repeating timer.
    pub fn create_repeating(duration: f64) -> Timer {
        Timer::new(duration.max(0.0), true)
    }

    /// Advances the timer by the given delta time.
    ///
    /// # Arguments
    ///
    /// - `f64` - The time elapsed since the last update, in seconds.
    ///
    /// # Returns
    ///
    /// - `u32` - The number of times the timer fired during this update.
    pub fn update(&mut self, delta_time: f64) -> u32 {
        if self.get_paused() || self.get_finished() || self.get_duration() <= 0.0 {
            return 0;
        }
        *self.get_mut_elapsed() += delta_time.max(0.0);
        let mut fire_count: u32 = 0;
        while self.get_elapsed() >= self.get_duration() {
            fire_count += 1;
            if self.get_repeating() {
                *self.get_mut_elapsed() -= self.get_duration();
            } else {
                self.set_elapsed(self.get_duration());
                self.set_finished(true);
                break;
            }
        }
        fire_count
    }

    /// Resets the timer to its initial state so it can count down again.
    pub fn reset(&mut self) {
        self.set_elapsed(0.0);
        self.set_finished(false);
    }

    /// Pauses the timer, preserving the accumulated elapsed time.
    pub fn pause(&mut self) {
        self.set_paused(true);
    }

    /// Resumes a paused timer.
    pub fn resume(&mut self) {
        self.set_paused(false);
    }

    /// Returns whether the timer is currently paused.
    ///
    /// # Returns
    ///
    /// - `bool` - True if paused.
    pub fn is_paused(&self) -> bool {
        self.get_paused()
    }

    /// Returns whether a one-shot timer has fired and stopped.
    ///
    /// # Returns
    ///
    /// - `bool` - True if finished.
    pub fn is_finished(&self) -> bool {
        self.get_finished()
    }

    /// Returns the countdown progress in the range 0.0 to 1.0.
    ///
    /// # Returns
    ///
    /// - `f64` - The progress ratio.
    pub fn progress(&self) -> f64 {
        if self.get_duration() <= 0.0 {
            return 1.0;
        }
        (self.get_elapsed() / self.get_duration()).min(1.0)
    }

    /// Returns the time remaining until the next firing, in seconds.
    ///
    /// # Returns
    ///
    /// - `f64` - The remaining time.
    pub fn remaining(&self) -> f64 {
        (self.get_duration() - self.get_elapsed()).max(0.0)
    }
}

/// Forwards `Timer::update` through the [`Updatable`] trait so timers can
/// participate in the same generic update loop as entities, animators,
/// scenes, and physics worlds.
impl Updatable for Timer {
    /// Advances the simulation by `delta_time` seconds.
    fn update(&mut self, delta_time: f64) {
        let _: u32 = Timer::update(self, delta_time);
    }
}
