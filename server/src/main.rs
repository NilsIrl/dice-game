#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod user;

use rocket::http::Status;
use rocket::request::LenientForm;
use rocket_contrib::databases::rusqlite;
use rocket_contrib::json::Json;

#[database("sqlite_game")]
struct SqliteGameConn(rusqlite::Connection);

#[post("/user", data = "<user>")]
fn register_user(user: LenientForm<user::User>, conn: SqliteGameConn) -> Result<Status, Status> {
    user.register_user(&*conn)
}

#[delete("/user", data = "<user>")]
fn delete_user(user: LenientForm<user::User>, conn: SqliteGameConn) {
    user.delete_user(&*conn);
}

#[get("/user?<n>")]
fn get_user(n: u32, conn: SqliteGameConn) -> Json<std::vec::Vec<user::LeaderboardRow>> {
    user::User::get_leaderboard(n, &*conn)
}

#[get("/user/<name>")]
fn user_exists(name: String, conn: SqliteGameConn) -> Json<bool> {
    Json(user::User::user_exists(name, &*conn))
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![register_user, delete_user, get_user, user_exists],
        )
        .attach(SqliteGameConn::fairing())
        .launch();
}
