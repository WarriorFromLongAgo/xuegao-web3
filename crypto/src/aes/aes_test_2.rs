// use openssl::symm::{Cipher, Crypter, Mode};
// use openssl::rand::rand_bytes;
//
// // openssl = { version = "0.10", features = ["vendored"] }
//
// fn aes_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
//     // Check if key length is appropriate for AES-256
//     if key.len() != 32 {
//         return Err("AES-256 key must be 32 bytes long".to_string());
//     }
//
//     // Create encrypter instance
//     let cipher = Cipher::aes_256_cbc();
//     let mut encrypter = Crypter::new(cipher, Mode::Encrypt, key, Some(iv))
//         .map_err(|e| format!("Crypter creation failed: {:?}", e))?;
//
//     // Determine the output buffer size
//     let mut ciphertext = vec![0; plaintext.len() + cipher.block_size()];
//     let mut count = encrypter.update(plaintext, &mut ciphertext)
//         .map_err(|e| format!("Encryption error: {:?}", e))?;
//
//     count += encrypter.finalize(&mut ciphertext[count..])
//         .map_err(|e| format!("Finalization error: {:?}", e))?;
//
//     ciphertext.truncate(count);
//     Ok(ciphertext)
// }
//
// #[test]
// fn test_2_aes256_cbc() {
//     // Example usage
//     let key = &[0u8; 32]; // Replace with your actual key
//     let iv = &[0u8; 16]; // Replace with your actual IV
//     let plaintext = b"Hello, OpenSSL AES!";
//
//     // Encrypt
//     let encrypted = aes_encrypt(key, iv, plaintext).expect("Encryption failed");
//     println!("Encrypted: {:?}", encrypted);
// }
