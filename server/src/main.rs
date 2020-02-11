#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use schema::{games, rounds};

use rocket::http::Status;
use rocket::request::LenientForm;
use rocket_contrib::json::Json;

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use models::AuthenticatedUser;

use rand::Rng;

// sql_function!(fn crypt(password: Text, salt: Text) -> Text);
// sql_function!(fn gen_salt(hash_algorithm_type: Text) -> Text);

#[database("game")]
struct GameDbConn(diesel::PgConnection);

embed_migrations!();

#[derive(Insertable, Queryable, serde::Serialize)]
#[table_name = "rounds"]
struct Round {
    game_id: i32,
    round_count: i16,
    player1_throws: Vec<i16>,
    player2_throws: Vec<i16>,
}

impl Round {
    fn new(round_count: i16, rng: &mut rand::rngs::ThreadRng) -> Round {
        let mut round = Round {
            game_id: 0,
            round_count: round_count,
            player1_throws: vec![rng.gen_range(1, 7), rng.gen_range(1, 7)],
            player2_throws: vec![rng.gen_range(1, 7), rng.gen_range(1, 7)],
        };

        if round.player1_throws[0] == round.player1_throws[1] {
            round.player1_throws.push(rng.gen_range(1, 7));
        }

        if round.player2_throws[0] == round.player2_throws[1] {
            round.player2_throws.push(rng.gen_range(1, 7));
        }

        round
    }
}

#[derive(Insertable)]
#[table_name = "games"]
struct LobbyGame {
    player1_id: i32,
    player1_score: i16,
    player2_score: i16,
    player1_extra: Vec<i16>,
    player2_extra: Vec<i16>,
}

#[derive(serde::Serialize)]
struct LobbyEntry {
    game_id: i32,
    player1: String,
}

#[get("/games?<n>")] // TODO: maybe n can be optional
fn get_games(n: i64, conn: GameDbConn) -> Json<Vec<LobbyEntry>> {
    Json(
        schema::games::table
            .filter(schema::games::player2_id.is_null())
            .select((schema::games::id, schema::games::player1_id))
            .limit(n)
            .load(&*conn)
            .unwrap()
            .iter()
            .map(|(game_id, player1_id): &(i32, i32)| LobbyEntry {
                game_id: *game_id,
                player1: schema::users::table
                    .filter(schema::users::id.eq(player1_id))
                    .select(schema::users::username)
                    .get_result(&*conn)
                    .unwrap(),
            })
            .collect(),
    )
}

#[post("/games")]
fn create_game(user: AuthenticatedUser, connection: GameDbConn) -> Json<i32> {
    let mut rng = rand::thread_rng(); // TODO: investigate whether this should be called on each request
    let mut rounds = [
        Round::new(0, &mut rng),
        Round::new(1, &mut rng),
        Round::new(2, &mut rng),
        Round::new(3, &mut rng),
        Round::new(4, &mut rng),
    ];
    let mut player1_score = 0;
    let mut player2_score = 0;
    let mut player1_extra = vec![];
    let mut player2_extra = vec![];
    for round in &rounds {
        player1_score += round.player1_throws[0] + round.player1_throws[1];
        player2_score += round.player2_throws[0] + round.player2_throws[1];
        player1_score += round.player1_throws.get(2).unwrap_or(&0);
        player2_score += round.player2_throws.get(2).unwrap_or(&0);

        player1_score = if player1_score % 2 == 0 {
            player1_score + 10
        } else if player1_score - 5 >= 0 {
            player1_score - 5
        } else {
            0
        };
        player2_score = if player2_score % 2 == 0 {
            player2_score + 10
        } else if player2_score - 5 >= 0 {
            player2_score - 5
        } else {
            0
        };
    }

    while player1_score == player2_score {
        player1_extra.push(rng.gen_range(1, 7));
        player2_extra.push(rng.gen_range(1, 7));
        player1_score += player1_extra.last().unwrap();
        player2_score += player2_extra.last().unwrap();
    }

    let lobby = LobbyGame {
        player1_id: user.id,
        player1_score: player1_score,
        player2_score: player2_score,
        player1_extra: player1_extra,
        player2_extra: player2_extra,
    };
    let game_id = diesel::insert_into(schema::games::table)
        .values(&lobby)
        .returning(schema::games::id)
        .get_result(&*connection)
        .unwrap();

    for round in &mut rounds {
        round.game_id = game_id;
    }
    diesel::insert_into(schema::rounds::table)
        .values(&rounds[..])
        .execute(&*connection)
        .unwrap();
    Json(game_id)
}

