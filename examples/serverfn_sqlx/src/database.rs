static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

pub async fn init_db() {
    let pool = sqlx::SqlitePool::connect("sqlite:db.sqlite3")
        .await
        .expect("Could not make pool.");
    let _ = DB.set(pool);
}

pub fn get_db<'a>() -> &'a sqlx::SqlitePool {
    DB.get().expect("database unitialized")
}
