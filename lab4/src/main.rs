mod client;
mod client_authenticator;
mod security_base;
mod server;
mod server_authenticator;

use primes::Range;

use crate::{client::Client, security_base::SecurityBase, server::Server};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(50);

    let security_base = SecurityBase::new(&range, &mut rng);

    // Server side
    let mut server = Server::new(&security_base);
    server.register("hello", "world", &mut rng);

    // Client side
    let mut client = Client::new("hello", &security_base);
    let client_authenticator = client.authenticate("world", &mut rng);

    // Server side
    let server_authenticator = server.authenticate(&client.username, &mut rng).unwrap();
}
