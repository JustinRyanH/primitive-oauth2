#[allow(unused_imports)]
use futures::Future;
use spectral::prelude::*;

use client::authenticator::BaseAuthenticator;
use client::mock_client::MockClient;
use client::storage::MemoryStorage;
#[allow(unused_imports)]
use client::ClientStorage;

#[inline]
fn base_auth() -> BaseAuthenticator {
    BaseAuthenticator::new(
        "someid@example.com",
        "test",
        "http://example.com/auth",
        "http://example.com/token",
    ).unwrap()
}

#[inline]
fn expected_redirect() -> &'static str {
    "https://localhost:8080/oauth/example"
}

#[inline]
fn mock_client() -> MockClient {
    MockClient::new(base_auth(), expected_redirect()).unwrap()
}

mod given_a_memory_storage {
    use super::*;

    type Subject = MemoryStorage;

    mod when_store_is_empty {
        use super::*;
        fn subject() -> Subject {
            let storage = MemoryStorage::new();
            assert_that(&*storage.read().unwrap()).is_empty();
            storage
        }

        #[test]
        fn client_storage_set() {
            let mut subject = subject();
            let client = mock_client();
            subject.set("foo", client).unwrap();
            assert_that(&*subject.read().unwrap()).contains_key(&String::from("foo"));
        }
    }

    mod when_storage_has_data {
        use super::*;
        use std::ops::Deref;

        fn subject() -> Subject {
            let storage = MemoryStorage::new();
            storage
                .deref()
                .write()
                .unwrap()
                .insert(String::from("EXAMPLE_STATE"), mock_client());
            assert_that(&*storage.read().unwrap()).contains_key(String::from("EXAMPLE_STATE"));
            storage
        }

        #[test]
        fn client_storage_get() {
            let subject = subject();
            assert_that(&subject.get("EXAMPLE_STATE")).is_ok();
        }

        #[test]
        fn client_storage_drop() {
            let mut subject = subject();
            assert_that(&subject.drop("EXAMPLE_STATE")).is_ok();
        }
    }

}
