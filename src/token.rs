use std::time::{Duration, SystemTime};
use serde_json::Error;
use std::process::exit;
use serde_json;
use reqwest;
/// An OAuth token. Will usually expire after an hour.
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    /// How many seconds before this token expires
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
    pub access_token: String,
    /// The uid of the user this token corresponds to
    pub user_id: String,
    /// The token that refresh uses
    pub refresh_token: String,
    pub session_id: String,
    #[serde(default = "cur_date")]
    pub updated_at: u64,
}
fn cur_date() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("System time is before Unix Epoch")
        .as_secs()
}
impl Token {
    /// Creates a token from a response from /token
    pub fn from_response(response: &str) -> Result<Token, Error> {
        serde_json::from_str(response)
    }
    /// Fetches a token using a login code
    pub fn from_login_code(code: &str) -> Result<Token, Error> {
        let res = reqwest::get(&("https://auth.gog.com/token?client_id=46899977096215655&client_secret=9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9&grant_type=authorization_code&code=".to_string()+&code+"&redirect_uri=https://embed.gog.com/on_login_success?origin=client"));
        if res.is_ok() {
            Token::from_response(&res.unwrap().text().unwrap())
        } else {
            res.expect("Couldn't get token");
            exit(0);
        }
    }
    /// Checks if token has expired
    pub fn is_expired(&self) -> bool {
        self.updated_at + self.expires_in - cur_date() <= 0
    }
    /// Attempts to fetch an updated token
    pub fn refresh(&self) -> Result<Token, Error> {
        let res = reqwest::get(&("https://auth.gog.com/token?client_id=46899977096215655&client_secret=9d85c43b1482497dbbce61f6e4aa173a433796eeae2ca8c5f6129f2dc4de46d9&grant_type=refresh_token&redirect_uri=https://embed.gog.com/on_login_success?origin=client&refresh_token=".to_string()+&self.refresh_token));
        if res.is_ok() {
            let try = serde_json::from_str(&res.unwrap().text().unwrap());
            return try;
        } else {
            res.expect("Couldn't refresh token");
            exit(0);
        }
    }
}
