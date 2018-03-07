extern crate futures;
extern crate url;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authenticator {
    client_id: String,
    client_secret: String,
}

impl Default for Authenticator {
    fn default() -> Self {
        Authenticator {
            client_id: Default::default(),
            client_secret: Default::default(),
        }
    }
}

impl Authenticator {
    pub fn get_client_id(&self) -> &str {
        self.client_id.as_ref()
    } 

    pub fn get_client_secret(&self) -> &str {
        self.client_secret.as_ref()
    }
}
