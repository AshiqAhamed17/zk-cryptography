use k256::ecdsa::SigningKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand_core::OsRng;

fn main() {
    let signing_key = SigningKey::random(&mut OsRng);
    let verify_key = signing_key.verifying_key();

    let encoded_point = verify_key.to_encoded_point(false);
    let x = encoded_point.x().unwrap();
    let y = encoded_point.y().unwrap();

    println!("Private key: {:?}", signing_key.to_bytes());
    println!("Public key X: {:?}", x);
    println!("Public key Y: {:?}", y);
}
