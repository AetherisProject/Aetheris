use x25519_dalek::{EphemeralSecret, PublicKey, ReusableSecret, SharedSecret};
use rand::rngs::OsRng;
use zeroize::Zeroize;

fn main() {
    // Test X25519 static keypair generation
    let mut csprng = OsRng;
    let reusable_secret = ReusableSecret::random(&mut csprng);
    let public_key = PublicKey::from(&reusable_secret);
    println!("Static keypair generated: {:?}", public_key.to_bytes());

    // Test X25519 ephemeral keypair generation
    let ephemeral_secret = EphemeralSecret::random(&mut csprng);
    let ephemeral_public_key = PublicKey::from(&ephemeral_secret);
    println!("Ephemeral keypair generated: {:?}", ephemeral_public_key.to_bytes());

    // Test key exchange
    let shared_secret = reusable_secret.diffie_hellman(&ephemeral_public_key);
    println!("Shared secret: {:?}", shared_secret.to_bytes());
}