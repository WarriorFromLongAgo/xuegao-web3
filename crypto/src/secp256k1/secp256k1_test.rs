#[test]
fn test_secp256k1_ecdsa_1() {
    use k256::ecdsa::{SigningKey, VerifyingKey};
    // 生成密钥对：

    // 生成一个新的签名密钥 (私钥)
    let signing_key = SigningKey::random(&mut rand::thread_rng());

    // 从签名密钥派生验证密钥 (公钥)
    let verifying_key = VerifyingKey::from(&signing_key);

    println!("私钥: {:?}", signing_key.to_bytes());
    // 打印私钥的十六进制表示
    println!("私钥: {}", hex::encode(&signing_key.to_bytes()));

    println!("公钥: {:?}", verifying_key.to_sec1_bytes());
    // 打印公钥的十六进制表示
    println!("公钥: {}", hex::encode(&verifying_key.to_sec1_bytes()));
}

#[test]
fn test_secp256k1_schnorr_1() {
    use k256::schnorr::{Signature, SigningKey};
    use k256::ecdsa::signature::{Signer, Verifier};
    use rand::rngs::OsRng;
    use hex;

    // 生成一个新的签名密钥 (私钥)
    let signing_key = SigningKey::random(&mut OsRng);


    // 从签名密钥派生验证密钥 (公钥)
    let verifying_key = signing_key.verifying_key();

    let message = b"Hello, world!";

    // 对消息进行签名
    let signature: Signature = signing_key.sign(message);

    // 验证签名
    assert!(verifying_key.verify(message, &signature).is_ok());

    // 打印消息内容
    println!("消息: {:?}", std::str::from_utf8(message).unwrap());

    // 打印签名内容（以十六进制格式）
    println!("签名: {}", hex::encode(signature.to_bytes()));

    println!("签名验证成功!");
}

#[test]
fn test_secp256k1_sign_1() {
    use k256::ecdsa::{Signature, signature::Verifier, SigningKey, VerifyingKey};
    use k256::ecdsa::signature::Signer;
    use rand::rngs::OsRng;
    // 签名和验证消息：

    // 生成一个新的签名密钥 (私钥)
    let signing_key = SigningKey::random(&mut OsRng);

    // 从签名密钥派生验证密钥 (公钥)
    let verifying_key = VerifyingKey::from(&signing_key);

    let message: &[u8] = b"Hello, world!";

    // 对消息进行签名
    let signature: Signature = signing_key.sign(message);

    // 验证签名
    assert!(verifying_key.verify(message, &signature).is_ok());

    println!("消息: {:?}", std::str::from_utf8(message).unwrap());
    // 打印签名内容
    let signature_bytes = signature.to_bytes();
    println!("签名: {:?}", hex::encode(signature_bytes));
    println!("签名验证成功!");
}

