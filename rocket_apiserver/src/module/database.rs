use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{process,env};
use super::format_str::console_time;

/*相關資料結構 */
pub struct AppState {
    pub db: MySqlPool,
}

impl AppState {
    pub async fn new() -> Self {
        
        dotenv().ok(); // 戴入設定檔內容

        /*資料庫連線*/
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(10) // 最大連線數
            .connect(&database_url)
            .await
            .unwrap_or_else(|err| {
                eprintln!("[{}]  Failed to connect to the database: {:?}",console_time(), err);
                process::exit(1);
            }
        );
        
         println!("[{}] Connection to the database service(MariaDB/MySQL) ........",console_time());
         Self { db: pool }
        
    }
}
