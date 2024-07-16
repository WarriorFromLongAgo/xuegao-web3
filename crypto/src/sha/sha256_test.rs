use sha3::{Digest, Sha3_256};

#[test]
fn test_sha_256_1() {
    let mut hasher = Sha3_256::new();
    hasher.update(b"abc");
    let hash: sha3::digest::Output<Sha3_256> = hasher.finalize();

    // 将哈希值转换为十六进制字符串
    let hash_hex = hex::encode(hash);

    println!("Hash (hex): {}", hash_hex);

    // 使用预期的SHA3-256哈希值进行断言
    assert_eq!(hash_hex, "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
}

