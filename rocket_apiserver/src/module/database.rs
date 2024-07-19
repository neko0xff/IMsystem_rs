use chrono::{DateTime, Local};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::{process,env};

/*相關資料結構 */
pub struct AppState {
    pub db: MySqlPool,
}

impl AppState {
    pub async fn new() -> Self {
        
        dotenv().ok(); // 戴入設定檔內容

        /*時問格式化輸出*/
        let current_local: DateTime<Local> = Local::now();
        let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
        
        /*資料庫連線*/
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let pool = MySqlPoolOptions::new()
            .max_connections(10) // 最大連線數
            .connect(&database_url)
            .await
            .unwrap_or_else(|err| {
                eprintln!("[{}]  Failed to connect to the database: {:?}",custom_format, err);
                process::exit(1);
            }
        );
        
         println!("[{}] Connection to the database service(MariaDB/MySQL) ........",custom_format);
         Self { db: pool }
        
    }
}
