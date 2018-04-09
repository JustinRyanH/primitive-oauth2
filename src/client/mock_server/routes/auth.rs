use url::Url;

use client::mock_client::MockReq;
use client::mock_server::{MockServer, ServerResp, mock_server::single_param};
use errors::Result;

pub fn parse_state(url: &Url) -> Result<String> {
    single_param("state", url)
}

pub fn auth_response(server: &MockServer, req: MockReq) -> ServerResp {
    if let Some(ref err) = server.error {
        return ServerResp::redirect_err(err);
    }
    ServerResp::Redirect(auth(server, &req))
}

#[inline]
pub fn auth(_: &MockServer, req: &MockReq) -> Result<MockReq> {
    let state = parse_state(&req.url)?;

    Ok(MockReq {
        url: Url::parse_with_params(
            "https://localhost/example/auth",
            vec![("state", state), ("code", "MOCK_CODE".into())],
        ).unwrap(),
        body: String::from(""),
    }).into()
}
