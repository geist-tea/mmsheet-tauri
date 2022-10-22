#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod data;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use data::{CharSheet, AdvantageInfo};


#[tauri::command]
fn new_sheet() -> CharSheet{
    CharSheet::new()
}

#[tauri::command]
fn get_advantage_list() -> Vec<AdvantageInfo> {
    let reader = File::open("data/advantages.json")
        .map(BufReader::new).unwrap();
    let data: Vec<AdvantageInfo> = serde_json::from_reader(reader).unwrap();
    data
}

#[tauri::command]
fn save_sheet(c: CharSheet, path: String) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    serde_json::to_writer(file, &c);
}

#[tauri::command]
fn load_sheet(path: String) -> CharSheet {
    let reader = File::open(path)
        .map(BufReader::new).unwrap();
    let data:CharSheet = serde_json::from_reader(reader).unwrap();
    data
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![new_sheet, get_advantage_list, save_sheet, load_sheet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
