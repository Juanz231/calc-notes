// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn calculate_min_grade(grade: f64, percentage: f64) -> String {
    let remaining_percentage = 100.0 - percentage;
    let min_grade_for_remaining = (6.0 - (grade * (percentage / 100.0))) / (remaining_percentage / 100.0);

    if min_grade_for_remaining < 0.0 {
        format!("You have already passed the class!")
    } else {
        format!("You need a minimum grade of {:.2} in the remaining {}% of the class to pass.", min_grade_for_remaining, remaining_percentage)
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![calculate_min_grade])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
