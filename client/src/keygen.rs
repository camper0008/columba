use openssl::{error::ErrorStack, pkey::Private, rsa::Rsa};

pub enum KeyGenError {
    OpenSSLStack(ErrorStack),
}

pub fn generate_key_pair() -> Result<Rsa<Private>, KeyGenError> {
    let pair_res = match Rsa::generate(4096) {
        Ok(v) => Ok(v),
        Err(e) => Err(KeyGenError::OpenSSLStack(e)),
    };
    pair_res
}
