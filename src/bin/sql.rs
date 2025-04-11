#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use sqlx::postgres::PgPoolOptions;

    let pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect("postgres://postgres:postgres@127.0.0.1:5432/aloha?sslmode=disable")
        .await?;

    let row: (i32,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;

    println!("DB connection OK: {}", row.0);
    Ok(())
}
