mod grid;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            grid::generate_sudoku,
            grid::check_solution,
            grid::is_valid_move,
            grid::is_complete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
