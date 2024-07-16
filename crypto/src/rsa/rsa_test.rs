use rand::rngs::OsRng;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::pkcs8::EncodePrivateKey;

#[test]
fn test_rsa_1() {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    println!("priv_key {:?}", priv_key);
    let pub_key = RsaPublicKey::from(&priv_key);
    println!("pub_key {:?}", pub_key);

    // Encrypt
    let data: &[u8] = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    println!("enc_data {:?}", enc_data.clone());
    // println!("enc_data {:?}", String::from_utf8(enc_data.clone()).unwrap());
    println!("enc_data {:?}", enc_data.iter().map(|&x| format!("{:02x}", x)).collect::<String>());
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    println!("dec_data {:?}", dec_data.clone());
    println!("dec_data {:?}", String::from_utf8(dec_data.clone()).unwrap());
    assert_eq!(&data[..], &dec_data[..]);
}

#[test]
fn test_rsa_pkcs_1() {
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

    // DER encoding
    let pkcs1_der = priv_key.to_pkcs1_der().expect("failed to encode to PKCS#1 DER");
    let pkcs8_der = priv_key.to_pkcs8_der().expect("failed to encode to PKCS#8 DER");

    // PEM encoding
    let pkcs1_pem = priv_key.to_pkcs1_pem(LineEnding::LF).expect("failed to encode to PKCS#1 PEM");
    let pkcs8_pem = priv_key.to_pkcs8_pem(LineEnding::LF).expect("failed to encode to PKCS#8 PEM");

    // Print encoded keys
    println!("PKCS#1 DER: {:?}", pkcs1_der);
    println!("PKCS#8 DER: {:?}", pkcs8_der);
    println!("PKCS#1 PEM: {:?}", pkcs1_pem);
    println!("PKCS#8 PEM: {:?}", pkcs8_pem);
}