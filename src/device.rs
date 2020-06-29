//! System calls for modifying the state of the PeachCloud device.

use std::io;
use std::process::{Command, Output};

/// Executes a system command to reboot the device immediately.
pub fn device_reboot() -> io::Result<Output> {
    info!("Rebooting the device");
    // ideally, we'd like to reboot after 5 seconds to allow time for JSON
    // response but this is not possible with the `shutdown` command alone.
    // TODO: send "rebooting..." message to `peach-oled` for display
    Command::new("sudo")
        .arg("shutdown")
        .arg("-r")
        .arg("now")
        .output()
}

/// Executes a system command to shutdown the device immediately.
pub fn device_shutdown() -> io::Result<Output> {
    info!("Shutting down the device");
    // ideally, we'd like to reboot after 5 seconds to allow time for JSON
    // response but this is not possible with the `shutdown` command alone.
    // TODO: send "shutting down..." message to `peach-oled` for display
    Command::new("sudo").arg("shutdown").arg("now").output()
}
