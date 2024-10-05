use sqlx::{PgPool, Row};
use std::env;
use dotenv::dotenv;

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}

async fn create_user(pool: &PgPool, name: &str, email: &str) -> Result<User, sqlx::Error> {
    let row = sqlx::query!(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        name,
        email
    )
    .fetch_one(pool)
    .await?;
    
    Ok(User {
        id: row.id,
        name: row.name,
        email: row.email,
    })
}

async fn read_user(pool: &PgPool, user_id: i32) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(User {
            id: row.id,
            name: row.name,
            email: row.email,
        })),
        None => Ok(None),
    }
}

async fn update_user(pool: &PgPool, user_id: i32, name: &str, email: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
        name,
        email,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match row {
        Some(row) => Ok(Some(User {
            id: row.id,
            name: row.name,
            email: row.email,
        })),
        None => Ok(None),
    }
}

async fn delete_user(pool: &PgPool, user_id: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // Create a user
    let user = create_user(&pool, "Alice", "alice@example.com").await?;
    println!("Created User: {:?}", user);

    // Read the user
    if let Some(user) = read_user(&pool, user.id).await? {
        println!("Read User: {:?}", user);
    }

    // Update the user
    if let Some(updated_user) = update_user(&pool, user.id, "Alice Smith", "alice.smith@example.com").await? {
        println!("Updated User: {:?}", updated_user);
    }

    // Delete the user
    if delete_user(&pool, user.id).await? {
        println!("Deleted User with ID: {}", user.id);
    }

    Ok(())
}
