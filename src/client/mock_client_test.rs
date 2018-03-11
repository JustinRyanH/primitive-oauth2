use std::collections::HashMap;

use futures::Future;
use futures_cpupool::CpuPool;
use rspec::{self, given};
use spectral::prelude::*;

use client::mock_client::*;
use client::OauthClient;

#[test]
fn mock_client() {
    let pool = CpuPool::new(4);

    rspec::run(&given("A Mock Client", MockClient::new().unwrap(), |ctx| {
        ctx.context("Generating Auth Request for User", |ctx| {
            ctx.it(
                "Generics Mock Oauth2 Authorization Request",
                move |client| {
                    let result_params =
                        pool.spawn(client.get_user_auth_request().and_then(|req| {
                            let pairs: Vec<(String, String)> = req.url
                                .query_pairs()
                                .into_iter()
                                .map(|(k, v)| (String::from(k), String::from(v)))
                                .collect();
                            Ok(pairs)
                        })).wait()
                            .unwrap();
                    // Params from [RFC](https://tools.ietf.org/html/rfc6749#section-4.1.1)
                    // assert_that(&result_params).contains_key("client_id".to_string()).is_equal_to("someid@example.com".to_string());
                    // assert_that(&result_params).contains_key("redirect_uri".to_string()).is_equal_to("https://localhost/auth".to_string());
                    // assert_that(&result_params).contains_key("scope".to_string()).is_equal_to("api.example.com/user.profile api.example.com/user.me".to_string());
                    // assert_that(&result_params).contains_key("response_type".to_string()).is_equal_to("code".to_string());
                },
            )
        });
    }));
}
