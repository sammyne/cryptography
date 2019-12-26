mod hash;

pub mod aes;
pub mod cipher;
pub mod ed25519;
pub mod md5;
pub mod rand;
pub mod ripemd160;
pub mod secp256k1;
pub mod sha256;
pub mod sha512;

pub use hash::*;

pub trait PrivateKey {
    type PublicKey;

    fn public(&self) -> Self::PublicKey;
}
