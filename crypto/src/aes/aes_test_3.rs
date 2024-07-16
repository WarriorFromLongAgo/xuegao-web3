use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, AesGcm};
use aes_gcm::aead::consts::U12;
use aes_gcm::aead::Nonce;
use aes_gcm::aes::Aes256;

// aes-gcm = "0.10.3"

#[test]
fn test_3_aes256_cbc() {
    // 生成一个随机的 AES256-GCM 加密密钥 key。
    // The encryption key can be generated randomly:
    let key = Aes256Gcm::generate_key(OsRng);

    // 使用生成的密钥 key 创建 AES256-GCM 加密器 cipher，然后生成一个 96 位长度的随机 nonce nonce。
    // 使用 cipher 对 "plaintext message" 进行加密得到 ciphertext，然后再对 ciphertext 进行解密得到 plaintext。
    // 最后使用 assert_eq! 宏来验证解密后的 plaintext 是否与原始消息相同。
    let cipher: AesGcm<Aes256, U12> = Aes256Gcm::new(&key);
    // 生成随机 nonce（96 bits）
    // let nonce = GenericArray::from_slice(&Aes256Gcm::generate_nonce(&mut OsRng));
    // let nonce: &GenericArray<u8, <Aes256Gcm as Aead>::NonceSize> = GenericArray::from_slice(&Aes256Gcm::generate_nonce(&mut OsRng));
    // let nonce: GenericArray<u8, <Aes256Gcm as Aead>::NonceSize> = ...;
    let nonce: Nonce<Aes256Gcm> = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    // 将明文消息作为字节数组传递给 encrypt 函数。
    let plaintext_before: &[u8] = b"plaintext message";
    // let plaintextStr = "plaintext message".to_string();
    let ciphertext = cipher.encrypt(&nonce, plaintext_before).unwrap();
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();

    match String::from_utf8(plaintext.clone()) {
        Ok(string) => println!("Converted string: {}", string),
        Err(e) => println!("Conversion failed: {}", e),
    }

    assert_eq!(&plaintext, b"plaintext message");
}


