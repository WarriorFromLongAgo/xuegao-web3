use ed25519_dalek::SigningKey;
use rand::RngCore;
use rand::rngs::OsRng;

#[test]
fn test_ed25519_1() {
    use ed25519::signature::{Signer, Verifier};
    use ed25519_dalek::{Signer as DalekSigner, Verifier as DalekVerifier, Signature as DalekSignature};

    pub struct HelloSigner<S>
    where
        S: Signer<ed25519::Signature>,
    {
        pub signing_key: S,
    }

    impl<S> HelloSigner<S>
    where
        S: Signer<ed25519::Signature>,
    {
        pub fn sign(&self, person: &str) -> ed25519::Signature {
            // NOTE: use `try_sign` if you'd like to be able to handle
            // errors from external signing services/devices (e.g. HSM/KMS)
            // <https://docs.rs/signature/latest/signature/trait.Signer.html#tymethod.try_sign>
            self.signing_key.sign(format_message(person).as_bytes())
        }
    }

    pub struct HelloVerifier<V> {
        pub verifying_key: V,
    }

    impl<V> HelloVerifier<V>
    where
        V: Verifier<ed25519::Signature>,
    {
        pub fn verify(
            &self,
            person: &str,
            signature: &ed25519::Signature,
        ) -> Result<(), ed25519::Error> {
            self.verifying_key.verify(format_message(person).as_bytes(), signature)
        }
    }

    fn format_message(person: &str) -> String {
        format!("Hello, {}!", person)
    }

    /// `HelloSigner` defined above instantiated with `ed25519-dalek` as
    /// the signing provider.
    pub type DalekHelloSigner = HelloSigner<ed25519_dalek::SigningKey>;

    // 生成随机密钥
    let mut rng = OsRng {};
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    let signing_key: SigningKey =  ed25519_dalek::SigningKey::from_bytes(&mut secret_bytes);
    let signer = DalekHelloSigner { signing_key };
    let person = "Joe"; // Message to sign
    let signature = signer.sign(person);

    /// `HelloVerifier` defined above instantiated with `ed25519-dalek`
    /// as the signature verification provider.
    pub type DalekHelloVerifier = HelloVerifier<ed25519_dalek::VerifyingKey>;

    let verifying_key: ed25519_dalek::VerifyingKey = signer.signing_key.verifying_key();
    let verifier = DalekHelloVerifier { verifying_key };
    assert!(verifier.verify(person, &signature).is_ok());
}


#[test]
fn test_ed25519_dalek_1() {
    use ed25519_dalek::{Signature, Signer, SigningKey};
    use rand::rngs::OsRng;
    use rand::RngCore;

    // 生成随机密钥
    let mut rng = OsRng {};
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);

    let signing_key: SigningKey = SigningKey::from_bytes(&mut secret_bytes);
    let verifying_key = signing_key.verifying_key();

    let good: &[u8] = "test message".as_bytes();
    let bad: &[u8] = "wrong message".as_bytes();
    let good_sig: Signature = signing_key.sign(good);
    let bad_sig: Signature = signing_key.sign(bad);

    assert!(!verifying_key.is_weak());

    assert!(
        signing_key.verify(good, &good_sig).is_ok(),
        "Verification of a valid signature failed!"
    );
    assert!(
        verifying_key.verify_strict(good, &good_sig).is_ok(),
        "Strict verification of a valid signature failed!"
    );
    assert!(
        signing_key.verify(good, &bad_sig).is_err(),
        "Verification of a signature on a different message passed!"
    );
    assert!(
        verifying_key.verify_strict(good, &bad_sig).is_err(),
        "Strict verification of a signature on a different message passed!"
    );
    assert!(
        signing_key.verify(bad, &good_sig).is_err(),
        "Verification of a signature on a different message passed!"
    );
    assert!(
        verifying_key.verify_strict(bad, &good_sig).is_err(),
        "Strict verification of a signature on a different message passed!"
    );
}

