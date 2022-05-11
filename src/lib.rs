#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

mod utils;

/// Dark mode struct
pub struct DarkMode {
    enabled: bool,
}

impl DarkMode {
    /// Create new DarkMode
    pub fn new() -> Self {
        Self {
            enabled: dark_mode_status(),
        }
    }

    /// Toggle dark mode
    pub fn toggle(&mut self) {
        if self.is_enabled() {
            disable_dark_mode()
        } else {
            enable_dark_mode()
        };
        self.enabled = !self.enabled;
    }

    /// Check if dark mode is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enable dark mode
    pub fn enable(&mut self) {
        enable_dark_mode();
        self.enabled = true;
    }

    /// Disable dark mode
    pub fn disable(&mut self) {
        disable_dark_mode();
        self.enabled = false;
    }

    /// Get dark mode status
    pub fn status(&self) -> bool {
        dark_mode_status()
    }
}

impl Default for DarkMode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

    #[test]
    fn test_dark_mode_status() {
        enable_dark_mode();
        sleep(std::time::Duration::from_secs(5));
        assert!(dark_mode_status());
    }

    #[test]
    fn test_enable_disable_mode() {
        enable_dark_mode();
        sleep(std::time::Duration::from_secs(5));
        assert!(dark_mode_status());
        disable_dark_mode();
        sleep(std::time::Duration::from_secs(5));
        assert!(!dark_mode_status());
    }
}
