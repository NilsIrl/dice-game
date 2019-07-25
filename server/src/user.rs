extern crate serde;
extern crate serde_json;

use rocket_contrib::databases::rusqlite::Connection;
use rocket_contrib::json::Json;

use rocket::http::Status;

use serde::Serialize;

#[derive(FromForm)]
pub struct User {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Serialize)]
pub struct LeaderboardEntry {
    pub username: String,
    pub score: i64,
}

impl User {
    pub fn register_user(&self, conn: &Connection) -> Result<Status, Status> {
        conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY ASC UNIQUE, username TEXT NOT NULL UNIQUE, hashed_password TEXT NO NULL, score INTEGER DEFAULT 0)", &[]).unwrap();
        match conn.execute(
            "INSERT INTO users (username, hashed_password) VALUES (?1, ?2)",
            &[&self.username, &self.hashed_password],
        ) {
            Ok(_) => Ok(Status::Created),
            Err(_) => Err(Status::Conflict),
        }
    }

    pub fn delete_user(&self, conn: &Connection) {
        conn.execute(
            "DELETE FROM users WHERE username = ?1 AND hashed_password = ?2",
            &[&self.username, &self.hashed_password],
        )
        .unwrap();
    }

    pub fn get_leaderboard(n: u32, conn: &Connection) -> Json<Vec<LeaderboardEntry>> {
        let mut statement = conn
            .prepare("SELECT username, score FROM users ORDER BY score DESC LIMIT ?1")
            .unwrap();
        let rows = statement
            .query_map(&[&n], |row| {
                       LeaderboardEntry {
                username: row.get(0),
                score: row.get(1),
            }})
            .unwrap();
        Json(rows.flatten().collect())
    }
    pub fn user_exists(username: String, conn: &Connection) -> bool {
        conn.query_row(
            "SELECT username FROM users WHERE username = ?1",
            &[&username],
            |_row| true,
        )
        .is_ok()
    }
}
