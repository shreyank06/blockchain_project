use crate::arbitrage::Opportunity;
use sqlx::{SqlitePool, Error};

// Initialize the database (call once at startup)
pub async fn init_db(db_url: &str) -> Result<SqlitePool, Error> {
    let pool = SqlitePool::connect(db_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS opportunities (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dex_buy TEXT NOT NULL,
            dex_sell TEXT NOT NULL,
            price_buy REAL NOT NULL,
            price_sell REAL NOT NULL,
            profit REAL NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// Insert opportunity into DB - changed return type
pub async fn log_opportunity(pool: &SqlitePool, op: &Opportunity, profit: f64) -> Result<(), Error> {
    sqlx::query(
        r#"
        INSERT INTO opportunities (dex_buy, dex_sell, price_buy, price_sell, profit)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(&op.buy_dex)
    .bind(&op.sell_dex)
    .bind(op.buy_price)
    .bind(op.sell_price)
    .bind(profit)
    .execute(pool)
    .await?;

    Ok(())
}