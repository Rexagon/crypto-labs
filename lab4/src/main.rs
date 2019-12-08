mod security_base;
mod server;
mod srp6;

use primes::Range;

use crate::{security_base::SecurityBase, server::Server};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Range::new(50);

    let security_base = SecurityBase::new(&range, &mut rng);

    // Server side
    let mut server = Server::new(&security_base);
    server.register("hello", "world", &mut rng);

    // Client side
    let username = String::from("hello");
    let password = String::from("world");

    for _ in 0..10 {
        let (a, client_ephemeral_value) =
            srp6::create_client_ephemeral_value(&security_base, &mut rng);

        println!("a: {}", &a);
        println!("A: {}", &client_ephemeral_value);

        // Server side
        let authentication_data = server.find(&username).expect("User not found");
        let salt = &authentication_data.salt;

        let (b, server_ephemeral_value) =
            srp6::create_server_ephemeral_value(&authentication_data, &security_base, &mut rng);

        println!("b: {}", &b);
        println!("B: {}", &server_ephemeral_value);

        // Client side
        let (client_session_key, client_session_key_hash) = srp6::create_client_session_key(
            &username,
            &password,
            salt,
            (&a, &client_ephemeral_value),
            &server_ephemeral_value,
            &security_base,
        );

        println!("Client session key: {}", client_session_key);

        // Server side
        let (server_session_key, server_session_key_hash) = srp6::create_server_session_key(
            authentication_data,
            &client_ephemeral_value,
            (&b, &server_ephemeral_value),
            &security_base,
        );

        println!("Server session key: {}", server_session_key);

        // Client side
        let client_session_key_proof = srp6::calculate_session_key_proof(
            &username,
            salt,
            &client_ephemeral_value,
            &server_ephemeral_value,
            &client_session_key_hash,
            &security_base,
        );

        // Server side
        let server_session_key_proof = srp6::calculate_session_key_proof(
            &authentication_data.username,
            &authentication_data.salt,
            &client_ephemeral_value,
            &server_ephemeral_value,
            &server_session_key_hash,
            &security_base,
        );

        if server_session_key_proof == client_session_key_proof {
            let server_final_proof = srp6::calculate_final_proof(
                &client_ephemeral_value,
                &server_session_key_proof,
                &server_session_key_hash,
            );

            // Client side
            let client_final_proof = srp6::calculate_final_proof(
                &client_ephemeral_value,
                &client_session_key_proof,
                &server_session_key_hash,
            );

            assert_eq!(server_final_proof, client_final_proof);

            println!("Successfully authorized");
        } else {
            println!("Unauthorized");
        }

        println!("\n");
    }
}