#[post("/games/<game>/join")]
fn join_game(game: i32, user: AuthenticatedUser, db_conn: GameDbConn) {
    diesel::update(
        schema::games::table.filter(
            schema::games::id
                .eq(game)
                .and(schema::games::player2_id.is_null()),
        ),
    )
    .set(schema::games::player2_id.eq(user.id))
    .execute(&*db_conn)
    .unwrap();
}

#[get("/games/<game>/rounds/<round>")]
fn get_round(game: i32, round: i16, db_conn: GameDbConn) -> Result<Json<Round>, Status> {
    if (0..=4).contains(&round) {
        // TODO: not hard code this
        // TODO: maybe use sql for this
        Ok(Json(
            schema::rounds::table
                .filter(
                    schema::rounds::game_id
                        .eq(game)
                        .and(schema::rounds::round_count.eq(round)),
                )
                .select((
                    schema::rounds::game_id,
                    schema::rounds::round_count,
                    schema::rounds::player1_throws,
                    schema::rounds::player2_throws,
                ))
                .first(&*db_conn)
                .unwrap(),
        ))
    } else {
        Err(Status::BadRequest) // TODO: is this a BadRequest or a NotFound error?
    }
}

#[post("/users/<user>", data = "<password>")] // TODO: hash paswsword, maybe use a guard to hash it. This can be done in SQL or in Rust
fn register_user(
    user: String,
    password: LenientForm<models::Password>,
    conn: GameDbConn,
) -> Result<Status, Status> {
    diesel::insert_into(schema::users::table)
        .values((
            schema::users::username.eq(&user),
            schema::users::password.eq(&password.password),
        ))
        .execute(&*conn)
        .unwrap();
    Ok(Status::Created)
}

#[delete("/users/<user>", data = "<password>")] // TODO: maybe use the AuthenticatedUser request guard
fn delete_user(user: String, password: LenientForm<models::Password>, conn: GameDbConn) {
    diesel::delete(
        schema::users::table.filter(
            schema::users::username
                .eq(user)
                // TODO: it might be possible to omit the end
                .and(schema::users::username.eq(&password.password)), // TODO: double check this, it should probably be `schema::users::password`
        ),
    )
    .execute(&*conn)
    .unwrap();
}

#[get("/users?<n>")] // TODO: maybe n can be optional
fn get_users(n: i64, conn: GameDbConn) -> Json<std::vec::Vec<models::LeaderboardEntry>> {
    Json(
        schema::users::table
            .order(schema::users::score.desc())
            .select((schema::users::username, schema::users::score))
            .limit(n)
            .load(&*conn)
            .unwrap(),
    )
}

#[get("/users/<username>")]
fn user_exists(username: String, conn: GameDbConn) -> Json<bool> {
    Json(
        schema::users::table
            .filter(schema::users::username.eq(username))
            .count()
            .get_result::<i64>(&*conn)
            .unwrap()
            >= 1,
    )
}

#[get("/auth")]
fn authenticated_user(_credentials: AuthenticatedUser) -> Status {
    Status::Accepted
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                register_user,
                delete_user,
                get_users,
                user_exists,
                authenticated_user,
                create_game,
                get_games,
                get_round,
            ],
        )
        .attach(GameDbConn::fairing())
        .attach(rocket::fairing::AdHoc::on_attach(
            "Database Migrations",
            |rocket| match embedded_migrations::run(&*GameDbConn::get_one(&rocket).unwrap()) {
                Ok(_) => Ok(rocket),
                Err(_) => Err(rocket),
            },
        ))
        .launch();
}
