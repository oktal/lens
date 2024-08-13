// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lens::Lens;

mod cmd;
mod common;
mod lens;
mod query;

fn main() {
    let (lens, query_executor) = Lens::new();

    tauri::Builder::default()
        .setup(|_| {
            tauri::async_runtime::spawn(query_executor.run());
            Ok(())
        })
        .manage(lens)
        .invoke_handler(tauri::generate_handler![
            cmd::create::create_datasource,
            cmd::list::list_datasources,
            cmd::list::list_databases,
            cmd::sql::sql,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
