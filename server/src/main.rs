#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod models;
mod schema;

use schema::games;
use schema::rounds;

use rocket::http::Status;
use rocket::request::LenientForm;
use rocket_contrib::json::Json;

use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use rand::Rng;

// sql_function!(fn crypt(password: Text, salt: Text) -> Text);
// sql_function!(fn gen_salt(hash_algorithm_type: Text) -> Text);

#[database("game")]
struct GameDbConn(diesel::PgConnection);

#[derive(Insertable, Queryable)]
#[table_name = "rounds"]
struct Round {
    game_id: i32,
    round_count: i16,
    player1_throws: Vec<i16>,
    player2_throws: Vec<i16>,
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

#[post("/games", data = "<credentials>")]
fn create_game(
    credentials: LenientForm<models::UserCredentials>,
    connection: GameDbConn,
) -> Json<i32> {
    assert!(credentials.authenticated(&connection));
    let mut rng = rand::thread_rng();
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
        if player2_score % 2 == 0 {
            player1_score += 10;
        } else {
            player1_score = if player1_score - 5 >= 0 {
                player1_score - 5
            } else {
                0
            };
        }

        if player2_score % 2 == 0 {
            player2_score += 10;
        } else {
            player2_score = if player2_score - 5 >= 0 {
                player2_score - 5
            } else {
                0
            };
        }

        player1_score += round.player1_throws.get(2).unwrap_or(&0);
        player2_score += round.player2_throws.get(2).unwrap_or(&0);
    }

    while player1_score == player2_score {
        player1_extra.push(rng.gen_range(1, 7));
        player2_extra.push(rng.gen_range(1, 7));
        player1_score += player1_extra.last().unwrap();
        player2_score += player2_extra.last().unwrap();
    }

    let lobby = LobbyGame {
        player1_id: credentials.get_id(&connection),
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

#[post("/users/<user>", data = "<password>")]
fn register_user(
    user: String,
    password: LenientForm<models::Password>,
    conn: GameDbConn,
) -> Result<Status, Status> {
    diesel::insert_into(schema::users::table)
        .values((
            schema::users::username.eq(&user),
            schema::users::password_crypt.eq(&password.password_crypt),
        ))
        .execute(&*conn)
        .unwrap();
    Ok(Status::Created)
}

#[delete("/users/<user>", data = "<password>")]
fn delete_user(user: String, password: LenientForm<models::Password>, conn: GameDbConn) {
    diesel::delete(
        schema::users::table.filter(
            schema::users::username
                .eq(user)
                .and(schema::users::username.eq(&password.password_crypt)),
        ),
    )
    .execute(&*conn)
    .unwrap();
}

#[get("/users?<n>")]
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

#[get("/users/<username>?<password>")]
fn authenticated_user(username: String, password: String, connection: GameDbConn) -> Json<bool> {
    Json(
        models::UserCredentials {
            username: username,
            password_crypt: password,
        }
        .authenticated(&connection),
    )
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                register_user,
                delete_user,
                get_user,
                user_exists,
                authenticated_user,
                create_game
            ],
        )
        .attach(GameDbConn::fairing())
        .launch();
}
