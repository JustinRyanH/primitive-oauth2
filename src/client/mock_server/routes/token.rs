use errors::Result;

use client::mock_server::{MockServer, ServerResp};
use client::{TokenResponse, mock_client::{MockReq, MockResp}};

pub static MOCK_TOKEN: &'static str = "TU9DS19UT0tFTg==";

#[inline]
pub fn token_response(server: &MockServer, req: MockReq) -> ServerResp {
    if let Some(ref err) = server.error {
        return ServerResp::response_err(err);
    };
    ServerResp::Response(token(server, &req))
}

#[inline]
pub fn token(server: &MockServer, _: &MockReq) -> Result<MockResp> {
    let mut token_resp = TokenResponse::new(MOCK_TOKEN, "bearer");

    if let Some(expiration) = server.token_ops.expiration {
        token_resp = token_resp.with_expiration(expiration);
    }

    if !server.token_ops.scope.is_empty() {
        token_resp = token_resp.with_scope(&server.token_ops.scope);
    }

    MockResp::parse_access_token_response(&token_resp)
}
