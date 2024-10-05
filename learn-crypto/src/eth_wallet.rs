use secp256k1::{PublicKey, Secp256k1, SecretKey, rand::{rngs, SeedableRng}};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = StdRng::from(SeedableRng::from_rng(&mut OsRng).unwrap());
    secp.generate_keypair(&mut rng)
}

#[test]
fn test_generate_keypair() {
    let (secret_key, public_key) = generate_keypair();
    assert_eq!(secret_key.to_bytes().len(), 32);
    println!("public key: {}", &public_key.to_string());
}