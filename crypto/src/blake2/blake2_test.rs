use blake2::{Blake2b512, Blake2s256, Digest};
use blake2::digest::Output;

#[test]
fn test_blake2_1() {
    // create a Blake2b512 object
    let mut hasher = Blake2b512::new();

    // write input message
    hasher.update(b"hello world");

    // read hash digest and consume hasher
    let hash: Output<Blake2b512> = hasher.finalize();

    // 将哈希值转换为十六进制字符串
    let hash_hex = hex::encode(hash);

    println!("Hash (hex): {}", hash_hex);

    // Expected hash value for "hello world"
    let expected_hash_hex = "021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0";

    // Assert that the calculated hash matches the expected hash
    assert_eq!(hash_hex, expected_hash_hex);
}

#[test]
fn test_blake2_2() {
    // same example for Blake2s256:
    let mut hasher = Blake2s256::new();
    hasher.update(b"hello world");
    let hash = hasher.finalize();
    // 将哈希值转换为十六进制字符串
    let hash_hex = hex::encode(hash);
    assert_eq!(hash_hex, "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b");

    // Hex-encode hash using https://docs.rs/base16ct
    let hex_hash = base16ct::lower::encode_string(&hash);
    assert_eq!(hex_hash, "9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b");
}
