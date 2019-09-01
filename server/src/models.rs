use crate::schema::users;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct LeaderboardEntry {
    pub username: String,
    pub score: i32,
}

#[derive(Insertable, FromForm)]
#[table_name = "users"]
pub struct UserCredentials {
    pub username: String,
    pub password_crypt: String,
}
