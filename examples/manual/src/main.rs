extern crate dotenv;
extern crate envy;
extern crate primitive_oauth2;

extern crate serde;

use dotenv::dotenv;
use primitive_oauth2::client::{mock_client::MockClient, storage::MemoryStorage, OauthClient};

fn main() {
    dotenv().unwrap();
    let mut storage = MemoryStorage::new();
    let client = envy::prefixed("GOOGLE_").from_env::<MockClient>().unwrap();
    println!("Client: {:?}", client.get_user_auth_request(&mut storage));
}
