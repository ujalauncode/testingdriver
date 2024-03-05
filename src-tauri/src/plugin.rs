// src-tauri/src/plugin.rs

use tauri::api::process::Command;

// Define your Plugin struct
pub struct Plugin;

impl Plugin {
    pub fn new() -> Self {
        Plugin
    }

    #[tauri::command]
    pub fn execute_powerShell_command() -> Result<String, String> {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("$driverInfo = Get-WmiObject Win32_PnPSignedDriver | Select-Object DeviceName, DriverVersion; ConvertTo-Json $driverInfo")
            .output()
            .expect("Failed to execute command");

        let result = String::from_utf8_lossy(&output.stdout).to_string();
        println!(result);
        Ok(result)
    }
}
