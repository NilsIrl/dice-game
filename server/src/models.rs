use serde::Serialize;

use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};

use rocket::request::FromRequest;

#[derive(Serialize, Queryable)]
pub struct LeaderboardEntry {
    pub username: String,
    pub score: i32,
}

// TODO: It could be possible to use a type alias since we are just implementing a trait
pub struct AuthenticatedUser {
    pub id: i32,
}

use rocket::request::Outcome;
impl FromRequest<'_, '_> for AuthenticatedUser {
    type Error = ();
    fn from_request(request: &rocket::request::Request) -> Outcome<Self, ()> {
        use crate::schema;
        let authorization_header = request.headers().get_one("Authorization").unwrap();
        let mut header_iter = authorization_header.splitn(2, ' ');
        match header_iter.next() {
            Some("Basic") => {
                let valid_path = &base64::decode(header_iter.next().unwrap()).unwrap();
                let (username, password) = {
                    let mut iter = std::str::from_utf8(valid_path).unwrap().splitn(2, ':');
                    (iter.next().unwrap(), iter.next().unwrap())
                };
                match schema::users::table
                    .filter(
                        schema::users::username
                            .eq(username)
                            .and(schema::users::password.eq(password)),
                    )
                    .select(schema::users::id)
                    .get_result::<i32>(&*request.guard::<crate::GameDbConn>().unwrap())
                {
                    Ok(id) => Outcome::Success(AuthenticatedUser { id }),
                    // TODO more data should be sent back including a "WWW-Authenticate" header as defined in RFC7235 and RFC7617
                    Err(_) => Outcome::Failure((rocket::http::Status::Unauthorized, ())),
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(FromForm)]
pub struct Password {
    pub password: String,
}
