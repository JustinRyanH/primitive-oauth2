use client::mock_server::{MockServer, ServerResp};
use client::{TokenResponse, mock_client::{MockReq, MockResp}};

pub fn token(server: &MockServer, _: MockReq) -> ServerResp {
    if let Some(ref err) = server.error {
        return ServerResp::response_err(err);
    };

    MockResp::parse_access_token_response(&TokenResponse::new("2YotnFZFEjr1zCsicMWpAA", "bearer"))
        .into()
}
