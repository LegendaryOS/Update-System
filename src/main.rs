use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Run sudo zypper update
    println!("[INFO] Running zypper update...");
    let zypper_status = Command::new("sudo")
    .args(&["zypper", "update"])
    .status()
    .expect("Failed to execute zypper update");
    if !zypper_status.success() {
        eprintln!("[ERROR] zypper update failed");
        std::process::exit(1);
    }

    // Run flatpak update -y
    println!("[INFO] Running flatpak update...");
    let flatpak_status = Command::new("flatpak")
    .args(&["update", "-y"])
    .status()
    .expect("Failed to execute flatpak update");
    if !flatpak_status.success() {
        eprintln!("[ERROR] flatpak update failed");
        std::process::exit(1);
    }

    // Run fwupdmgr update
    println!("[INFO] Running fwupdmgr update...");
    let fwupdmgr_status = Command::new("fwupdmgr")
    .arg("update")
    .status()
    .expect("Failed to execute fwupdmgr update");
    if !fwupdmgr_status.success() {
        eprintln!("[ERROR] fwupdmgr update failed");
        std::process::exit(1);
    }

    // Run LegendaryOS verification script
    println!("[INFO] Running LegendaryOS verification script...");
    let script_status = Command::new("sudo")
    .args(&["/usr/share/LegendaryOS/SCRIPTS/verify-legendaryos.py"])
    .status()
    .expect("Failed to execute LegendaryOS verification script");
    if !script_status.success() {
        eprintln!("[ERROR] LegendaryOS verification script failed");
        std::process::exit(1);
    }

    // Final message and wait
    println!("[INFO] Update Complete");
    sleep(Duration::from_secs(15));
}
