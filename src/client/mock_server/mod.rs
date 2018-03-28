pub mod mock_server;
pub mod server_resp;
#[cfg(test)]
mod spec;

pub use self::server_resp::ServerResp;
pub use self::mock_server::*;

pub const VALID_SCOPES: [&'static str; 2] =
    ["api.example.com/user.profile", "api.example.com/add_item"];
