#[allow(unused_imports)]

mod given_param_values {
    use std::iter::FromIterator;

    use spectral::prelude::*;
    use client::params::{ParamValue, UrlQueryParams};
    use client::params::test_helpers::ParamValueHelper;

    mod describe_spectral_methods {
        use super::*;

        #[test]
        fn have_multiple_values_ok() {
            let ref multi_param = ParamValue::from_iter(vec!["a", "b"]);

            assert_that(multi_param)
                .have_multiple_values()
                .has_length(2);
        }

        #[test]
        fn have_a_single_value_ok() {
            let ref single_param: ParamValue = "a".into();

            assert_that(single_param)
                .have_a_single_value()
                .contains("a")
        }
    }

    mod describe_from_iter {
        use super::*;

        mod vec_of_into_string {
            use super::*;

            fn subject() -> Vec<(&'static str, &'static str)> {
                vec![
                    ("scope", "profile.email"),
                    ("scope", "profile.full_name"),
                    ("scope", "filesystem.read"),
                ]
            }

            #[test]
            fn reads_from_iterator() {
                assert_that(&*UrlQueryParams::from_iter(subject()))
                    .contains_key(&"scope".to_string())
                    .have_multiple_values()
                    .has_length(3);
            }
        }
    }
}
