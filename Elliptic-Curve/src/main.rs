use k256::ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand_core::OsRng;
use hex::encode;
use sha3::{Digest, Keccak256};

fn main() {

    // Generate a random private key
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    // Get the public key in compressed format
    let encoded_point = verify_key.to_encoded_point(false);
    let pubkey_uncompressed = encoded_point.as_bytes();
    let x = encoded_point.x().unwrap();
    let y = encoded_point.y().unwrap();

    // Convert the keys to hex format for readability
    let private_key_hex = hex::encode(signing_key.to_bytes());
    let pub_x_hex = hex::encode(x);
    let pub_y_hex = hex::encode(y);

     // Ethereum address derivation
    let hash = Keccak256::digest(&pubkey_uncompressed[1..]); // Skip the first byte (0x04) for uncompressed pubkey
    let address = &hash[12..]; // Take the last 20 bytes

    // Print the keys in hex format
    println!("Private key: 0x{}", private_key_hex);
    println!("Public key X: 0x{}", pub_x_hex);
    println!("Public key Y: 0x{}", pub_y_hex);
    println!("Address: 0x{}", hex::encode(address));


    // Signing and verifying a message
    let message = b"Hello ZK world!";
    let message_hash = Keccak256::digest(message);

    let signature: Signature = signing_key.sign(&message_hash);
    println!("Signature: 0x{}", encode(signature.to_der()));

    // Verify the signature
    match verify_key.verify(&message_hash, &signature) {
        Ok(_) => println!("Signature verified successfully"),
        Err(_) => println!("Signature verification failed"),
    }
}
