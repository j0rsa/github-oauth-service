use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    // expired after
    pub exp: i64,
    // valid not before
    pub nbf: i64,
    // issued at
    pub iat: i64,
    // jwt id
    pub jti: String,
    pub name: String,
    // comma separated list of scopes
    pub oauth_provider: String,
    pub github_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTokenResponse {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub token: String,
    pub oauth_provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GhTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub avatar_url: String,
}
