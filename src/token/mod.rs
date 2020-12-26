use actix_web::{HttpRequest, HttpResponse, web, http};

use models::*;
use reqwest::*;
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
                _ => return HttpResponse::InternalServerError().body("Unable to parse access token response")
            }
        },
        _ => return HttpResponse::BadRequest().body("Unable to get the access token text")
    };
    debug!("Received an access token {:?}", token_response);
    let user = match user_info(&token_response).await {
        Ok(value) => value,
        Err(e) => return HttpResponse::BadRequest().body(format!("Unable to get user information {}", e))
    };
    let token = internal::generate_token(user.id.to_string(),user.login, token_response.access_token);
    HttpResponse::Ok().json(NewTokenResponse { token })
}

async fn user_info(token: &GhTokenResponse) -> Result<User> {
    reqwest::Client::new()
        .get("https://api.github.com/user")
        .header(http::header::AUTHORIZATION, format!("Bearer {}", token.access_token))
        .header(http::header::USER_AGENT, "gh_auth_module")
        .send()
        .await?
        .json()
        .await
}

pub async fn refresh(request: web::Json<RefreshTokenRequest>) -> HttpResponse {
    match internal::refresh_token(&request.token) {
        Ok(token) => {
            let new_token = NewTokenResponse { token };
            HttpResponse::Ok().json(new_token)
        }
        _ => HttpResponse::Unauthorized().body("")
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
                    .header("X-Github-Token", claims.github_token)
                    .body("")}
                Err(e) => HttpResponse::Unauthorized().body(format!("Token is invalid: {}", e.to_string()))
            }
        }
        _ => HttpResponse::Unauthorized().body("No Authorization Bearer Header")
    }
}
