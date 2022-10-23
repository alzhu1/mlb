mod client;
mod io;
mod player;
mod requests;

pub fn create_client() -> client::MlbClient<'static> { client::MlbClient::new() }