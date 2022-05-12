use registry::{Data, Hive, Security};

/// Check dark mode status
pub fn dark_mode_status() -> bool {
    let regkey = Hive::CurrentUser
        .open(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            Security::Read,
        )
        .unwrap();

    let a = regkey.value("AppsUseLightTheme").unwrap();

    a.to_string() == "0x0000000000000000"
}

/// Enable dark mode
pub fn enable_dark_mode() {
    let regkey = Hive::CurrentUser
        .open(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            Security::Write,
        )
        .unwrap();

    regkey
        .set_value("AppsUseLightTheme", &Data::U32(0))
        .expect("Could not set to dark mode");

    regkey
        .set_value("SystemUsesLightTheme", &Data::U32(0))
        .expect("Could not set to dark mode");
}

/// Disable dark mode
pub fn disable_dark_mode() {
    let regkey = Hive::CurrentUser
        .open(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Themes\Personalize",
            Security::Write,
        )
        .unwrap();

    regkey
        .set_value("AppsUseLightTheme", &Data::U32(1))
        .expect("Could not set to dark mode");

    regkey
        .set_value("SystemUsesLightTheme", &Data::U32(1))
        .expect("Could not set to dark mode");
}
