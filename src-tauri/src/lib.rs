use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tauri::{command, Manager, State};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let db_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let rt = tokio::runtime::Runtime::new().unwrap();
            let pool: Pool<Postgres> = rt.block_on(async {
                PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to create pool")
            });
            app.manage(Mutex::new(pool));
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_tables])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize,Deserialize, Debug)]
struct Table {
    table_name: Option<String>
}

#[command]
async fn list_tables(
    state_mutex: State<'_, Mutex<Pool<Postgres>>>,
) -> Result<String, String> {
    let pool = {
        let guard = state_mutex.lock().unwrap();
        guard.clone()
    };

    let rows: Vec<Table> = sqlx::query_as!(
            Table,
            "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'"
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch tables");

    if rows.is_empty() {    
        return Err("No tables found".to_string());
    }

    println!("Tables: {:?}", rows);

    Ok(rows.iter()
        .filter_map(|row| row.table_name.clone())
        .collect::<Vec<String>>()
        .join(", "))
}
