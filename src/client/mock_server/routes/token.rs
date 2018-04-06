use errors::Result;

use client::mock_server::{MockServer, ServerResp};
use client::{TokenResponse, mock_client::{MockReq, MockResp}};

#[inline]
pub fn token_response(server: &MockServer, req: MockReq) -> ServerResp {
    if let Some(ref err) = server.error {
        return ServerResp::response_err(err);
    };
    ServerResp::Response(token(server, &req))
}

#[inline]
pub fn token(_: &MockServer, _: &MockReq) -> Result<MockResp> {
    MockResp::parse_access_token_response(&TokenResponse::new("2YotnFZFEjr1zCsicMWpAA", "bearer"))
}
