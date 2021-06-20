use reqwest::header::AUTHORIZATION;
use reqwest::header::HeaderMap;

pub fn get_bearer(secret: &str) -> String {
    format!("Bearer {}", secret)
}

pub fn get_secret() -> String {
    std::env::var("SECRET").expect("Environment variable not found.")
}

pub async fn set_get_headers(bearer_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer_token.parse().unwrap());
    headers.insert("Notion-Version", "2021-05-13".parse().unwrap());

    headers
}

pub async fn set_headers(bearer_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, bearer_token.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Notion-Version", "2021-05-13".parse().unwrap());

    headers
}
