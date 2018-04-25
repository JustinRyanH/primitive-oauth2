extern crate dotenv;
extern crate envy;
extern crate primitive_oauth2;

extern crate serde;

use dotenv::dotenv;
use primitive_oauth2::client::mock_client::MockClient;
use primitive_oauth2::client::OauthClient;

fn main() {
    dotenv().unwrap();
    let client = envy::prefixed("GOOGLE_").from_env::<MockClient>().unwrap();
    // println!("Client: {:?}", client.get_user_auth_request());
}
