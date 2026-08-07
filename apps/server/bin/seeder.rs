use chrono::Utc;
use sqlx::{Pool, Postgres};
use std::{env, error::Error};
use uuid::Uuid;

fn expect_env(key: &str) -> String {
	env::var(key).unwrap_or_else(|_| panic!("Missing expected env var: {key}"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let db_url = expect_env("LOCAL_POSTGRES_DATABASE_URL");
	let admin_name = expect_env("SEED_ADMIN_NAME");
	let admin_email = expect_env("SEED_ADMIN_EMAIL");
	let admin_username = expect_env("SEED_ADMIN_USERNAME");
	let admin_password = expect_env("SEED_ADMIN_PASSWORD");

	println!("Connecting to: {db_url}");

	let db = Pool::connect(&db_url).await?;
	println!("Database connection success!");

	let pwd_hash = bcrypt::hash(admin_password, 12)?;

	sqlx::query::<Postgres>(
		"INSERT INTO users (id, username, name, email, password_hash, role, last_login_at)
             VALUES ($1, $2, $3, $4, $5, $6::user_role, $7)
             ON CONFLICT (username) DO UPDATE
             SET name = EXCLUDED.name,
                 email = EXCLUDED.email,
                 password_hash = EXCLUDED.password_hash,
                 role = EXCLUDED.role,
                 last_login_at = EXCLUDED.last_login_at",
	)
	.bind(Uuid::new_v4())
	.bind(admin_username)
	.bind(admin_name)
	.bind(admin_email)
	.bind(pwd_hash)
	.bind("admin")
	.bind(Utc::now())
	.execute(&db)
	.await?;

	Ok(())
}
