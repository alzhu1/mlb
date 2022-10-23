use std::io::BufRead;

mod client;
mod io;
mod player;
mod requests;

pub fn create_client<R>(reader: R) -> client::MlbClient<'static, R> where R: BufRead { client::MlbClient::new(reader) }