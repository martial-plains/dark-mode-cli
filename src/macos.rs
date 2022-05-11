use crate::utils::run_apple_script;

/// Check dark mode status
pub fn dark_mode_status() -> bool {
    let script =
        "tell application \"System Events\" to tell appearance preferences to get dark mode";
    let output = run_apple_script(script);
    output.contains("true")
}

/// Enable dark mode
pub fn enable_dark_mode() {
    let script = "tell application \"System Events\" to tell appearance preferences to set dark mode to true";
    run_apple_script(script);
}

/// Disable dark mode
pub fn disable_dark_mode() {
    let script =
        "tell application \"System Events\" to tell appearance preferences to set dark mode to false";
    run_apple_script(script);
}
