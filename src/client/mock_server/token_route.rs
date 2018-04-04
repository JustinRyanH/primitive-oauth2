use client::mock_server::{MockServer, ServerResp};
use client::{TokenResponse, mock_client::{MockReq, MockResp}};

pub fn token(_: &MockServer, _: MockReq) -> ServerResp {
    MockResp::parse_access_token_response(&TokenResponse::new("2YotnFZFEjr1zCsicMWpAA", "bearer"))
        .into()
}
