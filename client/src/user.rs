use serde::Deserialize;
use sha3::{Digest, Sha3_512};

pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LeaderboardEntry {
    pub username: String,
    pub score: u64,
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn registering() {
        let user = User {
            username: String::from("rodolphe"),
            password: String::from("verysecurepassword"),
        };
        user.register();
    }

    #[test]
    fn user_doesnt_exist() {
        let user = User {
            username: String::from("somebody"),
            username: String::from("uselesspassword"),
        };
        assert!(user.register().is_ok());
        // TODO please make sure there are no side effects of user exists
    }

    #[test]
    fn user_already_exists() {
        let user = User {
            username: String::from("somebody"),
            username: String::from("uselesspassword"),
        };
        assert!(user.register().is_ok());
        assert!(user.register().is_err());
    }
}

impl User {
    pub fn new() -> User {
        User {
            username: String::new(),
            password: String::new(),
        }
    }
    pub fn register(&self) -> Result<(), &str> {
        let mut hasher = Sha3_512::new();
        hasher.input(&self.password);
        match reqwest::Client::new()
            .post(&format!("http://localhost:8000/users/{}", &self.username)) // TODO: make the address a constant
            .form(&[("password_crypt", &hex::encode(&hasher.result()))])
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

    pub fn create_game(&self) -> i32 {
        let mut hasher = Sha3_512::new();
        hasher.input(&self.password);
        reqwest::Client::new()
            .post("http://localhost:8000/games")
            .form(&[
                ("username", &self.username),
                ("password_crypt", &hex::encode(hasher.result())),
            ])
            .send()
            .unwrap()
            .json()
            .unwrap()
    }

    pub fn authenticate(&self) -> bool {
        let mut hasher = Sha3_512::new();
        hasher.input(&self.password);
        reqwest::get(
            format!(
                "http://localhost:8000/users/{}?password={}",
                &self.username,
                &hex::encode(&hasher.result())
            )
            .as_str(),
        )
        .unwrap()
        .json()
        .unwrap()
    }

    pub fn user_exists(username: &str) -> Result<bool, reqwest::Error> {
        reqwest::get(&format!("http://localhost:8000/users/{}", username))?.json()
    }

    pub fn leaderboard(n: usize) -> Result<Vec<LeaderboardEntry>, reqwest::Error> {
        reqwest::get(&format!("http://localhost:8000/users?n={}", n))?.json()
    }
}
