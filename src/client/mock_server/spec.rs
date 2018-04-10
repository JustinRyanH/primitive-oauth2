use spectral::prelude::*;
use url::Url;

use client::MockReq;
use client::MockResp;
use client::mock_server::*;

fn server() -> MockServer {
    MockServer::new()
}

mod no_route {
    use super::*;

    fn request() -> MockReq {
        MockReq {
            url: Url::parse("https://example.net/").unwrap(),
            body: "".to_string(),
        }
    }

    #[test]
    fn it_returns_404_response() {
        let expected_resp: MockResp = "{\
                                       \"error\":\"server_error\",\
                                       \"error_description\":\"404: Route not found\"\
                                       }"
            .into();
        assert_that(&server().send_request(request()).response())
            .is_ok()
            .is_equal_to(expected_resp);
    }
}
