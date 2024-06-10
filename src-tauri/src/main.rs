#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use sqlx::Row;

struct TimeCard {
  pub user_id: i32,
  pub start_time_epoch: i64,
  pub end_time_epoch: i64,
  pub description: String,
}

async fn open_timecard(user_id: i32, description: String, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {

  let timecard = TimeCard {
    user_id,
    start_time_epoch: std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs() as i64,
    end_time_epoch: 0,
    description,
  };

  let query = "INSERT INTO timecards (user_id, start_time, end_time, description) VALUES ($1, $2, $3, $4)";
  sqlx::query(query)
    .bind(timecard.user_id)
    .bind(timecard.start_time_epoch)
    .bind(timecard.end_time_epoch)
    .bind(timecard.description)
    .execute(pool)
    .await?;

  Ok(())
}

async fn close_timecard(user_id: i32, description: String, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {

  let query = "SELECT * FROM timecards WHERE user_id = $1 AND end_time = 0";
  let row = sqlx::query(query)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

  let mut timecard = TimeCard {
    user_id: row.get(0),
    start_time_epoch: row.get(1),
    end_time_epoch: row.get(2),
    description: row.get(3),
  };

  timecard.end_time_epoch = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH)?.as_secs() as i64;
  timecard.description = description;


  let query = "UPDATE timecards SET end_time = $1, description = $2 WHERE user_id = $3 AND end_time = 0";
  sqlx::query(query)
    .bind(timecard.end_time_epoch)
    .bind(timecard.description)
    .bind(timecard.user_id)
    .execute(pool)
    .await?;


  Ok(())
}

async fn get_pool() -> Result<sqlx::PgPool, Box<dyn Error>> {
  let url = "postgres://admin:testing@localhost:5432/timecards";
  let pool = sqlx::postgres::PgPool::connect(url).await?;

  //migration
  sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;

  Ok(pool)
}

fn main() {

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![clock_in, clock_out])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn clock_in() -> () {
  println!("Clocking in.");
  let pool = get_pool().await.expect("Error getting pool");

  open_timecard(1, "Clocked In".to_string(), &pool).await.expect("Error clocking in");

  ()
}

#[tauri::command]
async fn clock_out() -> () {
  println!("Clocking out");
  let pool = get_pool().await.expect("Error getting pool");
  close_timecard(1, "Clocked Out".to_string(), &pool).await.expect("Error clocking out");
  ()
}