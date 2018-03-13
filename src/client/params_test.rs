use rspec::{self, given};
use spectral::prelude::*;

use client::params::{params_into_hash, ParamValue};
use client::params::test_helpers::ParamValueHelper;

#[test]
fn spectral_param_value_have_multiple_values() {
    let ref multi_param = ParamValue::Multi(vec!["a".to_string(), "b".to_string()]);

    assert_that(multi_param)
        .have_multiple_values()
        .has_length(2);
}

#[test]
fn spectral_param_value_have_a_single_value() {
    let ref single_param = ParamValue::Single("a".to_string());

    assert_that(single_param)
        .have_a_single_value()
        .contains("a")
}

#[test]
fn mock_client() {
    let params: Vec<(String, String)> = vec![];
    rspec::run(&given(
        "Parameters as a Vector of String Tuple",
        params,
        |ctx| {
            ctx.when("there are multiple values of the same key", |ctx| {
                ctx.before_each(|env| {
                    *env = vec![
                        ("scope".to_string(), "profile.email".to_string()),
                        ("scope".to_string(), "profile.full_name".to_string()),
                        ("scope".to_string(), "filesystem.read".to_string()),
                    ];
                });
                ctx.it("then places them in a Vector of Strings", |env| {
                    let multi = params_into_hash(env);
                    assert_that(&multi)
                        .contains_key(&"scope".to_string())
                        .have_multiple_values()
                        .has_length(3);
                })
            });
        },
    ));
}
