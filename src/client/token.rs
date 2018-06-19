#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    access_type: String,
    token_type: String,
    expires_in: Option<usize>,
    refresh_token: Option<String>,
    scope: Vec<String>,
}
