use std::io;
use std::process::{Command, Output};

/// Executes a system command to reboot the device after 5 seconds have passed.
///
pub fn device_reboot() -> io::Result<Output> {
    info!("Rebooting the device");
    // reboot after 5 seconds (allows time for JSON response)
    Command::new("sudo")
        .arg("shutdown")
        .arg("-r")
        .arg("-t")
        .arg("5")
        .output()
}

/// Executes a system command to shutdown the device after 5 seconds have passed.
///
pub fn device_shutdown() -> io::Result<Output> {
    info!("Shutting down the device");
    // shutdown after 5 seconds (allows time for JSON response)
    Command::new("sudo")
        .arg("shutdown")
        .arg("-t")
        .arg("5")
        .output()
}
