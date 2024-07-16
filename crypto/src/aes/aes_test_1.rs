// use aes::Aes256;
// use block_modes::{BlockMode, Cbc};
// use block_modes::block_padding::Pkcs7;
// use rand::seq::SliceRandom;
//
// // aes = "0.7.5"
// // block-modes = "0.8.1"
// // hex-literal = "0.2.1"
// // rand = "0.8.4"
// // bytebuffer = "0.2.1"
// // base64 = "0.13.0"
//
// type AesCbc = Cbc<Aes256, Pkcs7>;
//
// const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
//
// fn gen_ascii_chars(size: usize) -> String {
//     let mut rng = &mut rand::thread_rng();
//     String::from_utf8(
//         BASE_STR.as_bytes()
//             .choose_multiple(&mut rng, size)
//             .cloned()
//             .collect()
//     ).unwrap()
// }
//
// fn encrypt(key: &str, data: &str) -> String {
//     let iv_str = gen_ascii_chars(16);
//     let iv = iv_str.as_bytes();
//     let cipher = AesCbc::new_from_slices(key.as_bytes(), iv).unwrap();
//     let ciphertext = cipher.encrypt_vec(data.as_bytes());
//     let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv);
//     buffer.write_bytes(&ciphertext);
//     base64::encode(buffer.to_bytes())
// }
//
// fn decrypt(key: &str, data: &str) -> String {
//     let bytes = base64::decode(data).unwrap();
//     let cipher = AesCbc::new_from_slices(key.as_bytes(), &bytes[0..16]).unwrap();
//     String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
// }
//
//
// #[test]
// fn test_aes256_cbc() {
//     let plaintext = "hello world";
//     let key = "01234567012345670123456701234567";
//     let enc = encrypt(key, plaintext);
//     println!("{}", enc);
//     let dec = decrypt(key, &enc);
//     assert_eq!(plaintext, dec);
//     println!("{}", dec);
// }
//
