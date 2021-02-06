use actix_web::{HttpRequest, HttpResponse, web, http};

use models::*;
use reqwest::*;
use crate::token::internal::get_claims;
use actix_web::http::header::ToStrError;

mod internal;

pub mod models;
mod conf;

pub async fn redirect_to_login() -> HttpResponse {
    let mut url = Url::parse("https://github.com/login/oauth/authorize").unwrap();
    url.query_pairs_mut()
        .append_pair("client_id", &conf::gh_client_id())
        .append_pair("scope", &conf::gh_scope())
        .append_pair("redirect_url", &conf::gh_code_redirect());

    HttpResponse::MovedPermanently()
        .header(http::header::LOCATION, url.as_str())
        .finish()
}

pub async fn get_token(request: web::Json<TokenRequest>) -> HttpResponse {
    let code = &request.code;
    debug!("Getting the token with the code: {}", code);
    let response = match reqwest::Client::new()
        .post("https://github.com/login/oauth/access_token")
        .query(&vec![
            ("client_id", conf::gh_client_id()),
            ("client_secret", conf::gh_client_secret()),
            ("code", code.clone())
        ])
        .send()
        .await {
        Ok(value) => value,
        _ => return HttpResponse::BadRequest().body("Unable to get the access token")
    };

    let token_response = match response.text().await {
        Ok(text) => {
            match serde_urlencoded::from_str::<GhTokenResponse>(&text) {
                Ok(token_response) => token_response,
                _ => return HttpResponse::InternalServerError().body(format!("Unable to parse access token response: {}", text))
            }
        }
        _ => return HttpResponse::BadRequest().body("Unable to get the access token text")
    };
    debug!("Received an access token {:?}", token_response);
    let user = match user_info(&token_response.access_token).await {
        Ok(value) => value,
        Err(e) => return HttpResponse::BadRequest().body(format!("Unable to get user information {}, token: {}", e, token_response.access_token))
    };
    let token = internal::generate_token((&user.id).to_string(), (&user.login).clone(), token_response.access_token);
    HttpResponse::Ok().json(user_token(&user, &token))
}

fn user_token(user: &User, token: &String) -> NewTokenResponse {
    NewTokenResponse {
        id: user.id.clone(),
        login: user.login.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        avatar_url: Some(user.avatar_url.clone()),
        token: token.clone(),
        oauth_provider: "Github".to_string(),
    }
}

async fn user_info(token: &String) -> Result<User> {
    reqwest::Client::new()
        .get("https://api.github.com/user")
        .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
        .header(http::header::USER_AGENT, "gh_auth_module")
        .send()
        .await?
        .json()
        .await
}

pub async fn refresh(req: HttpRequest) -> HttpResponse {
    let new_token = req.headers().get(http::header::AUTHORIZATION)
        .and_then(|header_value| match header_value.to_str() {
            Ok(v) => Some(v),
            _ => None
        })
        .and_then(|auth| internal::get_bearer_token(auth.to_string()))
        .and_then(|token| match internal::refresh_token(&token) {
            Ok(v) => Some(v),
            _ => None
        });
    match new_token {
        Ok(token) => {
            let claims = get_claims(&token).unwrap();
            match user_info(&claims.oauth_token).await {
                Ok(user) => HttpResponse::Ok().json(user_token(&user, &token)),
                _ => HttpResponse::BadRequest().body("unable to get user info")
            }
        }
        _ => HttpResponse::Unauthorized().body("unable to refresh token")
    }
}

pub async fn check(req: HttpRequest) -> HttpResponse {
    let header = req.headers().get(http::header::AUTHORIZATION);
    match header {
        Some(header) => {
            let authorization_header_value = header.to_str()
                .expect("Authorization has no string value")
                .to_string();
            check_auth_value(authorization_header_value)
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Header")
    }
}

fn check_auth_value(auth: String) -> HttpResponse {
    let token = internal::get_bearer_token(auth);
    match token {
        Some(bearer) => {
            match internal::get_claims(&bearer) {
                Ok(claims) => {
                    HttpResponse::Ok()
                        .header("X-Auth-Id", claims.sub)
                        .header("X-Auth-User", claims.name)
                        .header("X-OAuth-Token", claims.oauth_token)
                        .body("")
                }
                Err(e) => HttpResponse::Unauthorized().body(format!("Token is invalid: {}", e.to_string()))
            }
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Bearer Header")
    }
}
