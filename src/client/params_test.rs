use rspec::{self, given};
use spectral::prelude::*;

use std::iter::FromIterator;

use client::params::{UrlQueryParams, ParamValue};
use client::params::test_helpers::ParamValueHelper;

#[test]
fn spectral_param_value_have_multiple_values() {
    let ref multi_param = ParamValue::from_iter(vec!["a", "b"]);

    assert_that(multi_param)
        .have_multiple_values()
        .has_length(2);
}

#[test]
fn spectral_param_value_have_a_single_value() {
    let ref single_param: ParamValue = "a".into();

    assert_that(single_param)
        .have_a_single_value()
        .contains("a")
}

#[test]
fn mock_client() {
    let params: Vec<(&str, &str)> = vec![];
    rspec::run(&given(
        "Parameters as a Vector of String Tuple",
        params,
        |ctx| {
            ctx.when("there are multiple values of the same key", |ctx| {
                ctx.before_each(|env| {
                    *env = vec![
                        ("scope", "profile.email"),
                        ("scope", "profile.full_name"),
                        ("scope", "filesystem.read"),
                    ];
                });
                ctx.it("then places them in a Vector of Strings", |env| {
                    let multi = UrlQueryParams::from_iter(env.clone());
                    assert_that(&*multi)
                        .contains_key(&"scope".to_string())
                        .have_multiple_values()
                        .has_length(3);
                })
            });
        },
    ));
}