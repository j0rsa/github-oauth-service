use std::env;
use std::str::FromStr;

pub fn env_iss() -> String {
    env::var("JWT_ISS").unwrap_or("".to_string())
}

pub fn env_aud() -> String {
    env::var("JWT_AUD").unwrap_or("".to_string())
}

pub fn env_exp_days() -> i64 {
    return match i64::from_str(
        env::var("JWT_EXP_DAYS").unwrap_or("30".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

pub fn env_nbf_days() -> i64 {
    return match i64::from_str(
        env::var("JWT_NBF_DAYS").unwrap_or("0".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

pub fn env_token_secret() -> String {
    env::var("JWT_SECRET").expect("No token secret found!")
}

pub fn env_leeway() -> i64 {
    return match i64::from_str(
        env::var("JWT_LEEWAY_SEC").unwrap_or("0".to_string()).as_ref()
    ) {
        Ok(v) => v,
        Err(e) => panic!(e)
    };
}

pub fn gh_scope() -> String {
    env::var("GH_SCOPE").unwrap_or("user:read,user:email".to_string())
}

pub fn gh_client_id() -> String {
    env::var("GH_CLIENT_ID").expect("Github OAuth App client id is required!")
}

pub fn gh_client_secret() -> String {
    env::var("GH_CLIENT_SECRET").expect("Github OAuth App client secret is required!")
}

pub fn gh_code_redirect() -> String {
    env::var("GH_CODE_REDIRECT").expect("Github redirect page is required!")
}
