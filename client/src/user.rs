use serde::Deserialize;

pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LeaderboardEntry {
    pub username: String,
    pub score: u64,
}

#[derive(Deserialize)]
pub struct LobbyEntry {
    pub game_id: i32,
    pub player1: String,
}

impl User {
    pub fn new() -> User {
        User {
            username: String::new(),
            password: String::new(),
        }
    }
}

pub struct ServerConnection {
    client: reqwest::Client,
    pub server_addr: String,
}

impl ServerConnection {
    pub fn new(server_addr: &str) -> ServerConnection {
        ServerConnection {
            client: reqwest::Client::new(),
            server_addr: format!("http://{}", server_addr),
        }
    }

    pub fn register_user(&self, user: &User) -> Result<(), &str> {
        match self
            .client
            .post(&format!("{}/users/{}", self.server_addr, user.username)) // TODO: make the address a constant
            .form(&[("password", &user.password)])
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    Err("This user already exists")
                }
            }
            Err(_) => Err("Couldn't connect to the internet, check your internet"),
        }
    }

    pub fn user_exists(&self, username: &str) -> Result<bool, reqwest::Error> {
        self.client
            .get(&format!("{}/users/{}", self.server_addr, username))
            .send()?
            .json()
    }

    pub fn leaderboard(&self, n: usize) -> Result<Vec<LeaderboardEntry>, reqwest::Error> {
        self.client
            .get(&format!("{}/users?n={}", self.server_addr, n)) // TODO: use the query method https://docs.rs/reqwest/0.9.22/reqwest/struct.RequestBuilder.html#method.query
            .send()?
            .json()
    }

    pub fn authenticate(&self, user: &User) -> bool {
        use reqwest::StatusCode;
        match self
            .client
            .get(&format!("{}/auth", self.server_addr))
            .basic_auth(&user.username, Some(&user.password))
            .send()
            .unwrap()
            .status()
        {
            StatusCode::ACCEPTED => true,
            StatusCode::UNAUTHORIZED => false,
            _ => unimplemented!(),
        }
    }

    pub fn create_game(&self, user: &User) -> i32 {
        self.client
            .post(&format!("{}/games", self.server_addr))
            .basic_auth(&user.username, Some(&user.password))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn get_games(&self) -> Vec<LobbyEntry> {
        self.client
            .get(&format!("{}/games?n={}", self.server_addr, 5))
            .send()
            .unwrap()
            .json()
            .unwrap()
    }
}
