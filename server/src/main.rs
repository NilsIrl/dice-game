#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use rocket::http::Status;
use rocket::request::LenientForm;
use rocket_contrib::json::Json;

use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

// sql_function!(fn crypt(password: Text, salt: Text) -> Text);
// sql_function!(fn gen_salt(hash_algorithm_type: Text) -> Text);

#[database("game")]
struct GameDbConn(diesel::PgConnection);

#[post("/user", data = "<user>")]
fn register_user(
    user: LenientForm<models::UserCredentials>,
    conn: GameDbConn,
) -> Result<Status, Status> {
    diesel::insert_into(schema::users::table)
        .values(&user.into_inner())
        .execute(&*conn)
        .unwrap();
    Ok(Status::Created)
}

#[delete("/user", data = "<user>")]
fn delete_user(user: LenientForm<models::UserCredentials>, conn: GameDbConn) {
    diesel::delete(
        schema::users::table.filter(
            schema::users::username
                .eq(&user.username)
                .and(schema::users::username.eq(&user.password_crypt)),
        ),
    )
    .execute(&*conn)
    .unwrap();
}

#[get("/user?<n>")]
fn get_user(n: i64, conn: GameDbConn) -> Json<std::vec::Vec<models::LeaderboardEntry>> {
    Json(
        schema::users::table
            .limit(n)
            .order(schema::users::score.desc())
            .select((schema::users::username, schema::users::score))
            .load(&*conn)
            .unwrap(),
    )
}

#[get("/user/<name>")]
fn user_exists(name: String, conn: GameDbConn) -> Json<bool> {
    Json(
        schema::users::table
            .filter(schema::users::username.eq(name))
            .count()
            .get_result::<i64>(&*conn)
            .unwrap()
            >= 1,
    )
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![register_user, delete_user, get_user, user_exists],
        )
        .attach(GameDbConn::fairing())
        .launch();
}
