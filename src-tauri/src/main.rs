// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



// src-tauri/main.rs

// #[tauri::command]
// fn upload_photo(file: String){
//     let mut img = open_image(&file).expect("File should open");
// 	threshold(&mut img, 60_u32);
// 	save_image(img, "new_image.png").expect("File should be saved");
// }
#[tauri::command]
fn upload_photo(){
    //let mut img = open_image(&file).expect("File should open");

}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![upload_photo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
