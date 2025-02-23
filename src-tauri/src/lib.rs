
mod database;
mod state;

use state::{AppState, ServiceAccess};
use tauri::{State, Manager, AppHandle};

pub fn run() {
  tauri::Builder::default()
      .manage(AppState { db: Default::default() })
      .invoke_handler(tauri::generate_handler![greet])
      .setup(|app| {
          let handle = app.handle();

          let app_state: State<AppState> = handle.state();
          let db = database::initialize_database(&handle).expect("Database initialize should succeed");
          *app_state.db.lock().unwrap() = Some(db);

          Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

#[tauri::command]
fn greet(app_handle: AppHandle, name: &str) -> String {
    // Should handle errors instead of unwrapping here
    app_handle.db(|db| database::add_item(name, db)).unwrap();

    let items = app_handle.db(|db| database::get_all(db)).unwrap();

    let items_string = items.join(" | ");

    format!("Your name log: {}", items_string)
}