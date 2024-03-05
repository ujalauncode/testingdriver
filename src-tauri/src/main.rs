// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]

fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use std::process::{Command, Stdio};
use std::os::windows::process::CommandExt; // Add this line

#[tauri::command]
fn mine_driver() -> Result<String, String> {
    let output = Command::new("powershell")
        .args(&["-Command", "$driverInfo = Get-WmiObject Win32_PnPSignedDriver | Select-Object DeviceName, DriverVersion, DriverStatus; ConvertTo-Json $driverInfo"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Command failed with error: {}", stderr))
    }
}

// use std::convert::TryInto;

use mongodb::{Client, Collection};
use serde::Serialize;
use std::error::Error;
#[derive(Serialize)]
struct SystemInfo {
    os_info: String,
    cpu_info: String,
    disk_info: String,
    video_controller_info: String,
    product_id: String,
    memory_info: String,
}

async fn insert_system_info(system_info: SystemInfo) -> Result<(), Box<dyn Error>> {
    // Connect to MongoDB
    let client = Client::with_uri_str("mongodb+srv://user1:user123@cluster0.g1p3xeq.mongodb.net/driversdbs").await?;
    let db = client.database("systeminfo");
    let collection: Collection<SystemInfo> = db.collection("system_info");

    // Insert system info into the collection
    collection.insert_one(system_info, None).await?;

    Ok(())
}

#[tauri::command]
fn __cmd__testing() -> SystemInfo {
    use std::process::{Command, Stdio};

    let os_info = Command::new("wmic")
        .args(&["os", "get", "Caption"])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag

        .output()
        .expect("Failed to execute command")
        .stdout;

    let os_info_str = String::from_utf8_lossy(&os_info).lines().nth(1).unwrap_or("").trim().to_string();

    let cpu_info = Command::new("wmic")
        .args(&["cpu", "get", "name"])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag

        .output()
        .expect("Failed to execute command")
        .stdout;

    let cpu_info_str = String::from_utf8_lossy(&cpu_info).lines().nth(1).unwrap_or("").trim().to_string();

    let disk_info = Command::new("wmic")
        .args(&["diskdrive", "get", "size"])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag

        .output()
        .expect("Failed to execute command")
        .stdout;

    let disk_info_str = String::from_utf8_lossy(&disk_info).lines().nth(1).unwrap_or("").trim().to_string();

    let disk_info_gb = match disk_info_str.parse::<u64>() {
        Ok(bytes) => bytes / (1024 * 1024 * 1024), // Convert bytes to gigabytes
        Err(_) => 0, // Handle parse errors gracefully
    };

    let disk_info_gb_str = format!("{}", disk_info_gb);

    let video_controller_info = Command::new("wmic")
        .args(&["path", "Win32_VideoController", "get", "name"])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag

        .output()
        .expect("Failed to execute command")
        .stdout;

    let video_controller_info_str = String::from_utf8_lossy(&video_controller_info).lines().nth(1).unwrap_or("").trim().to_string();

    let product_id = Command::new("wmic")
        .args(&["bios", "get", "serialnumber"])
        .stdout(Stdio::piped())
        .creation_flags(0x08000000) // CREATE_NO_WINDOW flag

        .output()
        .expect("Failed to execute command")
        .stdout;

    let product_id_str = String::from_utf8_lossy(&product_id).lines().nth(1).unwrap_or("").trim().to_string();

    let memory_info = Command::new("wmic")
    .args(&["ComputerSystem", "get", "TotalPhysicalMemory"])
    .stdout(Stdio::piped())
    .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
    .output()
    .expect("Failed to execute command")
    .stdout;

let memory_info_str = String::from_utf8_lossy(&memory_info)
    .lines()
    .nth(1)
    .unwrap_or("")
    .trim()
    .to_string();


let memory_info_gb = match memory_info_str.parse::<f64>() {
    Ok(bytes) => bytes / (1024.0 * 1024.0 * 1024.0), // Convert bytes to gigabytes
    Err(_) => 0.0, // Handle parse errors gracefully
};

let rounded_memory_size_gb: u64 = memory_info_gb.ceil() as u64;

    SystemInfo {
        os_info: os_info_str,
        cpu_info: cpu_info_str,
        disk_info: disk_info_gb_str,
        video_controller_info: video_controller_info_str,
        product_id: product_id_str,
        memory_info:format!("{} ", rounded_memory_size_gb),
    }

   
}



// #[tauri::command]
// fn __cmd__checkagain() -> String {
//     use std::process::Command;
//     let _ = Command::new("powershell")
//         .args(&["-Command", "Install-Module -Name PSWindowsUpdate -Force -AllowClobber"])
//         .output()
//         .expect("Failed to execute PowerShell command to install module");
//     let _ = Command::new("powershell")
//         .args(&["-Command", "Import-Module PSWindowsUpdate"])
//         .output()
//         .expect("Failed to execute PowerShell command to import module");

//     let check_updates_cmd = Command::new("powershell")
//         .args(&["-Command", "Get-WindowsUpdate -MicrosoftUpdate -Summary"])
//         .output();

//     match check_updates_cmd {
//         Ok(output) => {
//             let updates_output = String::from_utf8_lossy(&output.stdout);
            
//             println!("Updates Output: {}", updates_output);

//             if updates_output.contains("No Updates Found") {
//                 "No updates are available.".to_string()
//             } else {
//                 updates_output.to_string()
//             }
//         },
//         Err(error) => {
//             eprintln!("Error executing PowerShell command: {:?}", error);
//             "Error executing PowerShell command.".to_string()
//         }
//     }
// }



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, mine_driver, __cmd__testing])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}