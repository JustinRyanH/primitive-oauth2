use futures::future::FutureResult;

pub trait OauthStorage: Sized {
    type Error;
    type Lookup;
    type Value;

    fn set(&mut self, lookup: Self::Lookup, value: Self::Value) -> FutureResult<Self, Self::Error>;
    fn get(&self, lookup: Self::Lookup) -> FutureResult<Self::Value, Self::Error>;
    fn drop(&mut self, lookup: Self::Lookup) -> FutureResult<(Self, Self::Value), Self::Error>;
}
