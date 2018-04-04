pub mod auth_route;
pub mod mock_server;
pub mod server_resp;
pub mod token_route;

#[cfg(test)]
mod spec;

pub use self::mock_server::*;
pub use self::server_resp::ServerResp;

pub const VALID_SCOPES: [&'static str; 2] =
    ["api.example.com/user.profile", "api.example.com/add_item"];
