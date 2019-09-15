use crate::schema::users;
use serde::Serialize;

use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

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

impl UserCredentials {
    pub fn authenticated(&self, connection: &diesel::PgConnection) -> bool {
        users::table
            .filter(
                users::username
                    .eq(&self.username)
                    .and(users::password_crypt.eq(&self.password_crypt)),
            )
            .count()
            .get_result::<i64>(connection)
            .unwrap()
            >= 1
    }

    pub fn get_id(&self, connection: &diesel::PgConnection) -> i32 {
        users::table
            .select(users::id)
            .filter(
                users::username
                    .eq(&self.username)
                    .and(users::password_crypt.eq(&self.password_crypt)),
            )
            .get_result::<i32>(connection)
            .unwrap()
    }
}

#[derive(FromForm)]
pub struct Password {
    pub password_crypt: String,
}
