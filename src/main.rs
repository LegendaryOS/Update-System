use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};

fn main() {
    // ANSI color codes
    const BLUE: &str = "\x1b[1;34m";   // Bold Blue for info
    const GREEN: &str = "\x1b[1;32m";  // Bold Green for success
    const RED: &str = "\x1b[1;31m";    // Bold Red for errors
    const YELLOW: &str = "\x1b[1;33m"; // Bold Yellow for headers
    const MAGENTA: &str = "\x1b[1;35m"; // Bold Magenta for completion
    const RESET: &str = "\x1b[0m";     // Reset colors

    // Header
    println!("{}===== System Update Started ====={}", YELLOW, RESET);

    // Run rpm-ostree update
    println!("{}[INFO] Initiating rpm-ostree update...{}", BLUE, RESET);
    let rpm_ostree_status = Command::new("sudo")
        .args(&["/usr/lib/legendaryos/rpm-ostree", "update"])
        .status()
        .expect("Failed to execute rpm-ostree update");
    if !rpm_ostree_status.success() {
        eprintln!("{}[ERROR] rpm-ostree update failed{}", RED, RESET);
        std::process::exit(1);
    }
    println!("{}[SUCCESS] rpm-ostree update completed{}", GREEN, RESET);

    // Run rpm-ostree upgrade
    println!("{}[INFO] Initiating rpm-ostree upgrade...{}", BLUE, RESET);
    let rpm_ostree_upgrade_status = Command::new("sudo")
        .args(&["/usr/lib/legendaryos/rpm-ostree", "upgrade"])
        .status()
        .expect("Failed to execute rpm-ostree upgrade");
    if !rpm_ostree_upgrade_status.success() {
        eprintln!("{}[ERROR] rpm-ostree upgrade failed{}", RED, RESET);
        std::process::exit(1);
    }
    println!("{}[SUCCESS] rpm-ostree upgrade completed{}", GREEN, RESET);

    // Run flatpak update
    println!("{}[INFO] Initiating flatpak update...{}", BLUE, RESET);
    let flatpak_status = Command::new("flatpak")
        .args(&["update", "-y"])
        .status()
        .expect("Failed to execute flatpak update");
    if !flatpak_status.success() {
        eprintln!("{}[ERROR] flatpak update failed{}", RED, RESET);
        std::process::exit(1);
    }
    println!("{}[SUCCESS] flatpak update completed{}", GREEN, RESET);

    // Run fwupdmgr update
    println!("{}[INFO] Initiating fwupdmgr update...{}", BLUE, RESET);
    let fwupdmgr_status = Command::new("fwupdmgr")
        .arg("update")
        .status()
        .expect("Failed to execute fwupdmgr update");
    if !fwupdmgr_status.success() {
        eprintln!("{}[ERROR] fwupdmgr update failed{}", RED, RESET);
        std::process::exit(1);
    }
    println!("{}[SUCCESS] fwupdmgr update completed{}", GREEN, RESET);

    // Final message and wait
    println!("{}===== System Update Completed Successfully ====={}", MAGENTA, RESET);
    io::stdout().flush().unwrap();
    sleep(Duration::from_secs(15));
}
