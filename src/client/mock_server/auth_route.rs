use url::Url;

use client::mock_client::MockReq;
use client::mock_server::{ServerResp,
                          mock_server::{maybe_single_param, single_param, validate_required_uri},
                          VALID_SCOPES};
use client::params::UrlQueryParams;
use errors::{Error, Result};

use client::mock_server::MockServer;

pub fn parse_state(url: &Url) -> Result<String> {
    single_param("state", url)
}

pub fn parse_client_id(url: &Url) -> Result<String> {
    let client_id = single_param("client_id", url)?;

    if client_id != "someid@example.com" {
        return Err(Error::unauthorized_client(
            Some("Unauthorized: Client Not Authorized"),
            None,
        ));
    }
    Ok(client_id)
}

pub fn parse_redirect_uri(server: &MockServer, url: &Url) -> Result<Option<Url>> {
    match maybe_single_param("redirect_uri", url) {
        Some(v) => Ok(validate_required_uri(v)?),
        None => match server.redirect_uri_required {
            true => {
                return Err(Error::invalid_request(
                    Some("Bad Request: Missing `redirect_uri`"),
                    None,
                ));
            }
            false => Ok(None),
        },
    }
}

pub fn parse_scope(url: &Url) -> Result<()> {
    let scope: Vec<String> = match UrlQueryParams::from(url.query_pairs()).get("scope") {
        Some(v) => v.into_iter().collect(),
        None => vec![],
    };

    for value in scope {
        if !VALID_SCOPES.into_iter().any(|&v| v == value) {
            return Err(Error::invalid_request(
                None,
                Some(format!(
                    "https://docs.example.com/scopes?invalid_scope={}",
                    value
                )),
            ));
        }
    }

    Ok(())
}

pub fn auth(server: &MockServer, req: MockReq) -> ServerResp {
    if let Some(ref err) = server.error {
        return ServerResp::redirect_err(err);
    }
    let state = match parse_state(&req.url) {
        Ok(k) => k,
        Err(e) => return ServerResp::redirect_err(&e),
    };

    match parse_client_id(&req.url) {
        Ok(_) => (),
        Err(e) => return ServerResp::redirect_err(&e),
    };

    match parse_redirect_uri(server, &req.url) {
        Ok(_) => (),
        Err(e) => return ServerResp::redirect_err(&e),
    };

    match parse_scope(&req.url) {
        Ok(_) => (),
        Err(e) => return ServerResp::redirect_err(&e),
    };

    Ok(MockReq {
        url: Url::parse_with_params(
            "https://localhost/example/auth",
            vec![("state", state), ("code", "MOCK_CODE".into())],
        ).unwrap(),
        body: String::from(""),
    }).into()
}
