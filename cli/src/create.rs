use crate::utils::logger::possible_write_error;
use openssl::rsa::Rsa;
use std::io::Write;
use std::net::TcpStream;

pub fn handle(mut stream: TcpStream) {
    let pair_res = Rsa::generate(4096);
    if pair_res.is_err() {
        possible_write_error(stream.write(b"error generating private-public key pair"));
        return;
    }
    let pair = pair_res.unwrap();

    let pem_private_res = pair.private_key_to_pem();
    if pem_private_res.is_err() {
        possible_write_error(stream.write(b"error generating private key"));
        return;
    }
    let pem_private = pem_private_res.unwrap();

    let pem_public_res = pair.public_key_to_pem();
    if pem_public_res.is_err() {
        possible_write_error(stream.write(b"error generating private key"));
        return;
    }
    let pem_public = pem_public_res.unwrap();

    possible_write_error(
        stream.write(
            &vec![
                "==private:".bytes().collect(),
                pem_private,
                "== public:".bytes().collect(),
                pem_public,
            ]
            .into_iter()
            .flat_map(|v| v.into_iter().chain(['\n' as u8]))
            .collect::<Vec<u8>>(),
        ),
    );
}
