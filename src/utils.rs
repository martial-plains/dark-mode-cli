use std::process::Command;

/// Run AppleScript
#[cfg(target_os = "macos")]
pub fn run_apple_script(script: &str) -> String {
    let process = Command::new("osascript")
        .arg("-l")
        .arg("AppleScript")
        .arg("-e")
        .arg(script)
        .output();

    let output = process.expect("Failed to run AppleScript");
    String::from_utf8(output.stdout).unwrap()
}
