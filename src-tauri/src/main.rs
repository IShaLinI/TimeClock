#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![clock_in, clock_out])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn clock_in() -> () {
  println!("Clocking in");
  ()
}

#[tauri::command]
fn clock_out() -> () {
  println!("Clocking out");
  ()
}